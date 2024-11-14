

fn main() {
    // let j:i64 = 8_000_000_000;
//     let j: f32 = 8_000_.01;
//  print!("{}",j);
// -------
// let a: i16 = 12;
//  let b: u32 = 4;
//  let c: f32 = 3.7;
//  print!("{}", b as f32);
//  print!("{}", b as i8);
// ------
// This code has a behavior somewhat difficult to predict:
// let a = 500 as i16;
// let b = 100_000 as u32;
// let c = 10_000_000_000 as u64;
// print!("{} {} {}", a, b, c);
// Perhaps surprisingly, it will print: "500 100000 10000000000"

// -------
// let _: i8 = 127;
// let _: i16 = 32_767;
// let _: i32 = 2_147_483_647;
// let _: i64 = 9_223_372_036_854_775_807;
// let _: isize = 100; // The maximum value depends on the target architecture
// let _: u8 = 255;
// let _: u16 = 65_535;
// let _: u32 = 4_294_967_295;
// let c: u64 = 18_446_744_073_709_551_615;
// let _: usize = 100; // The maximum value depends on the target architecture
// let _: f32 = 1e38;
// let _: f64 = 1e308;
// println!("{}", c)
// ---------

// println!("{}", true as u8);
// println!("{}", '€' as u8);
// println!("{}", 'à' as u8);

// println!("{}", 224 as char);

// TO check the unicode
// for i in 0..256 {
//     println!("{}: [{}]", i, i as u8 as char);
//     }


//     let a: () = ();
//  let b = { 12; 87; 283 };
//  let c = { 12; 87; 283; };
//  let d = {};
//  let e = if false { };
//  let f = while false { };
//  let person: (&str, i32, f64) = ("Alice", 30, 5.7);
//  print!("{:?} {:?} {:?} {:?} {:?} {:?} {:?}",
//  a, b, c, d, e, f, person);



// ------
print!("{}", 4/3);
println!("{}", 4.0 / 3.0);

let mut value = 10;
value /= 2; // value is now 5
println!("{}", value)



}
