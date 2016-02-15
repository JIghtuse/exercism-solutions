#include "grade_school.h"

using namespace grade_school;

void school::add(const std::string& name, int grade)
{
    roster_[grade].push_back(name);
}
