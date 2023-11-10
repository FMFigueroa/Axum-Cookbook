<<<<<<< HEAD
# REST API with Axum by Tokio

![](./public/img/architecture.png)
=======
# Axum cookbook

<p align="center">
<img  src="https://res.cloudinary.com/rustlatamgroup/image/upload/v1685920322/assets/Banner_LinkedIn_gvrmzo.png">
</p>

## Breve Descripción

<p align="justify">
Bienvenido a todos, aquí aprenderás los primeros pasos para el desarrollo backend con Rust, a través de tutoriales demo, conocerás en profundidad los crates más populares del ecosistema para diseñar una API RESTful desde cero. Rust es un lenguaje muy potente y uno de sus principales focos de diseño es la concurrencia. Iniciaremos haciendo un resumen breve de la concurrrencia en Rust para entender los fundamentos y cómo implementar estos creates.
El crate mas popular de los frameworks backend en Rust se llama Tokio; pero Tokio también ha creado su propio framework backend llamado Axum, y aquí vamos a conocerlo desde cero.
>>>>>>> abeda8d1baa157ef0858a8c4f1b63a192df734b3

### Crates

- [axum](https://crates.io/crates/axum): web framework that focuses on ergonomics and modularity.

- [tokio](https://tokio.rs): platform for writing asynchronous I/O backed applications.

- [tower](https://crates.io/crates/tower): library for building robust clients and servers.

<<<<<<< HEAD
- [serde](https://crates.io/crates/serde): serialization/deserialization framework.
=======
- [Serde](https://crates.io/crates/serde): serialization/deserialization framework.
>>>>>>> abeda8d1baa157ef0858a8c4f1b63a192df734b3

<hr>

## Quick Dev with  Cargo Watch

> NOTE: Install globally cargo watch with `cargo install cargo-watch`.

### Terminal 1 - To run the server.
``` 
cargo watch -q -c -w src/ -x "run"
```
### Terminal 2 - To run the client.
```
cargo watch -q -c -w examples/ -x "run --example quick_dev"
```