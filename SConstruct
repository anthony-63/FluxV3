#!/usr/bin/env python
import os
import sys

env = SConscript("godot-cpp/SConstruct")

# For reference:
# - CCFLAGS are compilation flags shared between C and C++
# - CFLAGS are for C-specific compilation flags
# - CXXFLAGS are for C++-specific compilation flags
# - CPPFLAGS are for pre-processor flags
# - CPPDEFINES are for pre-processor defines
# - LINKFLAGS are for linking flags

# tweak this if you want to use different folders, or more folders, to store your source code in.
env.Append(CPPPATH=["src/", "godot-cpp/gen/include"])

sources = Glob("src/*/*.cpp") + Glob("src/*.cpp")

if env["platform"] == "windows":
    env.Append(CPPFLAGS="/O2")
else:
    env.Append(CPPFLAGS="-O3")

if env["platform"] == "macos":
    library = env.SharedLibrary(
        "godot/bin/libflux.{}.{}.framework/libflux.{}.{}".format(
            env["platform"], env["target"], env["platform"], env["target"]
        ),
        source=sources,
    )
else:
    library = env.SharedLibrary(
        "godot/bin/libflux{}{}".format(env["suffix"], env["SHLIBSUFFIX"]),
        source=sources,
    )

Default(library)
