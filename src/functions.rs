pub fn run() {
    greeting("hello", "Lax");
    let sum = add(5, 10);
    println!("sum of 5 and 10 is  = {}", sum);

    // closure
    // we can use outside variables in closures 
    // e.g. n3 is outside the closure but we can still use it in closure
    // not possible in fn because of the scope
    let n3:i32 = 9;
    let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("closure sum of 5 and 10 and 9 is  = {}", add_nums(5, 10));
}

fn greeting(greet: &str, name: &str) {
    println!("{} {}", greet, name);
}

// -> impiles return type
//  id there is no ; at the end of the last statement then its the return value
fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2
}
