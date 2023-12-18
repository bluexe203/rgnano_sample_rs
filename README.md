# Funkey SDK for Rust on Windows

## How to Use

- Install Rust
  
  https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe

- Add armv7-unknown-linux-musleabihf
  
  ```powershell
  rustup target add armv7-unknown-linux-musleabihf
  ```

- Install This SDK
  
  https://github.com/bluexe203/rgnano_sample_rs/releases/download/v0.0.1/FunKey-sdk-win_min.zip
  
  Extract C:\
  
  Add Path C:\FunKey-sdk-win\bin
  
  ```powershell
  #type this cmd
  arm-funkey-linux-musleabihf-gcc -v
  
  Using built-in specs.
  COLLECT_GCC=C:\FunKey-sdk-win\bin\arm-funkey-linux-musleabihf-gcc.exe
  COLLECT_LTO_WRAPPER=c:/funkey-sdk-win/bin/../libexec/gcc/arm-funkey-linux-musleabihf/10.2.0/lto-wrapper.exe
  Target: arm-funkey-linux-musleabihf
  Configured with: ../src_gcc/configure --enable-languages=c,c++ --with-float=hard --with-arch=armv7-a --with-fpu=neon-vfpv4 --disable-bootstrap --disable-assembly --disable-werror --target=arm-funkey-linux-musleabihf --prefix= --libdir=/lib --disable-multilib --with-sysroot=/ --enable-tls --disable-libmudflap --disable-libsanitizer --disable-gnu-indirect-function --disable-libmpx --enable-initfini-array --enable-libstdcxx-time=rt --build=x86_64-pc-linux-gnu --host=x86_64-w64-mingw32
  Thread model: posix
  Supported LTO compression algorithms: zlib
  gcc version 10.2.0 (GCC)
  ```

- Clone This Sample Project
  
  ```powershell
  git clone https://github.com/bluexe203/rgnano_sample_rs.git
  ```
  
  exec build.bat (build rg nano binary)
  
  ```powershell
  cd rgnano_sample_rs
  build.bat
  ```
  
  Create opk file
  
  Use OpenPackageCreator https://github.com/Harteex/OpenPackageCreator/releases
  
  Create .funkey-s.desktop follow this
  
  [Build Programs using SDK - FunKey Project Documentation](https://doc.funkey-project.com/developer_guide/tutorials/build_system/build_program_using_sdk/)

- Copy opk file to RG Nano SD Card



## Debug SDL 1.2 App On Windows

- Install SDL 1.2 On Windows
  
  exec build_toolchain/Install_SDL1_2_forWindows_Rust.ps1 on PowerShell
  
  ```powershell
  cd build_toolchain
  Install_SDL1_2_forWindows_Rust.ps1
  ```

- Create /.vscode/launch.json
  
  ```json
  {
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

- VSCode Debug Select "Debug executable 'rgnano_sample_rs'"


