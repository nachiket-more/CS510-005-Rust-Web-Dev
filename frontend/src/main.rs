use warp::{http::Response, reject::custom, Filter, Rejection};

#[tokio::main]
async fn main() {
    // Define a route to serve index.html at the root endpoint
    let index_route = warp::path::end().and(warp::fs::file("index.html"));

    let css_route = warp::path("index.css").and(warp::fs::file("index.css"));

    // Define a route to fetch data from the backend API
    let questions_route = warp::path("questions")
        .and_then(fetch_questions)
        .map(|response| {
            Response::builder()
                .header("content-type", "application/json")
                .body(response)
                .unwrap()
        });

    // Combine the routes and run the server
    let routes = index_route.or(questions_route).or(css_route);
    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}

#[derive(Debug)]
struct FetchError;
impl warp::reject::Reject for FetchError {}

// Function to fetch all questions from the backend API
async fn fetch_questions() -> Result<String, Rejection> {
    let backend_url = "http://localhost:5000/questions";
    match reqwest::get(backend_url).await {
        Ok(response) => match response.text().await {
            Ok(data) => Ok(data),
            Err(_) => Err(custom(FetchError)),
        },
        Err(_) => Err(custom(FetchError)),
    }
}
