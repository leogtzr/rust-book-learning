use traits1::{NewsArticle, Summary, Tweet};

fn main() {
    let news_article = NewsArticle {
        headline: String::from("OK"),
        location: String::from("OK"),
        author: String::from("OK"),
        content: String::from("OK"),
    };

    println!("Value = {}", news_article.summarize());

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
}
