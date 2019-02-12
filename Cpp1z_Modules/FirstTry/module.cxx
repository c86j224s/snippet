#include <iostream>

import submodule;

int main() {
    std::cout << "hello world" << std::endl;

    int i = 1;

    submodule::foo f;
    f.bar(i);
    std::cout << "i = " << i << std::endl;
    f.bar(i);
    std::cout << "i = " << i << std::endl;
    f.bar(i);
    std::cout << "i = " << i << std::endl;

    return 0;
}
