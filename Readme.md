# compute-pi

This crate provides a function to compute the value of Pi to a specified number of digits using the Gauss-Legendre algorithm.

## Installation

You can include this crate in your `Cargo.toml` file as follows:

```toml
[dependencies]
compute-pi = "0.1.2"
```

## Usage

To use the `compute_pi` function in your Rust code, add the following to your crate root:

```rust
use rug::Float;
use compute_pi::compute_pi;

fn main() {
    // Specify the number of digits of Pi you want to compute
    let digits = 10;

    // Compute Pi
    let pi = compute_pi(digits);

    // Print the result
    println!("Pi to {} decimal places: {}", digits, pi.to_string_radix(10, Some(digits + 2)));
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

This crate is licensed under the MIT license. See the [LICENSE](LICENSE) file for details.
