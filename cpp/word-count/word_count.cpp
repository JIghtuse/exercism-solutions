#include "word_count.h"
#include <boost/algorithm/string.hpp>
#include <boost/algorithm/cxx11/one_of.hpp>

using namespace boost::algorithm;
using namespace std;

map<string, int> word_count::words(const string& text)
{
    map<string, int> result;

    auto terminator = is_space() || is_from_range(',', ',');

    typedef split_iterator<string::const_iterator> sp_iter;
    for (sp_iter it = make_split_iterator(text, token_finder(terminator, token_compress_on));
         it != sp_iter{};
         ++it) {
        auto word = boost::copy_range<string>(*it);
        trim_if(word, !is_alnum());
        to_lower(word);
        if (!word.empty())
            result[word] += 1;
    }

    return result;
}
