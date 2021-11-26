pub fn demo() {
    let mut x = 1;
    while x < 1000 {
        x *= 2;

        if x == 64 {
            continue;
        }

        println!("x = {}", x);
    }

    let mut y = 1;

    loop
    // while true
    {
        y *= 2;
        println!("y = {}", y);

        // stop at 2^10
        if y == 1 << 10 {
            break;
        }
    }
}
