#include "anagram.h"
#include <algorithm>
#include <vector>

using namespace std;

namespace anagram {

string to_lower(const string& s)
{
    string smod{ s };
    transform(smod.begin(), smod.end(), smod.begin(), ::tolower);
    return smod;
}

bool is_anagram(const string& word, const string& input)
{
    if (word.length() != input.length())
        return false;

    auto word_lower = to_lower(word);
    auto input_lower = to_lower(input);

    if (word_lower == input_lower)
        return false; // word is not an anagram to itself

    sort(word_lower.begin(), word_lower.end());
    sort(input_lower.begin(), input_lower.end());
    return word_lower == input_lower;
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
