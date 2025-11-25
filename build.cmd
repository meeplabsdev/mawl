@echo off
mkdir build

@REM DEVELOPMENT
cd common && cargo build && cd ..
robocopy common/target/x86_64-pc-windows-msvc build /e /xf CACHEDIR.TAG .rustc_info.json > nul

cd drv && cargo make sign && cd ..
robocopy drv/target/x86_64-pc-windows-msvc build /e /xf CACHEDIR.TAG .rustc_info.json > nul

cd lib && cargo build && cd ..
robocopy lib/target build /e /xf CACHEDIR.TAG .rustc_info.json > nul

cd bin && cargo build && cd ..
robocopy bin/target build /e /xf CACHEDIR.TAG .rustc_info.json > nul

@REM PRODUCTION
@REM cd common && cargo build --release && cd ..
@REM robocopy common/target/x86_64-pc-windows-msvc build /e /xf CACHEDIR.TAG .rustc_info.json > nul

@REM cd drv && cargo make -p production sign && cd ..
@REM robocopy drv/target/x86_64-pc-windows-msvc build /e /xf CACHEDIR.TAG .rustc_info.json > nul

@REM cd lib && cargo build --release && cd ..
@REM robocopy lib/target build /e /xf CACHEDIR.TAG .rustc_info.json > nul

@REM cd bin && cargo build --release && cd ..
@REM robocopy bin/target build /e /xf CACHEDIR.TAG .rustc_info.json > nul
