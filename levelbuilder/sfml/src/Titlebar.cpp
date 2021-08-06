#include "Titlebar.h"
#include "App.h"
#include "ResourceManager.h"
#include "EventManager.h"

namespace sks {
	Titlebar::Titlebar(App* app_ptr){
		this->app = app_ptr;
		
		bg.setSize(sf::Vector2f(1920, 100)); //FULL WIDTH
		bg.setFillColor(sf::Color(56, 56, 56));
		
		name.setFont(ResourceManager::getFont("bolo"));
		name.setString("SS");
		name.setCharacterSize(100 - 20);
		name.setPosition(10, 10);
		
		fileButton.setPosition(100 + 10 + 10, 0);
		fileButton.setSize(sf::Vector2f(200, 100));
		fileButton.setFillColor(sf::Color(56, 90, 56));
		
		fileLabel.setFont(ResourceManager::getFont("bolo"));
		fileLabel.setString("File");
		fileLabel.setCharacterSize(50);
		fileLabel.setPosition(100 + 50, 25);
		
		fileSaveButton.setPosition(100 + 10 + 10, 100);
		fileSaveButton.setSize(sf::Vector2f(200, 100));
		fileSaveButton.setFillColor(sf::Color(56, 90, 56));
		
		fileSaveLabel.setFont(ResourceManager::getFont("bolo"));
		fileSaveLabel.setString("Save");
		fileSaveLabel.setCharacterSize(50);
		fileSaveLabel.setPosition(100 + 50, 25 + 100);
		
		
		for(int i = 0; i != BUTTON_NUM; i++){
			buttonStates[i] = 0;
		}
		
		
		
		app->eventManager->on(EventType::MOUSE_EVENT, [&](const Event& e){
			if(e.payload.mouse.action != MouseAction::RELEASE){
				return;
			}
			
			auto mousePos = sf::Vector2f(e.payload.mouse.x, e.payload.mouse.y);
			
			const sf::Vector2f size = fileButton.getSize();
			const sf::Vector2f pos = fileButton.getPosition();
			if(mousePos.x > pos.x && mousePos.x < pos.x + size.x && mousePos.y > pos.y && mousePos.y < pos.y + size.y){
				int buttonNum = mousePos.y / 100;
				std::cout << "File: " << buttonNum << std::endl;
				
				switch(buttonNum){
					case 0: {
						toggleButton(0);
						break;
					}
					case 1: {
						Event e;
						e.type = EventType::SAVE_EVENT;
						e.payload.save.action = SaveType::SAVE;
						app->eventManager->fire(e);
						break;
					}
				}
			}
			
		});
	}
	
	void Titlebar::toggleButton(int index){
		Event e;
		e.type = EventType::POPUP_EVENT;
						
		if(buttonStates[index] > 0){
			fileButton.setSize(sf::Vector2f(200, 100));
			buttonStates[index] = 0;
			e.payload.popup.open = 0;
		}else {
			fileButton.setSize(sf::Vector2f(200, 200));
			buttonStates[index] = 1;
			e.payload.popup.open = 1;
		}
			
		app->eventManager->fire(e);
	}
	
}