# Rust Echo Server

This project is a simple echo server written in Rust. The server listens for incoming TCP connections and echoes back any data it receives from clients.

## Features

- Listens for incoming connections on port `1337`.
- Handles each client connection in a separate thread.
- Echoes back any data received from the client.

## Prerequisites

- Rust and Cargo installed. You can install Rust using `rustup`, the recommended installer for Rust:

  ```sh
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
