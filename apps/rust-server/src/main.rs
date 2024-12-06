use warp::Filter;

#[tokio::main]
async fn main() {
    // Define the `/sayhi` route
    let say_hi = warp::path("sayhi").map(|| warp::reply::json(&"Hello from Rust HTTP server!"));

    // Start the server on port 3030
    warp::serve(say_hi).run(([127, 0, 0, 1], 3030)).await;
}
