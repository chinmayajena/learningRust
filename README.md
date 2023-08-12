# Lets get started with Rust

## Again a new language ?

Yes, RUST is a system programming language. Learning Rust offers several benefits that make it an attractive choice for developers. Here are some reasons why learning Rust can be valuable:

1. Performance and Efficiency: Rust is designed to provide high performance and efficiency. It achieves this by leveraging a combination of low-level control and modern language features. Rust's zero-cost abstractions and strict compile-time guarantees help eliminate common bugs like null pointer dereferences, buffer overflows, and data races.

2. Memory Safety: Rust's unique ownership system ensures memory safety without the need for garbage collection. It enforces strict rules for memory management, preventing common issues like dangling pointers and memory leaks. This feature makes Rust a strong choice for systems programming, where control over resources is crucial.

3. Concurrency and Parallelism: Rust provides excellent support for writing concurrent and parallel code. Its ownership and borrowing system enables safe and efficient concurrency without the risk of data races. The built-in concurrency primitives like threads and message passing (through channels) make it easier to write concurrent programs.

4. Safety and Reliability: Rust's focus on safety makes it suitable for writing critical and robust software. Its strict compiler checks, known as the borrow checker, catch many bugs at compile time, reducing the likelihood of runtime errors. This characteristic makes Rust an appealing choice for systems with strict safety requirements, such as embedded systems, network infrastructure, and operating systems.

5. Cross-platform Development: Rust has excellent cross-platform support, allowing you to write code that runs on multiple operating systems and hardware architectures. This versatility makes Rust well-suited for building systems software and applications that need to target diverse environments.

6. Growing Ecosystem: Rust has a rapidly growing ecosystem of libraries, frameworks, and tools. This means you can leverage existing crates (Rust's package manager) to accelerate your development process. From web development to game development and from networking to cryptography, there are numerous libraries available for various domains.

7. Career Opportunities: Rust is gaining popularity and has a vibrant community. Learning Rust can open up new career opportunities, particularly in areas such as systems programming, embedded development, blockchain, and performance-critical applications. Having Rust on your resume can make you stand out among other developers.

While learning Rust may require some initial effort due to its unique concepts, the benefits it offers make it a worthwhile investment for developers looking to enhance their skills and tackle challenging projects.

![Why Rust](./_images/Rust-Why.png)
![Energy Footprint](./_images/Rust-Energy-footprint.png)

## How to install on Windows 10 - 64bit arch?

You could follow the [link](https://www.rust-lang.org/tools/install)

Some useful links I followed:

- [How to Install / Setup the Rust on Windows 10 - Quick and Easy Guide](https://www.youtube.com/watch?v=enk0o7eWNsc)
- [Unable to compile Rust hello world on Windows: linker link.exe not found](https://stackoverflow.com/questions/55603111/unable-to-compile-rust-hello-world-on-windows-linker-link-exe-not-found)

## All set ? Lets run first program!

Create a directory and place a file name main.rs

```
fn main() {
    println!("Hello, world!");
}
```

Now open a terminal and go to that directory. You could use the below command to compile the file:

```
rustc main.rs

```

this will create a .exe file. Now you could go to the terminal and use the command

```
./main

```

This will print "Hello World!"

You could use **rustfmt** vscode extenstion for formatting the code you are writing.

## Currently following the course:

[Youtube link](https://www.youtube.com/watch?v=BpPEoZW5IiY)

![Course Goals](./_images/Rust-Course-Goals.png)

## Issue when using Actix Web framework

```
 Can't install cargo-binutils library: Failed to find tool. Is `gcc.exe` installed?
```

How to solve this?

The windows machine needs a C and C++ compiler.
You can visit the [URL](https://winlibs.com/)

Download either the Windows 32-bit/64-bit version based on your system architecture.

Extract this content and copy them into a folder C:\MinGW.

Now open your environment variable and add a path C:\MinGW\bin.

Now run the command in cmd prompt:

$ gcc --version

gcc.exe (MinGW-W64 x86_64-ucrt-posix-seh, built by Brecht Sanders) 13.2.0  
Copyright (C) 2023 Free Software Foundation, Inc.
This is free software; see the source for copying conditions. There is NO
warranty; not even for MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.

If you can see this, then you are done with gcc.exe installation.
