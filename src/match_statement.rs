pub fn demo() {
    let country_code = 999;

    let country = match country_code {
        44 => "UK",
        46 => "Sweden",
        7 => "Russia",
        1..=1000 => "unknown", // inclusive range ... is deprecated
        _ => "invalid",        // try commenting this out - must cover all cases!
    };

    println!("the country with code {} is {}", country_code, country);

    let x = false;

    let s = match x {
        true => "yes",
        false => "no",
    };
}
