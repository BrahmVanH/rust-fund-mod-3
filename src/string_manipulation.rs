
pub fn string_manipulation() {
  
  // todo!("Create string with whitespace");

  let sentence = String::from("Lenny jumped on Odins back and he freaked out");

  // todo!("Use slicing to get first three characters of sentence, print to console");
  // println!("The first three characters of the sentence are: {:?}", &sentence[0..3]);

  // todo!("create a format!ted string that takes in the sentence and adds a title, print to console");
  let butts = format!("Title: a little diddy about odin and lenny\n{}", sentence);
  print!("{}", butts);

  // todo!("iterated over the characters in a sentence and println with vowel if found, use pattern matching");
  let mut acc = 0;
  for c in sentence.to_lowercase().chars() {
    match c {
     'a' | 'e' | 'i' | 'o' | 'u' => {acc = acc + 1; print!("\nFound a vowel: {}, this is the {}th vowel ", c, acc)},
     _ => continue


    }
  }

  print!("There were {} vowels in the sentence", acc);


  // todo!("Split and collect sentence into vector of words and print to console");

  let words: Vec<&str> = sentence.split_whitespace().collect();
  print!("Vector of words {:?}", words);

  // todo!("split sentence into characters, reverse, collect into string")

  let reversed = sentence.chars().rev().collect::<String>();
  print!("Reversed string: {}", reversed);



}
// Print longest word in sentence
pub fn longest_word(sentence: &String) {
  let words = sentence.split_whitespace().collect::<Vec<&str>>();

  let mut longest_word = "";

  for word in words {
   if word.len() > longest_word.len() {
    longest_word = word;
   } 
  }

  println!("Longest word: {}", longest_word);
}


