#ifndef GRADE_SCHOOL_H
#define GRADE_SCHOOL_H

#include <map>
#include <string>
#include <vector>

namespace grade_school {

using Names = std::vector<std::string>;
using Roster = std::map<int, Names>;

class school {
public:
    Roster roster() const { return roster_; }
    void add(const std::string& name, int grade);
    Names grade(int grade) const;
private:
    Roster roster_;
};

}

#endif // GRADE_SCHOOL_H
