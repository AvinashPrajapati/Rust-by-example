/*
function definition
*/
// fn f() {
//     print!("1");
// }

fn main() {
    // fn line() {
    //     println!("----------");
    //     }
    //     line();
    //     line();
    //     line();

    // -------- will run
    // {
    //     fn f() { print!("a"); }
    //     f(); f();
    // }
    // {
    //     fn f() { print!("b"); }
    //     f();
    // }

    // ---------
    // f(); // Prints 2
    // {
    //     f(); // Prints 3
    //     fn f() {
    //         print!("3");
    //     }
    // }
    // f(); // Prints 2
    // fn f() {
    //     print!("2");
    // }

    // fn double(x: f64) -> f64 {
    //     // -> f64  means return type is f64
    //     x * 2.
    // }
    // print!("{}", double(17.3));


    // ------------
//     // So, this code is valid:
//  fn f1() -> i32 { 4.5; "abc"; 73i32 }
//  fn f2() -> i32 { 4.5; "abc"; 73 }
//  fn f3() -> i32 { 4.5; "abc"; 73 + 100 }
// //  While this code is not:
//  fn f1() -> i32 { 4.5; "abc"; false }
//  fn f2() -> i32 { 4.5; "abc"; () }
//  fn f3() -> i32 { 4.5; "abc"; {} }
//  fn f4() -> i32 { 4.5; "abc"; }

// ---------
    fn f(x: f64) -> f64 {
        if x <= 0. { return 0.; }
        x + 3.
        }
        print!("{} {}", f(1.), f(-1.));
}
