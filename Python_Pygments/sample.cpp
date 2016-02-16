#include <iostream>

using namespace std;

namespace Sample {
	void Fn1 ();

	void Fn1 () {
		return;
	}

	void Fn2 (int);
};

void Sample::Fn2 (int num) {
	return;
}

class Class {
	public:
		Class () {
		}

		Class (int a) {
		}

		virtual ~Class () {
		}

		void Call () {
		}

};

int a(int);

int main (int argc, char ** argv) {
    a(10);
    std::cout << "hello" << std::endl;
    return 0;
}

int a(int b) { return 0; }
