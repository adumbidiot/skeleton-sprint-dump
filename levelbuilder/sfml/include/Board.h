#ifndef SKS_BOARD_H
#define SKS_BOARD_H
#include <SFML/Graphics.hpp>
#include "ResourceManager.h"

namespace sks {
	class App;
	
	class Board {
		public:
		Board(App* app);
		
		void update();
		
		void render(sf::RenderWindow &window){
			window.draw(bg);
			window.draw(blockBG);
			for(int i = 0; i != BLOCK_NUM; i++){
				if(blockStates[i] > 0){
					window.draw(blockSprites[i]);
				}
			}
		}
		
		void writeBlock(int index, std::string key){
			float block_width = (1600.0 / 32.0);
			blockSprites[index].setTexture(nullptr);
			if(key == "00"){
				blockStates[index] = 0;
				return;
			}
			
			blockStates[index] = 1;
			blockSprites[index].setTexture(&ResourceManager::getTexture(key), true);
			blockSprites[index].setSize(sf::Vector2f(block_width, block_width));
		}
		
		std::string exportLBL(){
			std::string data = "";
			for(int i = 0; i != BLOCK_NUM; i++){
				data += blockIDs[i] + "\n";
			}
			return data;
		}
		
		private:
		static constexpr unsigned int BLOCK_NUM_X = 32;
		static constexpr unsigned int BLOCK_NUM_Y = 18;
		static constexpr unsigned int BLOCK_NUM = BLOCK_NUM_X * BLOCK_NUM_Y;

		sf::RectangleShape bg;
		sf::RectangleShape blockBG;
		
		int blockStates[BLOCK_NUM];
		
		std::string blockIDs[BLOCK_NUM];
		sf::RectangleShape blockSprites[BLOCK_NUM];
		
		int currentBG = 0;
		std::array<char, 2> activeBlock = {0, 0};
		bool locked = false;
	};
}
#endif