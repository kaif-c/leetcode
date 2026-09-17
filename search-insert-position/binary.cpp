#include <cstdint>
#include <vector>
using namespace std;

/// Array is sorted, so use binary search while works with time = O(logN)
/// Space = O(1)
class Solution {
public:
    int searchInsert(vector<int>& nums, int target) {
        /// Bounds of our search
        int32_t upper = nums.size() - 1;
        int32_t lower = 0;

        if (nums.back() < target)
            return nums.size();
        if (nums.front() > target)
            return 0;

        while (lower < upper) {
            /// This to reduce overflow exceptions
            const uint32_t mid = lower + (upper - lower) / 2;
            if (nums[mid] == target)
                return mid;

            /// We must cut our bound range in half
            if (target < nums[mid])
                /// Do not add 1 as this may be where the value should be inserted
                upper = mid;
            else
                /// We know <= mid is never possible
                /// We add 1 to make sure lower can = upper
                lower = mid + 1;
        }
        return upper;
    }
};
