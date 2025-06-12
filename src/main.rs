extern crate rand;
use std::io::{self, Write};
use std::cmp::Ordering;
use rand::Rng;

fn main () {
  /* let mut counter = 0;

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
  println!("It took you {} attempts to guess the correct number", counter); */

  let new_number: i32 = "31".parse().expect("Not a number");
  println!("The number is {}", new_number);

  let tup = (500, 6.4, 1);
  //let (x, y, z) = tup;

  println!("The value of x is: {}", tup.0);
  println!("The value of y is: {}", tup.1);
  println!("The value of z is: {}", tup.2);

  let x = tup.0;
  let y = tup.1;
  let z = tup.2;

  println!("The value of x is: {}", x);
  println!("The value of y is: {}", y);
  println!("The value of z is: {}", z);

  let a_array = [1, 2, 3, 4, 5];

  let months = ["January", "February", "March", "April",
                                    "May", "June", "July", "August", "September", 
                                    "October", "November", "December"];

  println!("The first month of the year is {}", months[0]);
let me = 5;
let you = {let a = 3; a + 1};

println!("The value of me is {}", me);
println!("The value of you is {}", you);

println!("{}", grade_converter('B'));

let number = 6;

if number % 2 == 0 {
  println!("number is divisible by 2");
} 
else if number % 4 == 0 {
  println!("number is divisible by 4");
}
else {
  println!("The number is not divisible by 2 or 4");
}

let condition = true;
let number = if condition {
  5
} else {
  6
};

println!("The value of the number is: {}", number);

}

fn grade_converter(grade: char) -> f64 {
  match grade {
    'A' => 5.0,
    'B' => 4.0,
    'C' => 3.0,
    'D' => 2.0,
    'F' => 0.0,
    _ => -1.0
  }
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