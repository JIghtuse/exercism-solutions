#ifndef NUCLEOTIDE_COUNT
#define NUCLEOTIDE_COUNT

#include <experimental/string_view>
#include <map>

namespace dna {

class counter {
public:
    counter(std::experimental::string_view strand);

    std::map<char, int> nucleotide_counts() const;
    int count(char nucleotide) const;
private:
    std::experimental::string_view strand;
    std::map<char, int> stats;
};

}

#endif // NUCLEOTIDE_COUNT
