#! /bin/sh

for name in avcodec avfilter avformat avutil swscale; do
  echo "$name"
  env TARGET=wasm32-wasi bindgen ~/src/ffmpeg/out/include/*/"${name}.h" \
    --no-layout-tests \
    --with-derive-default --with-derive-eq \
    --with-derive-partialeq \
    --with-derive-hash --with-derive-ord --with-derive-partialord \
    --enable-function-attribute-detection \
    -o "src/${name}.rs" \
    -- -I ~/src/ffmpeg/out/include \
    -I /opt/zig/lib/zig/libc/include/wasm-wasi-musl \
    -fvisibility=default
done
