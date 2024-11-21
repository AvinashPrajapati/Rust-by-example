fn main() {
    // Iterator consumers
    // // 1. a normal way
    // let s = "Hello, world!";
    // let ch = 'R';
    // let mut contains = false;
    // for c in s.chars() {
    //     if c == ch {
    //         contains = true;
    //     }
    // }
    // print!(
    //     "\"{}\" {} '{}'.",
    //     s,
    //     if contains {
    //         "contains"
    //     } else {
    //         "does not contain"
    //     },
    //     ch
    // );

    // // using ANY consumer
    // let s = "Hello, world!";
    // let ch = 'R';
    // print!(
    //     "\"{}\" {} '{}'.",
    //     s,
    //     if s.chars().any(|c| c == ch) {
    //         "contains"
    //     } else {
    //         "does not contain"
    //     },
    //     ch
    // );

    // // another simple example
    // print!("{} ", [45, 8, -2, 6].iter().any(|n| *n < 0));

    // ALL  consumer
    print!("F{} ",[45, 8, 2, 6].iter().all(|n: &i32| -> bool { *n > 0 }));
}
