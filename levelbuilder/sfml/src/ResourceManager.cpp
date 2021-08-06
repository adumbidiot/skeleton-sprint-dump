#include "ResourceManager.h"

namespace sks {
	std::unordered_map<std::string, std::unique_ptr<sf::Texture>> ResourceManager::textures;
	std::unordered_map<std::string, std::unique_ptr<sf::Font>> ResourceManager::fonts;
	
	sf::Texture& ResourceManager::getTexture(std::string name){
		auto it = textures.find(name);
			if(it == textures.end()){
				std::cout << "No Texture found for key " << name << std::endl;
				textures[name] = std::make_unique<sf::Texture>();
			}
			return *textures[name];
	}
	
	bool ResourceManager::addTexture(std::string name, std::string path){
		textures[name] = std::make_unique<sf::Texture>();
		if(!(textures[name]->loadFromFile(path))){
			std::cout << "No Texture found at " << path << std::endl;
			return false; //TODO: Blank texture
		}
		return true;
	}
	
	sf::Font& ResourceManager::getFont(std::string name){
		auto it = fonts.find(name);
		if(it == fonts.end()){
			std::cout << "No Font found for key " << name << std::endl;
			fonts[name] = std::make_unique<sf::Font>();
		}
		return *fonts[name];
	}
	
	bool ResourceManager::addFont(std::string name, std::string path){
		fonts[name] = std::make_unique<sf::Font>();
		if(!(fonts[name]->loadFromFile(path))){
			std::cout << "No Font found at " << path << std::endl;
			return false; //TODO: Blank texture
		}
		return true;
	}
}