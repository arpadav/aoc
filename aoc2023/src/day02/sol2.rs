#![allow(non_snake_case)]
use super::INPUT;

pub fn solve() {
    // --------------------------------------------------
    // solution 1
    // --------------------------------------------------
    let start = std::time::Instant::now();
    let solution = solution_1();
    let duration = start.elapsed();
    println!("Solution 1: {}", solution);
    println!("Solution 1 time elapsed: {:?}", duration);
    println!();
}

fn solution_1() -> usize {
    let mut game = Game::new();
    INPUT
        .lines()
        .fold(0, |acc, line| acc + game.calculate_power(line.as_bytes()) )
}

struct Game {
    max_red: usize,
    max_blue: usize,
    max_green: usize,
}

impl Game {
    fn new() -> Self {
        Self {
            max_red: 0,
            max_blue: 0,
            max_green: 0,
        }
    }

    fn power(&self) -> usize {
        self.max_red * self.max_blue * self.max_green
    }

    fn reset(&mut self) {
        self.max_red = 0;
        self.max_blue = 0;
        self.max_green = 0;
    }

    fn calculate_power(&mut self, input: &[u8]) -> usize {
        self.reset();
        let mut idx = 0;
        let mut temp_num: usize = 0;
        let mut flag__look_for_color = false;
        let mut flag__game_started = false;

        // iterate through input
        while idx < input.len() {
            match input[idx] {
                b'\n' => break,
                b':' => {
                    flag__game_started = true;
                    idx += 2;
                    continue;
                },
                c => {
                    if !flag__game_started { idx += 1; continue; }
                    match c {
                        b'0'..=b'9' => {
                            temp_num = (c - b'0') as usize;
                            idx += 1;
                            while let Some(num) = is_digit(&input[idx]) {
                                temp_num = (temp_num << 3) + (temp_num << 1) + num as usize;
                                idx += 1;
                            }
                            flag__look_for_color = true;
                            idx += 1;
                            continue;
                        },
                        b'r' | b'b' | b'g' => {
                            if !flag__look_for_color {
                                idx += 1;
                                continue;
                            }
                            flag__look_for_color = false;
                            match c {
                                b'r' => {
                                    if temp_num > self.max_red { self.max_red = temp_num; }
                                }
                                b'b' => {
                                    if temp_num > self.max_blue { self.max_blue = temp_num; }
                                }
                                b'g' => {
                                    if temp_num > self.max_green { self.max_green = temp_num; }
                                }
                                _ => {},
                            }
                            // smallest jump is for 'red' (skip "ed, ", next char is number or EOL)
                            idx += 5;
                            continue;
                        },
                        _ => {},
                    }
                    idx += 1;
                    continue;
                },
            }
        }
        self.power()
    }
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
fn is_digit(c: &u8) -> Option<u8> {
    // --------------------------------------------------
    // ASSUMPTION: `c`` is a single ASCII character
    // --------------------------------------------------
    if c >> 4 != 0b0000_0011 { return None; }
    let last_four = c & 0b0000_1111;
    if last_four > 0b0000_1010 { return None; }
    Some(last_four)
}