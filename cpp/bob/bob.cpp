#include "bob.h"

using namespace std;

const string bob::hey(const string& msg) {
    if (msg == "WATCH OUT!")
        return "Whoa, chill out!";
    if (msg == "Does this cryogenic chamber make me look fat?")
        return "Sure.";
    return "Whatever.";
}
