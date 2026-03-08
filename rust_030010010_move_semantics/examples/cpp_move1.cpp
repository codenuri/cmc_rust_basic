// C++ 코드
#include <string>
#include <print>
using namespace std;

int main() {
	
	string s1 = "to be or not to be";
	string s2 = s1;

	string s3 = "to be or not to be";
	string s4 = move(s3);

	println("{}", s1); // "to be or not to be"
	println("{}", s3); // "" 
}
