fn main() {
    // USE THE RESULT TYPE TO HANDLE ERRORS

    // Rust provides the Result<T, E> enum for returning and propagating errors. By convention, the Ok(T) variant represents a success and contains a value, and the variant Err(E) represents and error and contains an error value.

    // The Result<T, E> enum is defined as:
    // enum Result<T, E> {
    //     Ok(T): // A value T was obtained
    //     Err(E): // An error of type E was encountered instead
    // }

    // In contrast to the Option type, which describes the possibility of the absence of a value, the Result type is best suited whenever failures are expected.

    // The Result type also has the unwrap and expect methods, which do either of the following:
    // Return the value inside the Ok variant, if this is the case.
    // Cause the program to panic, if the variant is an Err.

    // In the following example code, there's an implementation for a safe_division function that returns either of the following:
    // A Result value with an Ok variant that carries the result of a successful division.
    // An Err variant that carries a struct DivisionByZeroError which signals an unsuccessful division.

    println!("{:?}", safe_division(9.0, 3.0));
    println!("{:?}", safe_division(4.0, 0.0));
    println!("{:?}", safe_division(0.0, 2.0));

    // Example:

    assert!(read_file_contents(PathBuf::from("src/main.rs")).is_ok());
    assert!(read_file_contents(PathBuf::from("non-existent-file.txt")).is_err());


}

#[derive(Debug)]
struct DivisionByZeroError;

fn safe_division(dividend: f64, divisor: f64) -> Result<f64, DivisionByZeroError> {
    if divisor == 0.0 {
        Err(DivisionByZeroError)
    } else {
        Ok(dividend / divisor)
    }
}

// The #[derive(Debug)] part that precedes the DivisionByZeroError struct is a macro that tells the Rust compiler to make the type printable for debugging purposes. This will be covered more later.

///////////////////////////////////////////////////////////////////////////////////////

use std::fs::File;
use std::io::{Error as IoError, Read};
use std::path::PathBuf;

fn read_file_contents(path: PathBuf) -> Result<String, IoError> {
    let mut string = String::new();

    // TODO #1: Handle this match expression.
    // --------------------------------------
    // Pass the variable to the `file` variable on success, or
    // Return from the function early if it is an error.
    let mut file: File = match File::open(path) {
        Ok(file_handle) => file_handle,
        Err(io_error) => return Err(io_error),
    };

    // TODO #2: Handle this error.
    // ---------------------------
    // The success path is already filled in for you.
    // Return from the function early if it is an error.
    match file.read_to_string(&mut string) {
        Ok(_) => (),
        Err(io_error) => return Err(io_error),
    };

    // TODO #3: Return the `string` variable as expected by this function signature.
    Ok(string)
}
