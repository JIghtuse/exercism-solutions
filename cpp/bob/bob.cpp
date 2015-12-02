#include "bob.h"
#include <boost/algorithm/string.hpp>
#include <boost/algorithm/cxx11/none_of.hpp>
#include <boost/algorithm/cxx11/any_of.hpp>

using namespace boost::algorithm;
using namespace std;

bool is_shouting(const string& s) {
    return any_of(s, is_from_range('A', 'Z'))
        && none_of(s, is_from_range('a', 'z'));
}

bool is_question(const string& s) {
    return s[s.length() - 1] == '?';
}

const string bob::hey(const string& s) {
    const string msg = trim_right_copy(s);
    if (is_shouting(msg))
        return "Whoa, chill out!";
    if (is_question(msg))
        return "Sure.";
    if (msg.empty())
        return "Fine. Be that way!";
    return "Whatever.";
}
