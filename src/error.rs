//! 错误类型定义
//!
//! 定义 SDK 中使用的错误类型，实现 std::error::Error trait

use thiserror::Error;

/// SDK 错误类型
///
/// 包含所有可能的错误情况：
/// - 网络请求错误
/// - API 业务错误
/// - JSON 解析错误
/// - 参数验证错误
/// - URL 编码错误
#[derive(Debug, Error)]
pub enum ZtkError {
    /// 网络请求错误
    ///
    /// 当 HTTP 请求失败时返回此错误
    #[error("网络请求失败: {0}")]
    Network(#[from] reqwest::Error),

    /// API 业务错误
    ///
    /// 当 API 返回业务错误时返回此错误
    #[error("API 错误 [{code}]: {message}")]
    Api {
        /// 错误码
        code: i32,
        /// 错误消息
        message: String,
        /// 子错误码
        sub_code: Option<String>,
        /// 子错误消息
        sub_msg: Option<String>,
    },

    /// JSON 解析错误
    ///
    /// 当 JSON 反序列化失败时返回此错误
    #[error("JSON 解析失败: {0}")]
    Parse(#[from] serde_json::Error),

    /// 参数验证错误
    ///
    /// 当请求参数验证失败时返回此错误
    #[error("参数验证失败: {0}")]
    Validation(String),

    /// URL 编码错误
    ///
    /// 当 URL 编码失败时返回此错误
    #[error("URL 编码失败: {0}")]
    UrlEncode(String),
}

/// Result 类型别名
///
/// SDK 中所有返回 Result 的方法都使用此类型
pub type ZtkResult<T> = Result<T, ZtkError>;

impl ZtkError {
    /// 创建 API 错误
    ///
    /// # Arguments
    ///
    /// * `code` - 错误码
    /// * `message` - 错误消息
    pub fn api(code: i32, message: impl Into<String>) -> Self {
        ZtkError::Api {
            code,
            message: message.into(),
            sub_code: None,
            sub_msg: None,
        }
    }

    /// 创建带子错误信息的 API 错误
    ///
    /// # Arguments
    ///
    /// * `code` - 错误码
    /// * `message` - 错误消息
    /// * `sub_code` - 子错误码
    /// * `sub_msg` - 子错误消息
    pub fn api_with_sub(
        code: i32,
        message: impl Into<String>,
        sub_code: Option<String>,
        sub_msg: Option<String>,
    ) -> Self {
        ZtkError::Api {
            code,
            message: message.into(),
            sub_code,
            sub_msg,
        }
    }

    /// 创建参数验证错误
    ///
    /// # Arguments
    ///
    /// * `message` - 验证失败原因
    pub fn validation(message: impl Into<String>) -> Self {
        ZtkError::Validation(message.into())
    }

    /// 创建 URL 编码错误
    ///
    /// # Arguments
    ///
    /// * `message` - 编码失败原因
    pub fn url_encode(message: impl Into<String>) -> Self {
        ZtkError::UrlEncode(message.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_error_display() {
        let err = ZtkError::api(1001, "测试错误");
        assert_eq!(format!("{}", err), "API 错误 [1001]: 测试错误");
    }

    #[test]
    fn test_validation_error_display() {
        let err = ZtkError::validation("参数不能为空");
        assert_eq!(format!("{}", err), "参数验证失败: 参数不能为空");
    }

    #[test]
    fn test_url_encode_error_display() {
        let err = ZtkError::url_encode("无效的 URL 字符");
        assert_eq!(format!("{}", err), "URL 编码失败: 无效的 URL 字符");
    }

    #[test]
    fn test_error_is_std_error() {
        let err: Box<dyn std::error::Error> = Box::new(ZtkError::validation("test"));
        assert!(err.source().is_none());
    }

    #[test]
    fn test_api_error_with_sub() {
        let err = ZtkError::api_with_sub(
            1001,
            "主错误",
            Some("SUB_001".to_string()),
            Some("子错误消息".to_string()),
        );
        match err {
            ZtkError::Api {
                code,
                message,
                sub_code,
                sub_msg,
            } => {
                assert_eq!(code, 1001);
                assert_eq!(message, "主错误");
                assert_eq!(sub_code, Some("SUB_001".to_string()));
                assert_eq!(sub_msg, Some("子错误消息".to_string()));
            }
            _ => panic!("Expected Api error"),
        }
    }
}
