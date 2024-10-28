use reqwest;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // 定义API URL
    let api_url = "http://127.0.0.1:21011/routePlan";

    // 创建HTTP客户端
    let client = reqwest::Client::new();

    // 发送GET请求
    let response = client.get(api_url).send().await?;

    // 解析响应体为字符串
    let response_text = response.text().await?;

    // 打印响应
    println!("Response: {}", response_text);

    Ok(())
}
