// function add elements to vector using push

fn using_push(mut ns: Vec<i32>, n: i32) -> Vec<i32> {
  ns.push(n);
  ns
}

// Extend adds contents of other iterable to self vec
fn using_extend(mut ns: Vec<i32>, xs: &[i32]) -> Vec<i32> {
  ns.extend(xs);
  ns
}


// Append will add other vector to self, requires other 
// vec to be mutably borrowed
fn using_append(mut ns: Vec<i32>, xs: &mut Vec<i32>) -> Vec<i32> {
  ns.append(xs);
  ns
}

//  inserts provided elements into vec at provided index
fn using_insert(mut ns: Vec<i32>, n: i32, i: usize) -> Vec<i32> {
  ns.insert(i, n);
  ns
}

// Challenge question
fn instert_beginning_and_end(mut ns: Vec<i32>, n: i32) -> Vec<i32> {
  ns.insert(0, n);
  ns.push(n);
  ns

}

// Appends second vector to first vector
fn append_vectors(mut vs: Vec<i32>, xs: &mut Vec<i32>) -> Vec<i32> {
  vs.append(xs);
  vs
}

fn main() {

}


#[cfg(test)]
mod tests {

  use super::*;
  #[test]
  fn test_using_push() {
    let n = 1;
    let mut ns = vec!(4, 3, 2);
    let result = vec!(4, 3, 2, 1);

    assert_eq!(using_push(ns, n), result);
  }

  #[test]

   fn test_using_extend() {
    let xs = [1, 2];
    let ns = vec!(4, 3, 2);
    let result = vec!(4, 3, 2, 1, 2);

    assert_eq!(using_extend(ns, &xs), result);
  }
  #[test]

   fn test_using_append() {
    let mut xs = vec!(1, 2);
    let ns = vec!(4, 3, 2);
    let result = vec!(4, 3, 2, 1, 2);

    assert_eq!(using_append(ns, &mut xs), result);
  }
  #[test]

   fn test_using_insert() {
    let n = 1;
    let i = 0;
    let ns = vec!(4, 3, 2);
    let result = vec!(1, 4, 3, 2);

    assert_eq!(using_insert(ns, n, i), result);
  }

  #[test]
  fn test_insert_beginning_and_end() {
    // Function should take a vec and a value and insert
    // the value at the beginning and end of vector
    let n = 4;
    let ns = vec!(1, 2, 3, 4, 5);
    let result = vec!(4, 1, 2, 3, 4, 5, 4);
    assert_eq!(instert_beginning_and_end(ns, n), result);
  }
  #[test]

  fn test_append_vectors() {
    let mut vs = vec!(1, 2, 3, 4, 5);
    let mut xs: Vec<i32> = vec!(6, 7, 8);
    let result = vec!(1, 2, 3, 4, 5, 6, 7, 8);

    assert_eq!(append_vectors(vs, &mut xs), result)
  }
}