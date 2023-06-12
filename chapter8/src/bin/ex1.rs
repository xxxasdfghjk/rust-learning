struct Statistics {
    median: f64,
    mean: f64,
}

fn statistics(vec: &mut Vec<i32>) -> Statistics {
    let mut clone_vec = vec.clone();
    clone_vec.sort();
    let mut sum = 0;
    for elem in &clone_vec {
        sum += elem;
    }
    let mean = (sum as f64) / (clone_vec.len() as f64);
    let median = clone_vec.get(clone_vec.len() / 2).unwrap();
    let median = *median as f64;
    Statistics { median, mean }
}

fn main() {
    let mut vec = vec![100, 100, 100, 100, 1, 0, 100];
    let statistic = statistics(&mut vec);
    println!("mean:{}\nmedian:{}", statistic.mean, statistic.median);
}
