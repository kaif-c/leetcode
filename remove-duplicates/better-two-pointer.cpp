#include <vector>
using namespace std;

class Solution {
public:
    int removeDuplicates(vector<int>& nums) {
        int unique_elem_index = 0;
        for (int i = 0; i < nums.size(); ++i) {
            /// The array before this is already cleared of duplicates
            if (nums[i] != nums[unique_elem_index])
                nums[++unique_elem_index] = nums[i];
        }
        return unique_elem_index + 1;
    }
};
