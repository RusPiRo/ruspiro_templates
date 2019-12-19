# RusPiRo Templates

Each template project available to be used can be found as a separate branch to this repository.

## Usage
To create a project from one of the existing templates use [`cargo-generate`](https://crates.io/crates/cargo-generate).
After installation of the tool create a new project like so:
```
$> cargo generate --git https://github.com/RusPiRo/ruspiro_templates.git --branch <template_branch_name>
```

## Available templates
Template                                      | Description
----------------------------------------------|-------------------------------------------------------------------------------
[01_minimal](../../tree/templates/01_minimal) | Bare minimum template to build a simple bare metal kernel for the Raspberry Pi

## License
Licensed under Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0) or MIT ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)) at your choice.