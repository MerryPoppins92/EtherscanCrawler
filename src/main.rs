use reqwest::Client;
use error::Error;
use std::time::Duration;
use serde::{Serialize, Deserialize};
use warp::{Filter, Rejection};

type WebResult<T> = std::result::Result<T, Rejection>;
type DB = Vec<Block>;
mod error;
mod handler;

#[tokio::main]
async fn main()-> Result<(), Error> {
    let books = warp::path("query");
    let new = warp::path("new");

    let books_routes = books
    .and(new)
    .and(warp::get())
    .and_then(handler::new_book_handler)
    .or(books
        .and(new)
        .and(warp::post())
        .and(warp::body::form())
        .and_then(handler::create_book_handler));

    let welcome_route = warp::path::end().and_then(handler::welcome_handler);
    let routes = welcome_route
    .or(books_routes)
    .recover(error::handle_rejection);
    println!("Open your browser and go to http://localhost:8080/ to start");

    warp::serve(routes).run(([0, 0, 0, 0], 8080)).await;
   

    Ok(())

}

#[derive(Debug, Clone)]
pub struct EthSpider {
    http_client: Client,
}

impl EthSpider {
    pub fn new() -> Self {
        let http_timeout = Duration::from_secs(6);
        let http_client = Client::builder()
            .timeout(http_timeout)
            .build()
            .expect("spiders/cvedetails: Building HTTP client");

            EthSpider { http_client }
    }

    pub async fn scrape(&self, url: String) -> Result<(), Error>{
        let http_res = self.http_client.get(url).send().await;
        println!("{:?}", http_res);
        Ok(())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub status: String,
    pub message: String,
    pub result: DB,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Block {
    #[serde(rename = "blockNumber")]
    block_number: String,
    #[serde(rename = "timeStamp")]
    time_stamp: String,
    hash: String,
    nonce: String,
    #[serde(rename = "blockHash")]
    block_hash: String,
    #[serde(rename = "transactionIndex")]
    transaction_index: String,
    #[serde(rename = "from")]
    fromi: String,
    #[serde(rename = "to")]
    tou: String,
    value: String,
    gas: String,
    #[serde(rename = "gasPrice")]
    gas_price: String,
    #[serde(rename = "isError")]
    is_error: String,
    txreceipt_status : String,
    input: String,
    #[serde(rename = "contractAddress")]
    contractaddress: String,
    #[serde(rename = "cumulativeGasUsed")]
    cumulative_gas_used: String,
    #[serde(rename = "gasUsed")]
    gasused: String,
    confirmations: String,
    #[serde(rename = "methodId")]
    methodid: String,
    #[serde(rename = "functionName")]
    function_name: String,
}