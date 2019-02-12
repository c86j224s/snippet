#include <iostream>
export module submodule;

export namespace submodule {
    class foo {
        int i;
    public:
        foo () : i(0) { std::cout << "foo" << std::endl; }
        ~foo () { std::cout << "~foo" << std::endl; }

        void bar (int & j) { 
            std::cout << "bar" << std::endl;

            j = i = i+j;
        }
    };
}
