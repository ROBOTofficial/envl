@echo off
setlocal enabledelayedexpansion

set BASEDIR=%~dp0
pushd %BASEDIR%

cd ../

if exist build (
    rmdir /s /q build
)

conan install . --output-folder=build --build=missing

cd build

cmake .. -DCMAKE_TOOLCHAIN_FILE=conan_toolchain.cmake -DCMAKE_BUILD_TYPE=Release

cmake --build .

popd
endlocal
