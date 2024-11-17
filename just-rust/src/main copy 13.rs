fn main() {
    //     print!("{} ", std::mem::size_of::<i64>());
    //  print!("{} ", std::mem::size_of_val(&12));
    // // ----------
    //  use std::mem;
    //  print!("{} ", mem::size_of::<i32>());
    //  print!("{} ", mem::size_of_val(&12));
    // //  or in this one:
    //  use std::mem::size_of;
    //  use std::mem::size_of_val;
    //  print!("{} ", size_of::<i32>());
    //  print!("{} ", size_of_val(&12));
    // // --------

    // // There is an even more compact form of it:
    //  use std::mem::*;
    //  print!("{} ", size_of::<i32>());
    //  print!("{} ", size_of_val(&12));

    // // // ---------
    // use std::mem::*;
    //  print!("{} {} {} {}",
    //  size_of::<isize>(),
    //  size_of::<usize>(),
    //  size_of::<&i8>(),
    //  size_of::<&u32>());

    // //  MOST IMPORTANT ONE
    // fn as_bytes<T>(o: &T) -> &[u8] {
    //     unsafe { std::slice::from_raw_parts(o as *const _ as *const u8, std::mem::size_of::<T>()) }  // you can remove _ as T
    // }
    // println!("{:?}", as_bytes(&1i8));
    // println!("{:?}", as_bytes(&2i16));
    // println!("{:?}", as_bytes(&3i32));
    // println!("{:?}", as_bytes(&(4i64 + 5 * 256 + 6 * 256 * 256))); // its explsnation
    /*
    394500 in binary (64 bits) is:
    00000000 00000000 00000000 00000000 00000000 00000110 00000101 00000100

    In little-endian order, the least significant byte comes first:
    [4, 5, 6, 0, 0, 0, 0, 0]

    Do you get it??
     */

    // --------------
    // println!("{:?}", as_bytes(&'A'));
    // println!("{:?}", as_bytes(&true));
    // println!("{:?}", as_bytes(&&1i8));  // for this
    /*
    as_bytes(&&1i8)
    If the address of &1i8 is 0x5678, its 64-bit representation (in hexadecimal) is:
    0x0000000000005678

    In memory (little-endian):
    [0x78, 0x56, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]
     */


    // more examples

    // ------- You can also discover the (virtual) memory location of any object, which is its address:
    let b1 = true;
    let b2 = true;
    let b3 = false;
    print!("{} {} {}",
    &b1 as *const bool as usize,
    &b2 as *const bool as usize,
    &b3 as *const bool as usize);
}
