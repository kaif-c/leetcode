#include <cstdint>
#include <string>
#include <vector>
using namespace std;

// INFO:
// A simple loop of loops?
// I wonder if there are some weird O(1) or O(N) things
// Time=O(Nm) where N = strs[i].len and m = strs.len
// Space=O(m)

class Solution {
public:
    string longestCommonPrefix(vector<string>& strs) {
        /// Taking the entire first one as the prefix
        string prefix = strs.front();
        /// O(m) where m = strs.size()
        for (uint32_t cur_str_ind = 1;
                cur_str_ind < strs.size();
                ++cur_str_ind
            ) {
            const auto &cur_str = strs[cur_str_ind];
            if (cur_str.size() < prefix.size())
                prefix.erase(cur_str.size());

            int32_t i = prefix.size() - 1;
            /// Loop over the string to find longest common
            /// O(N)
            while (i >= 0) {
                if (prefix.empty())
                    return prefix;

                if (prefix[i] != cur_str[i]) {
                    /// Erase everything in front
                    /// so that COMMON, UNCOMMON, COMMON works
                    prefix.erase(i);
                    i = prefix.size() - 1;
                }
                else
                    --i;
            }
        }
        return prefix;
    }
};
