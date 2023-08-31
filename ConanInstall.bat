conan install . --update --build=missing ^
 -c:h tools.cmake.cmake_layout:build_folder_vars="['settings.compiler']" ^
 -pr:h default -pr:b default ^
 -s:h build_type=Release -s:h compiler.runtime_type=Release
