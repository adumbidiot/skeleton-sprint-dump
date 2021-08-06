project('One', 'cpp')

includeDir = include_directories('include')
main_lib = static_library('one', 'one.cpp', include_directories: [includeDir])

onedep = declare_dependency(include_directories : includeDir, link_with : main_lib)
#library('One', ['one.cpp'])