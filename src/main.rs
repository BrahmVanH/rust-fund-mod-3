mod string_manipulation;

fn main() {

  let sentence = String::from("Lenny jumped on Odins back and he freaked out");

    // 4 - String Manipulation
    string_manipulation::string_manipulation();

    string_manipulation::longest_word(&sentence);
}
