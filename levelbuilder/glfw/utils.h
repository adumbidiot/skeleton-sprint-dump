#ifndef UTILS_H
#define UTILS_H
#include <string>
#include <thread>
#include <chrono>

int error(std::string error_string){
	std::cout << error_string << std::endl;
	glfwTerminate();
	return -1;
}

GLuint compileVertexShader(const char* vertexSource){
	GLuint vertexShader = glCreateShader(GL_VERTEX_SHADER);
	glShaderSource(vertexShader, 1, &vertexSource, NULL);
	glCompileShader(vertexShader);
	
	GLint status;
	glGetShaderiv(vertexShader, GL_COMPILE_STATUS, &status);
	
	if(status == GL_TRUE){
		std::cout << "Compiled Vertex Shader!" << std::endl;
	}else{
		std::cout << "Could not Compile Vertex Shader" << std::endl;
		char buffer[512];
		glGetShaderInfoLog(vertexShader, 512, NULL, buffer);
		std::cout << buffer << std::endl;
	}
	return vertexShader;
}

GLuint compileFragmentShader(const char* fragmentSource){
	GLuint fragmentShader = glCreateShader(GL_FRAGMENT_SHADER);
	glShaderSource(fragmentShader, 1, &fragmentSource, NULL);
	glCompileShader(fragmentShader);
	
	GLint status;
	glGetShaderiv(fragmentShader, GL_COMPILE_STATUS, &status);
	
	if(status == GL_TRUE){
		std::cout << "Compiled Fragment Shader!" << std::endl;
	}else{
		std::cout << "Could not Compile Fragment Shader" << std::endl;
		char buffer[512];
		glGetShaderInfoLog(fragmentShader, 512, NULL, buffer);
		std::cout << buffer << std::endl;
	}
	return fragmentShader;
}

GLuint createShaderProgram(GLuint vertexShader, GLuint fragmentShader){
	GLuint shaderProgram = glCreateProgram();
	glAttachShader(shaderProgram, vertexShader);
	glAttachShader(shaderProgram, fragmentShader);
	glBindFragDataLocation(shaderProgram, 0, "outColor");
	glLinkProgram(shaderProgram);
	
	return shaderProgram;
}

void sleep(int ms){
	std::this_thread::sleep_for(std::chrono::milliseconds(ms));
}
#endif 