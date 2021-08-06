#ifndef RESOURCE_MANAGER_H
#define RESOURCE_MANAGER_H
#include<unordered_map>
#include<string>
#include<SFML/Graphics.hpp>
#include<memory>
#include "Debug.h"

class ResourceManager{
	public:
	bool loadFont(const std::string &name, const std::string &path){
		fonts[name] = std::make_unique<sf::Font>();
		if(!fonts[name]->loadFromFile(path)){
			fonts.erase(name);
			return false;
		}
		return true;
	}
	sf::Font& getFont(const std::string &name){
		return *fonts[name];
	}
	bool loadTexture(const std::string &name, const std::string &path){
		textures[name] = std::make_unique<sf::Texture>();
		if(!textures[name]->loadFromFile(path)){
			textures.erase(name);
			return false;
		}
		return true;
	}
	sf::Texture& getTexture(const std::string &name){
		auto it = textures.find(name);
		if(it == textures.end()){
			Debug::getInstance().println("Failed to load texture asset " + name);
			return getTexture(FAIL_TEX);
		}
		return *textures[name];
	}
	private:
	std::unordered_map<std::string, std::unique_ptr<sf::Font>> fonts;
	std::unordered_map<std::string, std::unique_ptr<sf::Texture>> textures;
	const std::string FAIL_TEX = "B0";
};
#endif