// for variables

// variables hold premetive data or reference data
// variables are immutable by defautl
// rust is block scoped language
pub fn run() {
    let name = "Lax";
    let age = 32;
    // age=32;  // this line give error, cannot assign twice to immutable variable `age`
    // `mut` keyword makes the variable mutable
    let mut temp = 36;
    println!("{:?}", (name, age, temp));
    temp = 38;
    println!("{:?}", (name, age, temp));

    // const keyword for declaring constants
    const ID: i32 = 1129; // type is required for const
    println!("{}", ID);

    // Assign multiple variables
    let (key, val) = (2, "Pen");
    println!("{} is {}", key, val);
}
