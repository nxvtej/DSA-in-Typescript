fn main() {
    let ans = is_even(87653468343546546847354580000000000000);
    println!("{}", ans);
    println!("{}", fibo(3));
    println!("{}", is_even(2));
}
fn is_even(num: u128) -> bool {
    if num % 2 == 0 {
        return true;
    }
    return false;
}

const zero: i32 = 0;
const one: i32 = 1;

fn fibo(n: u128) -> u128 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        let mut nth_term = 0;
        nth_term = fibo(n - 1) + fibo(n - 2);
        print!("{} ", nth_term);
        return nth_term;
    }
}
