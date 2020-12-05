#include <fstream>
#include <string>
#include <iostream>
using namespace std;

const char dash = '-', space = ' ', colon = ':', newLine = '\n';

int day2() {
    int valid = 0, charCount, min, max, pos;
    char match;
    string s; 
    ifstream in ("data.txt");
    if (in.is_open()){
        for (int i = 0; i < 1000; ++i) {
            getline(in, s, dash);
            min = stoi(s);
            getline(in, s, space);
            max = stoi(s);
            getline(in, s, colon);
            match = s[0]; //hopefully??
            in.ignore(); // ignores 1 character
            getline(in, s, newLine);
            // line is now parsed into bits that I care about
            // min and max are what you think
            // match is the character to match
            // s is the 'password'

            charCount = 0;
            for (char c : s) {
                if (c == match)
                    ++charCount;
            }

            if (charCount >= min && charCount <= max)
                valid++;
        }
        in.close();
    }
    return valid;
}

int main () {
    cout << day2() << "\n";
    return 0;
}
