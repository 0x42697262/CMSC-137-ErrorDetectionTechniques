use std::io;
use std::io::Write;

mod simple_parity_check;
mod two_dimensional_parity_check;

/// The main entry point of the program.
///
/// This function displays a menu for choosing an error detection technique, reads the user's choice,
/// and then calls the corresponding function to execute the selected technique.
///
/// To be used alongside with `user_interface()` function
fn main() {
    println!("CMSC 137 Laboratory Exercise 6 — Error Detection Techniques");
    println!("");
    println!("Choose technique to perform (1-4):");
    println!("[1] Simple Parity Check");
    println!("[2] Two-Dimensional Parity Check");
    println!("[3] Checksum");
    println!("[4] Cyclic Redundancy Check");
    println!("[0] Exit");
    print!("Choice: ");
    io::stdout().flush().expect("Failed to flush stdout.");
    user_interface();
}

/// Handles user interaction for selecting an error detection technique.
///
/// This function reads the user's choice from the standard input, displays a menu of available techniques,
/// and dispatches the corresponding function based on the user's choice. If an invalid choice is entered,
/// it prints a farewell message and exits gracefully.
///
/// # Usage
///
/// To use this function, ensure that each valid choice corresponds to a function implementing a specific
/// error detection technique, and the user is prompted to input their choice.
///
/// # Examples
///
/// ```
/// // Example usage of the user_interface function:
/// user_interface();
/// ```
fn user_interface() {
    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read choice input!");

    match choice.trim() {
        "1" => simple_parity_check_ui(),
        "2" => two_dimensional_parity_check_ui(),
        "3" => checksum_ui(),
        "4" => cyclic_redundancy_check_ui(),
        _ => {
            println!("Good bye!");
        }
    }
}
/// Implements the user interface for the Simple Parity Check technique.
///
/// This function guides the user to input the original K-bit and the received N-bit, calculates the codeword,
/// and compares it to determine if the data is valid or not.
fn simple_parity_check_ui() {
    // Start taking user inputs for Input A and Input B (original k-bit and received n-bit)
    let mut byte_input;

    byte_input = user_input_handler(String::from("Input A: "));
    let dataword: u8 = match u8::from_str_radix(byte_input.trim(), 2) {
        Ok(byte) => byte,
        Err(_) => {
            eprintln!("Please enter a valid 8-bit integer.");
            std::process::exit(1);
        }
    };

    byte_input = user_input_handler(String::from("Input B: "));
    let received_codeword: u16 = match u16::from_str_radix(byte_input.trim(), 2) {
        Ok(byte) => byte,
        Err(_) => {
            eprintln!("Please enter a valid 8-bit integer.");
            std::process::exit(1);
        }
    };

    let codeword: u16 = simple_parity_check::compute_even_parity_bit(dataword);

    println!("@Sender");
    print!("Codeword: {:b}", codeword);
    println!("");
    println!("@Receiver");
    print!("Data word: ");
    // Compare received n-bit to the codeword
    if simple_parity_check::check_syndrome(received_codeword) == 0 {
        println!("Accepted");
    } else {
        println!("Discarded");
    }
}

fn two_dimensional_parity_check_ui() {}

fn checksum_ui() {}
fn cyclic_redundancy_check_ui() {}

/// Reads and handles user input, displaying a specified text prompt.
///
/// This function prints the provided prompt, reads the user's input, and returns the input as a string.
///
/// # Arguments
///
/// * `text` - The prompt to be displayed to the user.
///
/// # Returns
///
/// * A string containing the user's input.
fn user_input_handler(text: String) -> String {
    let mut input = String::new();
    print!("{text}");
    io::stdout().flush().expect("Failed to flush stdout.");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input bytes!");

    input
}
