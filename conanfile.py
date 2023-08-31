from conan import ConanFile
from conan.tools.cmake import CMake, CMakeDeps, CMakeToolchain, cmake_layout


class RaceNChaseConan(ConanFile):
    name = "RaceNChase"
    version = "0.1"
    package_type = "application"

    license = "MIT"
    author = "Mikhail Koviazin <github@mkmk.aleeas.com>"
    url = "https://github.com/mkmkme/race-n-chase"
    description = "Trying to reproduce GTA1 (almost) from scratch"

    # Binary configuration
    settings = "os", "compiler", "build_type", "arch"

    # Sources are located in the same place as this recipe, copy them to the recipe
    exports_sources = "CMakeLists.txt", "programs/*", "src/*"

    def requirements(self):
        self.requires("fmt/10.1.0")
        self.requires("spdlog/[^1.11.0]")
        self.requires("gtest/[^1.14.0]")

    def layout(self):
        cmake_layout(self)

    def generate(self):
        deps = CMakeDeps(self)
        deps.generate()
        tc = CMakeToolchain(self)
        tc.user_presets_path = "ConanPresets.json"
        tc.generate()

    def build(self):
        cmake = CMake(self)
        cmake.configure()
        cmake.build()

    def package(self):
        cmake = CMake(self)
        cmake.install()
