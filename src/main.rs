
const URL: &str ="https://newsapi.org/v2/everything?q=Apple&from=2022-07-05&sortBy=popularity&apiKey=";

fn main() {
    dotenv::from_filename(".env").unwrap();
    let api_key = dotenv::var("API_KEY").unwrap();
    let news_api_url = format!("{}{}", URL, api_key);

    let articles = news_api::get_articles(&news_api_url).unwrap();
    news_api::render_articles(&articles);
}
