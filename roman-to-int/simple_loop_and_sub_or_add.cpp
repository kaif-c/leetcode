/** THOUGHT:
 * We can just have a table that links every character to a number
 * then we look it up for each character
 *
 * IV, IX, CD?
 * Smaller is before larger
 * We are guarenteed a VALID roman number
 * so we will subtract smaller if it is before larger but add if its after
*/
#include <iostream>
#include <string>
#include <utility>
using namespace std;

// Array instead of map as its faster for data this small
const static pair<const char, int> maps[] = {
    {'I', 1},
    {'V', 5},
    {'X', 10},
    {'L', 50},
    {'C', 100},
    {'D', 500},
    {'M', 1000}
};

// Get equivalents for each character
static inline const int num_from_rom(const char c) {
    for (const auto p: maps) {
        if (p.first == c)
            return p.second;
    }

    cout << "ERROR THIS SHOULD NOT HAPPEN\n";
    return 0;
}

class Solution {
public:
    int romanToInt(const string s) {
        int num = 0;
        for (int i = 0; i < s.size(); ++i) {
            const int cur_num = num_from_rom(s[i]);
            // Subtract smaller number if before larger
            if (i < s.size() - 1 && cur_num < num_from_rom(s[i+1])) {
                num -= cur_num;
            }
            else 
                num += cur_num;
        } // For loop
        return num;
    }
};
