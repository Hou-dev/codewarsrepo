// Write a function that takes in a string of one or more words, and returns the same string, 
// but with all five or more letter words reversed (Just like the name of this Kata). Strings passed in 
// will consist of only letters and spaces. Spaces will be included only when more than one word is present.

// Examples: spinWords( "Hey fellow warriors" ) => returns "Hey wollef sroirraw" spinWords
// ( "This is a test") => returns "This is a test" spinWords( "This is another test" )=> returns "This is rehtona test"

fn spin_words(words: &str) -> String {
    let mut v_word = Vec::<String>::new();
    let mut result = Vec::<String>::new();
    for word in words.split(" "){
        v_word.push(word.to_string());
    }
    for i in v_word{
        if i.len() > 4{
            let mut m = String::new();
            for z in i.chars().rev(){
                m.push(z);
            }
            result.push(m)
        }else{
            result.push(i.to_string());
        }
    }
    let final_w = result.join(" ");
    //println!("{}",final_w);
    return final_w;
    // println!("{result:?}");
    todo!();
}

fn main(){
    spin_words("Just kidding there is still one more");
}