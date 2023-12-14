# RG Nano用のアプリをWindows上のRustで開発する

## 開発環境(Linux)

### SDK

https://github.com/DrUm78/FunKey-OS/releases/download/FunKey-OS-DrUm78/FunKey-sdk-2.3.0.tar.gz

### 参考

[RG Nano/開発環境 - kuran_kuran page!](https://daimonsoft.info/kuran_kuran/index.php?RG%20Nano/%E9%96%8B%E7%99%BA%E7%92%B0%E5%A2%83)

## 開発環境(Windows 10 64bit用)

開発環境のビルドを行います。

Ubuntu20.04LTS(VMWare上で動作)

```bash
sudo apt update
#必須では無いが入れておくと便利
sudo apt install open-vm-tools open-vm-tools-desktop cifs-utils

#開発環境ビルド用
sudo apt install build-essential mingw-w64 python git flex bison autogen automake autoconf libtool texinfo gawk libncurses5-dev libpython2.7-dev gcc-multilib g++-multilib dejagnu lsb zlib1g-dev p7zip-full
```

### musl-cross-make

[GitHub - richfelker/musl-cross-make: Simple makefile-based build for musl cross compiler](https://github.com/richfelker/musl-cross-make)

```bash
git clone https://github.com/richfelker/musl-cross-make.git
cd musl-cross-make
```

### musl-cross-makeの設定

musl-cross-make以下にconfig.makを作成します。

- コメントアウトしているHOSTは2回目のGCC作成(Windows用)で使用します。

- TARGETはFunkey-sdkと同じ名前にしています。

- TOOLCHAIN_CONFIGでRGNano用のアーキテクチャを指定しています。

- sysrootは/が指定されるため、必要なライブラリは/libに入れる必要があります。
  
  （Funkey-sdkはarm-funkey-linux-musleabihf/sysroot/usr/libにSDL等のlibがあります）

[config.mak]

```makefile
TARGET = arm-funkey-linux-musleabihf
#HOST = x86_64-w64-mingw32
TOOLCHAIN_CONFIG += --with-arch=armv7-a --with-fpu=neon-vfpv4

BINUTILS_VER = 2.33.1
GCC_VER = 10.2.0
MUSL_VER = 1.2.1
GMP_VER = 6.1.2
MPC_VER = 1.1.0
MPFR_VER = 4.0.2
LINUX_VER = 4.19.90
```

[hashes/gcc-10.2.0.tar.xz.sha1]（GCC_VER = 10.2.0を作成するために必要です）

```
8de0aecd3a52bb92b43082df8a9256356d1f03be  gcc-10.2.0.tar.xz
```

### make(1回目：Linux用のGCCの作成)

上記のファイルを設定後makeを実行します。
makeが終了したら、make installを行います。

```bash
# 実行するマシンのコア数を指定して実行する(かなりビルド時間に差が出ます)
make -j8
```

```bash
# outputフォルダにツールチェインが作成されます
make install
```

outputフォルダに作成したツールチェインにpathを通します。(2回目のビルドで使用)

```bash
export PATH=$PATH:~/musl-cross-make/output/bin
```

### make(2回目：Windows用のGCCの作成)

config.makをWindowsビルド用に修正します。#HOSTのコメントを外します。

[config.mak]

```makefile
TARGET = arm-funkey-linux-musleabihf
HOST = x86_64-w64-mingw32
TOOLCHAIN_CONFIG += --with-arch=armv7-a --with-fpu=neon-vfpv4

BINUTILS_VER = 2.33.1
GCC_VER = 10.2.0
MUSL_VER = 1.2.1
GMP_VER = 6.1.2
MPC_VER = 1.1.0
MPFR_VER = 4.0.2
LINUX_VER = 4.19.90
```

```bash
# 実行するマシンのコア数を指定して実行する(かなりビルド時間に差が出ます)
make -j8
```

```bash
# output-x86_64-w64-mingw32フォルダにツールチェインが作成されます
make install
```

不要なファイルを削除して圧縮をします。

```bash
7z a -l output.7z output-x86_64-w64-mingw32/
```

### Windows上での設定

先程作成したoutput.7zを展開します。(C:\\output-x86_64-w64-mingw32)

フォルダをリネームします（C:\FunKey-sdk-win）

SDKにパスを通します。

```powershell
 $tmpUserPath = [System.Environment]::GetEnvironmentVariable("Path", "User")
 $tmpUserPath += ";C:\FunKey-sdk-win\bin"
 [System.Environment]::SetEnvironmentVariable("Path", $tmpUserPath, "User")
```

 以下のコマンドを実行してパスが通ったか確認します

```powershell
arm-funkey-linux-musleabihf-gcc -v
```

FunKey-sdk-2.3.0.tar.gzから必要なlibをコピーします。

※必要なライブラリが足りない可能性がありますが、SDL系のライブラリのみ確認しています。（それ以外は次節のRustで代用可能）

```powershell
FunKey-sdk-2.3.0\arm-funkey-linux-musleabihf\sysroot\usr\lib\
→
C:\FunKey-sdk-win\lib
```

### Rust

### 環境のインストール

Rustが利用しているmuslのバージョンに注意

[Updating Rust's Linux musl targets | Rust Blog](https://blog.rust-lang.org/2023/05/09/Updating-musl-targets.html)

※最新版に更新するのが無難です。

```powershell
rustup update
```

ターゲットの追加(armv7-unknown-linux-musleabihf)

```powershell
rustup target add armv7-unknown-linux-musleabihf
```

### プロジェクトの作成

```powershell
cargo new sample_project
```

プロジェクト直下に.cargo/config.tomlを作成

```toml
# .cargo/config.toml
[target.armv7-unknown-linux-musleabihf]
linker = "arm-funkey-linux-musleabihf-gcc"

[profile.release]
strip = true
```

以下のbatを作成して実行します。

```batch
set target_arch=armv7-unknown-linux-musleabihf
cargo clean --target %target_arch%
cargo build --target %target_arch% --release
set THIS_PATH=%~dp0
for %%1 in ("%THIS_PATH:~0,-1%") do set FOLDER_NAME=%%~nx1
move .\target\%target_arch%\release\%FOLDER_NAME% ./
```

### ライブラリのリンク順序について

armv7-unknown-linux-musleabihfではライブラリはスタティックリンクされるため、

実際のリンク時のライブラリの指定順が非常に重要になっています。

例えば、SDL、SDL_mixerを使用する場合は以下の順序である必要があります。

```rust
// SDL_mixer, mikmod, SDLの順序で記述する必要があります
#[link(name="SDL_mixer")]
extern {
    fn Mix_Init(flags: c_int) -> c_int;
    fn Mix_Quit();
}
#[cfg(target_os = "linux")]
#[link(name="mikmod")]
extern {
}

#[link(name="SDL")]
extern {
    fn SDL_Quit();
    fn SDL_Delay(flags: uint32_t);
    fn SDL_Init(flags: uint32_t) -> c_int;
    fn SDL_Flip(screen: *mut SDL_Surface) -> c_int;
    fn SDL_InitSubSystem(flags: uint32_t) -> c_int;
    fn SDL_FillRect(dst: *mut SDL_Surface, dstrect: *mut SDL_Rect, color: uint32_t) -> c_int;
    fn SDL_SetVideoMode(width: c_int, height: c_int, bpp: c_int, flags: uint32_t) -> *mut SDL_Surface;
}
```

実際には上記のコードの場合リンカーに以下の引数が指定されます。

```powershell
"-Wl,-Bstatic" "-lSDL_mixer" "-lmikmod" "-lSDL"
```

### デバッグ

Windows環境ではSDL系のDLLにパスが通っていないとエラーになる。

rustのツールチェインのフォルダにパスを通しても良いが、そうでない場合は、

実行ディレクトリをツールチェインのフォルダにしてしまえば良い。

```json
"cwd": "${env:USERPROFILE}/.rustup/toolchains/stable-x86_64-pc-windows-msvc/bin/",
```

/.vscode/launch.json

```json
{
    // IntelliSense を使用して利用可能な属性を学べます。
    // 既存の属性の説明をホバーして表示します。
    // 詳細情報は次を確認してください: https://go.microsoft.com/fwlink/?linkid=830387
    "version": "0.2.0",
    "configurations": [
        {
            "type": "lldb",
            "request": "launch",
            "name": "Debug executable 'rgnano_sample_rs'",
            "cargo": {
                "args": [
                    "build",
                    "--bin=rgnano_sample_rs",
                    "--package=rgnano_sample_rs"
                ],
                "filter": {
                    "name": "rgnano_sample_rs",
                    "kind": "bin"
                }
            },
            "args": [],
            //"cwd": "${workspaceFolder}",
            "cwd": "${env:USERPROFILE}/.rustup/toolchains/stable-x86_64-pc-windows-msvc/bin/",
        }
    ]
}
```

## (参考)WindowsのRust環境でSDL1.2を使用する

GPUが搭載されていないハードウェアでは未だにSDL1.2が使用されていることが多いです。（SDL2だと十分な速度が出ない等の理由があるようです）

Rustではクロス環境でもほぼ同じコードが利用できるため、Windowsでビルド、テスト後にターゲットのハードウェアで動作を確認する運用が想定されます。

今回は代用が効かないSDL、SDL_mixierを導入する手順を記載します。

(SDL_xxxはユーティリティ的なライブラリで、ハードウェア依存の実装はSDLに全て実装されていると思われます。)

### インストール

SDLの各プロジェクトのビルド済みバイナリは以下のURLで取得できます。

[Index of /projects](https://www.libsdl.org/projects/)

- SDL1.2
  
  [Index of /release](https://www.libsdl.org/release/)

- SDL_mixer
  
  [Index of /projects/SDL_mixer/release](https://www.libsdl.org/projects/SDL_mixer/release/)

インストール用スクリプト

```powershell
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
```

### (参考)ソースコードのビルド

SDL1.2

[GitHub - libsdl-org/SDL-1.2: Simple Directmedia Layer, 1.2 branch ... ***DEPRECATED***, please use https://github.com/libsdl-org/SDL for new projects!](https://github.com/libsdl-org/SDL-1.2)

SDL_mixer

[GitHub - libsdl-org/SDL_mixer at SDL-1.2](https://github.com/libsdl-org/SDL_mixer/tree/SDL-1.2)

SDL1.2のビルド

- ソースコードの取得

```powershell
git clone https://github.com/libsdl-org/SDL-1.2.git
```

- /include/SDL_config_win32.hをSDL_config.hにリネーム

- /VisualC/SDL_VS2010.slnを開く→使用するVisualStudioのバージョンにコンバートする

- ソリューションのビルド(Release)
