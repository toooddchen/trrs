# Tiny Renderer written in Rust

Rust implementation of https://github.com/ssloy/tinyrenderer/wiki.

https://github.com/ssloy/tinyrenderer/wiki 的 rust 实现.

## Compile&Run 编译运行

```
git clone https://github.com/toooddchen/trrs.git &&
cd trrs
cargo run -r 
```
open http://localhost:8080/wire

## Description 说明

The program is implemented as an axum web service, with different routers corresponding to different lessons.

程序被实现为一个`axum` web服务, 不同的router对应不同的课节. 

### `/sample-line`
![](/public/sample-line.png)
### `/wire`
[Lesson 1](https://github.com/ssloy/tinyrenderer/wiki/Lesson-1:-Bresenham%E2%80%99s-Line-Drawing-Algorithm)
![](/public/wire.png)

### `/sample-triangle`
![](/public/sample-triangle.png)
### `/flat-shading`
Flat shading render
![](/public/flat-shading.png)
### `/linear-light`
Back-face culling
![](/public/linear-light.png)
### `/z-buf`
Hidden faces removal
![](/public/z-buf.png)
### `/move-camera`
Perspective projection & Moving the camera
Gouraud shading

![](/public/move-camera.png)
### `/shaders/gouraud`
Gouraud shading
![](/public/shaders-gouraud.png)
### `/shaders/gouraud6l`
Simple modification of the Gourad shading, where the intensities are allowed to have 6 values only,
![](/public/shaders-gouraud6l.png)
### `/shaders/texture`
![](/public/shaders-texture.png)
### `/shaders/normalmapping`
Normal Mapping
![](/public/shaders-normalmapping.png)
### `/shaders/specularmapping`
Specular Mapping
![](/public/shaders-specularmapping.png)
### `/shaders/shadowmapping`
Hard Shadows
![](/public/shaders-shadowmapping.png)
