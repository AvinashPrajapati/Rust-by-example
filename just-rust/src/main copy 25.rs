// Input/Output and Error Handling
fn main() {
    // // Proper Runtime Error Handling

    // example code
    fn f1(x: i32) -> Result<i32, String> {
        if x == 1 {
            Err(format!("Err. 1"))
        } else {
            Ok(x)
        }
    }
    fn f2(x: i32) -> Result<i32, String> {
        if x == 2 {
            Err(format!("Err. 2"))
        } else {
            Ok(x)
        }
    }
    fn f3(x: i32) -> Result<i32, String> {
        if x == 3 {
            Err(format!("Err. 3"))
        } else {
            Ok(x)
        }
    }
    fn f4(x: i32) -> Result<i32, String> {
        if x == 4 {
            Err(format!("Err. 4"))
        } else {
            Ok(x)
        }
    }
    // // use of the above funtions
    // fn f(x: i32) -> Result<i32, String> {
    //     match f1(x) {
    //         Ok(result) => match f2(result) {
    //             Ok(result) => match f3(result) {
    //                 Ok(result) => f4(result),
    //                 Err(err_msg) => Err(err_msg),
    //             },
    //             Err(err_msg) => Err(err_msg),
    //         },
    //         Err(err_msg) => Err(err_msg),
    //     }
    // }
    // // now use of f()
    // match f(2) {
    //     Ok(y) => println!("{}", y),
    //     Err(e) => println!("Error: {}", e),
    // }
    // match f(4) {
    //     Ok(y) => println!("{}", y),
    //     Err(e) => println!("Error: {}", e),
    // }
    // match f(5) {
    //     Ok(y) => println!("{}", y),
    //     Err(e) => println!("Error: {}", e),
    // }

    // // but a better way to handle these
    // fn f(x: i32) -> Result<i32, String> {
    //     let result1 = f1(x);
    //     if result1.is_err() {
    //         return result1;
    //     }
    //     let result2 = f2(result1.unwrap());
    //     if result2.is_err() {
    //         return result2;
    //     }
    //     let result3 = f3(result2.unwrap());
    //     if result3.is_err() {
    //         return result3;
    //     }
    //     f4(result3.unwrap())
    // }

    // match f(2) {
    //     Ok(y) => println!("{}", y),
    //     Err(e) => println!("Error: {}", e),
    // }
    // match f(4) {
    //     Ok(y) => println!("{}", y),
    //     Err(e) => println!("Error: {}", e),
    // }
    // match f(5) {
    //     Ok(y) => println!("{}", y),
    //     Err(e) => println!("Error: {}", e),
    // }

    // damm so compact
    fn f(x: i32) -> Result<i32, String> {
        f4(f3(f2(f1(x)?)?)?)
    }
    match f(2) {
        Ok(y) => println!("{}", y),
        Err(e) => println!("Error: {}", e),
    }
    match f(4) {
        Ok(y) => println!("{}", y),
        Err(e) => println!("Error: {}", e),
    }
    match f(5) {
        Ok(y) => println!("{}", y),
        Err(e) => println!("Error: {}", e),
    }
}
