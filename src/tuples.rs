fn sum_and_product(x: i32, y: i32) -> (i32, i32) {
    (x + y, x * y)
}

pub fn demo() {
    let x = 3;
    let y = 4;
    let sp = sum_and_product(3, 4);

    println!("sp = {:?}", sp);
    println!("{0} + {1} = {2}, {0} * {1} = {3}", x, y, sp.0, sp.1); // try sp.5

    // destructuring
    let (a, b) = sp;
    println!("a = {}, b = {}", a, b);

    // tuple of tuples
    let sp2 = sum_and_product(4, 7);
    let combined = (sp, sp2);
    println!("{:?}", combined);
    println!("last element is {}", (combined.1).1);

    let ((c, d), (e, f)) = combined;

    // tuple of different elements
    let foo = (true, 42.0, -1i8);
    println!("{:?}", foo);

    // tuple of a single element
    let meanings = (42,); // start w/o comma
    println!("{:?}", meanings);
}
