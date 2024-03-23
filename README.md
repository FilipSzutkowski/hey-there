# Hey-there

Hey There is a small web server that serves an index page and a 404 page on all other routes. It has been built as an exercise in Rust programming language, based on the official [Rust book's](https://doc.rust-lang.org/book/) chapter 20.

## Concepts

- Handling a simple TCP listener.
- Parsing and creating HTTP requests from scratch.
- Serving static files.
- Multithreading for concurrent requests:
  - Thread pooling with a limited number of threads.
  - Message queue for sending jobs to thread workers.
