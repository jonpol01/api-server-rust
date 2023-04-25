## Rust Server

This is a simple Rust server that serves static files over HTTP using the `std::net::TcpListener` and `std::net::TcpStream` modules.

### Usage

To start the server, run the following command:


By default, the server listens on `127.0.0.1:8080` and serves files from the `public/` directory. You can configure these settings in the `main()` function.

### Dependencies

The server uses the following Rust crates:

- `std::fs`: For reading files from disk.
- `std::io::prelude::*`: For reading and writing data to streams.
- `std::net`: For opening network sockets.
- `std::thread`: For handling incoming connections in separate threads.

### Contributing

Pull requests and bug reports are welcome! If you have any questions or feature requests, feel free to open an issue or reach out to the project maintainers.

### License

This project is licensed under the [MIT License](LICENSE).