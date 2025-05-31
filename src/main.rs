extern crate rand;
use std::io::{self, Write};
use std::cmp::Ordering;
use rand::Rng;

fn main(){
    println!("welcome to the Guessing game!!");
    
    let mut counter: i32 = 0; 

    loop {
let guess = loop {
    println!("Please enter your guess");
    io::stdout().flush().unwrap();
    let mut _guess = String::new();
    io::stdin().read_line(&mut _guess).expect("Failed to read input");
    match _guess.trim().parse::<i32>(){
        Ok(temp ) => break temp,
        Err(_) => {println!("Please enter a valid input");}
    }
    
};

let secret_number = rand::thread_rng().gen_range(1, 101);

match guess.cmp(&secret_number) {
    Ordering::Less => println!("Too Small"),
    Ordering::Greater => println!("Too Big"),
    Ordering::Equal => { println!("You Win!!!");
                            break;                
                                }

}

counter +=1;

   }

   println!("{} Times", counter);

}