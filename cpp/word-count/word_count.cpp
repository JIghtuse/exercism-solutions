#include "word_count.h"
#include <boost/algorithm/string.hpp>

using namespace boost::algorithm;
using namespace std;

map<string, int> word_count::words(const string& text) {
    map<string, int> result;

    auto terminator = is_space() || is_punct();
    const string s = trim_copy_if(text, terminator);

    typedef split_iterator<string::const_iterator> sp_iter;
    for (sp_iter it = make_split_iterator(s, token_finder(terminator, token_compress_on));
         it != sp_iter{};
         ++it) {
        auto key = boost::copy_range<string>(*it);
        to_lower(key);
        result[key] += 1;
    }

    return result;
}
