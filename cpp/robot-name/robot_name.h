#pragma once

#include <string>

namespace robot_name {

class robot {
public:
    robot();
    std::string name() const;

private:
    std::string m_name;
};

} // namespace robot_name
