// Traits
// -> Methods  :  We already saw that there are two possible syntaxes to invoke a function: "f(x, y)" and "x.f(y)". The first one is the “functional” syntax, and the second one is the “object-oriented” syntax.
fn main() {
    // fn double(x: i32) -> i32 {
    //     x * 2
    // }
    // print!("{}", double(7i32));
    // // but not this one
    // print!("{}", 7i32.double()); // invalid
    
    // so to do this one traits come in play

    // --------
    trait CanBeDoubled {  // note this naming convention
        fn double(self) -> Self;
    }
    impl CanBeDoubled for i32 {
        fn double(self) -> Self {
            self * 2
        }
    }
    print!("{}", 7i32.double()); // expression 7i32.double() means a shorthand for the expression i32::double(7i32)
    // BUT : while the expressions 7u32.double() and 7i64.double() are still invalid,
}
