#ifndef SKS_EVENT_MANAGER_H
#define SKS_EVENT_MANAGER_H

#include <functional>
#include <unordered_map>
#include <vector>
#include <string>

namespace sks {
	enum class EventType {
		MOUSE_EVENT,
		BLOCK_EVENT,
		POPUP_EVENT,
		SAVE_EVENT
	};
	
	enum class MouseAction {
		PRESS,
		RELEASE,
		MOVE
	};
	
	enum class SaveType {
		LOAD,
		SAVE,
		SAVE_AS
	};
	
	struct MousePayload {
		int x;
		int y;
		MouseAction action;
	};
	
	struct PopupPayload {
		int open;
	};
	
	struct BlockPayload {
		std::array<char, 2> block;
	};
	
	struct SavePayload{
		SaveType action;
	};
	
	struct Event {
		EventType type;
		union {
			MousePayload mouse;
			BlockPayload active;
			PopupPayload popup;
			SavePayload save;
		} payload;
	};
	
	class EventManager {
		typedef std::function<void(const Event&)> Callback;
		
		public:
		class EventListener {
			
		};
		
		void fire(const Event &e){
			auto callbacks = listeners[e.type];
			for(int i = 0; i != callbacks.size(); i++){
				callbacks[i](e);
			}
		}
		
		void on(EventType type, Callback cb){
			listeners[type].push_back(cb);
		}
		
		private:
		std::unordered_map<EventType, std::vector<Callback>> listeners;
	};
	
}

//https://codereview.stackexchange.com/questions/56363/casting-base-to-derived-class-according-to-a-type-flag
#endif