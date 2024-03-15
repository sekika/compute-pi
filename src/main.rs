use compute_pi::compute_pi;
use std::env;

/// The main function of the program. It parses the command-line arguments
/// to determine the number of decimal places of pi to calculate and then
/// prints the calculated value to the standard output.
///
/// # Arguments
///
/// This function expects a single command-line argument:
///
/// * The first argument should be the number of decimal places of pi to calculate.
///
/// # Panics
///
/// This function will panic if the command-line arguments are not provided
/// as expected or if the provided argument cannot be parsed into a `usize`.
///
/// # Examples
///
/// Run the program from the command line:
///
/// ```shell
/// compute-pi 100
/// ```
///
/// This will calculate and print the value of pi to 100 decimal places.
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <digits>", args[0]);
        return;
    }
    let digits: usize = args[1].parse().expect("Please provide a valid number of digits.");
    let pi = compute_pi(digits);
    let pi_str = pi.to_string_radix(10, Some(digits + 5));
    let pi_str_trimmed = &pi_str[0..(digits + 2)];
    println!("Pi to {} decimal places: {}", digits, pi_str_trimmed);
}
