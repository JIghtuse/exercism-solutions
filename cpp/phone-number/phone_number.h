#ifndef PHONE_NUMBER_H
#define PHONE_NUMBER_H

#include <experimental/string_view>
#include <string>
#include <sstream>

class phone_number {
public:
    phone_number(std::experimental::string_view sv);
    std::string number() const;

private:
    std::ostringstream oss;
};

#endif // PHONE_NUMBER_H
