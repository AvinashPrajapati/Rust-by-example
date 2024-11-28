// Input/Output
// use std::io::Write;
fn main() {
    // //  Writing to the Console
    // std::io::stdout().write("Hello ".as_bytes()).unwrap();

    // Converting a Value to a String
    let int_str: String = 45_00.to_string();
    print!("{}", int_str)
    /*
         The "to_string" function allocates a String object, whose header is in the stack and
    whose contents are in the heap. Therefore, it is not extremely efficient.
     */
}
