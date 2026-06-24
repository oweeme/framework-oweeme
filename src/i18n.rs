use dashmap::DashMap;
use serde_json::Value;
use std::sync::Arc;

/// Motor de internacionalización.
///
/// Las traducciones se cargan desde archivos JSON en `locales/`:
///   locales/es.json, locales/en.json, locales/pt.json ...
///
/// Formato JSON (anidado soportado):
/// {
///   "nav.home": "Inicio",
///   "articulo.by": "Por {autor}",
///   "seo.home.title": "Música para todos"
/// }
#[derive(Clone)]
pub struct I18n {
    translations: Arc<DashMap<String, DashMap<String, String>>>,
    default_lang: String,
}

impl I18n {
    pub fn new(default_lang: impl Into<String>) -> Self {
        I18n {
            translations: Arc::new(DashMap::new()),
            default_lang: default_lang.into(),
        }
    }

    /// Carga un idioma desde un objeto JSON.
    pub fn load_lang(&self, lang: &str, data: &Value) {
        let map = DashMap::new();
        flatten_json(data, "", &map);
        self.translations.insert(lang.to_string(), map);
    }

    /// Carga todos los archivos JSON del directorio `locales/`.
    pub fn load_dir(&self, dir: &str) -> anyhow::Result<()> {
        let path = std::path::Path::new(dir);
        if !path.exists() {
            return Ok(());
        }
        for entry in std::fs::read_dir(path)? {
            let entry = entry?;
            let file_path = entry.path();
            if file_path.extension().and_then(|e| e.to_str()) == Some("json") {
                let lang = file_path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("es")
                    .to_string();
                let content = std::fs::read_to_string(&file_path)?;
                let json: Value = serde_json::from_str(&content)?;
                self.load_lang(&lang, &json);
                tracing::info!("i18n: cargado idioma '{lang}'");
            }
        }
        Ok(())
    }

    /// Traduce una clave. Si no existe en el idioma solicitado, cae al default.
    /// Si tampoco existe, devuelve la clave tal cual.
    pub fn t(&self, lang: &str, key: &str) -> String {
        // Intenta el idioma solicitado
        if let Some(map) = self.translations.get(lang) {
            if let Some(val) = map.get(key) {
                return val.clone();
            }
        }
        // Fallback al idioma por defecto
        if lang != self.default_lang {
            if let Some(map) = self.translations.get(&self.default_lang) {
                if let Some(val) = map.get(key) {
                    return val.clone();
                }
            }
        }
        key.to_string()
    }

    /// Traduce con interpolación de variables.
    /// Ejemplo: t_with("es", "articulo.by", &[("autor", "Héctor")])
    ///   → "Por Héctor"
    pub fn t_with(&self, lang: &str, key: &str, vars: &[(&str, &str)]) -> String {
        let mut text = self.t(lang, key);
        for (k, v) in vars {
            text = text.replace(&format!("{{{k}}}"), v);
        }
        text
    }

    /// Devuelve todos los idiomas cargados.
    pub fn available_langs(&self) -> Vec<String> {
        self.translations.iter().map(|e| e.key().clone()).collect()
    }

    /// Construye el contexto Tera con todas las traducciones del idioma dado.
    /// Las claves quedan disponibles como `t.nav_home`, `t.articulo_by`, etc.
    pub fn tera_context(&self, lang: &str) -> tera::Context {
        let mut ctx = tera::Context::new();
        ctx.insert("lang", lang);

        let mut translations = serde_json::Map::new();
        if let Some(map) = self.translations.get(lang) {
            for entry in map.iter() {
                // "nav.home" → t["nav.home"]
                translations.insert(entry.key().clone(), Value::String(entry.value().clone()));
            }
        }
        ctx.insert("t", &Value::Object(translations));
        ctx
    }

    pub fn default_lang(&self) -> &str {
        &self.default_lang
    }
}

/// Aplana JSON anidado en claves dot-notation.
/// { "nav": { "home": "Inicio" } } → "nav.home" = "Inicio"
fn flatten_json(value: &Value, prefix: &str, map: &DashMap<String, String>) {
    match value {
        Value::Object(obj) => {
            for (k, v) in obj {
                let key = if prefix.is_empty() {
                    k.clone()
                } else {
                    format!("{prefix}.{k}")
                };
                flatten_json(v, &key, map);
            }
        }
        Value::String(s) => {
            map.insert(prefix.to_string(), s.clone());
        }
        other => {
            map.insert(prefix.to_string(), other.to_string());
        }
    }
}

/// Detecta el idioma preferido del usuario desde el header Accept-Language.
/// Devuelve el código corto (ej. "es", "en", "pt").
pub fn detect_lang_from_header(accept_language: Option<&str>, available: &[String]) -> String {
    let header = match accept_language {
        Some(h) if !h.is_empty() => h,
        _ => return "es".to_string(),
    };

    // Parsea "es-ES,es;q=0.9,en;q=0.8" → ["es", "en"]
    let preferred: Vec<&str> = header
        .split(',')
        .filter_map(|part| part.split(';').next())
        .map(|lang| lang.trim())
        .collect();

    for lang in &preferred {
        let short = lang.split('-').next().unwrap_or(lang);
        if available.iter().any(|a| a == short) {
            return short.to_string();
        }
    }

    "es".to_string()
}
