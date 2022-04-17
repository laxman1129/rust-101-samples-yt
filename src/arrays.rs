use std::mem;
// length is fixed
pub fn run() {
    // array of i32 , with length 5
    // This array must have 5 elements, else error at compile and runtime
    // also the type should be the declared type
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];

    println!("{:?}", numbers);
    // print single val
    println!("single value{}", numbers[0]);

    // in order to change the values in the array it has to be declared as mut
    let mut numbers: [i32; 5] = [0, 1, 2, 3, 4];
    println!("number at index 0 before change = {}", numbers[0]);

    numbers[0] = 1129;

    println!("number at index 0 after change = {}", numbers[0]);

    println!("length : {}", numbers.len());

    // arrays are stack allocated
    // by using & we pass the reference to the variable rather than the actual variable
    println!("array occupies {} bites", std::mem::size_of_val(&numbers));

    // can import with the help of the use keyword
    println!("array occupies {} bites", mem::size_of_val(&numbers));

    // get slice
    // this is the way to declare variable to take value as reference of another variable
    let slice: &[i32] = &numbers;
    println!("{:?}", slice);

    // x..y syntax gives the slices
    let slice: &[i32] = &numbers[0..2];
    println!("0..2  => {:?}", slice);
}
