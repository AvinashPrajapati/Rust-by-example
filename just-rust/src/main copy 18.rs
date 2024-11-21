fn main() {
    // // // Range slicing
    // // ---- to get all element
    // let arr = [11, 22, 33, 44];
    // let n = 2;
    // let sr1 = &arr[0..n];
    // let sr2 = &arr[n..arr.len()];
    // println!("{:?} {:?}", sr1, sr2);

    // let sr3 = &arr[0..arr.len()];
    // println!("{:?}", sr3);

    // let sr4 = &arr[..arr.len()];
    // print!("{:?}", sr4);

    // let sr5 = &arr[..];
    // print!("{:?}", sr5);

    // // ---- so number..  means start from that number and goes on
    // for i in 3.. {
    //     if i * i > 40 {
    //         break;
    //     }
    //     print!("{} ", i);
    // }

    //  generic type one
    let range: std::ops::RangeFull = ..;
    let a1 = [11, 22, 33, 44];
    let a2 = &a1[range];
    print!("{} {:?} {:?}", std::mem::size_of_val(&range), a1, a2); // it's good 0 size of refrence range
}
