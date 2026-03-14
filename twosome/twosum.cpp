#include <vector>
#include <unordered_map>
using namespace std;

class Solution {
public:
    // Time Complexity: O(n)
    // Space Complexity: O(n)
    vector<int> twoSum(vector<int>& nums, int target) {
        // Map of every element (key) to its index (value) 
        unordered_map<int, int> map;

        // Create the map
        map.reserve(nums.size());
        for (int i = 0; i < nums.size(); ++i) // O(n)
            map.insert({nums[i], i});

        for (int i = 0; i < nums.size(); ++i) { // O(n)
            // Find if an element exists that if added to nums[i]
            // will result in target
            auto it = map.find(target - nums[i]);
            // Checking if it exists AND that there isnt a repeat
            if (it != map.end() && it->second != i)
                return {it->second, i};
        }

        // Error case
        // should never happen with the constraints
        return {};
    }
};
