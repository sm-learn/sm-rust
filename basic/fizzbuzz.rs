//! Write a short program that prints each number from 1 to 100 on a new line. 
//! For each multiple of 3, print "Fizz" instead of the number. 
//! For each multiple of 5, print "Buzz" instead of the number. 
//! For numbers which are multiples of both 3 and 5, print "FizzBuzz" instead of the number.


fn main() {
    let n = 6;
    fizz_buzz(n);
}

fn fizz_buzz(n : u32) -> () {
  if is_divisible_by(n, 15) {
    println!("FizzBuzz");
  } else if is_divisible_by(n, 3) {
      println!("Fizz");
  } else if is_divisible_by(n, 5) {
      println!("Buzz");
  } else {
      println!("{}", n);
  }
}

fn is_divisible_by(numerator: u32, denominator: u32) -> bool {
    if denominator == 0 {
        return false;
    }
    numerator % denominator == 0
}
