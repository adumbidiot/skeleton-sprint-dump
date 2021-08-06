#include<iostream>
#include <one.h>

//#include<Graphics.hpp>
//#include "SFML/Graphics.hpp"
#include <SFML/Window.hpp>
int main(){
	sf::Window win(sf::VideoMode(800, 600), "WIN");
	
	One a;
	std::cout << "Hello World!" << "one: " << a.getOne() << std::endl;
	return 0;
}