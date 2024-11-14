use num_bigint::BigUint;
// use num_traits::One;
use num_traits::{Zero, One};

#[allow(dead_code)]
fn fib(n: usize) -> BigUint {
    // let mut f0 = BigUint::ZERO;
    let mut f0 = BigUint::zero();
    let mut f1 = BigUint::one();
    for _ in 0..n {
        let f2 = &f0 + &f1;
        f0 = f1;
        f1 = f2;
    }
    f0
}

fn main() {
    // This is a very large number.
    // println!("fib(1000) = {}", fib(100));
    // -----
    // let one_thousand = 1e3;
    // println!("{}",one_thousand)
    // ----
//     let a: i8 = 5;
//  let b: i16 = 5;
//  let c: i32 = 5;
//  let d: i64 = 5;
//  print!("{} {} {} {}", a, b, c, d); //but
//  --------
// let a1: i8 = 5;
// let b1: i16 = 5;
// print!("{}", a1 + b1);
// -------
let arr:[i8; 3] = [11, 22, 33];
 let i: usize = 2;  // usize and isize and Actually, only the usize type is allowed as an index of an array/vector.
 print!("{}", arr[i]);
}
