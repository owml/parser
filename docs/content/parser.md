# Parser Documentation

## About This Document

Owen's Markup Language (`owml` from now on) is a lightweight markup language. You can find the design specifications [here](language-spec.md).

In this document, we will briefly go over how to build from source and setup this parser in various enviroments.

## Installing

In this section, we will be going over how to install this parser.

### Downloading `owml-parser`

There is currently no way to download the latest owpm-parser build. It will hopefully be avalible with a [GitLab Job Artifact](https://docs.gitlab.com/ee/user/project/pipelines/job_artifacts.html) soon.

### Building From Source

To build from source, please first clone the git repository.

```bash
git clone https://gitlab.com/scOwez/owml-parser
```

Once you have the [git repository for owml](https://gitlab.com/scOwez/owml-parser/) saved locally, you will need to install Rust if you haven't already. You can find the steps to do this [here](https://www.rust-lang.org/tools/install/).

After installing Rust, please `cd` into `owml-parser/` and execute the following command to build owml

```bash
cargo build --release
```

This will build a release version of owml. It is reccomended to use the **nightly** version of Rust as it is guaranteed to work compared to Rust stable.

Once the build has completed, you can use the `owml-parser` library. The .rlib file (by default) is stored in the newly-created `target/release/` directory and should be called something similarly to **`owml_parser.rlib`**.
