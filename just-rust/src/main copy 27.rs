// Input/Output - FIle input/output



// FIX REQUIRE...................

use std::io::{Read, Write}; // - write_all
fn main() {
    // // //  FIle write
    // let mut file = std::fs::File::create("data.txt").unwrap();
    // file.write_all("eè€".as_bytes()).unwrap();

    // // to read
    // use std::io::Read;
    // let mut file = std::fs::File::open("data.txt").unwrap();
    // let mut contents = String::new();
    // file.read_to_string(&mut contents).unwrap();
    // print!("{}", contents);
    /*
         Notice that there is no need to close explicitly the files. As soon as the file handles
    exit their scopes, the files are automatically closed, saving and releasing all internal
    temporary buffers.
         */

    //--------------
    /*
         So now you can copy a file into another one. But if a file is huge, it is impossible to
    load it all into a string before writing it. It is required to read and write a portion at a time.
    However, it is inefficient to read and write small portions.
     Here is a rather efficient program to copy a file.
         */

    let mut command_line: std::env::Args = std::env::args();
    // println!("{:?}", command_line.next().unwrap())
    // command_line.next().unwrap();
    // let source = command_line.next().unwrap();
    // let destination = command_line.next().unwrap();
    // let mut file_in = std::fs::File::open(source).unwrap();
    // let mut file_out = std::fs::File::create(destination).unwrap();
    // let mut buffer = [0u8; 4096];
    // loop {
    //     let nbytes = file_in.read(&mut buffer).unwrap();
    //     file_out.write(&buffer[..nbytes]).unwrap();
    //     if nbytes < buffer.len() {
    //         break;
    //     }
    // }
}
