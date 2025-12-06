use actix_web::{get, web::ServiceConfig};
use shuttle_actix_web::ShuttleActixWeb;

use actix_web::HttpResponse;
use chrono::Local;

#[get("/")]
async fn hello_world() -> HttpResponse {
    let now = Local::now();
    let html = format!(
        r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <title>Current Time</title>
            <style>
                body {{
                    font-family: sans-serif;
                    display: flex;
                    justify_content: center;
                    align_items: center;
                    height: 100vh;
                    background-color: #f0f0f0;
                    margin: 0;
                }}
                .container {{
                    text-align: center;
                    background: white;
                    padding: 2rem;
                    border-radius: 10px;
                    box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
                }}
                h1 {{
                    color: #333;
                }}
                p {{
                    font-size: 1.5rem;
                    color: #666;
                }}
            </style>
        </head>
        <body>
            <div class="container">
                <h1>Current Local Time</h1>
                <p>{}</p>
                <p><a href="https://github.com/narumincho/shuttle">GitHub Repository</a></p>
            </div>
        </body>
        </html>
        "#,
        now.format("%Y-%m-%d %H:%M:%S")
    );

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(hello_world);
    };

    Ok(config.into())
}
