use reqwest::Client;
use serde::Serialize;
use std::collections::HashMap;
use std::error::Error;

pub struct HttpUtils {
    client: Client,
}

impl HttpUtils {
    pub fn new() -> Self {
        HttpUtils {
            client: Client::new(),
        }
    }

    // 发送一个HTTP GET请求到指定的URL，并附带查询参数
    pub async fn get_by_query(&self, api_url: &str, req_data: Option<HashMap<String, String>>) -> Result<String, Box<dyn Error>> {
        let mut url = reqwest::Url::parse(api_url)?;

        if let Some(data) = req_data {
            let mut query_pairs = url.query_pairs_mut();
            for (key, value) in data {
                query_pairs.append_pair(&key, &value);
            }
        }

        let response = self.client.get(url).send().await?;
        Ok(response.text().await?)
    }

    // 发送一个带有JSON数据的HTTP POST请求到指定的URL
    pub async fn post_by_json<T: Serialize>(&self, api_url: &str, req_data: Option<T>) -> Result<String, Box<dyn Error>> {
        let response = match req_data {
            Some(data) => self.client.post(api_url).json(&data).send().await?,
            None => self.client.post(api_url).send().await?,
        };

        Ok(response.text().await?)
    }

    // 发送一个带有表单数据的HTTP POST请求到指定的URL
    pub async fn post_by_form(&self, api_url: &str, req_data: Option<HashMap<String, String>>) -> Result<String, Box<dyn Error>> {
        let response = match req_data {
            Some(data) => self.client.post(api_url).form(&data).send().await?,
            None => self.client.post(api_url).send().await?,
        };

        Ok(response.text().await?)
    }
}