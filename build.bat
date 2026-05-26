@echo off
rustc --emit=obj -C panic="abort" src/main.rs
gcc -nostdlib -e _start main.o -o main.exe -lmsvcrt
.\main.exe