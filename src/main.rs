use std::io::Read;

fn main() {
    let mut input = String::new();
    let mut stdin = std::io::stdin();
    stdin.read_to_string(&mut input).unwrap();
    let json = serde_json::from_str::<serde_json::Value>(&input).unwrap();
    println!("{}", serde_json::to_string_pretty(&json).unwrap());
}
