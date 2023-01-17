#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;
extern crate web_view;
use web_view::*;

#[napi(object)]
#[derive(Clone)]
pub struct Config {
  pub width: i32,
  pub height: i32,
  pub path: String,
  pub title: String,
}

#[napi]
pub struct Application {
  pub config: Config,
}

#[napi]
impl Application {
  #[napi(constructor)]
  pub fn new(config: Config) -> Self {
    Self { config }
  }

  #[napi]
  pub fn run(&self) {
    let html_content = std::fs::read_to_string(self.config.path.clone()).expect("WISHIN: OPS!");

    web_view::builder()
      .title(&self.config.title)
      .content(Content::Html(html_content))
      .size(self.config.width, self.config.height)
      .resizable(true)
      .debug(true)
      .user_data(())
      .invoke_handler(|_webview, _arg| Ok(()))
      .run()
      .unwrap();
  }
}
