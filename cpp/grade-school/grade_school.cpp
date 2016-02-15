#include <algorithm>
#include "grade_school.h"

using namespace grade_school;

void school::add(const std::string& name, int grade)
{
    roster_[grade].push_back(name);
    sort(roster_[grade].begin(), roster_[grade].end());
}

Names school::grade(int grade) const
{
    return roster_.at(grade);
}
