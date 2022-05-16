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
    let mut count = 0;
    let mut start = 0;
    let mut end;
    let mut range;

    for i in s.chars(){
        count += 1;
        r_parentheses.push(i);
        if i == '('{
            start = count;
        }else if i == ')'{
            end = count;
            range = end - start;
            for _n in 0..= range{
                r_parentheses.pop();
            }
        }else{
            continue;
        }
        count = count + 0;
        r_parentheses.push(i);
    }
    let mut result: String = r_parentheses.iter().collect();
    //result = result.replace("(", "").replace(")","");

    println!("{}",result);
    return result;
}

fn main(){
    remove_parentheses("hello example (words(more words) here) something");
}