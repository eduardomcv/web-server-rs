# web-server-rs
Multithreaded web server project from the Rust Programming Language book. See https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html for more details.

# Usage
Run `cargo run` inside the root directory.

# Explanation
This project sets up a TCP connection on `127.0.0.1:7878`. It then proceeds to listen for incoming HTTP requests.
A thread pool is created to process incoming requests. A "job" is created for each incoming request and sent to the thread pool via a message queue,
which the receiving end is in the thread pool instance.
The number of threads created by the thread pool is hardcoded to 4, but this value can be changed easily.

# Improvements
This project contains additional improvements:
- The `ThreadPool` public API contains additional documentation. Check the available documentation by running `cargo doc --open`.
- There's an additional `build` method in `ThreadPool`, which is similar to `new` but returns a `Result` instead of panicking.
- Tests have been added to `lib.rs`.
- Replaced all `.unwrap()` calls with more robust error handling.
