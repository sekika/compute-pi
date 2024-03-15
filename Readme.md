# compute-pi

This crate provides a function to compute the value of Pi to a specified number of digits using the Gauss-Legendre algorithm.

## Installation

You can include this crate in your `Cargo.toml` file as follows:

```toml
[dependencies]
compute-pi = "0.2.1"
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

This crate is licensed under the MIT license. See the [LICENSE](LICENSE) file for details.
