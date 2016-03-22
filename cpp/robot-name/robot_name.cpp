#include "robot_name.h"

namespace robot_name {

robot::robot()
    : m_name{ "AB123" }
{
}

std::string robot::name() const
{
    return m_name;
}

} // namespace robot_name
