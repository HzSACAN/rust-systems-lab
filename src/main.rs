use std::io;

fn main() {
    println!("Bir tam Sayı giriniz:");

    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Girdi okunamadı");

    let number: i32 = match input.trim().parse() {
        Ok(value) => value,
        Err(_) => {
            eprintln!("Geçerli bir tam sayı girmediniz.");
            return;
        }
    };

    println!("{number} sayısının karesi: {}", number * number);
}
