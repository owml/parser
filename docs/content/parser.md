# Parser Documentation

## About This Document

Owen's Markup Language (`owml` from now on) is a lightweight markup language. You can find the design specifications [here](language-spec.md).

In this document, we will briefly go over how to build from source and setup this parser in various enviroments.

## Installing

In this section, we will be going over how to install this parser.

### Downloading `owml-parser`

The most recently passing [job artifacts](https://docs.gitlab.com/ee/user/project/pipelines/job_artifacts.html) (the system owml uses to automatically generate binaries on a sucsessful [CI run](https://docs.gitlab.com/ee/ci/)).

1. To get these "artifacts", please visit the **[Piplines](https://gitlab.com/scOwez/owml-parser/pipelines)** page for owml and find the most recent passing build. It should have an icon similar to:

    ![Example icon](https://i.imgur.com/o7kw9J8.png)

2. If a job has an icon like this, please view to the right of it where it should have the following button:

    ![Example download button](https://i.imgur.com/C50nhJ0.png)

3. Please press the dropdown button and select `Download img-build artifacts`.

4. Once you have downloaded the file, please unzip it and extract the `libowml-parser.rlib` file that should be located inside of the `target/release/` folder.

5. Congratulations! You have sucsessfully downloaded the cutting-edge owml release!

### Building From Source

1. To build from source, please first clone the git repository.

    ```bash
    git clone https://gitlab.com/scOwez/owml-parser
    ```

2. Once you have the [git repository for owml](https://gitlab.com/scOwez/owml-parser/) saved locally, you will need to install Rust if you haven't already. You can find the steps to do this [here](https://www.rust-lang.org/tools/install/).

3. After installing Rust, please `cd` into `owml-parser/` and execute the following command to build owml

    ```bash
    cargo build --release
    ```

4. This will build a release version of owml. It is reccomended to use the **nightly** version of Rust as it is guaranteed to work compared to Rust stable.

5. Once the build has completed, you can use the `owml-parser` library. The .rlib file (by default) is stored in the newly-created `target/release/` directory and should be called something similarly to **`owml_parser.rlib`**.
