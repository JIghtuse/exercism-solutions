#ifndef NUCLEOTIDE_COUNT
#define NUCLEOTIDE_COUNT

#include <experimental/string_view>
#include <map>

namespace dna {

class counter {
public:
    counter(std::experimental::string_view strand):
        strand{strand}
    {}

    std::map<char, int> nucleotide_counts() const;
private:
    std::experimental::string_view strand;
};

}

#endif // NUCLEOTIDE_COUNT
