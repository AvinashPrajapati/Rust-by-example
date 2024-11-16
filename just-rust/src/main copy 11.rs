fn main() {
    // fn divide(numerator: f64, denominator: f64) -> Result<f64, String> {
    //     if denominator == 0. {
    //         Err(format!("Divide by zero"))
    //     } else {
    //         Ok(numerator / denominator)
    //     }
    // }
    // let r1 = divide(8., 2.);
    // let r2 = divide(8., 0.);
    // println!("{} {}", r1.is_ok(), r2.is_ok());
    // println!("{} {}", r1.is_err(), r2.is_err());
    // println!("{}", r1.unwrap());
    // println!("{}", r2.unwrap());  // ERROR : called `Result::unwrap()` on an `Err` value: "Divide by zero"

    // ---------- another example
    let mut v = vec![11, 22, 33];
    for _ in 0..v.len() {    // just change v.len()  to number greater than lenth og the v vector
        print!("{}, ", v.pop().unwrap())
    }

    // The unwrap function is much used in quick-and-dirty Rust programs, where possible errors are not required to be handled in a user-friendly way.
}
