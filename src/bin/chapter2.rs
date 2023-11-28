// cargo run --bin chapter2
fn main() {
    let n = 10;
    let sum = for_l(n);
    println!("for loop sum {}", sum);

    let sum = for_nested(n);
    println!("for nested loop sum {}", sum);

    let sum = while_l(n);
    println!("while loop sum {}", sum);

    let sum = recursive(n);
    println!("recursive sum {}", sum);

    let sum = tail_recursive(n);
    println!("tail recursive sum {}", sum);

    for i in 1..=n {
        println!("fib({}) = {}", i, fib(i));
    }
}

fn for_l(n: i32) -> i32 {
    let mut sum = 0;
    for i in 0..=n {
        sum += i;
    }
    sum
}

fn for_nested(n: i32) -> i32 {
    let mut sum = 0;
    for i in 0..=n {
        for j in 0..=n {
            sum += i + j;
        }
    }
    sum
}

fn while_l(n: i32) -> i32 {
    let mut i = 0;
    let mut sum = 0;
    while i <= n {
        sum += i;
        i += 1;
    }
    sum
}

fn recursive(n: i32) -> i32 {
    if n == 0 {
        return 0;
    }
    n + recursive(n - 1)
}

fn tail_recursive(n: i32) -> i32 {
    fn helper(n: i32, acc: i32) -> i32 {
        if n == 0 {
            return acc;
        }
        helper(n - 1, acc + n)
    }
    helper(n, 0)
}

fn fib(n: i32) -> i32 {
    if n <= 2 {
        return n - 1;
    }
    fib(n - 1) + fib(n - 2)
}
