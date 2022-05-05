# gl2d-sandbox

Scalable text rendering with WebGL + Rust + WASM

This does not use CPU for font rasterization and does not have a font atlas cache.

Instead, the shaders are used to render glyphs.

## Demo

https://gl2d-sandbox.pages.dev/

## Algorithm

This uses the algorithm based on [Easy Scalable Text Rendering on the GPU | by Evan Wallace | Medium](https://medium.com/@evanwallace/easy-scalable-text-rendering-on-the-gpu-c3f4d782c5ac) but anti-aliasing is not implemented.
