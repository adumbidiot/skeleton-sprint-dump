cmake_minimum_required(VERSION 2.6)
project(lvlbuilder)


#MINGW 
set(CMAKE_EXE_LINKER_FLAGS "-static-libgcc -static-libstdc++ -static -fno-exceptions -fno-rtti")

include_directories("${PROJECT_BINARY_DIR}")

file(GLOB_RECURSE SRC "src/*.cpp")

#tinyfiledialogs
	file(GLOB_RECURSE TFD_SRC "ext/tinyfiledialogs/src/*.c")
	set(TFD_INCLUDE "ext/tinyfiledialogs/include")
	include_directories(${TFD_INCLUDE})
#end tfd

set(EXECUTABLE_NAME "lvlbuilder")
add_executable(${EXECUTABLE_NAME} main.cpp ${SRC} ${TFD_SRC})


set(SFML_STATIC_LIBRARIES TRUE)
set(CMAKE_MODULE_PATH "${CMAKE_SOURCE_DIR}/cmake_modules" ${CMAKE_MODULE_PATH})
find_package(SFML 2 REQUIRED network audio graphics window system)

if(SFML_FOUND)
	include_directories(${SFML_INCLUDE_DIR})
	target_link_libraries(${EXECUTABLE_NAME} ${SFML_LIBRARIES} ${SFML_DEPENDENCIES})
endif()

include_directories(include)

file(COPY ${CMAKE_CURRENT_SOURCE_DIR}/assets
        DESTINATION ${CMAKE_CURRENT_BINARY_DIR})