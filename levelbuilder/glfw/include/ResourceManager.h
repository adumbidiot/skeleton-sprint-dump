#ifndef RESOURCE_MANAGER_H
#define RESOURCE_MANAGER_H

#include <string>
#include <fstream>
#include <map>
#include <memory>

namespace sks {
	class Resource {
		public:
		virtual int load()=0;
		bool loaded = false;
	};
	
	class TextResource : public Resource {
		public:
		TextResource(std::string assetPath){
			path = assetPath;
		}
		
		virtual int load(){
			std::ifstream file;
			std::string content = "";
			std::string line = "";
			
			file.open(path);
			if(!file){
				return -1;
			}
	
			while (std::getline(file, line)) {
				content += line + '\n';
			}
	
			file.close();
			data = content;
			
			return 0;
		}
		
		virtual std::string getText(){
			if(loaded == false){
				load();
				loaded = true;
			}
			return data;
		}
		
		std::string data = "";
		std::string path = "";
	};
	
	class VertexShaderResource : public TextResource {
		public:
		VertexShaderResource(std::string assetPath) : TextResource(assetPath){
			
		}
	};
	
	class FragmentShaderResource : public TextResource {
		public:
		FragmentShaderResource(std::string assetPath) : TextResource(assetPath){
			
		}
	};
	
	class ResourceManager{
		public:
		void addTextResource(std::string name, std::string path);
		void addVertexShaderResource(std::string name, std::string path);
		void addFragmentShaderResource(std::string name, std::string path);
		TextResource &getTextResource(std::string name);
		VertexShaderResource &getVertexShaderResource(std::string name);
		FragmentShaderResource &getFragmentShaderResource(std::string name);
		
		private:
		std::map<std::string, std::unique_ptr<TextResource>> textAssets;
		std::map<std::string, std::unique_ptr<VertexShaderResource>> vertexShaderAssets;
		std::map<std::string, std::unique_ptr<FragmentShaderResource>> fragmentShaderAssets;
	};
};
#endif