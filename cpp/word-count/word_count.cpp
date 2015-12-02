#include "word_count.h"
#include <sstream>

using namespace std;

map<string, int> word_count::words(const string& text) {
    map<string, int> result;

    string word;
    for (istringstream iss{text}; iss >> word;) {
        result[word] += 1;
    }

    return result;
}
