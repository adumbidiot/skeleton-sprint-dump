#ifndef SKS_TOOLBAR_H
#define SKS_TOOLBAR_H
#include <SFML/Graphics.hpp>
#include "ResourceManager.h"
#include "EventManager.h"
#include <iostream>

namespace sks {
	class App;
	
	class Toolbar{
		public:
		Toolbar(App* app);
		
		void update(sf::RenderWindow &window){
			
		}
		
		void addBlock(unsigned int index, std::string name){
			if(index >= BLOCKS_NUM){
				return;
			}
			
			const auto textureSize = ResourceManager::getTexture(name).getSize();
			
			std::cout << textureSize.x << " " << textureSize.y << std::endl;
			
			float scale = 80.0 / static_cast<float>(textureSize.x);
			
			blocks[index].setSize(sf::Vector2f(80, 80));
			blocks[index].setTexture(&ResourceManager::getTexture(name));
			
			//blocks[index].setScale(scale, scale);
			int marginY = 11;
			int blocksPerCol = 10;
			blocks[index].setPosition(1700 + marginY + ((80 + marginY) * (index / blocksPerCol)), 120 + marginY + ((index % blocksPerCol) * (80 + marginY)));
			blockIDs[index] = {
				name[0], 
				name[1]
			};
		}
		
		void enableBlock(unsigned int index);
		
		void disableBlock(unsigned int index){
			blocks[index].setOutlineThickness(0);
			blockStates[index] = 0;
		}
		
		void render(sf::RenderWindow &window){
			window.draw(bg);
			for(int i = 0; i != BLOCKS_NUM; i++){
				window.draw(blocks[i]);
			}
		}
		
		private:
		static constexpr int BLOCKS_NUM = 11;
		const sf::Color ENABLED_COLOR = sf::Color(255, 0, 0);
		
		sf::RectangleShape bg;
		sf::RectangleShape blocks[BLOCKS_NUM];
		int blockStates[BLOCKS_NUM];
		std::array<char, 2> blockIDs[BLOCKS_NUM];
		App* app;
	};
}
#endif