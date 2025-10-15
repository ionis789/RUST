// --- Problem 1: Next Prime Number ---

// A simple function to check if a number is prime.
fn is_prime(n: u16) -> bool {
    if n <= 1 {
        return false;
    }
    // A simple loop is enough for a u16.
    for i in 2..=(n / 2) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

// Finds the next prime number after x.
// Returns None if the next prime would overflow u16.
fn next_prime(x: u16) -> Option<u16> {
    let mut num = x;
    while num < u16::MAX {
        num += 1;
        if is_prime(num) {
            return Some(num);
        }
    }
    None
}

// --- Problem 2: Addition and Multiplication with Panic ---

// Adds two u32 numbers, panics on overflow.
fn checked_add_panic(a: u32, b: u32) -> u32 {
    a.checked_add(b).expect("Addition overflow!")
}

// Multiplies two u32 numbers, panics on overflow.
fn checked_mul_panic(a: u32, b: u32) -> u32 {
    a.checked_mul(b).expect("Multiplication overflow!")
}

// --- Problems 3 & 4: Addition and Multiplication with Result ---

// A custom error for arithmetic operations.
#[derive(Debug)]
enum ArithmeticError {
    Overflow,
}

// Adds two u32 numbers and returns a Result.
fn checked_add_result(a: u32, b: u32) -> Result<u32, ArithmeticError> {
    match a.checked_add(b) {
        Some(val) => Ok(val),
        None => Err(ArithmeticError::Overflow),
    }
}

// Multiplies two u32 numbers and returns a Result.
fn checked_mul_result(a: u32, b: u32) -> Result<u32, ArithmeticError> {
    match a.checked_mul(b) {
        Some(val) => Ok(val),
        None => Err(ArithmeticError::Overflow),
    }
}

// This function uses the above operations and propagates errors.
// It calculates (a + b) * c
fn use_arithmetic_functions(a: u32, b: u32, c: u32) -> Result<u32, ArithmeticError> {
    let sum = checked_add_result(a, b)?;
    let product = checked_mul_result(sum, c)?;
    Ok(product)
}


// --- Problem 5: Character Processing with Result ---

// A custom error for character processing.
#[derive(Debug, PartialEq)]
enum CharError {
    NotAscii,
    NotADigit,
    NotHexDigit,
    NotALetter,
    NotPrintable,
}

fn to_uppercase(c: char) -> Result<char, CharError> {
    if !c.is_alphabetic() {
        Err(CharError::NotALetter)
    } else {
        Ok(c.to_ascii_uppercase())
    }
}

fn to_lowercase(c: char) -> Result<char, CharError> {
    if !c.is_alphabetic() {
        Err(CharError::NotALetter)
    } else {
        Ok(c.to_ascii_lowercase())
    }
}

fn print_char(c: char) -> Result<(), CharError> {
    if c.is_control() {
        Err(CharError::NotPrintable)
    } else {
        println!("Printed char: '{}'", c);
        Ok(())
    }
}

fn char_to_number(c: char) -> Result<u32, CharError> {
    if !c.is_ascii() {
        return Err(CharError::NotAscii);
    }
    if !c.is_ascii_digit() {
        return Err(CharError::NotADigit);
    }
    // .unwrap() is safe here because we already checked.
    Ok(c.to_digit(10).unwrap())
}

fn char_to_number_hex(c: char) -> Result<u32, CharError> {
    if !c.is_ascii() {
        return Err(CharError::NotAscii);
    }
    if !c.is_ascii_hexdigit() {
        return Err(CharError::NotHexDigit);
    }
    Ok(c.to_digit(16).unwrap())
}

// Prints a user-friendly message for a given CharError.
fn print_char_error(e: &CharError) {
    match e {
        CharError::NotAscii => println!("Error: Character is not ASCII."),
        CharError::NotADigit => println!("Error: Character is not a digit (0-9)."),
        CharError::NotHexDigit => println!("Error: Character is not a hex digit (0-9, a-f)."),
        CharError::NotALetter => println!("Error: Character is not a letter."),
        CharError::NotPrintable => println!("Error: Character is not printable."),
    }
}

// --- Problem 6: Custom Application with Option ---

// Parses a "key=value" string.
// Returns a tuple (&str, &str) if the format is correct, otherwise None.
fn parse_key_value(s: &str) -> Option<(&str, &str)> {
    match s.split_once('=') {
        Some((key, value)) => {
            if value.is_empty() { // Don't allow empty values like "key="
                None
            } else {
                Some((key, value))
            }
        }
        None => None, // No '=' character found
    }
}


fn main() {
    println!("--- Problem 1: Next Prime Number ---");
    let mut current_num = 0u16;
    loop {
        match next_prime(current_num) {
            Some(prime) => {
                print!("{} ", prime);
                current_num = prime;
            }
            None => {
                println!("\nNo more prime numbers available in a u16.");
                break;
            }
        }
    }
    println!("\n");

    println!("--- Problem 2: Arithmetic with Panic ---");
    // Success case
    println!("Addition (success): 100 + 200 = {}", checked_add_panic(100, 200));
    println!("Multiplication (success): 100 * 200 = {}", checked_mul_panic(100, 200));
    
    // Fail case. Uncommenting these lines will cause a panic.
    println!("To test panic, uncomment the lines in the main function.");
    // checked_add_panic(u32::MAX, 1);
    // checked_mul_panic(u32::MAX, 2);
    println!();


    println!("--- Problems 3 & 4: Arithmetic with Result ---");
    // Success case
    match use_arithmetic_functions(10, 20, 5) {
        Ok(val) => println!("Calculation (success): (10 + 20) * 5 = {}", val),
        Err(e) => println!("Unexpected error: {:?}", e),
    }

    // Fail case
    match use_arithmetic_functions(u32::MAX, 1, 1) {
        Ok(val) => println!("Unexpected success value: {}", val),
        Err(e) => println!("Calculation (fail): (u32::MAX + 1) * 1 -> Error: {:?}", e),
    }
    println!();

    println!("--- Problem 5: Character Processing ---");
    let test_chars = ['a', 'B', '7', 'f', '$', '\n', 'ș'];
    for &c in &test_chars {
        println!("-- Testing char '{}' --", c.escape_default());

        match to_uppercase(c) {
            Ok(upper) => println!("to_uppercase: Success -> '{}'", upper),
            Err(e) => {
                print!("to_uppercase: Fail -> ");
                print_char_error(&e);
            }
        }
        match char_to_number(c) {
            Ok(num) => println!("char_to_number: Success -> {}", num),
            Err(e) => {
                print!("char_to_number: Fail -> ");
                print_char_error(&e);
            }
        }
    }
     println!();

    println!("--- Problem 6: App with Option (key=value parser) ---");
    let valid_string = "user=admin";
    let invalid_string1 = "no_equals";
    let invalid_string2 = "key=";

    match parse_key_value(valid_string) {
        Some((key, value)) => println!("'{}' -> Success: key='{}', value='{}'", valid_string, key, value),
        None => println!("'{}' -> Unexpected failure.", valid_string)
    }

    match parse_key_value(invalid_string1) {
        Some((key, value)) => println!("'{}' -> Unexpected success: key='{}', value='{}'", invalid_string1, key, value),
        None => println!("'{}' -> Failure as expected.", invalid_string1)
    }

    match parse_key_value(invalid_string2) {
        Some((key, value)) => println!("'{}' -> Unexpected success: key='{}', value='{}'", invalid_string2, key, value),
        None => println!("'{}' -> Failure as expected.", invalid_string2)
    }
}


