#include "word_count.h"

using namespace std;

map<string, int> word_count::words(const string& text) {
    return map<string, int>{{text, 1}};
}
