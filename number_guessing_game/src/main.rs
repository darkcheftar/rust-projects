use rand::{Rng};
use std::io;
use std::io::{Write};
fn main() {
    let secret_num = rand::thread_rng().gen_range(1..101);
    let mut guesses = 0;
    loop{
        guesses += 1;
        let mut guess_str = String::new();
        print!("Enter Guess: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut guess_str).expect("Failed to read guess");
        println!("{}", guess_str);
        let guess:u8 = match guess_str.trim().parse(){
            Ok(num)=>num,
            Err(_)=>{
                println!("Please Enter and integer");
                continue;
            }
        };
        if guess==secret_num{
            println!("You did it in {} guesses!", guesses);
            break;
        }else if guess>secret_num {
            println!("Think a bit lower");
        }else{
            println!("Think a bit higher");
        }
    }
}
