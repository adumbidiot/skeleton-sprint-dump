#ifndef SKS_BLOCK_DEFS
#define SKS_BLOCK_DEFS

#include <string>
#include <unordered_map>
#include <array>

namespace sks {
	class BlockDefs {
		public: 
		static std::unordered_map<std::string, std::array<char, 2>> blocks;
		static std::array<char, 2> getLBL(std::string name);
		//static std::string getName(std::array<char, 2>);
	};
}
#endif