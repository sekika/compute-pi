# compute-pi

Compute-pi calculates the value of pi to an arbitrary number of digits using the [Gauss–Legendre algorithm](https://en.wikipedia.org/wiki/Gauss%E2%80%93Legendre_algorithm). It computes 1 million digits within a couple of seconds on your PC.

## Performance

On a MacBook Air (Apple M1, 16 GB), pi to 1 million digits was computed in 1.5 seconds, and to 320 million digits in 24 minutes, but the calculation did not complete within 10 hours for 330 million digits. Similarly, on a Mac mini (Apple M1, 16 GB), pi to 320 million digits was computed, but the calculation did not complete within 10 hours for 330 million digits. It is presumed that the calculation is taking a long time due to memory swapping, as it does not end in a panic due to memory allocation failure but instead continues indefinitely. Since both machines yielded the same result, it is considered that 320 million digits is the maximum number of digits that can be computed using compute-pi with 16 GB of memory. Another limitation is that the digit cannot exceed 1,292,913,983 due to the precision of [rug::Float](https://docs.rs/rug/latest/rug/struct.Float.html) being defined as u32. The computed result of pi to 320 million digits using compute-pi was confirmed to match exactly with the results obtained using [y-cruncher](http://www.numberworld.org/y-cruncher/) and the Chudnovsky algorithm.

## Installation

You can include this crate in your `Cargo.toml` file as follows:

```toml
[dependencies]
compute-pi = "0.2"
```

## Usage

To use the `compute_pi_str` function in your Rust code, add the following to your crate root:

```rust
use compute_pi::compute_pi_str;

fn main() {
    // Specify the number of digits of Pi you want to compute
    let digits = 100;

    // Compute Pi
    let pi = compute_pi_str(digits);

    // Print calculated decimal
    println!("Pi to {} decimal places: {}", digits, pi);
}
```

## Command Line Usage

You can also use the `compute-pi` command from the command line. After installing the crate with
```bash
cargo install compute-pi
```
run the following command:

```bash
compute-pi <digits>
```

Replace `<digits>` with the number of digits of Pi you want to compute. For example:

```bash
compute-pi 100
```

This will print the value of Pi to 100 decimal places.

## License

This crate is licensed under the [MIT license](https://en.wikipedia.org/wiki/MIT_License).
