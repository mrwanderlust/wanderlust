use askama::Template;
use dotenv::dotenv;
use sqlx::postgres::PgQueryAs;
use warp::Filter;

use wanderlust::auth::create_google_client;
use wanderlust::data::connect;

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

#[derive(Template)]
#[template(path = "get-started.html")]
struct GetStarted {
    current: String,
}

#[derive(sqlx::FromRow, Clone)]
struct Test {
    name: String,
}

#[tokio::main]
async fn main() {
    dotenv().expect("Failed to load .env file");
    let pool = connect()
        .await
        .expect("Failed to establish database connection");
    let mut conn = pool
        .acquire()
        .await
        .expect("Failed to get connection from pool");
    let test = sqlx::query_as::<_, Test>("SELECT name FROM test")
        .fetch_one(&mut conn)
        .await
        .expect("Query failed");
    println!("{}", test.name);

    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = warp::path!("hello" / String).map(|name: String| {
        let t = Hello {
            name: name,
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

    let get_started = warp::path!("get-started").map(|| {
        let t = GetStarted {
            current: "Home".to_string(),
        };
        warp::reply::html(t.render().unwrap())
    });
    let static_dir = warp::path("static").and(warp::fs::dir("src/static"));

    warp::serve(hello.or(static_dir).or(home).or(get_started))
        .run(([0, 0, 0, 0], 3030))
        .await;
}
