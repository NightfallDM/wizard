#include "slab.hpp"
#include <iostream>

using namespace std;
using slab::Slab;

int main(void) {
	Slab<int> xxx(16);
	unsigned long x1 = xxx.insert(1);
	unsigned long x2 = xxx.insert(2);
	unsigned long x3 = xxx.insert(3);
	unsigned long x4 = xxx.insert(4);
	unsigned long x5 = xxx.insert(5);
	cout << "x1 = " << x1 << endl;
	cout << "x5 = " << x5 << endl;

	cout << "remove x3 = " << xxx.remove(x3) << endl;
	unsigned long x6 = xxx.insert(6);
	cout << "x6 = " << x6 << endl;
}
