#include <functional>
#include <string>
#include <string_view>
using namespace std;

class Solution {
public:
    int strStr(string haystack, string needle) {
        auto needle_hash = hash<string>{}(needle);
        for (int i = 0; i < haystack.size(); ++i) {
            if (haystack[i] == needle[0]) {
                string_view hay_view = string_view(haystack.data() + i, needle.size());
                if (hash<string_view>{}(hay_view) == needle_hash)
                    return i;
            }
        }
        return -1;
    }
};
