#ifndef SKS_APP_H
#define SKS_APP_H

#include <SFML/Graphics.hpp>

namespace sks {
	class EventManager;
	class Toolbar;
	class Board;
	class Titlebar;
	
	class App{
		public:
		
		App();
		~App();
		
		void loadResources();
		void update(sf::RenderWindow &window);
		void render(sf::RenderWindow &window);
		void triggerClick(int x, int y, sf::RenderWindow &window);
		void triggerMouseMove(int x, int y, sf::RenderWindow &window);
		
		EventManager* eventManager;
		
		private:
		
		Toolbar* toolbar;
		Board* board;
		Titlebar* titlebar;
	};
}
#endif