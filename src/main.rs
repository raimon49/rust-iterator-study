fn triangle(n: i32) -> i32 {
    let mut sum = 0;
    for i in 1..n+1 {
        sum += i;
    }

    sum
}
fn main() {
    triangle(4);
}
