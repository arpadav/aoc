// ---------------------------------------------------------------------------------------------
// external
// ---------------------------------------------------------------------------------------------
use std::include_str;

// ---------------------------------------------------------------------------------------------
// constants
// ---------------------------------------------------------------------------------------------
const INPUT: &str = include_str!("../data/input.txt");

/// Advent of Code 2023
/// Day 1, Part 1
/// 
/// # Summary
/// 
/// See: [https://adventofcode.com/2023/day/1](https://adventofcode.com/2023/day/1)
/// 
/// The problem simply put:
/// - input is a text file of strings, delimited by newlines
/// - there are numbers in the strings, and each line has a 10's digit and a 1's digit to
///  represent a two digit number
/// - the 10's digit is always the first digit in the string
/// - the 1's digit is always the last digit in the string
/// - if the string only has 1 number (always will), both 10's and 1's digits are the same
/// - sum all the two digit numbers
fn main() {
    // ---------------------------------------------------------------------------------------------
    // pre-processing
    // - collect input as lines
    // - to pre-allocate memory for sum, based off length
    // ---------------------------------------------------------------------------------------------
    let input = INPUT.lines().collect::<Vec<&str>>();
    let mut line_sum: Vec<u8> = vec![0; input.len()];
    // ---------------------------------------------------------------------------------------------
    // start timer
    // ---------------------------------------------------------------------------------------------
    let start = std::time::Instant::now();
    // ---------------------------------------------------------------------------------------------
    // iterate through input (line by line)
    // ---------------------------------------------------------------------------------------------
    input
    .iter()
    .enumerate()
    .for_each(|(i, line)| {
    // ---------------------------------------------------------------------------------------------
    // get line as bytes
    // ---------------------------------------------------------------------------------------------
        let line_bytes = line.as_bytes();
    // ---------------------------------------------------------------------------------------------
    // loop through characters forward
    // if character is a digit, add it to sum and exit the loop
    // ---------------------------------------------------------------------------------------------
        for c in line_bytes {
            match custom_is_digit(c) {
                None => continue,
                Some(d) => {
                    store_upper_nibble(&mut line_sum[i], d);
                    break;
                },
            }
        }
    // ---------------------------------------------------------------------------------------------
    // loop through characters backward
    // if character is a digit, add it to sum and exit the loop
    // ---------------------------------------------------------------------------------------------
        for c in line_bytes.iter().rev() {
            match custom_is_digit(c) {
                None => continue,
                Some(d) => {
                    store_lower_nibble(&mut line_sum[i], d);
                    break;
                },
            }
        }
    });
    // ---------------------------------------------------------------------------------------------
    // collect/sum values (no overflow, cast higher)
    // ---------------------------------------------------------------------------------------------
    let sum: usize =
    line_sum
    .iter()
    .map(|&n| n as usize)
    .sum();
    // ---------------------------------------------------------------------------------------------
    // end timer and display results
    // ---------------------------------------------------------------------------------------------
    let duration = start.elapsed();
    println!("sum: {}", sum);
    println!("time elapsed: {:?}", duration);
}

/// Checks if the given character is a digit.
/// 
/// # Arguments
/// 
/// * `c` - A reference to a `u8` character.
/// 
/// # Returns
/// 
/// Returns `Some(u8)` if the character is a digit, otherwise `None`.
fn custom_is_digit(c: &u8) -> Option<u8> {
    // ---------------------------------------------------------------------------------------------
    // ASSUMPTION: `c`` is a single ASCII character
    // ---------------------------------------------------------------------------------------------
    if c >> 4 != 0b0000_0011 { return None; }
    let last_four = c & 0b0000_1111;
    if last_four > 0b0000_1010 { return None; }
    Some(last_four)
}

/// Stores the lower nibble of the given value `d` into the mutable reference `store`.
/// This is the same as multiplying by 1, and is used to store the 1's digit.
///
/// # Arguments
///
/// * `store` - A mutable reference to a `u8` value where the lower nibble will be stored.
/// * `d` - The value whose lower nibble will be stored.
///
/// # Example
///
/// ```
/// let mut store: u8 = 0;
/// store_lower_nibble(&mut store, 5);
/// assert_eq!(store, 5);
/// ```
fn store_lower_nibble(store: &mut u8, d: u8) {
    *store += d;
}

/// Stores the upper nibble of a byte in the given mutable reference.
/// This is the same as multiplying by 10, and is used to store the 10's digit.
///
/// # Arguments
///
/// * `store` - A mutable reference to the byte where the upper nibble will be stored.
/// * `d` - The byte from which the upper nibble will be extracted.
///
/// # Example
///
/// ```
/// let mut byte = 0b000_0100;
/// store_upper_nibble(&mut byte, 0b0000_0001);
/// assert_eq!(byte, 0b0000_1110);
/// ```
fn store_upper_nibble(store: &mut u8, d: u8) {
    *store += d << 3;
    *store += d << 1;
}