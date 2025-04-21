use std::str::FromStr;

use reqwest::Url;
use serde::Deserialize;
use wasm_bindgen::{JsCast, JsError, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{console, Request, RequestInit, RequestMode, Response, Window};

use super::powerlifter::Powerlifter;

#[derive(Debug, Deserialize)]
struct NextIndexApiResponse {
    pub(self) next_index: usize,
}

#[derive(Debug, Deserialize)]
struct PowerlifterApiResponse {
    pub(self) total_length: usize,
    pub(self) rows: Vec<Powerlifter>,
}

const PROXY: &str = "https://thingproxy.freeboard.io/fetch/";
const INDEX_URL: &str = "https://www.openpowerlifting.org/api/search/rankings";
const POWERLIFTER_URL: &str = "https://www.openpowerlifting.org/api/rankings";

pub async fn get_powerlifters(names: Vec<String>) -> Vec<(String, Option<Powerlifter>)> {
    let mut powerlifter: Option<Powerlifter>;
    let mut powerlifters: Vec<(String, Option<Powerlifter>)> = Vec::new();

    for name in names {
        powerlifter = get_powerlifter(&name).await;
        powerlifters.push((name, powerlifter));
    }

    powerlifters
}

async fn get_powerlifter(name: &String) -> Option<Powerlifter> {
    let Some(index) = get_powerlifter_index(name).await else {
        console::log_1(&JsValue::from_str("No next index"));
        return None;
    };

    console::log_1(&JsValue::from(index));

    let Some(powerlifter) = get_powerlifter_from_index(index).await else {
        return None;
    };

    Some(powerlifter)
}

async fn get_powerlifter_index(name: &String) -> Option<usize> {
    let params: [(&str, &str); 2] = [
        ("q", &name),
        ("start", "0"),
    ];

    let url: String = build_url_with_params(INDEX_URL, &params).ok()?;
    let response: JsValue = fetch_data(url).await.ok()?;

    console::log_1(&JsValue::from_str("response:"));
    console::log_1(&JsValue::from(&response));

    let json: NextIndexApiResponse = serde_wasm_bindgen::from_value(response).ok()?;

    console::log_1(&JsValue::from_str("response parsed"));

    Some(json.next_index)
}

async fn get_powerlifter_from_index(index: usize) -> Option<Powerlifter> {
    let params: [(&str, &str); 4] = [
        ("start", &index.to_string()),
        ("end", &index.to_string()),
        ("lang", "fr"),
        ("units", "kg"),
    ];

    let url: String = build_url_with_params(POWERLIFTER_URL, &params).ok()?;
    let response: JsValue = fetch_data(url).await.ok()?;

    console::log_1(&JsValue::from_str("response:"));
    console::log_1(&JsValue::from(&response));

    let json: PowerlifterApiResponse = match serde_wasm_bindgen::from_value(response){
        Ok(json) => json,
        Err(error) => {
            console::log_1(&JsValue::from_str(&error.to_string()));
            return None;
        },
    };

    console::log_1(&JsValue::from_str("response parsed"));

    json.rows.first().cloned()
}

fn build_url_with_params(url: &str, parameters: &[(&str, &str)]) -> Result<String, JsValue> {
    let query: String = parameters
        .iter()
        .map(|(key, value)| format!("{}={}", key, value))
        .collect::<Vec<String>>()
        .join("&");
    let url: String = format!("{}{}", PROXY, url);

    let mut url = match Url::from_str(&url) {
        Ok(url) => url,
        Err(error) => return Err(JsError::from(error).into()),
    };

    url.set_query(Some(&query));

    Ok(url.as_str().to_string())
}

pub async fn fetch_data(url: String) -> Result<JsValue, JsValue> {
    let opts: RequestInit = RequestInit::new();
    opts.set_method("GET");
    opts.set_mode(RequestMode::Cors);

    let request: Request = Request::new_with_str_and_init(&url, &opts)?;
    let window: Window = web_sys::window().expect("No global \"window\" exists");
    let response_value: JsValue = JsFuture::from(window.fetch_with_request(&request)).await?;
    let response: Response = response_value.dyn_into().unwrap();
    let json = JsFuture::from(response.json()?).await?;

    Ok(json)
}
