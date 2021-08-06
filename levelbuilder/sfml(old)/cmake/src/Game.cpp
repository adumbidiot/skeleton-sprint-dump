#include "Game.h"

int Game::loadAssets(){
	if(!manager.loadFont("default", "assets/bolonewt.ttf")){
		return 1;
	}
	if(!manager.loadTexture("B0", "assets/block.png")){
		return 2;
	}
	return 0;
}

void Game::update(){
	for(int i = 0; i != displayList.size(); i++){
		displayList.at(i)->update();
	}
	fps->update();
	builder->update();
}

void Game::render(){
	win.clear(sf::Color(48, 48, 48));
	for(int i = 0; i != displayList.size(); i++){
		win.draw(*displayList.at(i));
	}
	win.draw(*fps);
	win.draw(*builder);
	win.display();
}

int Game::run(){
	while(win.isOpen()){
		processEvents();
		update();
		render();
	}
	return error;
}

void Game::processEvents(){
	sf::Event e;
	while(win.pollEvent(e)){
		if(e.type == sf::Event::Closed){
			win.close();
		}
		if (e.type == sf::Event::Resized){
			int width = e.size.width;
			int height = e.size.height;
			sf::FloatRect visibleArea(0, 0, width, height);
			win.setView(sf::View(visibleArea));
			for(int i = 0; i != displayList.size(); i++){
				displayList.at(i)->resize(width, height);
			}
		}
	}
	if(error != 0){
		win.close();
	}
}