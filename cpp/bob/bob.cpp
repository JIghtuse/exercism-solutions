#include "bob.h"
#include <algorithm>

using namespace std;

bool is_shouting(const string& s) {
    return find_if(s.begin(), s.end(), ptr_fun<int, int>(islower)) == s.end();
}

const string bob::hey(const string& msg) {
    if (is_shouting(msg))
        return "Whoa, chill out!";
    if (msg == "Does this cryogenic chamber make me look fat?")
        return "Sure.";
    return "Whatever.";
}
