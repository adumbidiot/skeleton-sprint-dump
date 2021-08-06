cmake_minimum_required(VERSION 3.5)
project(levelbuilder)

#set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -std=c++14") #Look up proper setting
set (CMAKE_CXX_STANDARD 14)

#Disable unecessary build parts as per the documentation
set(NANOGUI_BUILD_EXAMPLE OFF CACHE BOOL " " FORCE)
set(NANOGUI_BUILD_PYTHON  OFF CACHE BOOL " " FORCE)
set(NANOGUI_INSTALL       OFF CACHE BOOL " " FORCE)

add_subdirectory(extlib/nanogui)
include_directories(extlib/nanogui/include)
include_directories(${NANOGUI_EXTRA_INCS})
add_definitions(${NANOGUI_EXTRA_DEFS})

#include_directories(extlib/embed-resource)
#add_subdirectory(extlib/embed-resource)

#embed_resources(MyResources shader/vert.glsl shader/frag.glsl)

add_executable(levelbuilder levelbuilder.cpp ${MyResources})
target_link_libraries(levelbuilder nanogui ${NANOGUI_EXTRA_LIBS})