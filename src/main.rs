fn triangle(n: i32) -> i32 {
    let mut sum = 0;
    for i in 1..n+1 {
        sum += i;
    }

    sum
}
fn main() {
    triangle(4);

    println!("There is:");
    let v = vec!["antimony", "arsenic", "alumium", "selenium"];
    for element in &v {
        println!("{}", element);
    }
    // 上記のループは以下と同等
    let mut iterator = (&v).into_iter();
    while let Some(element) = iterator.next() {
        // Some(element)が返されたらループボディ部を実行するがNoneが返されたら終了する
        println!("{}", element);
    }
}
