//! Write a function that returns the largest element in a list.
//! Basic implementation

fn main() {
    let vec = vec![2,6,5,0,10,23,12];
    println!("The largest number is {} ", find_largest_num(vec));
}

fn find_largest_num(list: Vec<u32>) -> u32 {
  let mut largest: u32 = 0;
  
  for x in &list {
      if x > &largest {
          largest = *x;
      }
  }
  largest
}
