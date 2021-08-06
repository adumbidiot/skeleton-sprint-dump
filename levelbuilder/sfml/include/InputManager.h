#ifndef SKS_INPUT_MANAGER_H
#define SKS_INPUT_MANAGER_H

#include <unordered_map>
#include <functional>
#include <vector>

namespace sks{
	class App; //Forward Declaration
	
	class InputManager {
		public:
		InputManager(App* app);
		
		private:
		App* app;
	};
}
#endif