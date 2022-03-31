mod user;
mod crenel;

fn main() {
    let crenels = crenel::get_crenels();
    dbg!(crenels);
}
