fn main() {
    println!("Инициализация языка программирования Rust...");
    say_hello("Yuri".to_string());
}

fn say_hello(name: String) {
    println!("Hello, {name}")
}