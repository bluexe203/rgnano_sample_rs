$SDL = "SDL-devel-1.2.15-VC.zip"
$SDL_mixer = "SDL_mixer-devel-1.2.12-VC.zip"
echo "download SDL1.2 lib for Windows"
if (!(Test-Path ".\$SDL"))
{
    curl.exe -LO https://www.libsdl.org/release/$SDL
}
echo "download SDL_mixer lib for Windows"
if (!(Test-Path ".\$SDL_mixer"))
{
    curl.exe -LO https://www.libsdl.org/projects/SDL_mixer/release/$SDL_mixer
}
echo "extract lib"
if (!(Test-Path ".\SDL-1.2.15"))
{
    Expand-Archive -Path .\$SDL -DestinationPath .\
}
if (!(Test-Path ".\SDL_mixer-1.2.12"))
{
    Expand-Archive -Path .\$SDL_mixer -DestinationPath .\
}
echo "copy library to .rustup"
echo $env:USERPROFILE\.rustup\toolchains\stable-x86_64-pc-windows-msvc\bin\
echo $env:USERPROFILE\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\x86_64-pc-windows-msvc\lib\
Copy-Item -Path .\*\lib\x64\*.dll -Destination $env:USERPROFILE\.rustup\toolchains\stable-x86_64-pc-windows-msvc\bin\ -Force
Copy-Item -Path .\*\lib\x64\*.lib -Destination $env:USERPROFILE\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\x86_64-pc-windows-msvc\lib\ -Force
