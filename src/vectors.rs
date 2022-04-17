use std::mem;
// length is fixed
pub fn run() {
    // this is how vectors are defined
    let numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

    println!("{:?}", numbers);
    // print single val
    println!("single value{}", numbers[0]);

    // in order to change the values in the array it has to be declared as mut
    let mut numbers: Vec<i32> = vec![0, 1, 2, 3, 4];
    println!("number at index 0 before change = {}", numbers[0]);

    numbers[0] = 1129;
    // add on to vector
    numbers.push(5);
    numbers.push(65);

    println!("{:?}", numbers);

    // popping off the value (last value)
    numbers.pop();
    println!("after popping the value {:?}", numbers);

    println!("number at index 0 after change = {}", numbers[0]);

    println!("length : {}", numbers.len());

    // vectors are stack allocated
    // by using & we pass the reference to the variable rather than the actual variable
    println!("vector occupies {} bites", std::mem::size_of_val(&numbers));

    // can import with the help of the use keyword
    println!("vector occupies {} bites", mem::size_of_val(&numbers));

    // get slice
    // this is the way to declare variable to take value as reference of another variable
    let slice: &[i32] = &numbers;
    println!("{:?}", slice);

    // x..y syntax gives the slices
    let slice: &[i32] = &numbers[0..2];
    println!("0..2  => {:?}", slice);

    // loop through vector value
    // iter() => &i32 => reference
    for x in numbers.iter() {
        println!("{}", x);
    }

    // loop and mutate value
    // similar to stream map
    for x in numbers.iter_mut() {
        *x *= 2
    }
    println!("after mutaion : {:?}", numbers);

    // loop through vector value
    //  without iter() => i32 => actual value
    // this loop empties the vectoe
    for x in numbers {
        println!("{}", x);
    }

    // uncommenting below will give error : value used here after move
    // for x in numbers {
    //     println!("{}", x);
    // }
}
