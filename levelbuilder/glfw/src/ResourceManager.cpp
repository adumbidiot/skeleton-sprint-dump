#include "ResourceManager.h"

namespace sks{
	void ResourceManager::addTextResource(std::string name, std::string path){
		textAssets[name] = std::make_unique<TextResource>(path);
	}
	
	void ResourceManager::addVertexShaderResource(std::string name, std::string path){
		vertexShaderAssets[name] = std::make_unique<VertexShaderResource>(path);
	}
	
	void ResourceManager::addFragmentShaderResource(std::string name, std::string path){
		fragmentShaderAssets[name] = std::make_unique<FragmentShaderResource>(path);
	}
	
	TextResource &ResourceManager::getTextResource(std::string name){
		return *(textAssets[name]); //TODO: error check
	}
	
	VertexShaderResource &ResourceManager::getVertexShaderResource(std::string name){
		return *(vertexShaderAssets[name]);
	}
	
	FragmentShaderResource &ResourceManager::getFragmentShaderResource(std::string name){
		return *(fragmentShaderAssets[name]);
	}
};
