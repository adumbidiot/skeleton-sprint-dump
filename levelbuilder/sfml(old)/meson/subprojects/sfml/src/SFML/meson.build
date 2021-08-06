sfml_src_inc = include_directories('..')

#src
sfml_src = [
	'Window/Window.cpp'
]

inc_list = [sfml_inc, sfml_src_inc]

if meson.get_compiler('cpp').get_define('__MINGW32__') == '1'
	inc_list += mingw_inc
	message('ENV is mingw, appending mingw headers')
endif


sfml_lib = static_library('SFML', sfml_src, include_directories: inc_list)