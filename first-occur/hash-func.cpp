#include <functional>
#include <string>
#include <string_view>
using namespace std;

/**
 * THOUGHT:
 * Instead of O(n^2) a O(n) approach woould be better since we dont know the sizes
 * We can use hash functions for reducing time complexity
*/

class Solution {
public:
    int strStr(const string haystack, const string needle) {
        const hash<string> str_hasher;
        const hash<string_view> view_hasher;
        const size_t needle_hash = str_hasher(needle);
        // Smaller stack edge case
        if (needle.size() > haystack.size())
            return -1;
        // O(n) loop
        for (int i = 0; i < haystack.size() - needle.size() + 1; ++i) {
            // Only check if first character matches
            if (haystack[i] == needle[0]) {
                string_view hay_view = haystack;
                hay_view = hay_view.substr(i, needle.size());
                // Found match
                if (view_hasher(hay_view) == needle_hash)
                    return i;
            }
        }
        return -1;
    }
};
