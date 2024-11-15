fn main() {
    // Library code
    // // ------ generic funtions
    // fn f<T>(ch: char, num1: T, num2: T) -> T {
    //     // '-> T'  can be different like '-> char'  etc.  also all the three T is comming from the <T>
    //     if ch == 'a' {
    //         num1
    //     } else {
    //         num2
    //     }
    // }
    // //...... Application code /valid
    // let a: i16 = f::<i16>('a', 37, 41);
    // let b: f64 = f::<f64>('b', 37.2, 41.1);
    // print!("{} {}", a, b);
    // also c=valid use

    // //..... Application code / valid
    // let a: i16 = f('a', 37, 41);
    // let b: f64 = f('b', 37.2, 41.1);
    // print!("{} {}", a, b);

    // // ............ with different parametrics
    // fn f<Param1, Param2>(_a: Param1, _b: Param2) {}
    // f('a', true);
    // f(12.56, "Hello");
    // f((3, 'a'), [5, 6, 7]);
    // // NOW YOU CAN USE YOUR BRAIN \^o^/

    // ---------generic STRUCT.....LOL
    // // example 1
    // struct S<T1, T2> {
    //     c: char,
    //     n1: T1,
    //     n2: T1,
    //     n3: T2,
    // }
    // let _s = S {
    //     c: 'a',
    //     n1: 34,
    //     n2: 782,
    //     n3: 0.02,
    // };
    // // example 2
    // struct SE<T1, T2>(char, T1, T1, T2);
    // let _se = SE('a', 34, 782, 0.02);

    // // something tricky
    // fn swap<T1, T2>(a: T1, b: T2) -> (T2, T1) { (b, a) }
    // let x = swap(3i16, 4u16);
    // let y = swap(5f32, true);
    // print!("{:?} {:?}", x, y);
    // i get it.......Your confusion
    println!("{} same as  {}", 4, 4i16)  // 4i16 is same as '4 as i16'




}
