// Complete the solution so that it returns true if the first argument(string) passed in ends with the 2nd argument (also a string).

// Examples:

// solution('abc', 'bc') // returns true
// solution('abc', 'd') // returns false

#include <string>
#include <iostream>


bool solution(std::string const &str, std::string const &ending) {
    const int end_size = ending.length();
    const int str_size = str.length();
    bool is_similar = true;
    if (str_size < end_size) {
        is_similar = false;
    }

    for(int i = 1; i <= end_size; i++){
        if(str[str_size - i] != ending[end_size - i]){
           is_similar =  false;
           break;
        }
      }
    return is_similar;
}

int main(){
    //solution('abc', 'bc');
    std::cout << solution("abc", "bc") << "\n";
    return 0;
}