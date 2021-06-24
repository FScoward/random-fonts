use std::env;

fn main() {
    println!("Hello, world!");

    let path_ame = "GFONT_API_KEY";
    println!("{}", env::var(path_ame).expect("GFONT_API_KEY not found."))
}
