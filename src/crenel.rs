use crate::user;

fn get_begin() -> i8 {
    loop {
        let value = user::get_i8_input();
        if value < 0 || value > 23 {
            println!("Le nombre doit être >= 0 ou <= 23");
            continue;
        }
        break value;
    }
}

fn get_end(begin: i8) -> i8 {
    loop {
        let value = user::get_i8_input();
        if value < 0 || value > 23 {
            println!("Le nombre doit être >= 0 ou <= 23");
            continue;
        }
        if value <= begin {
            println!("L'heure de fin doit être inférieur à l'heure de début!");
            continue;
        }
        break value;
    }
}

pub fn get_crenel() -> [i8; 2] {
    println!("Debut: ");
    let begin = get_begin();

    println!("Fin: ");
    let end = get_end(begin);

    [begin, end]
}

pub fn get_crenels() -> Vec<[i8; 2]> {
    let mut crenels = Vec::new();
    loop {
        crenels.push(get_crenel());
        if !user::is_want_continue() {
            break;
        }
    }
    crenels
}
