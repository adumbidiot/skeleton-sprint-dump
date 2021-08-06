#ifndef BLOCK_ARRAY_H
#define BLOCK_ARRAY_H
#include <SFML/Graphics.hpp>
#include <string>
#include "Entity.h"
#include "ResourceManager.h"
#include "Debug.h"
class BlockArray: public Entity{
	public:
		BlockArray(std::string data, ResourceManager &mngr){
			manager = &mngr;
			tiles.create(1600, 900);
			load(data);
		}
		bool load(std::string data){
			//Debug::getInstance().println(data);
			for(int i = 0; i*2 < data.length() && i < WIDTH * HEIGHT; i++){
				Debug::getInstance().println("i" + std::to_string(i) + " " + data.substr(i*2, 2));
				sf::Sprite s(manager->getTexture(data.substr(i,2)));
				s.setPosition((i % WIDTH) * 50, (i / WIDTH) * 50);
				tiles.draw(s);
			}
			tilesSprite = sf::Sprite(tiles.getTexture());
			tilesSprite.setPosition(0, 0);
			tilesSprite.setScale(1, 1);
			return true;
		}
		virtual void update(){
			return;
		}
		virtual void draw(sf::RenderTarget &target, sf::RenderStates states) const{
			target.draw(tilesSprite);
		}
		virtual void resize(int nWidth, int nHeight){
			double rWidth = static_cast<double>(nWidth) / static_cast<double>(width);
			double rHeight = static_cast<double>(nHeight) / static_cast<double>(height);
			double ratio = 1;
			if(std::abs(1-rHeight) > std::abs(1-rWidth)){
				ratio = rWidth;
			}else if(std::abs(1-rWidth) > std::abs(1-rHeight)){
				ratio = rHeight;
			}else if(width <= 800 || height <= 450){
				ratio = 1;
			}
			Debug::getInstance().println("Block Array Resize Ratio: " + std::to_string(ratio));
			Debug::getInstance().println("Block Array Resize Width Ratio: " + std::to_string(rWidth));
			Debug::getInstance().println("Block Array Resize Height Ratio: " + std::to_string(rHeight));
			tilesSprite.setScale(ratio, ratio);
		}
	private:
	int width = 1920;
	int height = 1080;
	constexpr static unsigned int WIDTH = 32;
	constexpr static unsigned int HEIGHT = 18; //TODO: Rename to clarify usage
	ResourceManager* manager;
	sf::RenderTexture tiles;
	sf::Sprite tilesSprite;
	//sf::Sprite tiles[WIDTH * HEIGHT];
};
#endif