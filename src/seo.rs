use serde::{Deserialize, Serialize};

/// Datos SEO que se inyectan en cada página renderizada.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SeoMeta {
    pub title: String,
    pub description: String,
    pub keywords: Option<String>,
    pub canonical: Option<String>,
    pub og_title: Option<String>,
    pub og_description: Option<String>,
    pub og_image: Option<String>,
    pub og_type: Option<String>,
    pub twitter_card: Option<String>,
    pub twitter_title: Option<String>,
    pub twitter_description: Option<String>,
    pub twitter_image: Option<String>,
    pub schema_json: Option<String>,
}

impl SeoMeta {
    pub fn new(title: impl Into<String>, description: impl Into<String>) -> Self {
        let title = title.into();
        let description = description.into();
        SeoMeta {
            og_title: Some(title.clone()),
            og_description: Some(description.clone()),
            og_type: Some("website".into()),
            twitter_card: Some("summary_large_image".into()),
            twitter_title: Some(title.clone()),
            twitter_description: Some(description.clone()),
            title,
            description,
            ..Default::default()
        }
    }

    pub fn with_image(mut self, url: impl Into<String>) -> Self {
        let url = url.into();
        self.og_image = Some(url.clone());
        self.twitter_image = Some(url);
        self
    }

    pub fn with_canonical(mut self, url: impl Into<String>) -> Self {
        self.canonical = Some(url.into());
        self
    }

    pub fn with_keywords(mut self, kw: impl Into<String>) -> Self {
        self.keywords = Some(kw.into());
        self
    }

    /// Genera marcado schema.org para un artículo.
    pub fn with_article_schema(
        mut self,
        author: impl Into<String>,
        published: impl Into<String>,
    ) -> Self {
        let schema = serde_json::json!({
            "@context": "https://schema.org",
            "@type": "Article",
            "headline": self.title,
            "description": self.description,
            "author": { "@type": "Person", "name": author.into() },
            "datePublished": published.into(),
            "image": self.og_image
        });
        self.schema_json = Some(schema.to_string());
        self
    }

    /// Genera marcado schema.org para una obra musical.
    pub fn with_music_schema(mut self, artist: impl Into<String>) -> Self {
        let schema = serde_json::json!({
            "@context": "https://schema.org",
            "@type": "MusicRecording",
            "name": self.title,
            "description": self.description,
            "byArtist": { "@type": "MusicGroup", "name": artist.into() },
            "image": self.og_image
        });
        self.schema_json = Some(schema.to_string());
        self
    }

    /// Genera marcado schema.org para una oferta de trabajo.
    pub fn with_job_schema(
        mut self,
        company: impl Into<String>,
        location: impl Into<String>,
    ) -> Self {
        let schema = serde_json::json!({
            "@context": "https://schema.org",
            "@type": "JobPosting",
            "title": self.title,
            "description": self.description,
            "hiringOrganization": { "@type": "Organization", "name": company.into() },
            "jobLocation": { "@type": "Place", "name": location.into() }
        });
        self.schema_json = Some(schema.to_string());
        self
    }

    /// Convierte a un mapa de variables para inyectar en Tera.
    pub fn to_tera_context(&self) -> tera::Context {
        let mut ctx = tera::Context::new();
        ctx.insert("seo_title", &self.title);
        ctx.insert("seo_description", &self.description);
        ctx.insert("seo_keywords", &self.keywords);
        ctx.insert("seo_canonical", &self.canonical);
        ctx.insert("seo_og_title", &self.og_title);
        ctx.insert("seo_og_description", &self.og_description);
        ctx.insert("seo_og_image", &self.og_image);
        ctx.insert("seo_og_type", &self.og_type);
        ctx.insert("seo_twitter_card", &self.twitter_card);
        ctx.insert("seo_twitter_title", &self.twitter_title);
        ctx.insert("seo_twitter_description", &self.twitter_description);
        ctx.insert("seo_twitter_image", &self.twitter_image);
        ctx.insert("seo_schema_json", &self.schema_json);
        ctx
    }
}
