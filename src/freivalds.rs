use ark_crypto_primitives::sponge::{
    constraints::{CryptographicSpongeVar, SpongeWithGadget},
    poseidon::{find_poseidon_ark_and_mds, PoseidonConfig, PoseidonSponge},
};
use ark_ff::PrimeField;
use ark_r1cs_std::{alloc::AllocVar, eq::EqGadget, fields::fp::FpVar};
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

    /*** BUILD CIRCUIT HERE ***/

    cs.finalize();
    Ok(cs)
}
