fn main() {
//     let mut x = vec!["This", "is", "a", "sentence"];
//  x.insert(1, "line");
//  x.insert(2, "contains");
//  x.remove(3);
//  x.push("about Rust");
//  x.pop();
//  for i in 0..x.len() { print!("{} ", x[i]); }
 
 // --------empty array and dynamic array
//  let x = ["'"; 5];
 
//  let x = vec![true; 0];
//  for i in 0..x.len() { print!("{} ", x[i]); }
//  ---  sortcut to print the array
// print!("{:?} {:?}", [1, 2, 3], vec![4, 5]);
// --- copy array
// let mut a1 = vec![4, 56, -2];
//  let a2 = vec![7, 81];
//  print!("{:?} ", a1);
//  a1 = a2;
//  print!("{:?}", a1);
//  -----
let my_string = "100000000000000000000000".to_string();  // `parse()` works with `&str` and `String`!
    let my_int = my_string.parse::<u128>().unwrap();
    let my_hex = format!("{:X}", my_int);
    println!("{}", my_hex);

}
