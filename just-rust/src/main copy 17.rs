fn main() {
    // // // Range slicing
    // let mut arr = [11, 22, 33, 44];
    // {
    //     let sl_ref = &mut arr[1..3]; // Note &mut
    //     print!("{:?}", sl_ref);
    //     sl_ref[1] = 0;
    //     print!(" {:?}", sl_ref);
    // }
    // print!(" {:?}", arr);
    
    
    // ----- another example
    let arr = [11, 22, 33, 44];
    {
        let mut sl_ref = &arr[1..3];
        print!("{:?}", sl_ref);
        sl_ref = & arr[0..1];  //  not &mut it's & only
        print!(" {:?}", sl_ref);
    }
    print!(" {:?}", arr);
}
