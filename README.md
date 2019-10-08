# rTst - Rust Test project

Sample project for testing purposes.

## Building

### Install bazelisk

Install the wrapper tool [`bazelisk`][] to make sure you build with the correct version of bazel.

On macOS this is just `brew install bazelisk`.

### Build

Build everything with:

```bash
bazelisk build ...
```

### Run

Run the built binary with:

```bash
bazelisk run rtst
```

Run the built docker image with:

```bash
bazelisk run rtst_image
```

[`bazelisk`]: https://github.com/bazelbuild/bazelisk
