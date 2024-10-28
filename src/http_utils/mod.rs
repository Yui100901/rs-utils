use reqwest::{Client, RequestBuilder};
use serde::Serialize;
use std::collections::HashMap;
use std::error::Error;
use std::time::Duration;

/// HTTP 工具类，用于发送 HTTP 请求。
pub struct HttpUtils {
    client: Client,
}

impl HttpUtils {
    /// 创建一个新的 HttpUtils 实例。
    pub fn new() -> Self {
        HttpUtils {
            client: Client::builder()
                .timeout(Duration::from_secs(10)) // 设置超时时间
                .build()
                .expect("Failed to build client"),
        }
    }

    /// 发送 HTTP 请求并返回响应文本。
    ///
    /// # 参数
    ///
    /// - `request`: 要发送的 HTTP 请求构建器。
    /// - `headers`: 请求头的键值对集合。
    ///
    /// # 返回
    ///
    /// 返回包含响应文本的 `Result`。
    async fn send_request(
        &self,
        request: RequestBuilder,
        headers: Option<HashMap<String, String>>,
    ) -> Result<String, Box<dyn Error>> {
        let mut request = request;
        if let Some(headers) = headers {
            for (key, value) in headers {
                request = request.header(&key, &value);
            }
        }

        let response = request.send().await?;
        if response.status().is_success() {
            Ok(response.text().await?)
        } else {
            Err(format!("Request failed with status: {:?}", response).into())
        }
    }

    /// 发送一个 HTTP GET 请求到指定的 URL，并附带查询参数和请求头。
    ///
    /// # 参数
    ///
    /// - `api_url`: 请求的目标 URL。
    /// - `headers`: 请求头的键值对集合。
    /// - `req_data`: 查询参数的键值对集合。
    ///
    /// # 返回
    ///
    /// 返回包含响应文本的 `Result`。
    pub async fn get_by_query(
        &self,
        api_url: &str,
        headers: Option<HashMap<String, String>>,
        req_data: Option<HashMap<String, String>>,
    ) -> Result<String, Box<dyn Error>> {
        let mut url = reqwest::Url::parse(api_url)?;
        if let Some(data) = req_data {
            let mut query_pairs = url.query_pairs_mut();
            for (key, value) in data {
                query_pairs.append_pair(&key, &value);
            }
        }

        let request = self.client.get(url);
        self.send_request(request, headers).await
    }

    /// 发送一个带有 JSON 数据的 HTTP POST 请求到指定的 URL，并附带请求头。
    ///
    /// # 参数
    ///
    /// - `api_url`: 请求的目标 URL。
    /// - `headers`: 请求头的键值对集合。
    /// - `req_data`: 请求体数据。
    ///
    /// # 返回
    ///
    /// 返回包含响应文本的 `Result`。
    pub async fn post_by_json<T: Serialize>(
        &self,
        api_url: &str,
        headers: Option<HashMap<String, String>>,
        req_data: Option<T>,
    ) -> Result<String, Box<dyn Error>> {
        let mut request = self.client.post(api_url);

        if let Some(data) = req_data {
            request = request.json(&data);
        }

        self.send_request(request, headers).await
    }

    /// 发送一个带有表单数据的 HTTP POST 请求到指定的 URL，并附带请求头。
    ///
    /// # 参数
    ///
    /// - `api_url`: 请求的目标 URL。
    /// - `headers`: 请求头的键值对集合。
    /// - `req_data`: 表单数据的键值对集合。
    ///
    /// # 返回
    ///
    /// 返回包含响应文本的 `Result`。
    pub async fn post_by_form(
        &self,
        api_url: &str,
        headers: Option<HashMap<String, String>>,
        req_data: Option<HashMap<String, String>>,
    ) -> Result<String, Box<dyn Error>> {
        let mut request = self.client.post(api_url);

        if let Some(data) = req_data {
            request = request.form(&data);
        }

        self.send_request(request, headers).await
    }
}
