# On-Chain Verifiable Computation Engine on Polkadot
This project provides a hyper-optimized, comprehensive library of mathematical, geometric, and cryptographic functions, executed as a smart contract on the Polkadot Asset Hub. By leveraging the power of Rust and PolkaVM, it offers a verifiable, high-performance computation layer for other smart contracts and dApps, unlocking capabilities that are either impossible or prohibitively expensive in native Solidity.

## Overview
The core of this project is a Rust smart contract that serves as an on-chain math co-processor. It contains a rich library of functions ranging from basic vector math and projectile physics to advanced number theory and elliptic curve cryptography.

To make these powerful functions accessible to the broader EVM ecosystem, a suite of lightweight Solidity wrapper contracts is deployed. These wrappers act as a bridge, allowing any EVM-compatible dApp or contract to call the Rust functions through a clean, familiar interface.

## Architecture
The project follows a three-layer architecture:

The Computation Layer (Rust Contract): A single, highly optimized Rust contract compiled to Wasm for PolkaVM. It contains the core logic for all mathematical operations and is deployed once to the Polkadot Asset Hub.

The Interface Layer (Solidity Wrappers): A set of small, modular Solidity contracts (RustPhysics, RustGeometry, RustNumberTheory, etc.). Each wrapper handles the ABI encoding for a specific category of functions and uses a low-level staticcall to execute the logic in the main Rust contract.

The Application Layer (Your dApp): Your decentralized application interacts with the familiar Solidity wrapper contracts using standard libraries like ethers.js or web3.js.
```
graph TD
    A[dApp Frontend <br> (ethers.js)] -->|Calls function| B(Solidity Wrapper <br> e.g., RustPhysics.sol);
    B -->|staticcall with ABI-encoded data| C{Polkadot Asset Hub};
    C -->|Executes logic in PolkaVM| D[Core Rust Contract <br> (Computation Engine)];
    D -->|Returns result| C;
    C -->|Returns data| B;
    B -->|Decodes result| A;
```
## Key Benefits
1. Leveraging Polkadot Asset Hub & PolkaVM
By deploying to the Polkadot Asset Hub, this computation engine becomes a shared, chain-level utility. Any other parachain or smart contract within the Polkadot ecosystem can call it, making it a powerful, interoperable resource. PolkaVM, a Wasm-based virtual machine, is designed for high-performance execution, running the compiled Rust code at near-native speeds. This is a fundamental departure from the EVM's design and is the key to the system's efficiency.

2. Drastic Gas Cost Reduction
Running complex math in the EVM is notoriously expensive. Every loop iteration, every multiplication, every storage access costs a significant amount of gas.

This project bypasses that limitation. A dApp makes a single, relatively cheap staticcall to the wrapper contract. All the heavy, iterative computation (like primality testing, trajectory simulation, or modular exponentiation) occurs within the highly efficient PolkaVM. The gas cost is primarily for the single call and data transfer, not for the millions of computational steps that might be happening inside the Rust VM. This can result in a 100x to 1000x+ reduction in gas fees for computationally intensive tasks compared to a pure-Solidity implementation.

3. Beyond Solidity's Capabilities
This architecture doesn't just make existing operations cheaper; it makes entirely new ones possible. Many of the functions in this library would be impossible to implement in native Solidity due to a combination of gas limits, stack depth limits, and missing native types.

## Capabilities Unlocked:

Native i64 and i128 Math: Solidity lacks native support for signed 64-bit integers and the complex fixed-point arithmetic they enable, which is crucial for physics simulations like projectile_y_at_time.

Complex Looping: The Miller-Rabin is_prime test involves multiple complex loops. Attempting this in Solidity for a u64 would instantly exceed the block gas limit. In Rust/PolkaVM, it's trivial.

Advanced Cryptography: Functions like point_add and point_double for elliptic curves are foundational for many zero-knowledge proof systems and advanced signature schemes, but are too complex and costly for the EVM.

No Stack Depth Limits: Recursive or deeply nested logic like the extended_gcd is safe in Rust, whereas it would quickly hit a "stack too deep" error in Solidity.

Guaranteed Determinism: Because the logic is on-chain, every dApp and user is guaranteed that the same inputs will produce the exact same physics, randomness (from a shared seed), or geometric result, which is critical for fair, turn-based games.

## Available Function Libraries
The engine is split into logical libraries, each accessible through its own Solidity wrapper:

- Core Math (RustMathCore): Basic arithmetic, trig functions, square roots, etc.

- Vectors (RustVectors): Vector addition/subtraction, dot/cross products, magnitude, normalization.

- Geometry (RustGeometry): Distance calculations, point-in-shape tests (circle, rect, triangle).

- Physics (RustPhysics): Full projectile trajectory calculation.

- Number Theory (RustNumberTheory): modexp, modinv, is_prime, gcd, lcm, phi, etc.

- Bitwise (RustBitwise): popcount, log2, bit rotations, carry-less multiplication (clmul).

- ECC (RustECC): Elliptic curve point addition and doubling.

**This project represents a paradigm shift—moving from slow, expensive on-chain computation to fast, cheap, and verifiable on-chain execution, paving the way for a new generation of more powerful and complex decentralized applications.**