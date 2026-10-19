/// Since the vector is sorted any copies are on the same area
/// We have an offset to the next unique and the old value
/// if old value is same as current then we put index+offset there and increment offset
#include <vector>
using namespace std;

class Solution {
public:
    int removeDuplicates(vector<int>& nums) {
        int cur_offset = 0;
        int old_val = 200; // 200 is out of constraints
        int unique = nums.size();

        for (int i = 0; i < nums.size(); ++i) {
            /// Only rpeating values left
            if (i + cur_offset >= nums.size())
                break;
            if (nums[i + cur_offset] == old_val) {
                /// As multiple copies can exist
                while (nums[i + cur_offset] == old_val) {
                    ++cur_offset;
                    --unique;

                    /// Only rpeating values left
                    if (i + cur_offset >= nums.size())
                        return unique;
                }
            }
            /// Set the current one to the offset
            /// May lead to redundant writes
            nums[i] = nums[i + cur_offset];
            /// Setup for next iteration
            old_val = nums[i];
        }
        return unique;
    }
};
