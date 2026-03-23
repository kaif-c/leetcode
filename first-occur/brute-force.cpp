#include <string>
using namespace std;

class Solution {
public:
    int strStr(string haystack, string needle) {
        if (haystack.size() < needle.size())
            return -1;

        for (int i = 0; i < haystack.size(); ++i) {
            if (haystack[i] == needle[0]) {
                int j;
                for (j = 0; j < needle.size(); ++j) {
                    if (haystack[i+j] != needle[j]) {
                        j = -1;
                        break;
                    }
                }
                if (j == needle.size())
                    return i;
            }
        }
        return -1;
    }
};
