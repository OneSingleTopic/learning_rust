pub trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) {
        println!("(Read more from {} ...)", self.summarize_author());
    }
}
pub struct BlogArticle {
    pub headline: String,
    pub author: String,
    pub content: String,
}
impl Summary for BlogArticle {
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub author: String,
    pub location: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }

    fn summarize(&self) {
        println!("{} by {} ({})", self.headline, self.author, self.content);
    }
}

pub struct Tweet {
    pub author: String,
    pub content: String,
    pub retweet: bool,
    pub reply: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }

    fn summarize(&self) {
        println!("{} : {}", self.author, self.content);
    }
}

pub fn notify<T>(media_content: &T)
where
    T: Summary,
{
    media_content.summarize();
}

fn main() {
    let article = NewsArticle {
        headline: "COUCOU".to_string(),
        author: "me".to_string(),
        location: "here".to_string(),
        content: "Nothing".to_string(),
    };

    let tweet = Tweet {
        author: "you".to_string(),
        content: "This".to_string(),
        retweet: false,
        reply: true,
    };

    let blog = BlogArticle {
        headline: "TITI".to_string(),
        author: "him".to_string(),
        content: "Titi is real".to_string(),
    };

    let media_content: Vec<T: Summary> = vec![article, tweet, blog];

    for content in media_content {
        content.summarize();
    }
}
