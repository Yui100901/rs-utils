mod network;

use std::collections::HashMap;
use std::error::Error;
use network::http_utils::http_utils::HttpUtils;

// 示例主函数调用
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let http_utils = HttpUtils::new();

    // 示例GET请求
    match http_utils.get_by_query("https://www.baidu.com", None).await {
        Ok(response) => println!("GET响应: {}", response),
        Err(e) => eprintln!("GET请求失败: {}", e),
    }

    // 示例POST请求 (JSON)
    match http_utils.post_by_json("https://api.example.com/data", Some(serde_json::json!({"key": "value"}))).await {
        Ok(response) => println!("POST JSON响应: {}", response),
        Err(e) => eprintln!("POST JSON请求失败: {}", e),
    }

    // 示例POST请求 (表单)
    let mut form_data = HashMap::new();
    form_data.insert("key".to_string(), "value".to_string());
    match http_utils.post_by_form("https://api.example.com/data", Some(form_data)).await {
        Ok(response) => println!("POST表单响应: {}", response),
        Err(e) => eprintln!("POST表单请求失败: {}", e),
    }

    Ok(())
}
