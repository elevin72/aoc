#include <cstdio>
#include <fstream>
#include <iostream>
#include <string>
using namespace std;

int day3part1() {
    int passed = 0;
    int pos = 0;
    ifstream in ("data.txt");
    if (in.is_open()) {
        string line;
        while (getline(in, line)) {
            if (line.at(pos % 31) == '#') {
                ++passed;
            }
            pos+=3;
        }
        cout << endl;

    }
    return passed;

}

long day3part2(int hor, int vert) {
    long passed = 0;
    int pos = 0;
    ifstream in ("data.txt");
    if (in.is_open()) {
        string line;
        while (getline(in, line)) {
            int vertCount = vert;
            if (line.at(pos % 31) == '#') {
                ++passed;
            }
            pos+=hor;
            while (vertCount > 1) {
                vertCount--;
                string garbage;
                getline(in, garbage);
            }
        }
        cout << endl;
    }
    return passed;
}

int main() {
    
    cout << day3part2(1,1)
    *day3part2(3,1)
    * day3part2(5,1)
    * day3part2(7,1)
    * day3part2(1,2) << endl;
}


