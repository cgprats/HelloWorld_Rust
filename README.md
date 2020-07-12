# HelloWorld_Rust
This is a "Hello World" style program Highlighting Many Common Programming Functions in the Rust Language. This is to be Used as Reference Material for More Complicated Programs

Hard Requirements: **Rustc**

Optional Requirements: **make build-essential**

To Compile run the following command on *nix
```
rustc main.rs -o HelloWorld.bin
```

Alternatively create and use a Makefile
Run the following commands in the working directory
on *nix to create the Makefile
```
touch Makefile
echo build: > Makefile
echo -e '\t'rustc main.rs -o HelloWorld.bin
```

To Use the Makefile, Simply Run:
```
make
```
This will require make to be installed.