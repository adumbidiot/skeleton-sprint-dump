#include "Toolbar.h"
#include "App.h"
#include "BlockDefs.h"

namespace sks {
	Toolbar::Toolbar(App* app){
		this->app = app;
		
		bg.setPosition(1700, 120);
		bg.setSize(sf::Vector2f(200, 940));
		bg.setFillColor(sf::Color(119, 119, 119));
		
		addBlock(0, "B0");
		addBlock(1, "BK");
		addBlock(2, "IK");
		//addBlock(3, "NO"); //Nedd special..
		addBlock(4, "D0");
		addBlock(5, "D1");
		addBlock(6, "P0");
		addBlock(7, "P1");
		addBlock(8, "E0");
		addBlock(9, "X0");
		addBlock(10, "00");
		
		app->eventManager->on(EventType::MOUSE_EVENT, [&](const Event& e){
			if(e.payload.mouse.action == MouseAction::MOVE){
				return;
			}
			auto mousePos = sf::Vector2f(e.payload.mouse.x, e.payload.mouse.y);
			for(int i = 0; i != BLOCKS_NUM; i++){
				const sf::Vector2f size = blocks[i].getSize();
				const sf::Vector2f pos = blocks[i].getPosition();
				if(mousePos.x > pos.x && mousePos.x < pos.x + size.x && mousePos.y > pos.y && mousePos.y < pos.y + size.y){
					if(blockStates[i] == 1){
						disableBlock(i);
					}else{
						enableBlock(i);
						for(int j = 0; j != BLOCKS_NUM; j++){
							if(blockStates[j] == 1 && i != j){
								disableBlock(j);
							}
						}
					}
					break;
				}
			}
		});
	}
	
	void Toolbar::enableBlock(unsigned int index){
		blocks[index].setOutlineColor(ENABLED_COLOR); //RUN on init?
		blocks[index].setOutlineThickness(5);
		blockStates[index] = 1;
			
		Event e;
		e.type = EventType::BLOCK_EVENT;
		e.payload.active.block = blockIDs[index];
		app->eventManager->fire(e);
	}
};