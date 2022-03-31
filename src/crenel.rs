use crate::user;

use std::collections::HashMap;

const WEEK_DAYS: [&str; 7] = [
    "Lundi", "Mardi", "Mercredi", "Jeudi", "Vendredi", "Samedi", "Dimanche",
];

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

// Amêloriation à venir
pub fn get_week_crenels() -> HashMap<String, Vec<[i8; 2]>> {
    let mut week_crenels = HashMap::new();
    for day in WEEK_DAYS {
        println!("{}", day.to_uppercase());
        let crenels = get_crenels();
        week_crenels.insert(day.to_string(), crenels);
    }
    week_crenels
}
