fn fib(n: i64) -> i64 {
    match n {
        0 => 0,
        1 => 1,
        2 => 1,
        _ => fib(n - 1) + fib(n - 2),
    }
}

fn fast_fib(n: i64) -> i64 {
    if n <= 1 {
        return n;
    };

    let mut fib1: i64 = 1;
    let mut fib2: i64 = 1;
    let mut current_n = 2;
    loop {
        if current_n == n {
            return fib2;
        }
        current_n = current_n + 1;
        let tmp = fib2;
        fib2 = fib1 + fib2;
        fib1 = tmp;
    }
}

fn super_fast_fib(n: i64) -> i64 {
    matrix_pow((1, 1, 1, 0), n - 1).2
}
type Matrix = (i64, i64, i64, i64);

fn matrix_pow(m: Matrix, power: i64) -> Matrix {
    if power == 0 {
        return (1, 0, 1, 0);
    }
    if power % 2 == 0 {
        return matrix_pow(matrix_multi(m, m), power / 2);
    } else {
        return matrix_multi(m, matrix_pow(matrix_multi(m, m), power / 2));
    }
}

fn matrix_multi(a: Matrix, b: Matrix) -> Matrix {
    (
        a.0 * b.0 + a.2 * b.1,
        a.1 * b.0 + a.3 * b.1,
        a.0 * b.2 + a.2 * b.3,
        a.1 * b.2 + a.3 * b.3,
    )
}

fn main() {
    println!("input number :");
    let input_number: i64 = {
        let mut buf = String::new();
        loop {
            std::io::stdin().read_line(&mut buf).expect("error");
            match buf.trim().parse() {
                Ok(res) => break res,
                Err(_) => continue,
            };
        }
    };
    // println!("fib({}) = {} by fib", input_number, fib(input_number));
    // println!(
    //     "fib({}) = {} by fast_fib",
    //     input_number,
    //     fast_fib(input_number)
    // );
    println!(
        "fib({}) = {} by super_fast_fib",
        input_number,
        super_fast_fib(input_number)
    );
}
