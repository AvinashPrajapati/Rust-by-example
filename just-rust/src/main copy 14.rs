fn main() {
    // let mut a = vec![(0i16, 0i64); 2];
    // a.pop();
    // println!("{:?}", a);

    // // --------- VECTORS

    // let mut v = vec![0; 0];
    // println!("{} {}", v.len(), v.capacity());
    // v.push(11);
    // println!("{} {}", v.len(), v.capacity());

    // // -----
    // let prev_capacity = std::f64::MAX;
    // println!("{}", prev_capacity)

    // ---------

    // ---------
    /*
    A Closure is just a handier kind of function, fit to define small anonymous functions,
    and to invoke them just where they have been defined
    */
    let mut arr = [4, 8, 1, 10, 0, 45, 12, 7];

    // // ---- inline using closure
    // use std::cmp::Ordering;
    // let desc = |a: &i32, b: &i32| -> Ordering {
    //     if a < b {
    //         Ordering::Greater
    //     } else if a > b {
    //         Ordering::Less
    //     } else {
    //         Ordering::Equal
    //     }
    // };
    // arr.sort_by(desc);
    // print!("{:?}", arr);

    // // ------ simplified one
    // arr.sort_by(|a, b| a.cmp(b));
    // print!("{:?}", arr);

    // // BUT------ to obtain the inverted order, you can use,
    arr.sort_by(|a, b| (&-*a).cmp(&-*b));
    /*
    &-*a
    *a dereferences the reference a to get the actual value.
    - negates the value (assuming the array contains numeric types).
    &-*a takes a reference to the negated value.
    Why take a reference to the negated value? The .cmp() method requires references to values for comparison.
     */
    // print!("{:?}", arr);
    // // OR
    // arr.sort_by(|a, b| b.cmp(a));
    // print!("{:?}", arr);

    // //---------------  Here is another example that shows six ways to invoke a closure:
    // let factor = 2;
    // let multiply = |a| a * factor;
    // print!("{}", multiply(13));
    // let multiply_ref: &(dyn Fn(i32) -> i32) = &multiply;  // dyn  : dynamic dispatch
    // // hover over  Fn(i32)  to see that it is a trait object
    // print!(
    //     " {} {} {} {} {}",
    //     (*multiply_ref)(13),
    //     multiply_ref(13),
    //     (|a| a * factor)(13),
    //     (|a: i32| a * factor)(13),
    //     |a| -> i32 { a * factor }(13)
    // );

    print!(
        "{}",
        (|v: &Vec<i32>| {
            let mut sum = 0;
            for i in 0..v.len() {
                sum += v[i];
            }
            sum
        })(&vec![11, 22, 34])
    );
}
