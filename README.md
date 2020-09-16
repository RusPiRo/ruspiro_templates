# RusPiRo Library Template

This is template to be used when creating new *RusPiRo library crates*.

## Contents

This template provides all default files necessary to jump start library development. It provides a `Makefile.toml` to execute local builds as well as a Travis-CI configuration file (`.travis.yml`) to support pipeline builds.

## Usage

The following command will create a new project based on this template

```shell
$> cargo generate --git https://github.com/RusPiRo/ruspiro_templates.git --branch templates/ruspiro-lib --name <project-name>
```

Once done the `README.md` file can be deleted and the `README.template` (providing the project readme skeleton) can be renamed to `README.md`. Before commiting the new projects to a github repository make sure the correct branch name is used (eg. *master*) as the `cargo generate` typically defined the branch the project was generated from as the initial branch.

## Local Builds

To execute local builds `cargo-make` and `cargo-xbuild` is required. Also ensure the `aarch64-unknown-none` rust build target is installed as well as it's `src` components. It's safe to use the following commands as a one time setup, assuming the `aarch64-none-elf` crosscompilation toolchain is installed already.

```shell
$> cargo install cargo-xbuild
$> cargo install cargo-make
$> rustup target add aarch64-unknown-none
$> rustup component add rust-src
$> rustup component add llvm-tools-preview
```

## License

Licensed under Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0) or MIT ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)) at your choice.
