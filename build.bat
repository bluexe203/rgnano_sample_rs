set target_arch=armv7-unknown-linux-musleabihf
cargo build --target %target_arch% --release
set THIS_PATH=%~dp0
for %%1 in ("%THIS_PATH:~0,-1%") do set FOLDER_NAME=%%~nx1
move .\target\%target_arch%\release\%FOLDER_NAME% ./