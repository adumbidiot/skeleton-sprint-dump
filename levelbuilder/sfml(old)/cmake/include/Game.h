#ifndef GAME_H
#define GAME_H
#include<SFML/Graphics.hpp>
#include<ResourceManager.h>
#include<vector>
#include<memory>
#include<fstream>

#include "BlockArray.h"
#include "FPSCounter.h"
#include "Entity.h"
#include "Ball.h"
#include "Builder.h"
class Game{
	public:	
	Game() : error(0), fpsCountTime(0), win(sf::VideoMode(1920, 1080), "Levelbuilder"){
		win.setVerticalSyncEnabled(true);
		error = loadAssets();
		if(error != 0){
			return;
		}
		displayList.emplace_back(new Ball(manager.getTexture("B0"), 0, 0, 3, 5, 1920, 1080));
		//win.setFramerateLimit(60);
		
		//displayList.emplace_back(new BlockArray(data, manager));
		//displayList.emplace_back(new Builder(&manager));
		fps = std::make_unique<FPSCounter>(manager);
		builder = std::make_unique<Builder>(&manager);
		builder->loadFromFile("1.txt");
	}
	int loadAssets();
	int run();
	void update();
	void render();
	void processEvents();
	ResourceManager& getResourceManager(){
		return manager;
	}	
	
	private:
	sf::RenderWindow win;
	ResourceManager manager;
	std::vector<std::unique_ptr<Entity>> displayList;
	int fpsCountTime;
	int error;
	std::unique_ptr<FPSCounter> fps;
	std::unique_ptr<Builder> builder;
};
#endif