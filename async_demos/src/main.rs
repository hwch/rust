use trpl::Either;
use trpl::Html;

async fn page_title(url: &str) -> (&str, Option<String>) {
    let response_text = trpl::get(url).await.text().await;
    (
        url,
        Html::parse(&response_text)
            .select_first("title")
            .map(|title| title.inner_html()),
    )
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    trpl::block_on(async {
        let f1 = page_title(&args[1]);
        let f2 = page_title(&args[2]);
        let (url, maybe_title) = match trpl::select(f1, f2).await {
            Either::Left(left) => left,
            Either::Right(right) => right,
        };
        println!("{url} returned first");
        match maybe_title {
            Some(title) => println!("Its page title was: '{title}'"),
            None => println!("It had no title."),
        }
    });
}
