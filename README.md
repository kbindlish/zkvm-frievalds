### Freivalds' Algorithm with Zero-Knowledge Proofs

This Rust project implements Freivalds' algorithm within a zero-knowledge proof framework using the arkworks ecosystem. It verifies matrix multiplication correctness without revealing private data.

A and B are private witnesses.

C is a public input. (Assumption: It's part of the instance and thus included in the transcript)

### Build and Run

`cargo build`

`cargo run`

`cargo test`


### Quadratic Non-Interactive Matrix Multiplication Verification

In this assignment, we want you to implement [Freivalds' Algorithm](https://en.wikipedia.org/wiki/Freivalds%27_algorithm) for checking the correctness of a matrix multiplication in quadratic (rather than the naive cubic) time. However we want you to implement this as an arithmetic circuit that could be proven non-interactively (e.g., using a SNARK), by way of `arkworks` implementation of the R1CS constraint system. Luckily, `arkworks` provides a circuit builder interface (a [`ConstraintSystem`](https://docs.rs/ark-relations/latest/ark_relations/r1cs/struct.ConstraintSystem.html)) that allows you to do this using high level idioms in Rust, without having to manipulate any circuit gates or wires directly. 

Concretely, your task is to fill out the `builder` function in [`freivalds.rs`](src/freivalds.rs), such that the [`main`](src/main.rs) function does not have any failing assertions when invoked using `cargo run`.[^1]

#### Fiat-Shamir

Since Freivalds' Algorithm requires random challenges, your circuit will need to rely upon the [Fiat-Shamir transform](https://www.zkdocs.com/docs/zkdocs/protocol-primitives/fiat-shamir/) to generate (pseudo)random challenges in a verifiable way. To assist you, the template for the task in [`freivalds.rs`](src/freivalds.rs) includes the configuration of a circuit-based random oracle using the Poseidon hash function. The [`CryptographicSpongeVar` trait](https://github.com/arkworks-rs/crypto-primitives/blob/main/crypto-primitives/src/sponge/constraints/mod.rs#L106-L196) covers what methods are available to absorb (input) data into the hash and squeeze (output) data from it. 

#### The Interface

The `builder` function has interface 

```rust
pub fn builder<F: PrimeField, const N: usize>(k: u32, a: &[[u32; N]; N], b: &[[u32; N]; N], c: &[[u32; N]; N]) -> Result<ConstraintSystemRef<F>, SynthesisError>;
```

Here `F` is the underlying [finite field](https://github.com/arkworks-rs/algebra/blob/master/ff/src/fields/prime.rs#L27-L96) the circuit will be defined over, `N` is the size of the matricies being multiplied (you may assume `N >= 2`), and `k` is the security parameter that defines how many iterations of the randomized test must occur. The test itself is to check whether `ab = c`. To turn integers into field elements you have `F::from`, while to create new inputs into the circuit you can use [`ark_r1cs_std::alloc::AllocVar`](https://docs.rs/ark-r1cs-std/latest/ark_r1cs_std/alloc/trait.AllocVar.html) to make new [`ark_r1cs_std::fields::fp::FpVar`](https://docs.rs/ark-r1cs-std/latest/ark_r1cs_std/fields/fp/enum.FpVar.html) that correspond to circuit wires. You can then implement your circuit over these `FpVar`s directly in Rust. 

Your implementation _must_ treat `a` and `b` as private inputs to the circuit, and `c` as a public input.

If you are unfamiliar with `arkworks` or Rust more generally, you may find their [tutorial](https://github.com/arkworks-rs/r1cs-tutorial) helpful. Also, although it's quite a bit more complex than the Freivalds' circuit we ask you to write here, you may also find our [HyperNova sumcheck verifier circuit implementation](https://github.com/nexus-xyz/nexus-zkvm/blob/main/nova/src/gadgets/cyclefold/hypernova/mod.rs#L82-L200) a helpful example of how to use the random oracle and specify arithmetic in the circuit using the builder APIs.

#### How Long Should This Take Me?

That likely depends on your familiarity and comfort with all of `arkworks`, Rust, circuit writing, and cryptography more generally. But the reference solution we have internally is 63 LoC (beyond what is provided in the template project), as written in a not particular compact style in well under an hour. We expect it may take your longer, especially if you are unfamiliar with the tooling, but we do not expect this assignment should take a significant amount of time.

#### Two Last Things...

We would appreciate you not publicly sharing your solution in a discoverable way. 

We would love to see some of your own code which demonstrates your engineering skills, and that you are able and willing to share. So we are not so much looking for something that involves cryptography or circuits or zkVMs (though that is not a negative either), but are more interested in something that demonstrates your ability to write clean, legible, and well-tested code, whatever the language and application. So along with your solution to this task, please send back (a link to where we can find) some of your code that you'd like us to see.

[^1]: If you see a debugging print statement starting with `Constraint trace requires enabling...` you may safely ignore it.
