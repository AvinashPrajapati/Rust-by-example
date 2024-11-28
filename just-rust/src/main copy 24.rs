// Input/Output and Error Handling
fn main() {
    // // Reading from the console
    // let mut line = String::new();
    // println!("{:?}", std::io::stdin().read_line(&mut line));
    // println!("'{}'", line);

    // // another example
    // let mut text = format!("First: ");
    // let inp = std::io::stdin();
    // inp.read_line(&mut text).unwrap();  // you can ommit unwrap()
    // text.push_str("Second: ");
    // inp.read_line(&mut text).unwrap();
    // println!("{}: {} bytes", text, text.len());

    // // omitted unwrap()
    // // but give warning because :  "read_line" returns a value of type "Result" and that value is ignored or not used.  ------- But in production code, matters are not so simple.
    // let mut text = format!("First: ");
    // let inp = std::io::stdin();
    // inp.read_line(&mut text);
    // text.push_str("Second: ");
    // inp.read_line(&mut text);
    // println!("{}: {} bytes", text, text.len());
}
