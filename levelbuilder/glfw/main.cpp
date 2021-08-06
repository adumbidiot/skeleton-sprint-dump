//Doesn't work?:
#define GLEW_STATIC
//
#include <iostream>
#include <GL/glew.h>
#include <GLFW/glfw3.h>

#include "ResourceManager.h"
#include "utils.h"

void window_resize_cb(GLFWwindow* win, int width, int height){
	glViewport(0, 0, width, height);
}

int main(){
	sks::ResourceManager mngr;
	mngr.addTextResource("test", "assets/test.txt");
	mngr.addVertexShaderResource("triangle.vert", "assets/triangle.vert");
	mngr.addFragmentShaderResource("triangle.frag", "assets/triangle.frag");
	
	std::cout << "Testing Asset Loading of file \"assets/test.txt\": " << mngr.getTextResource("test").getText() << std::endl;
	std::cout << "Triangle Vertex Shader: " << std::endl << mngr.getVertexShaderResource("triangle.vert").getText() << std::endl;
	std::cout << "Triangle Fragment Shader: " << std::endl << mngr.getFragmentShaderResource("triangle.frag").getText() << std::endl;
	
	glewExperimental = true;
	if(!glfwInit()){
		return error("Could not start GLFW!");
	}
	
	glfwWindowHint(GLFW_CONTEXT_VERSION_MAJOR, 3); //Opengl 3.3
	glfwWindowHint(GLFW_CONTEXT_VERSION_MINOR, 3);
	
	GLFWwindow* window = NULL; //Nullptr?
	window = glfwCreateWindow(1920, 1080, "SKS Levelbuilder", nullptr, nullptr);
	glfwSetWindowAspectRatio(window, 16, 9); //Keep same size. Might change later...
	
	if(window == NULL){
		return error("Could not create window!");
	}
	
	glfwSetWindowSizeCallback(window, window_resize_cb);
	glfwMakeContextCurrent(window);
	glfwSetInputMode(window, GLFW_STICKY_KEYS, GL_TRUE);
	glfwSwapInterval(1);
	
	glewExperimental = true;
	if (glewInit() != GLEW_OK) {
		return error("Failed to initialize GLEW");
	}
	//END INIT
	
	//GL Setup
	GLuint VertexArrayID; //VAO
	glGenVertexArrays(1, &VertexArrayID);
	glBindVertexArray(VertexArrayID); 
	
	static const GLfloat g_vertex_buffer_data[] = {
		-1.0f, -1.0f,
		1.0f, -1.0f,
		0.0f,  1.0f,
	};
	
	GLuint vertexbuffer; //VBO
	glGenBuffers(1, &vertexbuffer);
	glBindBuffer(GL_ARRAY_BUFFER, vertexbuffer);
	glBufferData(GL_ARRAY_BUFFER, sizeof(g_vertex_buffer_data), g_vertex_buffer_data, GL_STATIC_DRAW);
	
	GLuint vert = compileVertexShader(mngr.getVertexShaderResource("triangle.vert").getText().c_str());
	
	GLuint frag = compileFragmentShader(mngr.getFragmentShaderResource("triangle.frag").getText().c_str());
	GLuint shaderProgram = createShaderProgram(vert, frag);
	GLint posAttrib = glGetAttribLocation(shaderProgram, "position");
	
	glBindBuffer(GL_ARRAY_BUFFER, vertexbuffer);
	glVertexAttribPointer(posAttrib, 2, GL_FLOAT, GL_FALSE, 0, (void*)0);
	
	GLint uniColor = glGetUniformLocation(shaderProgram, "triangleColor");
	
	glEnableVertexAttribArray(posAttrib);
		
	float r = 0.0f;
	float g = 0.0f;
	float b = 1.0f;
	float step = 0.01f;
	
	float lastTime = glfwGetTime();
	GLenum error = GL_NO_ERROR;
	
	while(glfwGetKey(window, GLFW_KEY_ESCAPE) != GLFW_PRESS && glfwWindowShouldClose(window) == 0){
		float currentTime = glfwGetTime();
		
		glClear(GL_COLOR_BUFFER_BIT);
		
		glUseProgram(shaderProgram);
		glDrawArrays(GL_TRIANGLES, 0, 3);
		
		error = glGetError();
		if(error != GL_NO_ERROR){
			std::cout << "GL Error!" << std::endl;
			break;
		}
		
		if(r >= 1.0f){
			step = -0.01;
		}else if(r <= 0.0f){
			step = 0.01;
		}
		
		r += step;
		b -= step;
		
		glUniform3f(uniColor, r, g, b);
		
		glfwSwapBuffers(window);
		glfwPollEvents();
		//std::cout << currentTime - lastTime << std::endl;
		lastTime = currentTime;
	}
	
	glfwDestroyWindow(window);
	
	sleep(1000); //Closes window smoothly, then cleans up. Otherwise, it lags on close
	
	glfwTerminate();
	return 0;
}