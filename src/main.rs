use std::io;

fn main() {
    let mut s = 0;
    loop {
        let mut l = String::new();
        
        io::stdin().read_line(&mut l).unwrap();
        let l = l.trim();

        if l == "-1" {
            break; 
        }

        match l.parse::<i32>() {
            Ok(n) if n > 0 => s += n,
            _ => { println!("NaN"); return; }
        }
    }
    println!("{}", s);
}