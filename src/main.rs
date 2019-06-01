/*
 * Rustの所有権（ムーブ）。
 * CreatedAt: 2019-06-01
 */
fn main() {
    let a = String::from("Hello Ownership !!"); // ヒープ領域の確保
    let b = a; // ムーブ（ヒープ領域の所有権がポインタ変数`a`から`b`へ移った。所有できるのは必ず1つのポインタ変数のみ）
    println!("b = {}", b);
//    println!("a = {}", a); // error[E0382]: use of moved value: `a`
}

