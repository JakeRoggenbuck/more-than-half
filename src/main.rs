use statistical_tests_rs::{mean, population_standard_deviation};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::time::{Duration, Instant};

fn find_more_than_half_naive_hashmap_approach(nums: &Vec<i32>) -> i32 {
    let mut counts = HashMap::<i32, i32>::new();

    for x in nums {
        if counts.contains_key(&x) {
            counts.entry(*x).and_modify(|k| *k += 1);
        } else {
            counts.insert(*x, 1);
        }
    }

    let mut max = 0;
    let mut max_key = 1;

    for (k, v) in counts {
        if v > max {
            max = v;
            max_key = k;
        }
    }

    return max_key;
}

fn find_more_than_half_array_index_approach(nums: &Vec<i32>) -> i32 {
    let mut counts = [0; 100];

    for x in nums {
        counts[*x as usize] += 1;
    }

    let mut max = 0;
    let mut max_index = 0;

    for (n, k) in counts.iter().enumerate() {
        if k > &max {
            max = *k;
            max_index = n;
        }
    }

    return max_index as i32;
}

fn find_more_than_half_array_index_approach_iter_max(nums: &Vec<i32>) -> i32 {
    let mut counts = [0; 100];

    for x in nums {
        counts[*x as usize] += 1;
    }

    let index = counts
        .iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.cmp(b))
        .map(|(index, _)| index);

    return index.unwrap() as i32;
}

fn bench_func_once<T>(op: &dyn Fn(&Vec<i32>) -> T, reps: i64, nums: &Vec<i32>) -> Duration {
    let now = Instant::now();

    for _ in 0..reps {
        op(&nums);
    }

    now.elapsed()
}

fn bench_func<T>(op: &dyn Fn(&Vec<i32>) -> T, reps: i64, title: &str, nums: Vec<i32>) {
    let mut scores: [f64; 30] = [0.0; 30];

    for i in 1..30 {
        scores[i as usize] = bench_func_once(op, reps, &nums).as_nanos() as f64;
        print!(".");
        io::stdout().flush().unwrap();
    }

    println!(
        " {}:\tμ: {} nanos, σ: {}, n: {}, reps: {}",
        title,
        mean(&scores),
        population_standard_deviation(&scores),
        30,
        reps
    );
}

fn main() {
    let file = File::open("out.txt").expect("Should have opened out.txt");
    let mut reader = BufReader::new(file);

    let mut first_line = String::new();
    reader
        .read_line(&mut first_line)
        .expect("Unable to read line");

    let mut split = first_line.split(";");
    let size = split.next().unwrap().parse::<usize>().unwrap();

    let mut nums = Vec::with_capacity(size);

    for (n, line) in reader.lines().enumerate() {
        if n == 0 {
            continue;
        }

        let line = line.unwrap();
        let num = line.parse::<i32>().unwrap();
        nums.push(num);
    }

    bench_func(
        &find_more_than_half_naive_hashmap_approach,
        100,
        "find_more_than_half_naive_hashmap_approach",
        nums.clone(),
    );

    bench_func(
        &find_more_than_half_array_index_approach,
        100,
        "find_more_than_half_array_index_approach",
        nums.clone(),
    );

    bench_func(
        &find_more_than_half_array_index_approach_iter_max,
        100,
        "find_more_than_half_array_index_approach_iter_max",
        nums,
    );
}
