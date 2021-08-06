#ifndef ENTITY_H
#define ENTITY_H
#include<SFML/Graphics.hpp>
class Entity : public sf::Drawable, public sf::Transformable{
	public:
	virtual void update()=0;
	virtual void resize(int width, int height){
		
	}
};
#endif