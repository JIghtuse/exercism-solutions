#ifndef GRADE_SCHOOL_H
#define GRADE_SCHOOL_H

#include <map>
#include <string>
#include <vector>

namespace grade_school {

class school {
public:
    auto roster() const { return roster_; }
private:
    std::map<int, std::vector<std::string>> roster_;
};

}

#endif // GRADE_SCHOOL_H
