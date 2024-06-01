#pragma once

#include <godot_cpp/variant/utility_functions.hpp>

auto print = [](auto ... args){ godot::UtilityFunctions::print(args...); };
auto print_err = [](auto ... args){ godot::UtilityFunctions::push_error(args...); };
auto print_warn = [](auto ... args){ godot::UtilityFunctions::push_warning(args...); };