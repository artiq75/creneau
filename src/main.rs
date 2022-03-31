mod crenel;
mod user;

fn main() {
    let mut formatted_crenels: Vec<String> = Vec::new();

    let crenels = crenel::get_crenels();
    for crenel in &crenels {
        formatted_crenels.push(crenel.to_string());
    }

    println!("Le magasin ouvre: {}", formatted_crenels.join(" et "));
}
