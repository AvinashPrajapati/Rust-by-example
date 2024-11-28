// Traits
// -> Standard traits

fn main() {
    // //  You can write a function that, given a range, returns its third item,
    // fn get_third(r: std::ops::Range<u32>) -> Option<u32> {
    //     if r.len() >= 3 {
    //         // println!("{}", r.start);
    //         Some(r.start + 2)
    //     } else {
    //         None
    //     }
    // }
    // print!("{:?} {:?}", get_third(10..12), get_third(20..23));

    //  You can also write a function that, given a slice, returns its third item, if it has at least three items, or nothing, if it has not enough items
    fn get_third(s: &[f64]) -> Option<f64> {
        if s.len() >= 3 {
            Some(s[2])
        } else {
            None
        }
    }
    print!(
        "{:?} {:?}",
        get_third(&[1.0, 2.0]),
        get_third(&[1.1, 2.1, 3.1])
    );

    //  The “type” Keyword
    /*  Say you want to write a portion of code that now uses the "f32" type, but in the future could use the "f64" type or some other type. If you intersperse your code with the "f32" keyword, when you want to switch to the "f64" type you should search and replace all those occurrences, and that is laborious and error prone. */

    // //  instead of writing:
    // fn f1(x: f32) -> f32 {
    //     x
    // }
    // fn f2(x: f32) -> f32 {
    //     x
    // }
    // let a: f32 = 2.3;
    // let b: f32 = 3.4;
    // print!("{} {}", f1(a), f2(b));

    // // it is better to write
    // type Number = f32;
    // fn f1(x: Number) -> Number { x }
    // fn f2(x: Number) -> Number { x }
    // let a: Number = 2.3;
    // let b: Number = 3.4;
    // print!("{} {}", f1(a), f2(b));


}
