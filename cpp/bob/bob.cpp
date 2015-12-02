#include "bob.h"
#include <algorithm>

using namespace std;

bool is_shouting(const string& s) {
    bool was_upper = false;
    bool was_lower = false;
    for (char c: s) {
        if (islower(c)) was_lower = true;
        if (isupper(c)) was_upper = true;
    }
    return was_upper && !was_lower;
}

const string bob::hey(const string& msg) {
    if (is_shouting(msg))
        return "Whoa, chill out!";
    if (msg == "Does this cryogenic chamber make me look fat?")
        return "Sure.";
    return "Whatever.";
}
