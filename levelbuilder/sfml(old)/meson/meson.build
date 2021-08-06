project('lvlbuilder', 'cpp')

sfml_dep = dependency('sfml', fallback:['sfml', 'sfmldep'])
onedep = dependency('one', fallback : ['one', 'onedep'])
#sdl_dep = dependency('sdl2', fallback : ['sdl2', 'sdl2_dep'])


executable('lvlbuilder', 'main.cpp', dependencies: [onedep, sfml_dep])