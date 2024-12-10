use super::INPUT;

pub fn solve() {
    let start_time = std::time::Instant::now();
    solution_01();
    println!("time: {} us", start_time.elapsed().as_micros());
}

enum Ordering {
    Uninit,
    Ascending,
    Descending,
}

fn solution_01() {
    // --------------------------------------------------
    // iter
    // --------------------------------------------------
    let result = INPUT
        .lines()
        .into_iter()
        .fold(0, |acc, line| {
            let nums = line.split(" ").map(|s| s.parse().unwrap()).collect::<Vec<isize>>();
            let mut ord = Ordering::Uninit;
            // --------------------------------------------------
            // comp 2 numbers, if diff is 0 or > 3, return
            // otherwise, if oscillated, return
            // otherwise, is good! accumulate by 1
            // --------------------------------------------------
            for (idx, b) in nums.iter().skip(1).enumerate() {
                match nums[idx] - *b {
                    0 => return acc,
                    -3..=-1 => match ord {
                        Ordering::Uninit => ord = Ordering::Descending,
                        Ordering::Ascending => return acc,
                        Ordering::Descending => (),
                    },
                    1..=3 => match ord {
                        Ordering::Uninit => ord = Ordering::Ascending,
                        Ordering::Descending => return acc,
                        Ordering::Ascending => (),
                    },
                    _ => return acc,
                }
            }
            return acc + 1;
        });
    println!("solution 01: {}", result);
}