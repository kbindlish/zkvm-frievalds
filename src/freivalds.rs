use ark_crypto_primitives::sponge::{
    constraints::{CryptographicSpongeVar, SpongeWithGadget},
    poseidon::{find_poseidon_ark_and_mds, PoseidonConfig, PoseidonSponge},
};
use ark_ff::PrimeField;
use ark_r1cs_std::{
    alloc::AllocVar,
    eq::EqGadget,
    fields::{fp::FpVar, FieldVar},
};
use ark_relations::r1cs::{ConstraintSystem, ConstraintSystemRef, SynthesisError, SynthesisMode};

/// Returns poseidon config for sponge.
fn poseidon_setup<F: PrimeField>() -> PoseidonConfig<F> {
    const FULL_ROUNDS: usize = 8;
    const PARTIAL_ROUNDS: usize = 57;
    const ALPHA: u64 = 5;
    const RATE: usize = 2;
    const CAPACITY: usize = 1;

    let (ark, mds) = find_poseidon_ark_and_mds::<F>(
        F::MODULUS_BIT_SIZE as u64,
        RATE,
        FULL_ROUNDS as u64,
        PARTIAL_ROUNDS as u64,
        0,
    );

    PoseidonConfig {
        full_rounds: FULL_ROUNDS,
        partial_rounds: PARTIAL_ROUNDS,
        alpha: ALPHA,
        ark,
        mds,
        rate: RATE,
        capacity: CAPACITY,
    }
}

pub fn builder<F: PrimeField, const N: usize>(
    k: u32,
    a: &[[u32; N]; N],
    b: &[[u32; N]; N],
    c: &[[u32; N]; N],
) -> Result<ConstraintSystemRef<F>, SynthesisError> {
    let cs = ConstraintSystem::new_ref();
    cs.set_mode(SynthesisMode::Prove {
        construct_matrices: true,
    });

    let ro_config = poseidon_setup::<F>();
    let mut ro = <PoseidonSponge<F> as SpongeWithGadget<F>>::Var::new(cs.clone(), &ro_config);

    /*** THE CIRCUIT ***/
    // A is a private witness
    let a_vars: Vec<Vec<FpVar<F>>> = a
        .iter()
        .map(|row| {
            row.iter()
                .map(|&val| FpVar::new_witness(cs.clone(), || Ok(F::from(val))))
                .collect::<Result<_, _>>()
        })
        .collect::<Result<_, _>>()?;

    // B is a private witness
    let b_vars: Vec<Vec<FpVar<F>>> = b
        .iter()
        .map(|row| {
            row.iter()
                .map(|&val| FpVar::new_witness(cs.clone(), || Ok(F::from(val))))
                .collect::<Result<_, _>>()
        })
        .collect::<Result<_, _>>()?;

    // C is a public input
    let c_vars: Vec<Vec<FpVar<F>>> = c
        .iter()
        .map(|row| {
            row.iter()
                .map(|&val| FpVar::new_input(cs.clone(), || Ok(F::from(val))))
                .collect::<Result<_, _>>()
        })
        .collect::<Result<_, _>>()?;

    // Absorb A and B into the sponge as they are controlled by the prover
    // C is a public input and we are absorbing it assuming that it's a part of the instance
    // hence, under the control of the prover
    for matrix_vars in [&a_vars, &b_vars, &c_vars].iter() {
        for row in matrix_vars.iter() {
            for val in row.iter() {
                ro.absorb(val)?;
            }
        }
    }

    // Perform k iterations of Freivalds' algorithm
    for _ in 0..k {
        // Generate random vector r based on the transcript
        let r: Vec<FpVar<F>> = (0..N)
            .map(|_| {
                ro.squeeze_field_elements(1)
                    .and_then(|mut v| v.pop().ok_or(SynthesisError::AssignmentMissing))
            })
            .collect::<Result<Vec<_>, _>>()?;

        // Compute B * r
        let mut br: Vec<FpVar<F>> = vec![FpVar::<F>::zero(); N];
        for i in 0..N {
            for j in 0..N {
                br[i] += &b_vars[i][j] * &r[j];
            }
        }

        // Compute A * (B * r)
        let mut abr: Vec<FpVar<F>> = vec![FpVar::<F>::zero(); N];
        for i in 0..N {
            for j in 0..N {
                abr[i] += &a_vars[i][j] * &br[j];
            }
        }

        // Compute C * r
        let mut cr: Vec<FpVar<F>> = vec![FpVar::<F>::zero(); N];
        for i in 0..N {
            for j in 0..N {
                cr[i] += &c_vars[i][j] * &r[j];
            }
        }

        // Make sure that A * (B * r) = C * r
        for i in 0..N {
            abr[i].enforce_equal(&cr[i])?;
        }
    }

    cs.finalize();
    Ok(cs)
}

#[cfg(test)]
mod tests {
    use super::*;
    type F = <ark_bn254::g1::Config as ark_ec::models::CurveConfig>::BaseField;

    #[test]
    fn test_builder_valid_matrices() {
        let k = 1;
        const N: usize = 2;

        let a = [[1, 2], [3, 4]];
        let b = [[5, 6], [7, 8]];

        let c = [[19, 22], [43, 50]];

        let cs = builder::<F, N>(k, &a, &b, &c).unwrap();
        assert!(cs.is_satisfied().unwrap());
    }

    #[test]
    fn test_builder_invalid_matrices() {
        let k = 1;
        const N: usize = 2;
        let a = [[1, 2], [3, 4]];
        let b = [[5, 6], [7, 8]];

        // Invalid matrix
        let c = [[20, 22], [43, 50]];

        let cs = builder::<F, N>(k, &a, &b, &c).unwrap();
        assert!(!cs.is_satisfied().unwrap());
    }

    #[test]
    fn test_builder_different_k() {
        let k = 3;
        const N: usize = 2;
        let a = [[1, 2], [3, 4]];
        let b = [[5, 6], [7, 8]];

        let c = [[19, 22], [43, 50]];

        let cs = builder::<F, N>(k, &a, &b, &c).unwrap();
        assert!(cs.is_satisfied().unwrap());
    }

    #[test]
    fn test_builder_zero_matrices() {
        let k = 1;
        const N: usize = 2;

        let a = [[1, 2], [3, 4]];
        let b = [[0, 0], [0, 0]];

        let c = [[0, 0], [0, 0]];

        let cs = builder::<F, N>(k, &a, &b, &c).unwrap();
        assert!(cs.is_satisfied().unwrap());
    }

    #[test]
    fn test_builder_identity_matrices() {
        let k = 1;
        const N: usize = 2;

        let a = [[1, 2], [3, 4]];
        let b = [[1, 0], [0, 1]];

        let c = [[1, 2], [3, 4]];

        let cs = builder::<F, N>(k, &a, &b, &c).unwrap();
        assert!(cs.is_satisfied().unwrap());
    }

    #[test]
    fn test_builder_single_element_matrices() {
        let k = 1;
        const N: usize = 1;

        let a = [[1]];
        let b = [[2]];

        let c = [[2]];

        let cs = builder::<F, N>(k, &a, &b, &c).unwrap();
        assert!(cs.is_satisfied().unwrap());
    }
}
