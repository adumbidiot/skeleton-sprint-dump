#include "BlockDefs.h"

namespace sks {
	std::unordered_map<std::string, std::array<char, 2>> BlockDefs::blocks = {
		{"block", {'B', '0'}}
	};
	
	std::array<char, 2> BlockDefs::getLBL(std::string name){
		return blocks[name];
	}
}