fn main() {
    let ans = is_even(200000000000000000000000000);
    println!("{}", is_even(2));
}
fn is_even(num: u128) -> bool {
    if (num % 2 == 0) {
        return true;
    }

    return false;
}
