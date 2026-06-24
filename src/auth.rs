use axum::{
    body::Body,
    extract::Request,
    http::{header, Response, StatusCode},
    middleware::Next,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};

/// Datos del usuario autenticado inyectados como extensión de la request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthUser {
    pub id: String,
    pub username: String,
    pub lang: String,
    pub roles: Vec<String>,
}

impl AuthUser {
    pub fn is_admin(&self) -> bool {
        self.roles.iter().any(|r| r == "admin")
    }
}

/// Middleware de autenticación opcional.
///
/// Lee el token desde:
///   1. Cookie `session_token`
///   2. Header `Authorization: Bearer <token>`
///
/// Si el token existe lo valida contra la API del backend.
/// Si no existe, continúa sin usuario (rutas públicas).
/// Las rutas que requieren auth deben llamar a `require_auth`.
pub async fn optional_auth(mut req: Request, next: Next) -> Response<Body> {
    let token = extract_token(&req);

    if let Some(token) = token {
        // El token se valida en el backend — aquí solo lo propagamos
        // en una extensión para que los handlers puedan leerlo.
        // La validación real ocurre en el ApiClient al llamar con Bearer.
        req.extensions_mut().insert(RawToken(token));
    }

    next.run(req).await
}

/// Middleware que bloquea si no hay token.
/// Úsalo en rutas que requieren sesión activa.
pub async fn require_auth(req: Request, next: Next) -> Response<Body> {
    if req.extensions().get::<RawToken>().is_none() {
        return (StatusCode::UNAUTHORIZED, "Se requiere autenticación")
            .into_response();
    }
    next.run(req).await
}

/// Token crudo extraído de la request.
#[derive(Clone)]
pub struct RawToken(pub String);

fn extract_token(req: &Request) -> Option<String> {
    // 1. Cookie
    if let Some(cookie_header) = req.headers().get(header::COOKIE) {
        if let Ok(cookies) = cookie_header.to_str() {
            for cookie in cookies.split(';') {
                let cookie = cookie.trim();
                if let Some(val) = cookie.strip_prefix("session_token=") {
                    if !val.is_empty() {
                        return Some(val.to_string());
                    }
                }
            }
        }
    }

    // 2. Authorization: Bearer
    if let Some(auth_header) = req.headers().get(header::AUTHORIZATION) {
        if let Ok(value) = auth_header.to_str() {
            if let Some(token) = value.strip_prefix("Bearer ") {
                if !token.is_empty() {
                    return Some(token.to_string());
                }
            }
        }
    }

    None
}
