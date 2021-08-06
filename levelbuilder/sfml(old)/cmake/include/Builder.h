#ifndef BUILDER_H
#define BUILDER_H
#include "Entity.h"
#include <string>
#include <fstream>
#include <SFML/Graphics.hpp>
class Builder : public Entity{
	public:
	//Defs ==> paths
	Builder(ResourceManager *r){
		//Assumes defenitions are already loaded...
		resourceManager = r;
		dataTex.create(1600, 900);
		ratio = 1600.0f/static_cast<double>(WIDTH);
	}
	virtual void draw(sf::RenderTarget &target, sf::RenderStates states)const{
		target.draw(dataSprite);
	}
	virtual void update(){
		redrawTexture();
	}
	void loadFromFile(std::string path){
		std::ifstream input(path);
		std::string line;
		if(input.is_open()){
			for(int i = 0; i != BLOCK_NUM && std::getline(input, line); i++){
				for(int j = 0; j != BLOCK_REPRESENTATION_SIZE; j++){
					data[i][j] = line.at(j);
					
				}
				line = "";
			}
		}
		input.close();
		redrawTexture();
	}
	void redrawTexture(){
		//BG
		for(int i = 0; i != BLOCK_NUM; i++){
			sf::Sprite s(getTexture(std::string(data[i], BLOCK_REPRESENTATION_SIZE)));
			s.setPosition((i % BLOCK_WIDTH) * 50, (i / BLOCK_WIDTH) * 50);
			dataTex.draw(s);
			//Tiles
		}
		dataSprite.setTexture(dataTex.getTexture());
		dataSprite.setPosition(0, 0);
		dataSprite.setScale(ratio, ratio);
	}
	sf::Texture& getTexture(std::string id){
		//lbl id
		return resourceManager->getTexture(id); //TODO: Add internal system for loading independent of manager and maintain states of all
	}
	private:
	constexpr static int BLOCK_WIDTH = 32;
	constexpr static int BLOCK_HEIGHT = 18;
	constexpr static int BLOCK_NUM = BLOCK_WIDTH * BLOCK_HEIGHT;
	constexpr static int BLOCK_REPRESENTATION_SIZE = 2;
	constexpr static int WIDTH = 1920;
	constexpr static int HEIGHT = 1080;
	//END CONSTANTS
	ResourceManager* resourceManager;
	char data[BLOCK_NUM][BLOCK_REPRESENTATION_SIZE];
	double ratio = 1;
	sf::RenderTexture dataTex;
	sf::Sprite dataSprite;
};
#endif