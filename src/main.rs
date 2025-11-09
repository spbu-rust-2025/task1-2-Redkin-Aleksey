use std::io;

fn main() {

    let mut sum: i64 = 0;

    loop {
        let mut input = String::new();
        if let Err(e) = io::stdin().read_line(&mut input) {
            eprintln!("input error: {}", e);
            continue;
        }

        let number: i64 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("NaN");
                return
            }
        };

        if number == -1 {
            break;
        }
        sum += number;
    }

    println!("{sum}");
}
