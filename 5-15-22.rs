
// Complete the function that accepts a string parameter, and reverses each word in the string. All spaces in the string should be retained.

// Examples
// "This is an example!" ==> "sihT si na !elpmaxe"
// "double  spaces"      ==> "elbuod  secaps"

fn reverse_words(str: &str) -> String {
    let mut word_vec = Vec::<String>::new();
    let mut rev_string = String::new();
    for word in str.split(" "){
        word_vec.push(word.to_string());
    }
    for i in word_vec.iter(){
        // println!("{}", i);
        for z in i.chars().rev(){
            rev_string.push(z);
        }
        rev_string.push(' ');
    }
    println!("{}", rev_string);
    // println!("{}", word_vec.join(""));
    rev_string.pop();
    return rev_string
}
fn main(){
    reverse_words("The quick brown fox jumps over the lazy dog.").to_string();
}