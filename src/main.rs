use std::io;
use std::num::ParseIntError;

fn main() {
    println!("Bir tam Sayı giriniz:");

    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Girdi okunamadı");

    let number = match parse_number(&input) {
        Ok(value) => value,
        Err(_) => {
            eprintln!("Geçerli bir tam sayı girmediniz.");
            return;
        }
    };

    let number_sign = sign(number);
    let number_parity = parity(number);
    let result = square(number);

    println!("{}", number);
    println!("Sayı {number_sign}");
    println!("Sayı {number_parity}");
    println!("{number}'in karesi: {result}");
}
fn parse_number(text: &str) -> Result<i32, ParseIntError> {
    text.trim().parse()
}

fn square(number: i32) -> i32 {
    return number * number;
}

fn sign(number: i32) -> &'static str {
    let no_sign = if number > 0 {
        "pozitiftir"
    } else if number < 0 {
        "negatiftir"
    } else {
        "sıfırdır"
    };
    return no_sign;
}

fn parity(number: i32) -> &'static str {
    let no_parity = if number % 2 == 0 {
        "çifttir"
    } else {
        "tektir"
    };
    return no_parity;
}
