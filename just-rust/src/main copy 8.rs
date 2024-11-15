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
    // fn f(x: f64) -> f64 {
    //     if x <= 0. { return 0.; }
    //     x + 3.
    //     }
    //     print!("{} {}", f(1.), f(-1.));

    // -----------
    //         fn f() -> i32 { 3 }
    //  f();

    //  ---------
    // fn f() -> i32 { 3 }  // return type = i32
    // let a: i32 = f(); // variable type = i32
    // println!("{}", a)

    // //   return sveral values-------------
    // enum E { E1, E2 }
    // fn f1() -> E { E::E2 }
    // print!("{} ", match f1() { E::E1 => 1, _ => -1 });

    // struct S { a: i32, b: bool }
    // fn f2() -> S { S { a: 49, b: true } }
    // print!("{} ", f2().a);

    // struct TS (f64, char);
    // fn f3() -> TS { TS (4.7, 'w') }
    // print!("{} ", f3().1);

    // fn f4() -> [i16; 4] { [7, -2, 0, 19] }
    // print!("{} ", f4()[0]);

    // fn f5() -> Vec<i64> { vec![12000] }
    // print!("{} ", f5()[0]);

    // // ----  pass by reference
    // // FIRST WHY REQUIRED:
    // // example 1 with problem
    // fn double_negatives(mut a: [i32; 10]) {
    //     for i in 0..10 {
    //         if a[i] < 0 {
    //             a[i] *= 2;
    //         }
    //     }
    // }
    // let mut arr = [5, -4, 9, 0, -7, -1, 3, 5, 3, 1];
    // double_negatives(arr);
    // print!("{:?}", arr)  // BUT arr remain unchanged.

    // // ------------- SO
    // fn double_negatives(mut a: [i32; 10]) -> [i32; 10] {
    //     for i in 0..10 {
    //         if a[i] < 0 {
    //             a[i] *= 2;
    //         }
    //     }
    //     a  // return arr
    // }
    // let mut arr = [5, -4, 9, 0, -7, -1, 3, 5, 3, 1];
    // arr = double_negatives(arr);  // assigne arr
    // print!("{:?}", arr); // but the problem is that arr has been copied twice, not optimize SO
    // // ------pass by reference
    // fn double_negatives(a: &mut [i32; 10]) {
    //     println!("Original arr: {:?}", *a);
    //     for i in 0..10 {
    //         if (*a)[i] < 0 {  //(*a) or a is correct as a is an address
    //             a[i] *= 2;
    //         }
    //     }
    // }
    // let mut arr = [5, -4, 9, 0, -7, -1, 3, 5, 3, 1];
    // double_negatives(&mut arr);
    // print!("{:?}", arr); //NICE

    // -------- but how to use refrence
    // let a = 15;
    // let ref_a = &a;
    // print!("{} {} {}", a, *ref_a, ref_a);

    // // -------- niw it is just non sense but fine
    // let a = &&&7;
    // print!("{} {} {} {}", ***a, **a, *a, a);

    // // .......is there any way to mutate the originsl variable
    // let mut a: i32 = 10;
    // let mut b: i32 = 20;
    // let mut p: &mut i32 = &mut a; // line 3  declared p as mutable variable
    // print!("{} ", *p);  // ---> to a
    // *p += 1; // line 5 updated the a
    // print!("{} ", *p);

    // p = &mut b; // line 7
    // print!("{} ", *p);
    // *p += 1; // line 9
    // print!("{} ", *p);

    
}
