pub fn demo() {
    for x in 1..11
    // 1 to 10; 11..1 won't work
    {
        // skip 3
        if x == 3 {
            continue;
        }

        // stop at 7
        if x == 8 {
            break;
        }

        println!("x = {}", x);
    }

    for (pos, y) in (30..41).enumerate() {
        println!("{}: {}", pos, y);
    }
}
