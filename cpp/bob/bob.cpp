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

bool is_question(const string& s) {
    return s[s.length() - 1] == '?';
}

string rtrim(const string& s) {
    ssize_t pos = s.length() - 1;

    while (pos >= 0 && isspace(s[pos]))
        --pos;
    return s.substr(0, pos + 1);
}

const string bob::hey(const string& s) {
    string msg = rtrim(s);
    if (is_shouting(msg))
        return "Whoa, chill out!";
    if (is_question(msg))
        return "Sure.";
    return "Whatever.";
}
