extern crate rand;
use std::io::{self, Write};
use std::cmp::Ordering;
use rand::Rng;

fn main () {
  let mut counter = 0;

  loop {
    let guess = loop {
      println!("Guess the Secret Number");
      io::stdout().flush().unwrap();

      let mut _guess = String::new();
      io::stdin().read_line(&mut _guess).expect("Please enter a valid Input");

      match _guess.trim().parse::<i64>(){
        Ok(temp) => break temp,
        Err(_) => {println!("Please enter a valid input");}
      }
    
    };

    let secret_number = rand::thread_rng().gen_range(1, 1001);
    match guess.cmp(&secret_number){
      Ordering::Less => {println!("Too Low, Try Again");},
      Ordering::Greater => {println!("Too High, Try Again")},
      Ordering::Equal => {println!("You Win!!!"); break;}
    }

    counter += 1;
  }
  println!("It took you {} attempts to guess the correct number", counter);
}


/*fn main() {

  let mut counter = 0;
  

loop {
    let guess = loop{
      println!("Guess the Secret Number");
      io::stdout().flush().unwrap();
      let mut _guess = String::new();
      io::stdin().read_line(&mut _guess).expect("Failed to read input");
      match _guess.trim().parse::<i32>() {
          Ok(temp) => break temp,
          Err(_) => {println!("Please enter a valid input");}
      }
    };

    let secret_number = rand::thread_rng().gen_range(1, 1001);

    match guess.cmp(&secret_number){
        Ordering::Less => {println!("Too Small, Try Again");},
        Ordering::Greater => {println!("Too Big, Try Again");},
        Ordering::Equal => {println!("You Win!!!");break;}
    }
    counter += 1;


}
println!("It took you {} guesses to get the Secret Number", counter);
} */