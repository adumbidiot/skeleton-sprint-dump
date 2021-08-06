#ifndef FPS_COUNTER_H
#define FPS_COUNTER_H
#include "Entity.h"
#include "Game.h"
#include<string>
class FPSCounter : public Entity{
	public:
	FPSCounter(ResourceManager &m) : display("0", m.getFont("default")){
		interval = 0;
	}
	virtual void draw(sf::RenderTarget &target, sf::RenderStates states)const{
		target.draw(display, states);
	}
	virtual void update(){
		currentTime = clock.restart().asSeconds();
		interval++;
		interval %= 30;
		if(interval == 0){
			fps = static_cast<int>(1.0f / currentTime);
			display.setString(std::to_string(fps));
		}
	}
	private:
	float currentTime;
	int interval;
	int fps;
	sf::Text display;
	sf::Clock clock;
	
};
#endif