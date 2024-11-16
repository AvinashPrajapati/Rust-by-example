fn main() {
//     // enums can also be generic.
//  enum Result1<SuccessCode, FailureCode> {
//  Success(SuccessCode),
//  Failure(FailureCode, char),
//  Uncertainty,
//  }
//  let mut _res = Result1::Success::<u32,u16>(12u32);
//  _res = Result1::Uncertainty;
//  _res = Result1::Failure(0u16, 'd');


//  in case of vectors
let mut v = vec![11, 22, 33];
 for _ in 0..5 {
 let item: Option<i32> = v.pop();
 match item {
 Some(number) => print!("{}, ", number),
 None => print!("#, "),
 }
 }

}


/*
In Rust, Option is an enum in the standard library
enum Option<T> {
    Some(T),
    None,
}
// Some(T): enum variants of Option.
-------------------
The definition of this generic enum in the standard library is:
 enum Result<T, E> {
 Ok(T),
 Err(E),
 }
//Ok and Err are variants of the Result enum
*/