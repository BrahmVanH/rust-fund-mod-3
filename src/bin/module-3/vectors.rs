
// Use slices when: 
// - you want to borrow a portion of a collection rather than the whole collection
// - you want to pass around a reference to a sequence of items without copying them
// - you want to access a subset of a collection without copying
// Use vectors when:
// - you need to dynamically grow or shrink your collection
// - you need to own the collection and transfer ownership to another scope

fn main() {
  ownership();
  modifiable();
  
}

fn ownership() {
  // create vec of sequence of numbers
  let numbers = vec!(1, 2, 3, 4);
  // create slice from all numbers in vec
  let slice = &numbers[..];
  // print slice
  println!("Slice: {:?}", slice);

  
}
fn modifiable() {
  // create vec numbers same as before
  let mut numbers = vec!(1, 2, 3, 4);

  // create slice of numbers
  let slice = &mut numbers[..];

  // change the value of one of the slice elements to something else
  slice[0] = 10;

  // print slice
  println!("Slice: {:?}", slice);
}

