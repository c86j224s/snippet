#/bin/bash

clang++ -fmodules-ts -fmodules -std=c++1z --precompile -x c++-module -fbuiltin-module-map -o submodule.pcm submodule.cxx

clang++ -fmodules-ts -fmodules -std=c++1z -x c++ -fprebuilt-module-path=. -o module module.cxx
