use super::INPUT;

pub fn solve() {
    let start_time = std::time::Instant::now();
    solution_01();
    println!("time: {} us", start_time.elapsed().as_micros());
}

fn solution_01() {
    // --------------------------------------------------
    // get input
    // --------------------------------------------------
    let (mut vec_a, mut vec_b) = INPUT
        .lines()
        .into_iter()
        .map(|line| {
            let (a, b) = line.split_once("   ").unwrap();
            let (a, b) = (a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap());
            (a, b)
        })
        .fold((Vec::new(), Vec::new()), |(mut vec_a, mut vec_b), (a, b)| {
            vec_a.push(a);
            vec_b.push(b);
            (vec_a, vec_b)
        });
    // --------------------------------------------------
    // sort
    // --------------------------------------------------
    vec_a.sort();
    vec_b.sort();
    // --------------------------------------------------
    // accumulate diff
    // --------------------------------------------------
    let diff = vec_a
        .iter()
        .zip(vec_b.iter())
        .fold(0, |acc, (a, b)| acc + b.abs_diff(*a));
    println!("solution 01: {}", diff);
}