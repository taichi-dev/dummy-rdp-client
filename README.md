# Dummy RDP Client

## What does it do?

Windows just won't let you create OpenGL context on discrete GPU when no one is logged in, or you have logged in but your primary video adapter is a cripped one (eg. VMware VGA / KVM Cirrus / etc). This is a must in Taichi CI/CD pipeline.

But when you are on a RDP session, discrete GPU suddenly become usable(weird).

This is a minimal RDP client that can just connect and login, and nothing else.


## How to build

Setup rust development environment and

```bash
$ cargo build --release
```


## How to build (cross compile Linux to Windows)

```bash
$ sudo apt install -y binutils-mingw-w64 gcc-mingw-w64
$ rustup target add x86_64-pc-windows-gnu
$ cargo build --release --target=x86_64-pc-windows-gnu
```
