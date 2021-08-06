#ifndef SKS_RESOURCE_MANAGER_H
#define SKS_RESOURCE_MANAGER_H

#include <SFML/Graphics.hpp>
#include <unordered_map>
#include <memory>
#include <string>
#include <iostream>

namespace sks {
	class ResourceManager {
		public:
		static std::unordered_map<std::string, std::unique_ptr<sf::Texture>> textures;
		static sf::Texture &getTexture(std::string name);
		static bool addTexture(std::string name, std::string path);
		
		static std::unordered_map<std::string, std::unique_ptr<sf::Font>> fonts;
		static sf::Font &getFont(std::string name);
		static bool addFont(std::string name, std::string path);
	};
}
#endif