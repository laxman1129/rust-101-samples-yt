/*
Premitive types
Integers : u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (numbers of bits they take in memory)
Floats : f32, f64
Boolean (bool) :
Character (char) :
Tuples :
Arrays :
*/
// statically typed, but can infer type of variable based on declaration

pub fn run() {
    // default is i32
    let x = 1;
    // default is f64
    let y = 2.5;
    println!("{:?}", (x, y));

    // add explicit type
    let y: i64 = 928374983749283;
    println!("{:?}", (y));

    // find max size

    println!("max i32 :{}", std::i32::MAX);
    println!("max i64 :{}", std::i64::MAX);

    // boolean
    let is_active = true;
    let is_greater = 10 > 5;
    println!("{:?}", (is_active, is_greater));

    // character
    let a1 = 'a';
    let face = '\u{1F600}';
    println!("{:?}", (a1, face))
}
