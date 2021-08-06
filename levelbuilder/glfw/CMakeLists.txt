cmake_minimum_required(VERSION 3.10)
project(skslevelbuilder CXX)

set(SOURCES "main.cpp")
set(INCLUDE_DIRECTORY "include")
set(GLEW_STATIC true)

#set(CMAKE_CXX_FLAGS -Wl,--subsystem,windows)

#target_include_directories(skslevelbuilder ${OPENGL_INCLUDE_DIR})
#target_link_libraries(skslevelbuilder ${OPENGL_gl_LIBRARY})

find_package(glfw3 3.2 REQUIRED)
find_package(glew REQUIRED STATIC)
find_package(OpenGL REQUIRED)

include_directories(CMAKE_PREFIX_PATH)

link_directories(${CMAKE_PREFIX_PATH})

include_directories(${GLEW_INCLUDE_DIRS})
include_directories(${INCLUDE_DIRECTORY})

link_libraries(${OPENGL_LIBRARY})
link_libraries(${GLEW_LIBRARIES})

file(GLOB SRC "src/*.cpp")

#message(${GLEW_INCLUDE_DIRS}/../lib)
	
add_executable (skslevelbuilder ${SOURCES} ${SRC})
target_link_libraries(skslevelbuilder glfw)

#file(GLOB ASSETS "assets/*.*" )
#message(${ASSETS})
#configure_file(${ASSETS} ${CMAKE_BINARY_DIR} COPYONLY)

file(COPY assets DESTINATION ${CMAKE_BINARY_DIR})

#install(DIRECTORY "assets/" DESTINATION ${CMAKE_BINARY_DIR} FILES_MATCHING PATTERN "*.*" )