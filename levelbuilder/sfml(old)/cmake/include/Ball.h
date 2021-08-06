#ifndef BALL_H
#define BALL_H
#include <SFML/Graphics.hpp>
#include<Entity.h>
class Ball : public Entity{
	public:
	Ball(const sf::Texture &tex, int x = 0, int y = 0, int dx = 0, int dy = 0, int mX = 100, int mY = 100){
		this->sprite = sf::Sprite(tex);
		this->x = x;
		this->y = y;
		this->dx = dx;
		this->dy = dy;
		maxX = mX;
		maxY = mY;
	}
	virtual void update(){
		if(x < 0 || (x + getWidth()) > maxX){
			dx *= -1;
		}
		if(y < 0 || (y + getHeight()) > maxY){
			dy *= -1;
		}
		x += dx;
		y += dy;
		sprite.setPosition(x, y);
	}
	int getWidth(){
		return sprite.getTexture()->getSize().x;
	}
	
	int getHeight(){
		return sprite.getTexture()->getSize().y;
	}
	
	virtual void draw(sf::RenderTarget &target, sf::RenderStates states)const{
		target.draw(sprite, states);
	}
	private:
	int x;
	int y;
	int dx;
	int dy;
	int maxX;
	int maxY;
	sf::Sprite sprite;
};
#endif