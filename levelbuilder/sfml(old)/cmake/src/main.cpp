#include<SFML/Graphics.hpp>
#include<string>
#include<Game.h>

void runWinLoop(sf::Window &win){
	sf::Event e;
	while(win.isOpen()){
		while(win.pollEvent(e)){
			if(e.type == sf::Event::Closed){
				win.close();
			}
		}
	}
}

void displayErrorWin(const int &error){
	sf::Window errorWin(sf::VideoMode(800, 600), "ERROR: " + std::to_string(error));
	runWinLoop(errorWin);
}

int main(){
	Game game;
	int error = 0;
	error = game.run();
	if(error != 0){
		displayErrorWin(error);
	}
	return error;
}