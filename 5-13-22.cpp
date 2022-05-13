#include <iostream>
using namespace std;

int solution(int number)
{
    int sum = 0;
    for (int i = 0; i <= number; i++)
    {
        if (i % 3 == 0 || i % 5 == 0)
        {
            if( i == 0 || i == number){
                continue;
            }else{
                cout << i << "\n";
                sum +=i;
            }
        }
    }
    cout <<"This is the sum value: "<< sum << "\n";
    return sum;
}

int main(){
    solution(10);
    return 0;
}

