use reqwest;
use tokio::sync::Mutex as AsyncMutex;
use tokio::sync::MutexGuard;
use tokio::time;

async fn expensive_parse(guard: &MutexGuard<'_, Vec<String>>) {}

async fn process_requests(urls: Vec<String>) -> Vec<String> {
    let results = AsyncMutex::new(Vec::new());

    for url in &urls {
        let response = reqwest::get(url).await.unwrap().text().await.unwrap();
        time::sleep(std::time::Duration::from_millis(100)); // Rate limit
        let mut guard = results.lock().await;
        guard.push(response);
        expensive_parse(&guard).await; // Parse all results so far
    }

    results.into_inner()
}

fn main() {}
