fn main() {
    // Iterator adaptors

    // // 1. filter
    // let arr = [66, -8, 43, 19, 0, -31];
    // for n in arr.iter() {
    //     if *n < 0 {
    //         print!("{} ", n);
    //     }
    // }

    // // better version
    // let arr = [66, -8, 43, 19, 0, -31];
    // for n in arr.iter().filter(|x| **x < 0) {
    //     print!("{} ", n);
    // }

    // // what if i want to turn the negative values in the positive values
    // let mut arr = [66, -8, 43, 19, 0, -31];
    // for n in arr.iter_mut() {
    //     if *n < 0 {
    //         *n = -1**n
    //     }
    // }
    // print!("{:?}", arr);

    // // MAP
    // for n in arr.iter().map(|x| *x * 2) {
    //     print!("{} ", n);
    // }

    // // enumerate
    // // example 1 - to read elements of arr
    // let arr = ['a', 'b', 'c'];
    // for ch in arr.iter() {
    //     print!("{}, ", ch);
    // }
    // // using the traditional counter method
    // let arr = ['a', 'b', 'c'];
    // let mut i = 0;
    // for ch in arr.iter() {
    //     print!("{} {}, ", i, *ch);
    //     i += 1;
    // }
    // another possibility : But there is another possibility
    let arr = ['a', 'b', 'c'];
    for (i, ch) in arr.iter().enumerate() {
        print!("{} {}, ", i, *ch);
    }
}
