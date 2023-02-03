# web-server-rs
Multithreaded web server project from the Rust Programming Language book. See https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html for more details.

# Usage
Run `cargo run` inside the root directory.

# Explanation
This project sets up a TCP connection on `127.0.0.1:7878`. It then proceeds to listen for incoming HTTP requests.
A thread pool is created to process incoming requests. A "job" is created for each incoming request and sent to the thread pool via a message queue,
which the receiving end is in the thread pool instance. A job is just a closure which is run by a thread.

The number of threads created by the thread pool is hardcoded to 4, but this value can be changed easily. It's also possible for the thread pool to perform jobs other than processing requests.

Finally, each request gets processed and HTTP responses are created, containing response headers and some HTML. The request handler contains a simple router for `/` and `/sleep`. `/` sends a "hello world" page and `/sleep` sleeps for 5 seconds before sending the "hello world" page. If these routes aren't matched, a simple "404" page is returned.

# Improvements
This project contains additional improvements:
- The `ThreadPool` public API contains additional documentation. Check the available documentation by running `cargo doc --open`.
- There's an additional `build` method in `ThreadPool`, which is similar to `new` but returns a `Result` instead of panicking.
- Tests have been added to `lib.rs`.
- Replaced all `.unwrap()` calls with more robust error handling.
