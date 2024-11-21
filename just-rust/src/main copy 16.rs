fn main() {
    // // Range
    // for i in 0..12 { print!("{} ,", i); }

    // // ---- another possible way to write it:
    // let dozen = 0..12; // Range
    // for i in dozen {
    //     println!("{}", i);
    // }

    // // some more code using range
    // let range: std::ops::Range<usize> = 3..8;
    // println!(
    //     "{:?}, {}, {}, {}",
    //     range,
    //     range.start,
    //     range.end,
    //     range.len()
    // );
    // for i in range {
    //     print!("{}, ", i);
    // }

    // // ---- Nonsense but true but cannot be used in a for loop.
    // let _r1 = false..true;
    // let _r2 = "hello".."world";
    // let _r3 = 4.2..7.9;

    // // --------- passing a sequence to a function
    // fn min(arr: [i32; 8]) -> i32 {
    //     // LIMITAIONS
    //     // 1. It gets as argument a copy of the whole array, requiring a significant time to transfer it, and occupying a significant stack space and cache space
    //     // 2. it can receive only 8-number arrays or cannot receive a vector as argument.
    //     let mut minimum = arr[0];
    //     for i in 1..arr.len() {
    //         if arr[i] < minimum {
    //             minimum = arr[i];
    //         }
    //     }
    //     minimum
    // }
    // print!("{}", min([23, 17, 12, 16, 15, 28, 17, 30]));

    // // --------- to overcome these limitations
    // fn min(arr: &[i32, 8]) -> i32 {
    fn min(arr: &[i32]) -> i32 {
        // Let's assume 'start' is between 0 and 7,
        // and 'count' is between 1 and 8 - start.
        let mut minimum = arr[0];
        for i in 1..arr.len() {
            if arr[i] < minimum {
                minimum = arr[i];
            }
        }
        minimum
    }
    println!("{}", min(&[23, 17, 12, 16, 15, 28, 17, 30]));
    print!("{}", min(&vec![55, 22, 33, 44]));
}
