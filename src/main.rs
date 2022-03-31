mod user;
mod crenel;

fn main() {
    let mut formatted_crenels: Vec<String> = Vec::new();
    
    let crenels = crenel::get_crenels();
    for [begin, end] in &crenels {
        formatted_crenels.push(format!("de {}h à {}h", begin, end));
    }
    
    println!("Le magasin ouvre: {}", formatted_crenels.join(" et "));
}
