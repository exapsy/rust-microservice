# Rust microservice template

This is a template supposed to be used for rust microservices.

It contains an example implementation of a production user service.
Anything related to specific uses may be removed, but the general template should remain the same.

## How to use this template

1. Clone it `git clone https://gitlab.com/logifox/templates/rust/microservice.git`
2. Make sure you acquire the [required dependencies](#Requirements)
3. Change the [configuration](#Configurations) to your liking
4. Change the [CHANGELOG.md](./CHANGELOG.md) every time there is a new version.
5. Run `cargo run` :)
6. **Note:** As a rule of thumb, _do not change template related files. You may change only use-specific related files._

## Requirements

- `rustup`
- `rust 1.66.0-nightly`

## Configurations

### Rocket.toml

`Rocket.toml` is a configuration file that contains the configuration for Rocket http server library.
For more information take guidance from the [v0.5-rc documentation](https://rocket.rs/v0.5-rc/guide/configuration/).

### .env (Environment variables)

- `MONGO_URI` _(optional)_: used to specify the mongodb database URI. If not used then no MongoDB connection is set.
- `ROCKET_CONFIG`: used to specify the path for the rocket configuration. If not found, rocket will use the default values.

### Cargo.toml

You're writing rust. Come on. You either know what is this for,
or you may see a space rocket explode somewhere in China if you use this template without knowing what is this for.
/very professional/

### .gitignore

Used to ignore files from git versioning manager. You know what is this for come on. /not passive aggressive at all/
