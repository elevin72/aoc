#include <fstream>
#include <pthread.h>
#include <string>
#include <iostream>
using namespace std;

const char dash = '-', space = ' ', colon = ':', newLine = '\n';

struct parsedtext {
    int min;
    int max;
    char match;
    string password;
};

parsedtext parseText(ifstream& in) {
    parsedtext p;
    string util;
    getline(in, util, dash);
    p.min = stoi(util);
    getline(in, util, space);
    p.max = stoi(util);
    getline(in, util, colon);
    p.match = util[0];
    in.ignore();
    getline(in, util, newLine);
    p.password = util;
    return p;
}

int day2p1() {
    int valid = 0, charCount; 
    parsedtext p;
    ifstream in ("data.txt");
    if (in.is_open()){
        for(int i = 0; i < 1000; ++i) {

            p = parseText(in);

            charCount = 0;
            for (char c : p.password) { //I fricken love modern c++
                if (c == p.match)
                    ++charCount;
            }

            if (charCount >= p.min && charCount <= p.max)
                valid++;
        }

        in.close();
    }

    return valid;
}

int day2p2() {
    int valid = 0, charCount; 
    parsedtext p;
    ifstream in ("data.txt");
    if (in.is_open()){
        for(int i = 0; i < 1000; ++i) {

            p = parseText(in);

            charCount = 0;

            if ((p.match == p.password[p.min-1] && p.match != p.password[p.max-1])
                    || (p.match != p.password[p.min-1] && p.match == p.password[p.max-1] ))  // this is an ugly XOR
                ++valid;
        }

        in.close();
    }

    return valid;
}

int main () {
    cout << day2p1() << "\n";
    cout << day2p2() << "\n";
    return 0;
}
