#ifndef SKS_TITLEBAR_H
#define SKS_TITLEBAR_H

#include <SFML/Graphics.hpp>

namespace sks {
	class App;
	
	class Titlebar {
		public:
		Titlebar(App* app);
		
		void update(sf::RenderWindow &window){
			
		}
		
		void render(sf::RenderWindow &window){
			window.draw(bg);
			window.draw(name);
			window.draw(fileButton);
			window.draw(fileLabel);
			if(buttonStates[0] > 0){
				window.draw(fileSaveButton);
				window.draw(fileSaveLabel);
			}
		}
		
		void toggleButton(int index);
		
		private:
		constexpr static int BUTTON_NUM = 1;
		
		sf::RectangleShape bg;
		sf::Text name;
		sf::RectangleShape fileButton;
		sf::Text fileLabel;
		sf::RectangleShape fileSaveButton;
		sf::Text fileSaveLabel;
		
		int buttonStates[BUTTON_NUM];
		App* app;
	};
}
#endif