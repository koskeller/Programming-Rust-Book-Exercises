#![allow(unused_variables, unused_imports, dead_code)]
use actix_web::{web, App, HttpResponse, HttpServer};
use serde::Deserialize;

#[derive(Deserialize)]
struct GcdParameters {
    n: u64,
    m: u64,
}

fn main() {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(get_index))
            .route("/gcd", web::post().to(post_gcd))
    })
    .bind("127.0.0.1:3000")
    .expect("unable to bind to port 3000")
    .run()
    .expect("unable to run server");
}

fn get_index() -> HttpResponse {
    HttpResponse::Ok().content_type("text/html").body(
        r#"
        <title>Actix Web</title>
        <form action="/gcd" method="post">
        <input type="text" name="n" />
        <input type="text" name="m" />
        <button type="submit">Compute GCD</button>
        </form>
        "#,
    )
}

fn post_gcd(form: web::Form<GcdParameters>) -> HttpResponse {
    if form.n == 0 || form.m == 0 {
        return HttpResponse::BadRequest().content_type("text/html").body(
            r#"
            <title>Actix Web</title>
            <p>Computing the GCD with zero is boring.</p>
            <p><a href="/">Try again.</a></p>
            "#,
        );
    }

    let response = format!(
        r#"
        <title>Actix Web</title>
        <p>The greatest common divisor of the numbers {} and {} is <b>{}</b>.</p>
        <p><a href="/">Compute another GCD</a></p>
        "#,
        form.n,
        form.m,
        gcd(form.n, form.m)
    );

    HttpResponse::Ok().content_type("text/html").body(response)
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}
