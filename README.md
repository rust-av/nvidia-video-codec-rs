# NVIDIA Video Codec SDK bindings

[![LICENSE](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

It is a simple [binding][1] and safe abstraction over the [nvidia video codec sdk][2].

## Building

The bindings are generated using the headers and libraries that ought to be present in the system.

By default the headers are looked up on `/opt/cuda/include` and `/opt/nvidia-video-codec` and the libraries are assumed to be present in the default path (and provided by the driver).

It is possible to override the search paths for the headers by setting the environment variables `CUDA_INCLUDE_PATH` and `NVIDIA_VIDEO_CODEC_INCLUDE_PATH`.

## TODO

- [ ] support cuda
  - [x] Simple bindings
  - [ ] Safe abstraction
- [ ] support cuvid
  - [x] Simple bindings
  - [ ] Safe abstraction
- [ ] support nvenc
  - [x] Simple bindings
  - [ ] Safe abstraction
- [ ] Examples

[1]: https://github.com/servo/rust-bindgen
[2]: https://developer.nvidia.com/nvidia-video-codec-sdk
