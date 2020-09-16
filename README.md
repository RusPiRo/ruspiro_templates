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

## Prepare Pipeline Builds

To enable the full pipeline lifecycle for new crates the integration into `travis-ci` is required. The following prerequisits has to be configured:

1. Create a `release` branch in the github repository. Ensure that this branch is protected and requires PR review as well as travis-ci status checks (flag may only be available after the first travis-ci pipeline run).
2. Add the github repository of this crate to the travis-ci settings for the `RusPiRo` organisation.
3. create an OAuth github token and maintain it in the environment variable settings for this repo in travis-ci with the variable name `GIT_API_TOKEN`.
4. create a crates.io login token and mainain it in the environment variable settings for this repo in travis-ci with the variable name `CRATES_TOKEN`.

## Pipeline runs

The pipeline will run the following steps:

1. Each PR to `master` will trigger a first pipeline build. This build will do the following:
    - run the compilitaion of the library crate with `cargo make pi3`. This build will take the `Cargo.toml` and maintained dependencies *as-is*. This means the `[patch.crates-io]` section can be used to refer to dendent *ruspiro-* crates from their github repositories if multiple dependent changes are *in-flight*.
    - run the doc tests and unit tests of the crate with `cargo make doctest` and `cargo test --tests`
    - run a crates.io publish dry-run
2. If the PR is merged into the `master` branch the same steps from above are executed
    - run the compilitaion of the library crate with `cargo make pi3`. However, this time the `[patch.crates-io]` section of the `Cargo.toml` file will be removed and the used dependencies requires to be available on [crates.io](https://crates.io)
    - run the doc tests and unit tests of the crate with `cargo make doctest` and `cargo test --tests`
    - run a crates.io publish dry-run
    - create a new PR from `master` branch to `release` branch
3. The PR from `master` to `release` branch will be build exactly the same way the first PR to `master` was built.
4. If the PR is merged into the `release` branch it will do the following
    - run the compilitaion of the library crate with `cargo make pi3`. However, this time the `[patch.crates-io]` section of the `Cargo.toml` file will be removed and the used dependencies requires to be available on [crates.io](https://crates.io)
    - run the doc tests and unit tests of the crate with `cargo make doctest` and `cargo test --tests`
    - run a crates.io publish dry-run
    - create a github release with the version number provided in `Cargo.toml` file. This step will also update the `||VERSION||` placeholder in the respective files to ensure consistency
    - publish the actual crate to crates.io

After the first PR has been build using travis-ci you should revisit the branch protection rules for `master` and `release` branch to ensure the pipeline build step is marked as required before pushing to those branches is allowed.

## License

Licensed under Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0) or MIT ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)) at your choice.
