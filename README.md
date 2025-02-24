
# Lambert W Function in Rust 🦀

This project implements the Lambert W function in Rust. The Lambert W function is a mathematical function that is used to solve equations of the form `f(x) = x * exp(x)`.

## Description

The Lambert W function is a transcendental function that is used in many areas of mathematics and physics, including calculus, differential equations, and number theory. It is defined as the inverse function of the function `f(x) = x * exp(x)`, and it has two branches: the principal branch `W_0` and the secondary branch `W_{-1}`.


## Usage

To use this project, you can run the following command:

```bash
cargo run --release -- <input_value>
```

Replace `<input_value>` with the value you want to pass to the Lambert W function.

## Tests

This project includes several tests to ensure that the Lambert W function is working correctly. You can run the tests using the following command:

```bash
cargo test
```

