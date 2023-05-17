fn power(num: isize, pow: usize) -> isize {
    if pow == 0 {
        1
    } else {
        num * power(num, pow - 1)
    }
}
fn main() {
    let num = -3;
    println!("{} cube is {}", num, power(num, 3));
}
