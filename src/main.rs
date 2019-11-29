use std::env;
use reqwest;
use reqwest::header as hea;
use serde::{Serialize, Deserialize};
use serde_json::Result;
use serde_json::value;

mod conf;

pub const MLOGIN: &str =  "user.login";
pub const EVGET: &str = "event.get";

enum ZBXM {
    MLOGIN,

}
#[derive(Deserialize, Serialize,Debug)]
struct Auth {
    user: String,
    password: String,
}

struct Event {

}

#[derive(Deserialize,Serialize, Debug)]
struct TokenSucc {
    jsonrpc: String,
    result: String,
    id: u32
}
#[derive(Deserialize, Serialize, Debug)]
struct ZTokenFail {
    jsonrpc: String,
    error: ZError,
    id: u32
}

#[derive(Deserialize, Serialize, Debug)]
struct ZError {
    code: String,
    message: String,
    data: String,
}

#[derive(Deserialize, Serialize,Debug)]
struct Login<T> {
    jsonrpc: String,
    method: String,
    id: u32,
    auth: Option<String>,
    params: T,

}
fn main() {
    let js = Login {
        jsonrpc: String::from("2.0"),
        method: String::from("user.login"),
        id: 1 as u32,
        auth: None,
        params: Auth {
            user: String::from(env::var("ZUSER").unwrap()),
            password: String::from(env::var("ZPASS").unwrap()),
        }
    };
    println!("Json: {:?}", js);
    let apiUrl: String = format!("https://{}/api_jsonrpc.php", env::var("ZURL").unwrap()).to_string();
    let cl = reqwest::Client::new();
    let resp = cl.post(apiUrl.as_str())
                 .header("Content-Type", "application/json-rpc")
                 .json(&js)
                 .send().unwrap().text().unwrap();
    let autstr: TokenSucc = serde_json::from_str(resp.as_str()).unwrap();
    println!("{}", autstr.result);


}
