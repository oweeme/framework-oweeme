use crate::seo::SeoMeta;
use anyhow::Result;
use once_cell::sync::OnceCell;
use regex::Regex;
use std::path::Path;
use tera::{Context, Tera};

static COMPONENT_RE: OnceCell<Regex> = OnceCell::new();

fn component_regex() -> &'static Regex {
    COMPONENT_RE.get_or_init(|| {
        // Matches <ComponentName key="val" key2="val2" /> or <ComponentName ...>...</ComponentName>
        Regex::new(r#"<([A-Z][A-Za-z0-9]*)((?:\s+[a-z_][a-zA-Z0-9_]*="[^"]*")*)\s*/>"#).unwrap()
    })
}

/// Pre-procesa la sintaxis de componentes tipo Vue antes de pasarla a Tera.
///
/// Convierte:
///   `<Articulo titulo="Hola" descripcion="Mundo" />`
/// En:
///   `{% include "components/Articulo.html" %}`
///   con variables inyectadas como bloques set.
pub fn preprocess_components(source: &str) -> String {
    let re = component_regex();
    re.replace_all(source, |caps: &regex::Captures| {
        let name = &caps[1];
        let attrs_str = &caps[2];

        // Parsear atributos key="value"
        let attr_re = Regex::new(r#"([a-z_][a-zA-Z0-9_]*)="([^"]*)""#).unwrap();
        let mut sets = String::new();
        for attr in attr_re.captures_iter(attrs_str) {
            sets.push_str(&format!(
                "{{% set {} = \"{}\" %}}\n",
                &attr[1], &attr[2]
            ));
        }
        format!(
            "{}{{% include \"components/{}.html\" %}}",
            sets, name
        )
    })
    .to_string()
}

/// Motor de plantillas del framework.
pub struct TemplateEngine {
    tera: Tera,
}

impl TemplateEngine {
    /// Crea el motor cargando todas las plantillas del directorio.
    pub fn new(templates_dir: impl AsRef<Path>) -> Result<Self> {
        let pattern = format!("{}/**/*.html", templates_dir.as_ref().display());
        let mut tera = Tera::new(&pattern)?;
        tera.autoescape_on(vec!["html"]);
        Ok(TemplateEngine { tera })
    }

    /// Recarga plantillas (útil en dev watch).
    pub fn reload(&mut self) -> Result<()> {
        self.tera.full_reload()?;
        Ok(())
    }

    /// Renderiza una plantilla con SEO y datos de la API.
    ///
    /// - `template`: nombre relativo al directorio templates/ (ej. "pages/articulo.html")
    /// - `seo`: metadatos SEO inyectados automáticamente
    /// - `data`: datos JSON de la API convertidos a contexto Tera
    pub fn render(&self, template: &str, seo: &SeoMeta, data: &serde_json::Value) -> Result<String> {
        let mut ctx = seo.to_tera_context();
        // Inyecta todos los campos del JSON de la API directamente en el contexto
        if let serde_json::Value::Object(map) = data {
            for (k, v) in map {
                ctx.insert(k, v);
            }
        }
        ctx.insert("api_data", data);
        let html = self.tera.render(template, &ctx)?;
        Ok(html)
    }

    /// Renderiza con contexto manual (sin datos de API).
    pub fn render_ctx(&self, template: &str, ctx: &Context) -> Result<String> {
        let html = self.tera.render(template, ctx)?;
        Ok(html)
    }

    /// Registra una plantilla en memoria (útil para componentes dinámicos).
    pub fn add_raw_template(&mut self, name: &str, source: &str) -> Result<()> {
        let processed = preprocess_components(source);
        self.tera.add_raw_template(name, &processed)?;
        Ok(())
    }
}
