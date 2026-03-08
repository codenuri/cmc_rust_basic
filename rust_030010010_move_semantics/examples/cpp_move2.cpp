// C++ 코드
#include <string>
#include <print>
using namespace std;

int main() {
	
	string s1 = "Practice make Perfect";
	string s2 = "To be or not to be";

	// 깊은복사에 의한 swap
	string tmp1 = s1;	// deep copy
	s1 = s2;
	s2 = tmp1;

	// 이동(move)에 의한 swap
	string tmp2 = move(s1);
	s1 = move(s2);
	s2 = move(tmp2);
}
