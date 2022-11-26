use rocket::response::content::RawHtml;

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> RawHtml<&'static str> {
    RawHtml(
        r#"
        <html>
            <head>
                <title>Test</title>
            </head>
            <body id='test'>
                <div>Test</div>
            </body>
        </html>
    "#,
    )
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
