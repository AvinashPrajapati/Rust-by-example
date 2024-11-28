// Input/Output and Error Handling
fn main() {
    // // Command-Line Arguments
    // let command_line: std::env::Args = std::env::args();
    // for argument in command_line {
    //     println!("[{}]", argument);
    // }
    // // or
    // for a in std::env::args() {
    //     println!("[{}]", a);
    // }

    // // return code of a program
    // std::process::exit(107);  // valid one
    // let a = std::process::exit(107);println!("{}", a);  // invalid

    // // Environment Variables
    // for var in std::env::vars() {
    //     println!("[{}]=[{}]", var.0, var.1);
    // }
    // // specific value
    // print!("[{:?}]", std::env::var("abcd"));  // return either Ok or Err
    // // CAUTION!!  -  to run code
    // // std::env::set_var("abcd", "This is the value"); //
    // std::env::remove_var("abcd"); // to remove

    // // similar program A similar program is this one:
    // print!(
    //     "{}",
    //     if std::env::var("abcd").is_ok() {
    //         "Already defined"
    //     } else {
    //         "Undefined"
    //     }
    // );
    // // using match
    // print!(
    //     ", {}.",
    //     match std::env::var("abcd") {
    //         Ok(value) => value,
    //         Err(err) => format!("Still undefined: {}", err),
    //     }
    // );

}
