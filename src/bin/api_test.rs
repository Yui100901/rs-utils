use reqwest;
use rs_utils::http_utils;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // 定义API URL
    let api_url = "https://www.baidu.com";

    // 创建HTTP客户端
    let client = reqwest::Client::new();

    // 发送GET请求
    let response = client.get(api_url).send().await?;

    // 解析响应体为字符串
    let response_text = response.text().await?;

    let http = http_utils::HttpUtils::new();

    let s = http.get_by_query(api_url, None, None).await?;
    println!("返回响应：{}", s);

    // 打印响应
    println!("Response: {}", response_text);

    Ok(())
}
