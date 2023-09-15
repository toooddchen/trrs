use axum::{
    http,
    response::{AppendHeaders, IntoResponse},
    routing::get,
    Router,
};
mod camera;
mod geometry;
mod gl;
mod line;
mod matrix;
mod model;
mod shaders;
mod triangle;
mod util;
mod zbuf;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/wire", get(wire))
        .route("/line", get(sample_line))
        .route("/triangle", get(sample_triangle))
        .route("/flat-shading", get(flat_shading))
        .route("/z-buf", get(z_buf))
        .route("/move-camera", get(move_camera))
        .route("/linear-light", get(linear_light))
        .route("/shaders/gouraud", get(shader_gouraud))
        .route("/shaders/gouraud6l", get(shader_gouraud6l))
        .route("/shaders/texture", get(shader_texture))
        .route("/shaders/normalmapping", get(shader_normal_mapping))
        .route("/shaders/specularmapping", get(shader_specular_mapping))
        .route("/shaders/shadowmapping", get(shader_shadow_mapping));

    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service()) // Your code here
        .await
        .unwrap();
}

async fn wire() -> impl IntoResponse {
    let bs = line::wireframe();
    (
        AppendHeaders([(http::header::CONTENT_TYPE, "image/png")]),
        bs,
    )
}

async fn sample_line() -> impl IntoResponse {
    let bs = line::sample_line();
    (
        AppendHeaders([(http::header::CONTENT_TYPE, "image/png")]),
        bs,
    )
}

async fn sample_triangle() -> impl IntoResponse {
    let bs = triangle::sample_triangle();
    (
        AppendHeaders([(http::header::CONTENT_TYPE, "image/png")]),
        bs,
    )
}

async fn flat_shading() -> impl IntoResponse {
    let bs = triangle::flat_shading();
    (
        AppendHeaders([(http::header::CONTENT_TYPE, "image/png")]),
        bs,
    )
}

async fn linear_light() -> impl IntoResponse {
    let bs = triangle::linear_light();
    (
        AppendHeaders([(http::header::CONTENT_TYPE, "image/png")]),
        bs,
    )
}

async fn z_buf() -> impl IntoResponse {
    let bs = zbuf::z_buf();
    (
        AppendHeaders([(http::header::CONTENT_TYPE, "image/png")]),
        bs,
    )
}

async fn move_camera() -> impl IntoResponse {
    let bs = camera::move_camera();
    (
        AppendHeaders([(http::header::CONTENT_TYPE, "image/png")]),
        bs,
    )
}

async fn shader_gouraud() -> impl IntoResponse {
    let bs = shaders::gouraud::gouraud_render();
    (
        AppendHeaders([(http::header::CONTENT_TYPE, "image/png")]),
        bs,
    )
}

async fn shader_gouraud6l() -> impl IntoResponse {
    let bs = shaders::gouraud6l::gouraud6l_render();
    (
        AppendHeaders([(http::header::CONTENT_TYPE, "image/png")]),
        bs,
    )
}

async fn shader_texture() -> impl IntoResponse {
    let bs = shaders::texture::texture_render();
    (
        AppendHeaders([(http::header::CONTENT_TYPE, "image/png")]),
        bs,
    )
}

async fn shader_normal_mapping() -> impl IntoResponse {
    let bs = shaders::normalmapping::normal_mapping_render();
    (
        AppendHeaders([(http::header::CONTENT_TYPE, "image/png")]),
        bs,
    )
}

async fn shader_specular_mapping() -> impl IntoResponse {
    let bs = shaders::specularmapping::specular_mapping_render();
    (
        AppendHeaders([(http::header::CONTENT_TYPE, "image/png")]),
        bs,
    )
}

async fn shader_shadow_mapping() -> impl IntoResponse {
    let bs = shaders::shadowmapping::shadow_mapping_render();
    (
        AppendHeaders([(http::header::CONTENT_TYPE, "image/png")]),
        bs,
    )
}
