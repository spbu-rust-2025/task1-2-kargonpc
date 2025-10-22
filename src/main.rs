use std::io;

fn main() {
    let mut total_sum: i32 = 0;

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input = input.trim().to_string();
        if input == "-1" {
            break;
        }
        match input.parse::<i32>() {
            Ok(n) => {
                total_sum += n;
            }
            Err(_) => {
                println!("NaN");
                return;
            }
        }
    }
    println!("{}", total_sum);
}