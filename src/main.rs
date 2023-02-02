use std::{
    error::Error,
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    process, thread,
    time::Duration,
};

use web_server_rs::ThreadPool;

fn main() {
    match TcpListener::bind("127.0.0.1:7878") {
        Err(err) => {
            eprintln!("Error binding TCP listener: {err}");
            process::exit(1);
        }
        Ok(listener) => match ThreadPool::build(4) {
            Ok(pool) => {
                for stream in listener.incoming().take(2) {
                    match stream {
                        Ok(stream) => {
                            let result = pool.execute(|| {
                                if let Err(err) = handle_connection(stream) {
                                    eprintln!("Error handling connection: {err}");
                                }
                            });

                            if let Err(err) = result {
                                eprintln!("Error executing job: {err}");
                            }
                        }
                        Err(err) => {
                            eprintln!("Error reading TCP stream: {err}");
                        }
                    }
                }
            }
            Err(err) => {
                eprintln!("Error building thread pool: {err}");
                process::exit(1);
            }
        },
    }

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    let buf_reader = BufReader::new(&mut stream);
    let next_line = buf_reader.lines().next();

    match next_line {
        None => eprintln!("No lines left in TCP stream."),
        Some(line) => {
            let request_line = line?;

            let (status_line, filename) = match &request_line[..] {
                "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "src/hello.html"),
                "GET /sleep HTTP/1.1" => {
                    thread::sleep(Duration::from_secs(5));
                    ("HTTP/1.1 200 OK", "src/hello.html")
                }
                _ => ("HTTP/1.1 404 NOT FOUND", "src/404.html"),
            };

            let contents = fs::read_to_string(filename)?;
            let length = contents.len();
            let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

            stream.write_all(response.as_bytes())?;
            stream.flush()?;
        }
    }

    Ok(())
}
