# Creación de una API Rest con Rust

# INCOMPLETO!

-  ## Creación del proyecto con CARGO.

Cargo es el gestor de dependencias de Rust, para inicializar el proyecto ejecutaremos el siguiente comando:
```bash
cargo new api --bin
```

Al ser ejecutado en el directorio raiz de nuestro proyecto nos creará un directorio llamado "_api_" y el correspondiente fichero 
"_Cargo.toml_" el cual debe contener las siguientes líneas:

```rust
[package]
name = "api"
version = "0.1.0"
authors = ["GONZALO MATEOS GLEZ-SICILIA <gonzalomateosglezsicilia@gmail.com>"]

[dependencies]
diesel = { version = "1.0.0", features = ["postgres"] }
diesel_codegen = { version = "0.16.0" features = ["postgres"] }
dotenv = "0.9.0"
```

Para continuar con la instalación de dependencias debemos ejecutar la siguiente línea:

```bash
cargo install diesel_cli --no-default-features --features postgres
```

Esta se descará las dependencias y compilará lo necesario para poder trabajar.

- ## Configuracion del entorno.

Para configurar el entorno, lo primero que vamos a hacer es ejecutar el siguiente comando:

```bash
echo DATABASE_URL=postgres://postgres:postgres@localhost/api_dev > .env
```
**Ustedes deberán de modificar la cadena de conexión con los parámetros que correspondan**

Posteriormente deberemos realizar las configuraciones que ya hemos preparado con '_diesel_':

```bash
diesel setup
diesel migration generate create_books
diesel migration run
```
*NOTA:* Solo para hacer rollback:
```bash
diesel migration redo
```

El cual se conectará a la BBDD y creará la BBDD y el objeto que tenemos definido en el modulo `_models.rs_`

Creandonos un directorio llamado "_migrations_" el cual contiene los ficheros SQL que necesitamos.






