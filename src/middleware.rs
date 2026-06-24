use axum::{
    body::Body,
    extract::Request,
    http::{header, HeaderValue, Response},
    middleware::Next,
};
use std::time::Instant;

/// Middleware que inyecta headers de seguridad y SEO en todas las respuestas HTML.
pub async fn security_headers(req: Request, next: Next) -> Response<Body> {
    let mut res = next.run(req).await;
    let h = res.headers_mut();

    // Seguridad
    h.insert(
        "X-Content-Type-Options",
        HeaderValue::from_static("nosniff"),
    );
    h.insert("X-Frame-Options", HeaderValue::from_static("SAMEORIGIN"));
    h.insert(
        "Referrer-Policy",
        HeaderValue::from_static("strict-origin-when-cross-origin"),
    );
    h.insert(
        "Permissions-Policy",
        HeaderValue::from_static("geolocation=(), microphone=(), camera=()"),
    );
    h.insert(
        "Content-Security-Policy",
        HeaderValue::from_static(
            "default-src 'self'; \
             script-src 'self' 'unsafe-inline'; \
             style-src 'self' 'unsafe-inline'; \
             img-src 'self' data: https:; \
             font-src 'self' https:; \
             connect-src 'self' https:; \
             media-src 'self' https:",
        ),
    );

    // Cache: HTML sin cache, estáticos con cache largo
    let content_type = h
        .get(header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    if content_type.contains("text/html") {
        h.insert(
            header::CACHE_CONTROL,
            HeaderValue::from_static("public, max-age=60, stale-while-revalidate=300"),
        );
    }

    res
}

/// Middleware de logging de peticiones con tiempo de respuesta.
pub async fn request_logger(req: Request, next: Next) -> Response<Body> {
    let method = req.method().clone();
    let uri = req.uri().clone();
    let start = Instant::now();

    let res = next.run(req).await;

    let elapsed = start.elapsed();
    let status = res.status();
    tracing::info!(
        method = %method,
        path = %uri.path(),
        status = status.as_u16(),
        ms = elapsed.as_millis(),
    );

    res
}
