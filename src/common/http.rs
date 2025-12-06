//! HTTP 请求封装
//!
//! 封装 GET/POST 请求方法，处理 URL 编码和响应解析

use reqwest::Client;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::collections::HashMap;
use std::time::Duration;

use crate::error::{ZtkError, ZtkResult};

/// 默认请求超时时间 (秒)
const DEFAULT_TIMEOUT_SECS: u64 = 30;

/// 默认 API 基础地址
pub const DEFAULT_BASE_URL: &str = "https://api.zhetaoke.com:10001";

/// 备用 API 基础地址
pub const BACKUP_BASE_URL: &str = "http://api.zhetaoke.cn:10000";

/// HTTP 客户端封装
///
/// 提供统一的 HTTP 请求方法，处理 URL 编码和响应解析
#[derive(Debug, Clone)]
pub struct HttpClient {
    /// reqwest 客户端
    client: Client,
    /// API 基础地址
    base_url: String,
}

impl HttpClient {
    /// 创建新的 HTTP 客户端
    ///
    /// # Arguments
    ///
    /// * `base_url` - API 基础地址
    /// * `timeout` - 请求超时时间 (可选)
    ///
    /// # Returns
    ///
    /// 返回 HttpClient 实例或错误
    pub fn new(base_url: impl Into<String>, timeout: Option<Duration>) -> ZtkResult<Self> {
        let timeout = timeout.unwrap_or(Duration::from_secs(DEFAULT_TIMEOUT_SECS));

        let client = Client::builder()
            .timeout(timeout)
            .build()
            .map_err(ZtkError::Network)?;

        Ok(Self {
            client,
            base_url: base_url.into(),
        })
    }

    /// 使用默认配置创建 HTTP 客户端
    ///
    /// 使用默认的 API 基础地址和超时时间
    pub fn with_defaults() -> ZtkResult<Self> {
        Self::new(DEFAULT_BASE_URL, None)
    }

    /// 获取基础 URL
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    /// 发送 GET 请求
    ///
    /// # Arguments
    ///
    /// * `path` - API 路径
    /// * `params` - 查询参数
    ///
    /// # Returns
    ///
    /// 返回反序列化后的响应数据或错误
    pub async fn get<T, P>(&self, path: &str, params: &P) -> ZtkResult<T>
    where
        T: DeserializeOwned,
        P: Serialize + ?Sized,
    {
        self.get_with_base_url(&self.base_url, path, params).await
    }

    /// 发送 GET 请求 (使用自定义 base_url)
    ///
    /// # Arguments
    ///
    /// * `base_url` - 自定义的 API 基础地址
    /// * `path` - API 路径
    /// * `params` - 查询参数
    ///
    /// # Returns
    ///
    /// 返回反序列化后的响应数据或错误
    pub async fn get_with_base_url<T, P>(
        &self,
        base_url: &str,
        path: &str,
        params: &P,
    ) -> ZtkResult<T>
    where
        T: DeserializeOwned,
        P: Serialize + ?Sized,
    {
        let url = format!("{}{}", base_url, path);

        // 将参数序列化为查询字符串
        let query_string = self.serialize_params(params)?;
        let full_url = if query_string.is_empty() {
            url
        } else {
            format!("{}?{}", url, query_string)
        };

        let response = self
            .client
            .get(&full_url)
            .send()
            .await
            .map_err(ZtkError::Network)?;

        self.handle_response(response).await
    }

    /// 发送 POST 请求 (表单格式)
    ///
    /// # Arguments
    ///
    /// * `path` - API 路径
    /// * `params` - 表单参数
    ///
    /// # Returns
    ///
    /// 返回反序列化后的响应数据或错误
    pub async fn post_form<T, P>(&self, path: &str, params: &P) -> ZtkResult<T>
    where
        T: DeserializeOwned,
        P: Serialize + ?Sized,
    {
        let url = format!("{}{}", self.base_url, path);

        // 将参数序列化为表单数据
        let form_data = self.serialize_params(params)?;

        let response = self
            .client
            .post(&url)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(form_data)
            .send()
            .await
            .map_err(ZtkError::Network)?;

        self.handle_response(response).await
    }

    /// 发送 POST 请求 (JSON 格式)
    ///
    /// # Arguments
    ///
    /// * `path` - API 路径
    /// * `body` - JSON 请求体
    ///
    /// # Returns
    ///
    /// 返回反序列化后的响应数据或错误
    pub async fn post_json<T, B>(&self, path: &str, body: &B) -> ZtkResult<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let url = format!("{}{}", self.base_url, path);

        let response = self
            .client
            .post(&url)
            .json(body)
            .send()
            .await
            .map_err(ZtkError::Network)?;

        self.handle_response(response).await
    }

    /// 序列化参数为 URL 编码的查询字符串
    ///
    /// # Arguments
    ///
    /// * `params` - 要序列化的参数
    ///
    /// # Returns
    ///
    /// 返回 URL 编码的查询字符串或错误
    fn serialize_params<P: Serialize + ?Sized>(&self, params: &P) -> ZtkResult<String> {
        // 先序列化为 JSON，然后转换为 HashMap
        let json_value = serde_json::to_value(params)?;

        let map = match json_value {
            serde_json::Value::Object(map) => map,
            _ => return Ok(String::new()),
        };

        let mut pairs: Vec<String> = Vec::new();
        for (key, value) in map {
            let value_str = match value {
                serde_json::Value::Null => continue, // 跳过 null 值
                serde_json::Value::String(s) => s,
                serde_json::Value::Number(n) => n.to_string(),
                serde_json::Value::Bool(b) => b.to_string(),
                _ => serde_json::to_string(&value)?,
            };

            // URL 编码
            let encoded_value = url_encode(&value_str);
            pairs.push(format!("{}={}", key, encoded_value));
        }

        Ok(pairs.join("&"))
    }

    /// 处理 HTTP 响应
    ///
    /// # Arguments
    ///
    /// * `response` - HTTP 响应
    ///
    /// # Returns
    ///
    /// 返回反序列化后的数据或错误
    async fn handle_response<T: DeserializeOwned>(
        &self,
        response: reqwest::Response,
    ) -> ZtkResult<T> {
        let status = response.status();
        let text = response.text().await.map_err(ZtkError::Network)?;

        // 检查 HTTP 状态码
        if !status.is_success() {
            return Err(ZtkError::api(
                status.as_u16() as i32,
                format!("HTTP 错误: {}", text),
            ));
        }

        // 尝试解析为 API 响应
        // 首先检查是否是 API 错误响应
        if let Ok(api_error) = serde_json::from_str::<ApiErrorResponse>(&text) {
            if api_error.status != 200 && api_error.status != 0 {
                return Err(ZtkError::api(api_error.status, api_error.msg));
            }
        }

        // 解析为目标类型
        serde_json::from_str(&text).map_err(ZtkError::Parse)
    }
}

/// API 错误响应结构
#[derive(Debug, serde::Deserialize)]
struct ApiErrorResponse {
    /// 状态码
    #[serde(default)]
    status: i32,
    /// 错误消息
    #[serde(default)]
    msg: String,
}

/// URL 编码函数
///
/// 对字符串进行 URL 编码，保留字母数字和部分特殊字符
///
/// # Arguments
///
/// * `input` - 要编码的字符串
///
/// # Returns
///
/// 返回 URL 编码后的字符串
pub fn url_encode(input: &str) -> String {
    let mut encoded = String::with_capacity(input.len() * 3);

    for byte in input.bytes() {
        match byte {
            // 不编码的字符: 字母、数字、-、_、.、~
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                encoded.push(byte as char);
            }
            // 其他字符进行百分号编码
            _ => {
                encoded.push('%');
                encoded.push_str(&format!("{:02X}", byte));
            }
        }
    }

    encoded
}

/// URL 解码函数
///
/// 对 URL 编码的字符串进行解码
///
/// # Arguments
///
/// * `input` - URL 编码的字符串
///
/// # Returns
///
/// 返回解码后的字符串或错误
pub fn url_decode(input: &str) -> ZtkResult<String> {
    let mut decoded = Vec::with_capacity(input.len());
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '%' {
            // 读取两个十六进制字符
            let hex1 = chars
                .next()
                .ok_or_else(|| ZtkError::url_encode("无效的 URL 编码: 缺少十六进制字符"))?;
            let hex2 = chars
                .next()
                .ok_or_else(|| ZtkError::url_encode("无效的 URL 编码: 缺少十六进制字符"))?;

            let hex_str: String = [hex1, hex2].iter().collect();
            let byte = u8::from_str_radix(&hex_str, 16)
                .map_err(|_| ZtkError::url_encode(format!("无效的十六进制字符: {}", hex_str)))?;

            decoded.push(byte);
        } else if c == '+' {
            // + 号解码为空格
            decoded.push(b' ');
        } else {
            decoded.push(c as u8);
        }
    }

    String::from_utf8(decoded).map_err(|e| ZtkError::url_encode(format!("UTF-8 解码失败: {}", e)))
}

/// 构建带有 appkey 的请求参数
///
/// # Arguments
///
/// * `appkey` - 折淘客 AppKey
/// * `params` - 其他请求参数
///
/// # Returns
///
/// 返回包含 appkey 的参数 HashMap
pub fn build_params_with_appkey<P: Serialize>(
    appkey: &str,
    params: &P,
) -> ZtkResult<HashMap<String, String>> {
    let json_value = serde_json::to_value(params)?;

    let mut map = HashMap::new();
    map.insert("appkey".to_string(), appkey.to_string());

    if let serde_json::Value::Object(obj) = json_value {
        for (key, value) in obj {
            let value_str = match value {
                serde_json::Value::Null => continue,
                serde_json::Value::String(s) => s,
                serde_json::Value::Number(n) => n.to_string(),
                serde_json::Value::Bool(b) => b.to_string(),
                _ => serde_json::to_string(&value)?,
            };
            map.insert(key, value_str);
        }
    }

    Ok(map)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_url_encode_basic() {
        assert_eq!(url_encode("hello"), "hello");
        assert_eq!(url_encode("hello world"), "hello%20world");
        assert_eq!(url_encode("a=b&c=d"), "a%3Db%26c%3Dd");
    }

    #[test]
    fn test_url_encode_chinese() {
        let input = "淘口令";
        let encoded = url_encode(input);
        // 中文字符应该被编码
        assert!(encoded.contains('%'));
        assert!(!encoded.contains("淘"));
    }

    #[test]
    fn test_url_encode_special_chars() {
        assert_eq!(url_encode("test-value"), "test-value");
        assert_eq!(url_encode("test_value"), "test_value");
        assert_eq!(url_encode("test.value"), "test.value");
        assert_eq!(url_encode("test~value"), "test~value");
    }

    #[test]
    fn test_url_decode_basic() {
        assert_eq!(url_decode("hello").unwrap(), "hello");
        assert_eq!(url_decode("hello%20world").unwrap(), "hello world");
        assert_eq!(url_decode("a%3Db%26c%3Dd").unwrap(), "a=b&c=d");
    }

    #[test]
    fn test_url_decode_plus_sign() {
        assert_eq!(url_decode("hello+world").unwrap(), "hello world");
    }

    #[test]
    fn test_url_encode_decode_roundtrip() {
        let test_cases = vec![
            "hello world",
            "淘口令测试",
            "a=b&c=d",
            "special!@#$%^&*()",
            "mixed 中文 and English",
        ];

        for input in test_cases {
            let encoded = url_encode(input);
            let decoded = url_decode(&encoded).unwrap();
            assert_eq!(decoded, input, "Round-trip failed for: {}", input);
        }
    }

    #[test]
    fn test_url_decode_invalid() {
        // 不完整的编码
        assert!(url_decode("%").is_err());
        assert!(url_decode("%2").is_err());

        // 无效的十六进制
        assert!(url_decode("%GG").is_err());
    }

    #[test]
    fn test_default_base_url() {
        assert_eq!(DEFAULT_BASE_URL, "https://api.zhetaoke.com:10001");
    }

    #[test]
    fn test_backup_base_url() {
        assert_eq!(BACKUP_BASE_URL, "http://api.zhetaoke.cn:10000");
    }

    #[test]
    fn test_http_client_creation() {
        let client = HttpClient::new("https://example.com", None);
        assert!(client.is_ok());

        let client = client.unwrap();
        assert_eq!(client.base_url(), "https://example.com");
    }

    #[test]
    fn test_http_client_with_defaults() {
        let client = HttpClient::with_defaults();
        assert!(client.is_ok());

        let client = client.unwrap();
        assert_eq!(client.base_url(), DEFAULT_BASE_URL);
    }

    #[test]
    fn test_build_params_with_appkey() {
        #[derive(Serialize)]
        struct TestParams {
            name: String,
            value: i32,
        }

        let params = TestParams {
            name: "test".to_string(),
            value: 123,
        };

        let result = build_params_with_appkey("my_appkey", &params).unwrap();

        assert_eq!(result.get("appkey"), Some(&"my_appkey".to_string()));
        assert_eq!(result.get("name"), Some(&"test".to_string()));
        assert_eq!(result.get("value"), Some(&"123".to_string()));
    }

    #[test]
    fn test_build_params_with_optional_fields() {
        #[derive(Serialize)]
        struct TestParams {
            required: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            optional: Option<String>,
        }

        let params = TestParams {
            required: "value".to_string(),
            optional: None,
        };

        let result = build_params_with_appkey("my_appkey", &params).unwrap();

        assert_eq!(result.get("appkey"), Some(&"my_appkey".to_string()));
        assert_eq!(result.get("required"), Some(&"value".to_string()));
        assert!(!result.contains_key("optional"));
    }
}
