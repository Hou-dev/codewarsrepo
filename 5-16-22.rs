// Remove the parentheses
// In this kata you are given a string for example:

// "example(unwanted thing)example"
// Your task is to remove everything inside the parentheses as well as the parentheses themselves.

// The example above would return:

// "exampleexample"
// Notes
// Other than parentheses only letters and spaces can occur in the string. Don't worry about other brackets like "[]" and "{}" as these will never appear.
// There can be multiple parentheses.
// The parentheses can be nested.

fn remove_parentheses(s: &str) -> String {
    let mut r_parentheses = Vec::new();
    // let mut result = String::new();
    // let mut put_into = true;
    // let mut left_parent = 0;
    // for i in s.chars(){
    //     if i == '('{
    //         put_into = false;
    //     }
    //     else if i == ')'{
    //         put_into = true;
    //     }else if put_into == true{
    //             r_parentheses.push(i);
    //     }
    // }
    let mut count = 0;
    for i in s.chars(){
        if i == '('{
            count = count + 1;
        }
        if count == 0{
            r_parentheses.push(i);
        }
        if i == ')'{
            count = count - 1;
        }

    }
    let mut result: String = r_parentheses.iter().collect();
    //result = result.replace("(", "").replace(")","");

    println!("{}",result);
    return result;
}

fn main(){
    remove_parentheses("hello example (wordz(more words) here) something");
}