use std::io;

fn main() {
    let mut s: i32 = 0;
    let mut found_invalid = false; // Флаг для отслеживания некорректных данных
    
    loop {
        let mut l = String::new();
        io::stdin().read_line(&mut l).unwrap();
        let l = l.trim();
        
        if l == "-1" {
            break;
        }
        
        match l.parse::<i32>() {
            Ok(n) if n > 0 => {
                s += n;
            }
            _ => {
                // Запоминаем, что встретились некорректные данные
                found_invalid = true;
            }
        }
    }
    
    // Проверяем, были ли некорректные данные
    if found_invalid {
        println!("NaN");
    } else {
        println!("{}", s);
    }
}