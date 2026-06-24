use wasm_bindgen::prelude::*;

// ─── Slug ────────────────────────────────────────────────────────────────────

/// Genera un slug SEO a partir de un título.
/// "La Novia Baila! (2024)" → "la-novia-baila-2024"
/// Disponible en el navegador como `oweeme.slug("Mi Título")`.
#[wasm_bindgen]
pub fn slug(text: &str) -> String {
    let lower = text.to_lowercase();
    let mut result = String::with_capacity(lower.len());
    let mut last_was_dash = false;

    for ch in lower.chars() {
        if ch.is_ascii_alphanumeric() {
            result.push(ch);
            last_was_dash = false;
        } else if "áàäâ".contains(ch) {
            result.push('a'); last_was_dash = false;
        } else if "éèëê".contains(ch) {
            result.push('e'); last_was_dash = false;
        } else if "íìïî".contains(ch) {
            result.push('i'); last_was_dash = false;
        } else if "óòöô".contains(ch) {
            result.push('o'); last_was_dash = false;
        } else if "úùüû".contains(ch) {
            result.push('u'); last_was_dash = false;
        } else if ch == 'ñ' {
            result.push('n'); last_was_dash = false;
        } else if !last_was_dash && !result.is_empty() {
            result.push('-');
            last_was_dash = true;
        }
    }

    // Elimina guiones al final
    result.trim_end_matches('-').to_string()
}

// ─── Validación SEO ───────────────────────────────────────────────────────────

/// Resultado de validación SEO.
#[wasm_bindgen]
pub struct SeoValidation {
    pub valid: bool,
    issues: Vec<String>,
}

#[wasm_bindgen]
impl SeoValidation {
    /// Lista de problemas encontrados (separados por \n).
    pub fn issues_text(&self) -> String {
        self.issues.join("\n")
    }

    pub fn issues_count(&self) -> usize {
        self.issues.len()
    }
}

/// Valida un título y descripción SEO directamente en el navegador.
/// Úsalo antes de publicar contenido para detectar problemas.
#[wasm_bindgen]
pub fn validate_seo(title: &str, description: &str) -> SeoValidation {
    let mut issues = Vec::new();

    // Título
    if title.is_empty() {
        issues.push("El título no puede estar vacío".into());
    } else if title.len() < 10 {
        issues.push(format!("Título muy corto ({} chars, mín 10)", title.len()));
    } else if title.len() > 60 {
        issues.push(format!("Título muy largo ({} chars, máx 60)", title.len()));
    }

    // Descripción
    if description.is_empty() {
        issues.push("La descripción no puede estar vacía".into());
    } else if description.len() < 50 {
        issues.push(format!("Descripción muy corta ({} chars, mín 50)", description.len()));
    } else if description.len() > 160 {
        issues.push(format!("Descripción muy larga ({} chars, máx 160)", description.len()));
    }

    SeoValidation {
        valid: issues.is_empty(),
        issues,
    }
}

// ─── Contador de lectura ──────────────────────────────────────────────────────

/// Estima el tiempo de lectura en minutos (200 palabras/min promedio).
#[wasm_bindgen]
pub fn reading_time_minutes(text: &str) -> u32 {
    let words = text.split_whitespace().count();
    ((words as f32 / 200.0).ceil() as u32).max(1)
}

// ─── Truncar descripción SEO ──────────────────────────────────────────────────

/// Recorta un texto a `max_len` caracteres sin cortar palabras,
/// añadiendo "…" al final. Ideal para meta descriptions.
#[wasm_bindgen]
pub fn truncate_seo(text: &str, max_len: usize) -> String {
    if text.len() <= max_len {
        return text.to_string();
    }
    let cut = &text[..max_len];
    // Retrocede hasta el último espacio
    let last_space = cut.rfind(' ').unwrap_or(max_len);
    format!("{}…", &text[..last_space].trim_end())
}
