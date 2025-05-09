// test.rs
fn main() {
  // E0282 が出る例
  println!("{:?} {a} {a:?}", [], a = 1 + 1);
}
