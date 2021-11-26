fn use_slice(slice: &mut [i32]) {
    println!("first elem is {}, len = {}", slice[0], slice.len());
    slice[0] = 4321;

    // will crash
    //let z = slice[10];
}

pub fn demo() {
    // a slice is part of an array
    // its size is not known at compile time
    let mut data = [1, 2, 3, 4, 5];

    // start w/o mut, borrow as a slice
    use_slice(&mut data[1..4]);
    use_slice(&mut data); // entire array

    println!("data after slice use = {:?}", data);
}
