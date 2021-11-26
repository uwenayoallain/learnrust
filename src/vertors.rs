pub fn demo() {
    /*
    let mut a = Vec::new();

    a.push(1);
    a.push(2);
    a.push(3);
    */

    let mut a = vec![1, 2, 3]; // [1;10]
    println!("a = {:?}", a);

    let idx/*:i32*/ = 0; // will not work with :i32
                         // you need usize
    println!("a[0] = {}", a[idx]);

    // unsafe access
    //println!("a[5] = {}", a[5]);

    match a.get(5) {
        Some(x) => println!("a[5] = {}", x),
        None => println!("error, no such element"),
    }

    // iterate
    for x in &a {
        println!("{}", x);
    }

    // adding/removing
    a.push(44);
    println!("{:?}", a);

    let last_elem = a.pop(); // can easily yield nothing
    println!("last elem is {:?}, a = {:?}", last_elem, a);

    // explain why this doesn't work
    //let Some(last_value) = a.pop();

    // print the elements in reverse order
    while let Some(x) = a.pop() {
        println!("{}", x);
    }
}
