use askama::Template;
use warp::Filter;

#[derive(Template)]
#[template(path = "hello.html")]
struct Hello {
    name: String,
    current: String,
}

#[derive(Template)]
#[template(path = "home.html")]
struct Home {
    current: String,
}

#[tokio::main]
async fn main() {
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = warp::path!("hello" / String).map(|name: String| {
        let t = Hello {
            name: name.to_string(),
            current: "Home".to_string(),
        };
        warp::reply::html(t.render().unwrap())
    });
    let home = warp::path!("home").map(|| {
        let t = Home {
            current: "Home".to_string(),
        };
        warp::reply::html(t.render().unwrap())
    });
    let static_dir = warp::path("static").and(warp::fs::dir("src/static"));

    warp::serve(hello.or(static_dir).or(home))
        .run(([0, 0, 0, 0], 3030))
        .await;
}
