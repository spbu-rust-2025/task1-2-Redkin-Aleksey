use std::io;

fn main() {
    let mut sum: i64 = 0;
    let mut nan: bool = false;

    loop {
        let mut input = String::new();
        if let Err(e) = io::stdin().read_line(&mut input) {
            eprintln!("input error: {}", e);
            continue;
        }

        let number: i64 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                nan = true;
                continue;
            }
        };

        if number == -1 {
            break;
        }
        sum += number;
    }
    if nan {
        println!("NaN")
    }
    else {
        println!("{sum}")
    }
}
