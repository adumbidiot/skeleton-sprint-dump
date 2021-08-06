#ifndef DEBUG_H
#define DEBUG_H
#include <fstream>
#include <string>
class Debug{
	public:
	void print(const std::string &str){
		output << str;
	}
	void println(const std::string &str){
		output << str << std::endl;
	}
	static Debug& getInstance(){
		static Debug instance;
		return instance;
	}
	private:
	Debug(){
		output.open("debug.txt", std::fstream::out);
	}
	~Debug(){
		output.close();
	}
	std::fstream output;
};
#endif