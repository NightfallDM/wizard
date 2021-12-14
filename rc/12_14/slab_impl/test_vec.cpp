#include <vector>
#include <iostream>

using std::vector;

int main() {
	auto vec = vector<int>(1023);
	std::cout << vec.size() << std::endl;
	std::cout << vec.empty() << std::endl;
}
