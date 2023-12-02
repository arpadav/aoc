use std::include_str;

fn main() {
    const INPUT: &str = include_str!("../data/input.txt");
    let input = INPUT.lines().collect::<Vec<&str>>();

    // start timer
    let start = std::time::Instant::now();

    let mut line_sum: Vec<u8> = vec![0; input.len()];
    input
    .iter()
    .enumerate()
    .for_each(|(i, line)| {
        let line_bytes = line.as_bytes();
        // loop through characters forward
        // if character is a digit, add it to sum and exit the loop
        for c in line_bytes {
            // if c is not a digit, return
            match custom_is_digit(c) {
                None => continue,
                Some(d) => {
                    store_upper_nibble(&mut line_sum[i], d);
                    break;
                },
            }
        }
        // loop through characters backward
        // if character is a digit, add it to sum and exit the loop
        for c in line_bytes.iter().rev() {
            // if c is not a digit, return
            match custom_is_digit(c) {
                None => continue,
                Some(d) => {
                    store_lower_nibble(&mut line_sum[i], d);
                    break;
                },
            }
        }
    });

    // collect/sum values with no overflow
    let sum: u64 =
    line_sum
    .iter()
    .map(|&n| n as u64)
    .sum();

    // end timer
    let duration = start.elapsed();

    println!("sum: {}", sum);
    println!("time elapsed: {:?}", duration);

    ()

}

fn custom_is_digit(c: &u8) -> Option<u8> {
    // assume c is a single ascii character
    // return None if c is not a digit
    // return Some(d) if c is a digit, where d is the digit
    if c >> 4 != 0b0000_0011 {
        return None;
    }
    let last_four = c & 0b0000_1111;
    if last_four > 0b0000_1010 {
        return None;
    }
    Some(last_four)
}

fn store_lower_nibble(store: &mut u8, d: u8) {
    // add d
    // this is the same as multiplying by 1, so 1's digit
    *store += d;
}

fn store_upper_nibble(store: &mut u8, d: u8) {
    // add d << 3 and d << 1 to store
    // this is the same as multiplying by 8 and 2, so 10's digit
    *store += d << 3;
    *store += d << 1;
}
