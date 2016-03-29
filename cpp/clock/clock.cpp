#include "clock.h"

namespace date_independent {

clock clock::at(unsigned) {
    return clock{};
}

clock::operator std::string() {
    return "";
}

}
