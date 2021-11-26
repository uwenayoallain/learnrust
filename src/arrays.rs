use std::mem;

pub fn demo() {
    let mut a/*:[i32;5]*/ = [1, 2, 3, 4, 5];

    println!("a has {} elements, first is {}", a.len(), a[0]);
    a[0] = 321;
    println!("first value of a is {}", a[0]);

    assert_eq!(a, [321, 2, 3, 4, 5]);

    if a != [1, 2, 3, 5, 6]
    // size must match
    {
        println!("arrays not equal!");
    }

    // fill an array with 1s
    let b = [1u16; 10]; // try changing to 5
    for i in 0..b.len() {
        println!("{}", b[i]);
    }

    // just print the entire arary
    println!("{:?}", b);
    println!("b took up {} bytes", mem::size_of_val(&b));

    // multidimensional array
    let mtx: [[f32; 3]; 2] = [[1.0, 0.0, 0.0], [0.0, 2.0, 0.0]];
    println!("{:?}", mtx);

    // print all the diagonal values
    for i in 0..mtx.len() {
        for j in 0..mtx[i].len() {
            if i == j {
                println!("mtx[{}][{}] = {}", i, j, mtx[i][j]);
            }
        }
    }
}
