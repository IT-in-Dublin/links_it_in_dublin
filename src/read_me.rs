use pulldown_cmark::{html, Options, Parser};
use actix_web::{get, HttpResponse, Responder};
use std::path::Path;
use std::fs;


#[get("/")]
pub async fn handle_content() -> impl Responder {
    let path = Path::new("./README.md");
    let filename = path.file_name().unwrap();
    // let ext = path.extension().unwrap();

    let content = load_file(filename.to_str().unwrap());
    let title = filename.to_str().unwrap();

    render(title, &content)
}

fn load_file(filename: &str) -> String {
    match fs::read_to_string(filename) {
        Ok(x) => x,
        Err(_) => String::from("unable to load file")
    }
}

fn render( title: &str, content: &str) -> impl Responder {
    let content = to_html(title, content);
    HttpResponse::Ok().body(content)
}


fn to_html(title: &str, text: &str) -> String {

    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);

    let parser = Parser::new_ext(text, options);

    let mut html_output = String::new();
    println!("{}", html_output);
    html::push_html(&mut html_output, parser);

    format!("<html>\n\
        <head>\n\
        <meta charset=\"utf-8\">\n\
        <title>{}</title>\n\
        <link rel=\"stylesheet\" type=\"text/css\" href=\"/gt.css\">\n\
        </head>\n\
        <body>\n\
        <div class=\"markdown-body\">{}</div>\n\
        </body>\n\
        </html>", title, html_output)
}