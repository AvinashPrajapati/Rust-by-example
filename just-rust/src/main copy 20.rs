fn main() {
    // changing the iterator variable during the for loop ::  Iterations Without Mutation

    // let mut r = "abc".chars();
    // for i in r {
    //     r = "XY".chars();
    //     print!("{} {}; ", i, r.next().unwrap());
    // }

    // let r = 0..5;
    // for mut i in r {
    //     // using mut to change i iterator refrence
    //     i += 10;
    //     print!("{} ", i);
    //     // now you can use the i to access the element of the string means you do not need  of iterators that allow changes to the items of the sequence
    // }

    // //  ---- Iterations with Mutation :: that is to change the value of the iterator valur
    // let slice = &mut [3, 4, 5];
    // {
    //     let iterator = slice.iter_mut(); // not iter  it's iter_mut to change the items in slice
    //     for item_ref in iterator {
    //         *item_ref += 1;
    //     }
    // }
    // print!("{:?}", slice);

    // ---- here is the same program in which the values of the sequences are changed.
    // for item_ref in (&mut [11u8, 22, 33]).iter_mut() {
    //     *item_ref += 1;
    //     print!("{} ", *item_ref);
    // }
    // for item_ref in [44, 55, 66].iter_mut() {
    //     *item_ref += 1;
    //     print!("{} ", *item_ref);
    // }
    // for item_ref in vec!['a', 'b', 'c'].iter_mut() {
    //     *item_ref = if *item_ref == 'b' { 'B' } else { '-' };
    //     print!("{} ", *item_ref);
    // }

    // --- so this and now do same for the remaining two arr and vec
    let slice = &mut [11u8, 22, 33];
    for item_ref in slice.iter_mut() {
        *item_ref += 1;
    }
    print!("{:?} ", slice);  

}
