#ifndef PHONE_NUMBER_H
#define PHONE_NUMBER_H

#include <experimental/string_view>

class phone_number {
public:
    phone_number(std::experimental::string_view s):
        s_{s}
    {}
    std::experimental::string_view number() const { return s_; }
private:
    std::experimental::string_view s_;
};

#endif // PHONE_NUMBER_H
