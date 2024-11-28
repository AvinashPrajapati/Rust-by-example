// Traits
// -> Standard traits

fn main() {
    // "{}" placeholder : how could you make your own type support that placeholder?
    /*
     Actually, internally, those macros use the "fmt" function specified by the
    "std::fmt::Display" standard library trait. All the primitive types implement that trait,
    and if you do it for your types, you can obtain the same results
     */
    struct Complex {
        re: f64,
        im: f64,
    }
    impl std::fmt::Display for Complex {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(
                f,
                "{} {} {}i",
                self.re,
                if self.im >= 0. { '+' } else { '-' },
                self.im.abs()
            )
        }
    }
    let c1 = Complex { re: -2.3, im: 0. };
    let c2 = Complex { re: -2.1, im: -5.2 };
    let c3 = Complex { re: -2.2, im: 5.2 };
    print!("{}, {}, {}", c1, c2, c3);  // implemented so that it can diractly print.
}
