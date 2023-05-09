use rand::Rng;
use std::vec;

fn main() {
    const MAX_NUMBER_COUNT: usize = 100;
    let mut vec: Vec<i32> = Vec::new();
    let mut rng = rand::thread_rng();
    for _ in 0..MAX_NUMBER_COUNT {
        vec.push(rng.gen_range(0..100000));
    }
    println!("before_sort");
    vec_print(&vec);
    println!("sorted!");
    vec_merge_sort(&mut vec);
    vec_print(&vec);
}

fn vec_print(vec: &Vec<i32>) {
    for elem in vec {
        println!("{}", elem);
    }
}

fn vec_sort(vec: &mut Vec<i32>) {
    for i in 0..vec.len() {
        for j in 0..(vec.len() - 1 - i) {
            if vec[j] >= vec[j + 1] {
                let tmp = vec[j];
                vec[j] = vec[j + 1];
                vec[j + 1] = tmp;
            }
        }
    }
}

fn vec_merge_sort(vec: &mut Vec<i32>) {
    vec_merge_sort_recursive(&mut vec[..])
}

fn vec_merge_sort_recursive(vec: &mut [i32]) {
    if vec.len() <= 1 {
        return;
    };
    let size = vec.len();
    let half = vec.len() / 2;
    vec_merge_sort_recursive(&mut vec[0..half]);
    vec_merge_sort_recursive(&mut vec[half..size]);
    let mut former_cur = 0;
    let mut latter_cur = half;
    let mut current_index = 0;
    let mut tmp_array = vec![0; size];
    loop {
        if former_cur == half && latter_cur == size {
            break;
        }
        let value = if former_cur < half && latter_cur < size {
            if vec[former_cur] > vec[latter_cur] {
                latter_cur += 1;
                vec[latter_cur - 1]
            } else {
                former_cur += 1;
                vec[former_cur - 1]
            }
        } else if former_cur < half {
            former_cur += 1;
            vec[former_cur - 1]
        } else {
            latter_cur += 1;
            vec[latter_cur - 1]
        };
        tmp_array[current_index] = value;
        current_index += 1;
    }
    for i in 0..size {
        vec[i] = tmp_array[i];
    }
}
