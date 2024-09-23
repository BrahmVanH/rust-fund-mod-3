fn get_item(index: usize, numbers: Vec<i32>) -> i32 {
  // take in a var index integer
  // create vector of series of numbers
 

  // retrieve value from vector at index
  let gotted = &numbers[index];
  // print value
  println!("Gotted value: {}", gotted);

  gotted.to_owned()
}

fn main() {
  // create vec integers
  let numbers = vec!(1, 2,  3, 4, 5);

  // call get item

  get_sum_of_elements(&numbers);

  // retrieve a value from any index in vec
  let spanish = &numbers[3];
  let first_value = &numbers[1];
  // print this value
  println!("spanish: {}", spanish);
  // retrieve the first value from the vec using a method and pattern matching to handle the option
  match numbers.first() {
    Some(first_value) => println!("First numbers: {}", first_value),
    None => println!("No numbers found"),
  }
}

// function
//  takes vector as input and returns the sum of all its elements
fn get_sum_of_elements(ns: &Vec<i32>) -> i32 {
  let mut sum = 0;

  for n in ns {
    sum = sum + n;
  }

  println!("sum: {}", sum);

  sum

}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_get_item() {
    let index = 4;
    let nums: Vec<i32> = vec!(1, 2, 3, 4, 5);
    assert_eq!(get_item(2, nums), 3);
  }
  #[test]
  fn test_get_sum_of_elements() {
    let ns: Vec<i32> = vec!(1, 2, 3);
    assert_eq!(get_sum_of_elements(&ns), 6);
  }
}