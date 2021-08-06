#include "Board.h"
#include <fstream>
#include "App.h"
#include "EventManager.h"
#include "tinyfiledialogs.h"

namespace sks {
	Board::Board(App* app){
		bg.setPosition(20, 120);
		bg.setSize(sf::Vector2f(1660, 940));
		bg.setFillColor(sf::Color(119, 119, 119));	
			
		blockBG.setPosition(20 + 10, 120 + 10);
		blockBG.setSize(sf::Vector2f(1600, 900));
		
		blockBG.setTexture(&ResourceManager::getTexture("M0"));
		
		float block_width = (1600.0 / 32.0);
		for(int i = 0; i != BLOCK_NUM; i++){
			blockIDs[i] = "00";
			
			blockSprites[i].setSize(sf::Vector2f(block_width, block_width));
			blockSprites[i].setPosition(sf::Vector2f(30 + i % 32 * block_width, 120 + 10 + block_width * static_cast<int>(i / 32)));
		}
		
		app->eventManager->on(EventType::MOUSE_EVENT, [&](const Event& e){
			if(locked){
				return;
			}
			
			float block_width = (1600.0 / 32.0);
			auto mousePos = sf::Vector2f(e.payload.mouse.x, e.payload.mouse.y);
			
			if(e.payload.mouse.action == MouseAction::MOVE && !sf::Mouse::isButtonPressed(sf::Mouse::Button::Left)){
				return;
			}
			
			if(mousePos.x > 30 && mousePos.x < 30 + 1600 && mousePos.y > 130 && mousePos.y < 130 + 900 && activeBlock[0] != 0){
				int index = ((mousePos.x - 30.0) / block_width) + (static_cast<int>((mousePos.y - 130) / block_width) * 32);
				
				std::string key;
				key += activeBlock[0];
				key += activeBlock[1];
				blockIDs[index] = key;
				
				writeBlock(index, key);
			}
		});
		
		app->eventManager->on(EventType::BLOCK_EVENT, [&](const Event& e){
			//std::cout << e.payload.active.block[0] << e.payload.active.block[1] << std::endl;
			activeBlock[0] = e.payload.active.block[0];
			activeBlock[1] = e.payload.active.block[1];
		});
		
		app->eventManager->on(EventType::POPUP_EVENT, [&](const Event &e){
			if(e.payload.popup.open == 1){
				locked = true;
			}else{
				locked = false;
			}
		});
		
		app->eventManager->on(EventType::SAVE_EVENT, [&](const Event &e){
			char const * pattern[1] = { "*.txt"};
			char const* txt =  tinyfd_saveFileDialog("Save File", "level.txt", 1, pattern, "Skeleton Sprint Level File");
			if(txt != NULL){
				std::string data = exportLBL();
				
				std::ofstream output;
				output.open(txt);
				output << data;
				output.close();
			}
		});
	}
	
	void Board::update(){
		
	}
}