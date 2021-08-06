project('SFML', 'cpp')

#include
sfml_inc = include_directories('include')

subdir('extlibs')
subdir(join_paths('src', 'SFML'))

sfmldep = declare_dependency(link_with: sfml_lib, include_directories: inc_list)