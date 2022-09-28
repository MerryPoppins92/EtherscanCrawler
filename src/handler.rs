use warp::{reject, reply::html, Reply};
use askama::Template;
use serde::{Serialize, Deserialize};
// use crate::Block;
use crate::EthSpider;
use crate::Response;
// use crate::WelcomeTemplate;
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

// pub static URL2: &str = "https://api.etherscan.io/api?module=account&action=txlist&address=0xc5102fE9359FD9a28f877a67E36B0F050d81a3CC&startblock=9000000&endblock=last&page=1&offset=10&sort=asc&apikey=NH5MPZTWMKP3MQZ91KDV862TVYQVNZX52Y";
pub async fn create_book_handler(body: EthRequest) -> WebResult<impl Reply> {
    
    let url = format!("https://api.etherscan.io/api?module=account&action=txlist&address={}&startblock={}&endblock=last&page=1&offset=10&sort=asc&apikey={}", body.address, body.block, KEY);
    let url2 = format!("https://api.etherscan.io/api?module=account&action=txlist&address={}&startblock={}&endblock=last&page=1&offset=10&sort=asc&apikey={}", body.address, body.block, KEY);

    let x = EthSpider::new();
    let resp = x.http_client.get(url).send().await;
    // println!("{:?}", resp);
    let data = resp.unwrap();
    // println!("\n{:#?}", data);
    let data = data.json::<Response>().await;
    let data = data.unwrap();
    println!("data {:#?}", data);
    // let data = format!("{:#?}", data);

    let template = BooklistTemplate {
        books: &data
    };
    let res = template.render().map_err(|e| reject::custom(TemplateError(e)))?;
    Ok(html(res))
}


// pub async fn books_list_handler(db: String) -> WebResult<impl Reply> {
//     let x = EthSpider::new();
//     let resp = x.http_client.get(db).send().await;
//     // println!("{:?}", resp);
//     let data = resp.unwrap();
//     // println!("\n{:#?}", data);
//     let data = data.json::<Response>().await;
//     let data = data.unwrap(); 
//     let template = BooklistTemplate {
//         books: &data.result
//     };
//     let res = template.render().map_err(|e| reject::custom(TemplateError(e)))?; 
//     Ok(html(res)) 
// }

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
}