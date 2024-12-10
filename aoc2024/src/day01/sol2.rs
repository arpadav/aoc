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
    let (mut vec_a, map_b) = INPUT
        .lines()
        .into_iter()
        .map(|line| {
            let (a, b) = line.split_once("   ").unwrap();
            let (a, b) = (a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap());
            (a, b)
        })
        .fold((
            Vec::new(),
            std::collections::HashMap::new(),
        ), |(mut vec_a, mut map_b), (a, b)| {
            vec_a.push(a);
            *map_b.entry(b).or_insert(0) += 1;
            (vec_a, map_b)
        });
    // --------------------------------------------------
    // sort
    // --------------------------------------------------
    vec_a.sort();
    // --------------------------------------------------
    // loop through vec_b, count the number of consecutive
    // instances of each number
    // --------------------------------------------------
    let simscore = vec_a
        .iter()
        .filter(|n| map_b.contains_key(n))
        .fold(0, |acc, n| acc + n * map_b.get(n).unwrap());
    println!("solution 01: {}", simscore);
}