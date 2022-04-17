// reference pointers - point to a resource in memory

pub fn run() {
    // premitive array
    let arr1 = [1, 2, 3];
    let arr2 = arr1;

    println!("values{:?}", (arr1, arr2));

    // with non-premitives, if you assign another variable to a piece of data,
    // the first variable will no longer hold that value
    // You will need to use a reference (&) to point to the resource
    // move occurs because `vec1` has type `Vec<i32>`, which does not implement the `Copy` trait

    let vec1 = vec![1, 2, 3];
    let vec2 = &vec1; // to fix use &
    println!("values{:?}", (&vec1, vec2)); // to fix use &
}
