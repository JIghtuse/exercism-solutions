#include "anagram.h"
#include <algorithm>
#include <iostream>
#include <vector>

using namespace std;

namespace anagram {

vector<char> sorted_chars(const string& s) {
    vector<char> chars(s.length());
    for (char c : s)
        chars.push_back(c);
    sort(chars.begin(), chars.end());
    return chars;
}

bool is_anagram(const string& a, const string& b) {
    if (a.length() != b.length())
        return false;

    return sorted_chars(a) == sorted_chars(b);
}

vector<string> anagram::matches(const initializer_list<const string> inputs)
{
    vector<string> res;
    for (const string& input : inputs)
        if (is_anagram(word, input))
            res.push_back(input);
    return res;
}

}
