cmake_minimum_required(VERSION 3.0)
project(LibPrintInC C)
add_library(print_in_c STATIC print_in_c.c)
install(TARGETS print_in_c DESTINATION .)