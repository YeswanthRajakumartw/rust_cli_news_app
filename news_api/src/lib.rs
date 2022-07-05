use colour::cyan;
use serde::Deserialize;
use termimad::print_text;

#[derive(Deserialize, Debug)]
pub struct Article {
    pub title: String,
    pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct Articles {
    pub articles: Vec<Article>,
}

#[derive(Debug, thiserror::Error)]
pub enum NewsApiError {
    #[error("Failed to fetch articles")]
    RequestFailed(Box<ureq::Error>),
    #[error("Failed to parse the articles")]
    ArticleParsingFailed(serde_json::Error),

}

pub fn get_articles(url: &str) -> Result<Articles, NewsApiError> {
    let response = ureq::get(url)
        .call()
        .map_err(|e| NewsApiError::RequestFailed(Box::new(e)))?
        .into_string().unwrap();
    let articles: Articles = serde_json::from_str(&response)
        .map_err(NewsApiError::ArticleParsingFailed)
        .unwrap();

    Ok(articles)
}

pub fn render_articles(articles: &Articles) {
    for article in &articles.articles {
        print_text(&format!("> {}\n", &article.title));
        cyan!(" {}\n\n", &article.url);
        print_text("---");
    }
}
