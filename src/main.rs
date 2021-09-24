use warp::Filter;

#[tokio::main]
async fn main() {
    let routes = warp::get().map(|| "Hello, warp!");

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}