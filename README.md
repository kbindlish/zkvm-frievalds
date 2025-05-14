### Freivalds' Algorithm with Zero-Knowledge Proofs

This Rust project implements Freivalds' algorithm within a zero-knowledge proof framework using the arkworks ecosystem. It verifies matrix multiplication correctness without revealing private data.

A and B are private witnesses.

C is a public input. (Assumption: It's part of the instance and thus included in the transcript)

### Build and Run

`cargo build`

`cargo run`

`cargo test`


### Quadratic Non-Interactive Matrix Multiplication Verification

[Freivalds' Algorithm](https://en.wikipedia.org/wiki/Freivalds%27_algorithm) for checking the correctness of a matrix multiplication in quadratic (rather than the naive cubic) time. This is implemneted as an arithmetic circuit that could be proven non-interactively (e.g., using a SNARK), by way of `arkworks` implementation of the R1CS constraint system.
`arkworks` provides a circuit builder interface (a [`ConstraintSystem`](https://docs.rs/ark-relations/latest/ark_relations/r1cs/struct.ConstraintSystem.html)) that allows using high level idioms in Rust, without having to manipulate any circuit gates or wires directly. 

#### Fiat-Shamir

Since Freivalds' Algorithm requires random challenges, the circuit relies upon the [Fiat-Shamir transform](https://www.zkdocs.com/docs/zkdocs/protocol-primitives/fiat-shamir/) to generate (pseudo)random challenges in a verifiable way.

#### The Interface

The `builder` function has interface 

```rust
pub fn builder<F: PrimeField, const N: usize>(k: u32, a: &[[u32; N]; N], b: &[[u32; N]; N], c: &[[u32; N]; N]) -> Result<ConstraintSystemRef<F>, SynthesisError>;
```

`F` is the underlying [finite field](https://github.com/arkworks-rs/algebra/blob/master/ff/src/fields/prime.rs#L27-L96) the circuit will be defined over, 
`N` is the size of the matricies being multiplied (you may assume `N >= 2`),
`k` is the security parameter that defines how many iterations of the randomized test must occur. 

The test is to check whether `ab = c`.  
`a` and `b` as private inputs to the circuit, and `c` as a public input.


