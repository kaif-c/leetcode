/** THOUGHT:
 * Loop over every character
 * A vector containing last opened bracket
 * If an open bracket encountered then it is simply pushed
 * If a close bracket encountered then the last open bracket checked and popped or error returned
*/
#include <string>
#include <unordered_map>
#include <vector>
using namespace std;

// Time Complexity: O(N)
class Solution {
public:
    bool isValid(const string s) {
        vector<char> brackets = {};
        unordered_map<char, const char> bracket_pairs = {
            {')', '('},
            {'}', '{'},
            {']', '['}
        };

        for (const char c: s) switch (c) {
            // Open Brackets
            case '(':
            case '{':
            case '[':
                brackets.push_back(c);
                continue;

            // Close brackets
            case ')':
            case '}':
            case ']':
                if (brackets.empty() || brackets[brackets.size() - 1] != bracket_pairs[c])
                    return false;
                brackets.pop_back();
                continue;
        }

        // If the brackets are not empty then one or more wasnt closed
        return brackets.empty();
    }
};
