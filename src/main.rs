use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    
    let secret_number = rand::thread_rng().gen_range(1, 101);
    
    loop {
        println!("Guess the number!");
    
        let mut guess = String::new();
        
        println!("please input your guess");
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Fail to read line");
        
        let guess: u32 =  match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        println!("You guess: {}", guess);
        
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small! the number was {}!", secret_number ),
            Ordering::Greater => println!("Too Big! the number was {}!", secret_number),
            Ordering::Equal => {
                println!("You Win! the number was {}!", secret_number);
                break;
            }
        }
    }
}
