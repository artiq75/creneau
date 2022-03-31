pub fn input() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Erreur de lecture");
    input
}

pub fn is_want_continue() -> bool {
    println!("Voulez vous continuer (*/n) ?");
    match input().trim() {
        "n" => false,
        _ => true,
    }
}

pub fn i8_input() -> i8 {
    loop {
        match input().trim().parse() {
            Ok(value) => break value,
            Err(_) => {
                println!("Vous devez entrer un nombre!");
                continue;
            }
        }
    }
}
