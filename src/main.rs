use actix_web::{web, App, HttpResponse, HttpServer};
use serde::Deserialize;
use std::time;
use tokio;

#[derive(Deserialize)]
struct GCDParameters {
    n: u64,
    m: u64,
}

async fn print_odd() {
    let total = 10;
    let mut count = 0;
    let mut odd = 1;
    let three_seconds = time::Duration::from_millis(3000);
    while count < total {
        println!("Odd value: {odd}");
        odd += 2;
        count += 1;
        tokio::time::sleep(three_seconds).await;
    }
}

async fn print_even() {
    let total = 10;
    let mut count = 0;
    let mut even = 0;
    let three_seconds = time::Duration::from_millis(3000);
    while count < total {
        println!("Even value: {even}");
        even += 2;
        count += 1;
        tokio::time::sleep(three_seconds).await;
    }
}

fn func_with_closure<G>(f: G)
where
    G: FnOnce(&str),
{
    f("hello world");
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        // Exchange the values of m, n if m<n
        if m < n {
            let temp = m;
            m = n;
            n = temp;
        }
        m %= n
    }
    n
}

// #[tokio::main]
#[actix_web::main]
async fn main() {
    let odd = print_odd();
    let even = print_even();
    println!("Begin counting...");
    // This waits until both functions are executed
    tokio::join!(even, odd);
    println!("This is the end");
    let s = "The content of x is: ";
    let print_x_closure = |x: &str| {
        println!("{s} {x}");
    };
    func_with_closure(print_x_closure);

    // RUST HTTP SERVER
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(get_index))
            .route("/gcd", web::post().to(post_gcd))
    });
    println!("Server is running port 3000");
    server
        .bind("127.0.0.1:3000")
        .expect("Error binding to the port")
        .run()
        .await
        .expect("Error running the server");
}

async fn get_index() -> HttpResponse {
    HttpResponse::Ok().content_type("text/html").body(
        r#"
        <title> GCD Calculator</title>
        <form action="/gcd" method="post">
        <input type="text" name="n"/>
        <input type="text" name="m"/>
        <button type="submit">Compute GCD</button>
        </form>
        "#,
    )
}

async fn post_gcd(form: web::Form<GCDParameters>) -> HttpResponse {
    if form.m == 0 || form.n == 0 {
        return HttpResponse::BadRequest()
            .content_type("text/html")
            .body("Computing the GCD with ZERO is boring");
    }

    let response = format!(
        "The greatest common divisor of {} and {} is <b>{}</b>",
        form.m,
        form.n,
        gcd(form.m, form.n)
    );
    HttpResponse::Ok().content_type("text/html").body(response)
}
