fn main() {
    // // ---- Normal string  are static strings and have unmutable contents
    // let mut a = "Hel";
    // print!("{}", a);
    // a = "lo";
    // print!("{}", a); // LoL

    // // --------  so use dynamic strings
    // // example 1
    // let mut a: String = "He".to_string();
    // a.push('l');
    // a.push('l');
    // a.push('o');
    // print!("{}", a);

    // //example 2
    // let mut a: String = "Xy".to_string(); // "Xy"
    // a.remove(0); // "y"
    // a.insert(0, 'H'); // "Hy"
    // a.pop(); // "H"
    // a.push('i'); // "Hi"
    // print!("{}", a);

    // //------  several ways to create an empty dynamic string.
    // let s1 = String::new();
    // let s2 = String::from("");
    // let s3 = "".to_string();
    // let s4 = "".to_owned();
    // let s5 = format!("");
    // print!("({},{},{},{},{})", s1, s2, s3, s4, s5);

    // - example of string concatentation
    let mut dyn_str = "Hello".to_string();
    dyn_str = format!("{}{}", dyn_str, ", ");
    dyn_str = format!("{}{}", dyn_str, "world");
    dyn_str = format!("{}{}", dyn_str, "!");
    print!("{}", dyn_str);
    // more better way  :  push_str
    let mut dyn_str = "Hello".to_string();
    dyn_str.push_str(", ");
    dyn_str.push_str("world");
    dyn_str.push_str("!");
    print!("{}", dyn_str);
    // even more better way :  +=
    let mut dyn_str = "Hello".to_string();
    dyn_str += ", ";
    dyn_str += "world";
    dyn_str += "!";
    print!("{}", dyn_str);
    // Also keep these one in mind
    let comma = ", ".to_string();
    let world = "world".to_string();
    let excl_point = '!';
    let mut dyn_str = "Hello".to_string();
    dyn_str += &comma;
    dyn_str.push_str(&world);
    dyn_str.push(excl_point);
    print!("{}", dyn_str);

    //Any reference to a String can be implicitly converted to a reference to str
    let word = "bye".to_string();
    let w1: &str = &word;
    let w2: &String = &word;
    print!("{} {}", w1, w2);
}
