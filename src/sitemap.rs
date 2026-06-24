use crate::api::ApiClient;
use chrono::Utc;

#[derive(Debug, Clone)]
pub struct SitemapUrl {
    pub loc: String,
    pub lastmod: Option<String>,
    pub changefreq: Option<Changefreq>,
    pub priority: Option<f32>,
}

#[derive(Debug, Clone, Copy)]
pub enum Changefreq {
    Always,
    Hourly,
    Daily,
    Weekly,
    Monthly,
    Yearly,
    Never,
}

impl Changefreq {
    fn as_str(&self) -> &'static str {
        match self {
            Changefreq::Always => "always",
            Changefreq::Hourly => "hourly",
            Changefreq::Daily => "daily",
            Changefreq::Weekly => "weekly",
            Changefreq::Monthly => "monthly",
            Changefreq::Yearly => "yearly",
            Changefreq::Never => "never",
        }
    }
}

impl SitemapUrl {
    pub fn new(loc: impl Into<String>) -> Self {
        SitemapUrl {
            loc: loc.into(),
            lastmod: Some(Utc::now().format("%Y-%m-%d").to_string()),
            changefreq: Some(Changefreq::Weekly),
            priority: Some(0.8),
        }
    }

    pub fn with_priority(mut self, p: f32) -> Self {
        self.priority = Some(p.clamp(0.0, 1.0));
        self
    }

    pub fn with_changefreq(mut self, cf: Changefreq) -> Self {
        self.changefreq = Some(cf);
        self
    }

    pub fn with_lastmod(mut self, date: impl Into<String>) -> Self {
        self.lastmod = Some(date.into());
        self
    }
}

/// Genera sitemap.xml a partir de una lista de URLs.
pub fn generate_sitemap(urls: &[SitemapUrl]) -> String {
    let mut xml = String::from(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\
         <urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">\n",
    );
    for u in urls {
        xml.push_str("  <url>\n");
        xml.push_str(&format!("    <loc>{}</loc>\n", u.loc));
        if let Some(ref lm) = u.lastmod {
            xml.push_str(&format!("    <lastmod>{lm}</lastmod>\n"));
        }
        if let Some(ref cf) = u.changefreq {
            xml.push_str(&format!("    <changefreq>{}</changefreq>\n", cf.as_str()));
        }
        if let Some(p) = u.priority {
            xml.push_str(&format!("    <priority>{p:.1}</priority>\n"));
        }
        xml.push_str("  </url>\n");
    }
    xml.push_str("</urlset>\n");
    xml
}

/// Construye el sitemap completo consultando la API por todos los slugs.
///
/// Espera que la API tenga:
/// - GET /sitemap/articulos → [ { slug, updated_at } ]
/// - GET /sitemap/musica    → [ { slug, updated_at } ]
/// - GET /sitemap/trabajos  → [ { slug, updated_at } ]
pub async fn build_dynamic_sitemap(api: &ApiClient, site_url: &str) -> String {
    let today = Utc::now().format("%Y-%m-%d").to_string();
    let mut urls = vec![
        SitemapUrl::new(format!("{site_url}/"))
            .with_priority(1.0)
            .with_changefreq(Changefreq::Daily),
    ];

    // Artículos
    if let Ok(items) = api.get_json("/sitemap/articulos").await {
        if let Some(arr) = items.as_array() {
            for item in arr {
                let slug = item["slug"].as_str().unwrap_or_default();
                let lastmod = item["updated_at"].as_str().unwrap_or(&today).to_string();
                if !slug.is_empty() {
                    urls.push(
                        SitemapUrl::new(format!("{site_url}/articulo/{slug}"))
                            .with_priority(0.9)
                            .with_changefreq(Changefreq::Weekly)
                            .with_lastmod(lastmod),
                    );
                }
            }
        }
    }

    // Música
    if let Ok(items) = api.get_json("/sitemap/musica").await {
        if let Some(arr) = items.as_array() {
            for item in arr {
                let slug = item["slug"].as_str().unwrap_or_default();
                let lastmod = item["updated_at"].as_str().unwrap_or(&today).to_string();
                if !slug.is_empty() {
                    urls.push(
                        SitemapUrl::new(format!("{site_url}/musica/{slug}"))
                            .with_priority(0.8)
                            .with_changefreq(Changefreq::Monthly)
                            .with_lastmod(lastmod),
                    );
                }
            }
        }
    }

    // Trabajos
    if let Ok(items) = api.get_json("/sitemap/trabajos").await {
        if let Some(arr) = items.as_array() {
            for item in arr {
                let slug = item["slug"].as_str().unwrap_or_default();
                let lastmod = item["updated_at"].as_str().unwrap_or(&today).to_string();
                if !slug.is_empty() {
                    urls.push(
                        SitemapUrl::new(format!("{site_url}/trabajo/{slug}"))
                            .with_priority(0.7)
                            .with_changefreq(Changefreq::Daily)
                            .with_lastmod(lastmod),
                    );
                }
            }
        }
    }

    generate_sitemap(&urls)
}

/// Genera el robots.txt con las rutas correctas.
pub fn generate_robots(site_url: &str) -> String {
    format!(
        "User-agent: *\n\
         Allow: /\n\
         Disallow: /api/\n\
         Disallow: /admin/\n\
         \n\
         Sitemap: {site_url}/sitemap.xml\n"
    )
}
