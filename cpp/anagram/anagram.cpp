#include "anagram.h"

using namespace std;

namespace anagram {

vector<string> anagram::matches(const initializer_list<const string> inputs)
{
    vector<string> res;
    for (const string& input : inputs)
        if (input.length() == word.length())
            res.push_back(input);
    return res;
}

}
