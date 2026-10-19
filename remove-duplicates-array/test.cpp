#include "./better-two-pointer.cpp"
#include <iostream>
#include <string>

struct Case {
public:
    vector<int> input;
    struct Output {
    public:
        std::vector<int> out_arr;
        int num_of_unique;
    };
    Output output;
    Case(vector<int> in, Output out): input(in), output(out) {}
};

Case cases[] = {
    { {1, 1, 2},                      { .out_arr = {1, 2},           .num_of_unique = 2 } },
    { {0, 0, 1, 1, 1, 2, 2, 3, 3, 4}, { .out_arr = {0, 1, 2, 3, 4 }, .num_of_unique = 5 } },
};

string vec_to_str(const vector<int> &num) {
    string str = "[";
    for (size_t i = 0; i < num.size(); ++i) {
        str += to_string(num[i]);
        str += (i == num.size() - 1 ? "" : ", ");
    }
    return str + ']';
}

int main() {
    Solution sol;
    for (auto &cur_case: cases) {
        const int old_len = cur_case.input.size();
        const auto k = sol.removeDuplicates(cur_case.input);
        if (cur_case.input.size() != old_len) {
            std::cerr << "Size mismatch, got: " << cur_case.input.size()
                << ", but expected: " << old_len << '\n';
            continue;
        }

        if (k != cur_case.output.num_of_unique) {
            std::cerr << "Unique element mismatch, got: " << k
                << ", but expected: " << cur_case.output.num_of_unique << '\n';
            continue;
        }

        bool skip = false;
        for (int i = 0; i < k; ++i) {
            if (cur_case.input[i] != cur_case.output.out_arr[i]) {
                std::cerr << "Array element mismatch, got: " << vec_to_str(cur_case.input)
                    << ", but expected: " << vec_to_str(cur_case.output.out_arr) << '\n';
                skip = true;
                break;
            }
        }
        if (skip)
            continue;

        std::cout << "SUCCESS\n";
    }
    return 0;
}
