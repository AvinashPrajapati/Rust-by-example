fn main() {
    // ITERATORS or sometimes CURSOR
    // // 1. string iterator
    // fn print_nth_char(s: &str, mut n: u32) {
    //     let mut iter: std::str::Chars = s.chars();
    //     loop {
    //         let item: Option<char> = iter.next();
    //         match item {
    //             Some(c) => {
    //                 if n == 1 {
    //                     print!("{}", c);
    //                 }
    //             }
    //             None => {
    //                 break;
    //             }
    //         }
    //         n -= 1;
    //     }
    // }
    // print_nth_char("€èe", 3);

    // // 2. another one
    // fn print_codes(s: &str) {
    //     let mut iter = s.chars();
    //     loop {
    //         match iter.next() {
    //             Some(c) => {
    //                 println!("{}: {}", c, c as u32);
    //             }
    //             None => {
    //                 break;
    //             }
    //         }
    //     }
    // }
    // print_codes("€èe");

    // // ------using for loop
    // fn print_codes(s: &str) {
    //     println!("{:?}", s.chars() );
    //     // println!("{:?}", "avinash".chars() );
    //     for c in s.chars() {
    //         println!("{}: {}", c, c as u32);
    //     }
    // }
    // print_codes("€èe");

    // // -------It is possible also to iterate over the bytes of a string:
    // for byte in "€èe".bytes() {
    //     print!("{} ", byte);
    // }
    // // OR
    // let string: &str = "€èe";
    // let string_it: std::str::Bytes = string.bytes();
    // for byte in string_it {
    //     print!("{} ", byte);
    // }

    // // --------------  all ranges with a ' STARTING ' limit are iterators,as they have a next function.
    // // OK: std::ops::Range<u32> is an iterator
    // let _v1 = (0u32..10).next();
    // println!("{:?}", _v1);
    // // OK: std::ops::RangeFrom<u32> is an iterator
    // let _v2 = (5u32..).next();
    // println!("{:?}", _v2)
    // // Illegal: std::ops::RangeTo<u32> is not an iterator
    // // let _v3 = (..8u32).next();
    // // Illegal: std::ops::RangeFull is not an iterator
    // // let _v4 = (..).next()

    // // ---------  array/vector iterator
    // for item_ref in (&[11u8, 22, 33]).iter() {
    //     // refrence array
    //     print!("{} ", *item_ref);
    // }
    // for item_ref in [44, 55, 66].iter() {
    //     // array
    //     print!("{} ", *item_ref);
    // }
    // for item_ref in vec!['a', 'b', 'c'].iter() {
    //     // vector
    //     print!("{} ", *item_ref);
    // }
    // // --> this above line of the first for loop
    // let slice: &[u8] = &[11u8, 22, 33];
    // let slice_it: std::slice::Iter<u8> = slice.iter(); //pub struct Iter<'a, T>  here T is an & that is reference
    // println!("{:?}", slice_it);
    // for item_ref in slice_it {  // you can see that is it is &u8
    //     print!("{} ", *item_ref);
    // }

    // --- some creativity
    for byte in "€èe".as_bytes().iter() {  // here byte is & type of u8
        print!("{} {},", *byte, byte);
    }
}
