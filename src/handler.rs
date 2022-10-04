use warp::{reject, reply::html, Reply};
use askama::Template;
use serde::{Serialize, Deserialize};
use crate::EthSpider;
use crate::Response;
use crate::{error::Error::*, WebResult};

pub static KEY: &str = "NH5MPZTWMKP3MQZ91KDV862TVYQVNZX52Y";

pub async fn welcome_handler() -> WebResult<impl Reply> {
    let template = WelcomeTemplate {
        title: "Welcome",
        body: "EthCrawler!",
    };
    let res = template
        .render()
        .map_err(|e| reject::custom(TemplateError(e)))?;
    Ok(html(res))
}

pub async fn create_book_handler(body: EthRequest) -> WebResult<impl Reply> {
    
    let url = format!("https://api.etherscan.io/api?module=account&action=txlist&address={}&startblock={}&endblock=last&page=1&offset=1000&sort=asc&apikey={}", body.address, body.block, KEY);

    let x = EthSpider::new();
    let resp = x.http_client.get(url).send().await;
    let data = resp.unwrap();
    let data = data.json::<Response>().await;
    let data = data.unwrap();

    let x = data.result.iter().fold(0, |sum, block| sum + block.value.parse::<u32>().unwrap());

    let template = BooklistTemplate {
        books: &data,
        total: &x,
    };
    let res = template.render().map_err(|e| reject::custom(TemplateError(e)))?;
    Ok(html(res))
}

pub async fn new_book_handler() -> WebResult<impl Reply> {
    let template = NewEthTemplate {};
    let res = template
        .render()
        .map_err(|e| reject::custom(TemplateError(e)))?;
    Ok(html(res))
}

#[derive(Template)] // this will generate the code...
#[template(path = "hello.html")] //crate root
struct WelcomeTemplate<'a> { // the name of the struct can be anything
    title: &'a str, 
    body: &'a str,
}

#[derive(Template)]
#[template(path = "book/new.html")]
struct NewEthTemplate {}

#[derive(Serialize, Deserialize, Debug)]
pub struct EthRequest {
    pub address: String,
    pub block: String,
}

#[derive(Template)]
#[template(path = "book/list.html")]
struct BooklistTemplate<'a> {
    books: &'a Response,
    total: &'a u32,
}