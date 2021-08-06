#include "App.h"
#include "Toolbar.h"
#include "Board.h"
#include "Titlebar.h"
#include "EventManager.h"
#include "ResourceManager.h"

namespace sks {
	App::App(){
		loadResources();
		
		eventManager = new EventManager();
		toolbar = new Toolbar(this);
		board = new Board(this);
		titlebar = new Titlebar(this);
	}
	
	void App::loadResources(){
		ResourceManager::addTexture("M0", "assets/background.png");
		ResourceManager::addTexture("B0", "assets/block.png");
		ResourceManager::addTexture("BK", "assets/block_key.png");
		ResourceManager::addTexture("IK", "assets/item_key.png");
		ResourceManager::addTexture("NO", "assets/note.png"); //Pain in my ass...
		ResourceManager::addTexture("D0", "assets/decoration_scaffold.png");
		ResourceManager::addTexture("D1", "assets/decoration_sconce.png");
		ResourceManager::addTexture("P0", "assets/powerupburrow.png");
		ResourceManager::addTexture("P1", "assets/poweruprecall.png");
		ResourceManager::addTexture("E0", "assets/exit.png");
		ResourceManager::addTexture("X0", "assets/main.png");
		ResourceManager::addTexture("00", "assets/delete.png"); //Sneakyyy
		
		ResourceManager::addFont("bolo", "assets/bolonewt.ttf"); //Sneakyyy
	}
	
	void App::update(sf::RenderWindow &window){	
		
	}
	
	void App::render(sf::RenderWindow &window){
		window.clear(sf::Color::Black);
		
		board->render(window);	
		toolbar->render(window);
		titlebar->render(window);
	}
	
	void App::triggerClick(int x, int y, sf::RenderWindow &window){
		Event e;
		e.type = sks::EventType::MOUSE_EVENT;
		sf::Vector2f worldPos = window.mapPixelToCoords(sf::Vector2i(x, y));
		e.payload.mouse.x = worldPos.x;
		e.payload.mouse.y = worldPos.y;
		e.payload.mouse.action = MouseAction::RELEASE;
		eventManager->fire(e);
	}
	
	void App::triggerMouseMove(int x, int y, sf::RenderWindow &window){
		Event e;
		e.type = sks::EventType::MOUSE_EVENT;
		sf::Vector2f worldPos = window.mapPixelToCoords(sf::Vector2i(x, y));
		e.payload.mouse.x = worldPos.x;
		e.payload.mouse.y = worldPos.y;
		e.payload.mouse.action = MouseAction::MOVE;
		eventManager->fire(e);
	}
	
	App::~App(){
		delete eventManager;
		delete toolbar;
		delete board;
		delete titlebar;
	}
}