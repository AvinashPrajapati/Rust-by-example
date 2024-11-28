// Traits
fn main() {
    // // // trait
    // trait HasSquareRoot {
    //     fn sq_root(self) -> Self;
    // }
    // // it's implementation
    // impl HasSquareRoot for f32 {
    //     fn sq_root(self) -> Self {
    //         f32::sqrt(self)
    //     }
    // }
    // impl HasSquareRoot for f64 {
    //     fn sq_root(self) -> Self {
    //         f64::sqrt(self)
    //     }
    // }
    // // done with trait definition and implementaion

    // // a funtion
    // fn quartic_root<Number>(x: Number) -> Number
    // where
    //     Number: HasSquareRoot,
    // {
    //     x.sq_root().sq_root()
    // }
    // print!("{} {}", quartic_root(100f64), quartic_root(100f32))

    /*
    Above trait contained a function named "sq_root", to make it clear it was not the
    same thing as the "sqrt" standard library function.
     */

    // // However, we could also name that function "sqrt", obtaining this equivalent valid program
    // fn sqrt() {}
    // trait HasSquareRoot {
    //     fn sqrt(self) -> Self; // this "sqrt" is belong to HasSquareRoot trait scope means this wil be not affected by the above sqrt funtion.
    // }
    // // implementaion of the trait
    // impl HasSquareRoot for f32 {
    //     fn sqrt(self) -> Self {
    //         f32::sqrt(self)
    //     }
    // }
    // impl HasSquareRoot for f64 {
    //     fn sqrt(self) -> Self {
    //         f64::sqrt(self)
    //     }
    // }
    // // funtioin
    // fn quartic_root<Number>(x: Number) -> Number
    // where
    //     Number: HasSquareRoot,
    // {
    //     x.sqrt().sqrt()
    // }
    // sqrt(); // calling the above sqrt fn

    // print!("{} {}", quartic_root(100f64), quartic_root(-100f32)); // minus

    /*
        You know Rust does not allow two functions with the same name in the same scope.
    Nevertheless the code above is valid; that means that these four functions with the same
    name belong to four different scopes
         */
    //--------------

    // // Traits with Multiple Functions
    // // example 1
    // trait HasSqrtAndAbs {
    //     fn sqrt(self) -> Self;
    //     fn abs(self) -> Self;
    // }
    // impl HasSqrtAndAbs for f32 {
    //     fn sqrt(self) -> Self {
    //         f32::sqrt(self)
    //     }
    //     fn abs(self) -> Self {
    //         f32::abs(self)
    //     }
    // }
    // impl HasSqrtAndAbs for f64 {
    //     fn sqrt(self) -> Self {
    //         f64::sqrt(self)
    //     }
    //     fn abs(self) -> Self {
    //         f64::abs(self)
    //     }
    // }
    // fn abs_quartic_root<Number>(x: Number) -> Number
    // where
    //     Number: HasSqrtAndAbs,
    // {
    //     x.abs().sqrt().sqrt()
    // }
    // print!(
    //     "{} {}",
    //     abs_quartic_root(-100f64),
    //     abs_quartic_root(-100f32)
    // );

    // better version of example 1
    /*
         Sometimes, a few functions are strictly coupled, and so, every time one of them
    is needed, all of them should be implemented anyway. But what if in some generic
    functions you want to use a type that has a "sqrt" function, but not an "abs" function,
    or conversely? You are forced to implement even the functions you don’t need.
     To avoid this, you can declare a new trait, obtaining the following equivalent
    program:
         */
    // trait HasSquareRoot {
    //     // trait 1
    //     fn sqrt(self) -> Self;
    // }
    // impl HasSquareRoot for f32 {
    //     fn sqrt(self) -> Self {
    //         f32::sqrt(self)
    //     }
    // }
    // impl HasSquareRoot for f64 {
    //     fn sqrt(self) -> Self {
    //         f64::sqrt(self)
    //     }
    // }
    // trait HasAbsoluteValue {
    //     // trait 2
    //     fn abs(self) -> Self;
    // }
    // impl HasAbsoluteValue for f32 {
    //     fn abs(self) -> Self {
    //         f32::abs(self)
    //     }
    // }
    // impl HasAbsoluteValue for f64 {
    //     fn abs(self) -> Self {
    //         f64::abs(self)
    //     }
    // }
    // fn abs_quartic_root<Number>(x: Number) -> Number
    // where
    //     Number: HasSquareRoot + HasAbsoluteValue,  
    // // notice this plus that means with these two traits  This trait bound adds to the capabilities of "Number" the capabilities of both traits. In this way, you can pick the traits you need for your generic type
    // {
    //     x.abs().sqrt().sqrt()
    // }
    // print!(
    //     "{} {}",
    //     abs_quartic_root(-100f64),
    //     abs_quartic_root(-100f32)
    // );
}
