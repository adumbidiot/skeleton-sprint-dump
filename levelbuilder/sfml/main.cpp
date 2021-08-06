#include <SFML/Graphics.hpp>
#include <iostream>
#include "App.h"
#include "EventManager.h"

int main()
{
	sks::App a;
	
    sf::RenderWindow window(sf::VideoMode(1920, 1080), "Levelbuilder");
	window.setVerticalSyncEnabled(true);
	
	while (window.isOpen())
    {
        sf::Event event;
        while (window.pollEvent(event))
        {
            if (event.type == sf::Event::Closed){
                window.close();
			}else if(event.type == sf::Event::MouseButtonPressed){
				if(event.mouseButton.button == sf::Mouse::Left){
					a.triggerClick(event.mouseButton.x, event.mouseButton.y, window); //TODO: Trigger Mouse event (x, y, key, action)
				}
			}else if(event.type == sf::Event::MouseMoved){
				a.triggerMouseMove(event.mouseMove.x, event.mouseMove.y, window);
			}else if (event.type == sf::Event::Resized){
				//sf::FloatRect visibleArea(0, 0, 1920, 1080);
				//sf::FloatRect visibleArea(0, 0, event.size.width, event.size.height);
				//sf::View view(visibleArea);
				//window.setView(view);
				//a.triggerResize();
			}
        }

		a.update(window);
		a.render(window);
		
		window.display();
    }
	
    return 0;
}