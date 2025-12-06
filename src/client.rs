//! 核心客户端实现
//!
//! 提供 ZtkClient 和 ZtkClientBuilder，用于配置和发起 API 请求

use std::sync::Arc;
use std::time::Duration;

use crate::common::http::{HttpClient, DEFAULT_BASE_URL};
use crate::error::{ZtkError, ZtkResult};

/// 默认请求超时时间 (秒)
const DEFAULT_TIMEOUT_SECS: u64 = 30;

/// 折淘客 SDK 客户端构建器
///
/// 使用 Builder 模式配置客户端参数
///
/// # Example
///
/// ```rust,ignore
/// use ztk_rust_sdk::ZtkClient;
///
/// let client = ZtkClient::new("your_appkey")
///     .base_url("https://api.zhetaoke.com:10001")
///     .timeout(std::time::Duration::from_secs(60))
///     .build()?;
/// ```
#[derive(Debug, Clone)]
pub struct ZtkClientBuilder {
    /// 折淘客 AppKey
    appkey: String,
    /// API 基础地址 (可选，默认使用主接口地址)
    base_url: Option<String>,
    /// 请求超时时间 (可选，默认 30 秒)
    timeout: Option<Duration>,
}

impl ZtkClientBuilder {
    /// 创建新的客户端构建器
    ///
    /// # Arguments
    ///
    /// * `appkey` - 折淘客 AppKey
    pub fn new(appkey: impl Into<String>) -> Self {
        Self {
            appkey: appkey.into(),
            base_url: None,
            timeout: None,
        }
    }

    /// 设置 API 基础地址
    ///
    /// 默认使用主接口地址: https://api.zhetaoke.com:10001
    /// 备用接口地址: http://api.zhetaoke.cn:10000
    ///
    /// # Arguments
    ///
    /// * `url` - API 基础地址
    pub fn base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = Some(url.into());
        self
    }

    /// 设置请求超时时间
    ///
    /// 默认超时时间为 30 秒
    ///
    /// # Arguments
    ///
    /// * `timeout` - 超时时间
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// 构建客户端实例
    ///
    /// # Returns
    ///
    /// 返回配置好的 ZtkClient 实例或错误
    pub fn build(self) -> ZtkResult<ZtkClient> {
        // 验证 appkey 不为空
        if self.appkey.is_empty() {
            return Err(ZtkError::validation("appkey 不能为空"));
        }

        let base_url = self
            .base_url
            .unwrap_or_else(|| DEFAULT_BASE_URL.to_string());
        let timeout = self
            .timeout
            .unwrap_or(Duration::from_secs(DEFAULT_TIMEOUT_SECS));

        let http_client = HttpClient::new(&base_url, Some(timeout))?;

        Ok(ZtkClient {
            inner: Arc::new(ZtkClientInner {
                http_client,
                base_url,
                appkey: self.appkey,
            }),
        })
    }
}

/// 内部客户端数据
#[derive(Debug)]
struct ZtkClientInner {
    /// HTTP 客户端
    http_client: HttpClient,
    /// API 基础地址
    base_url: String,
    /// 折淘客 AppKey
    appkey: String,
}

/// 折淘客 SDK 客户端
///
/// 提供各平台 API 的访问入口
///
/// # Example
///
/// ```rust,ignore
/// use ztk_rust_sdk::ZtkClient;
///
/// #[tokio::main]
/// async fn main() -> ztk_rust_sdk::ZtkResult<()> {
///     let client = ZtkClient::new("your_appkey").build()?;
///     
///     // 调用淘宝 API
///     // let result = client.taobao().convert_by_item_id(request).await?;
///     
///     // 调用京东 API
///     // let result = client.jd().convert(request).await?;
///     
///     Ok(())
/// }
/// ```
#[derive(Debug, Clone)]
pub struct ZtkClient {
    /// 内部数据 (使用 Arc 实现 Clone)
    inner: Arc<ZtkClientInner>,
}

impl ZtkClient {
    /// 创建新的客户端构建器
    ///
    /// # Arguments
    ///
    /// * `appkey` - 折淘客 AppKey
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let client = ZtkClient::new("your_appkey")
    ///     .base_url("https://api.zhetaoke.com:10001")
    ///     .build()?;
    /// ```
    pub fn new(appkey: impl Into<String>) -> ZtkClientBuilder {
        ZtkClientBuilder::new(appkey)
    }

    /// 获取 AppKey
    pub fn appkey(&self) -> &str {
        &self.inner.appkey
    }

    /// 获取 API 基础地址
    pub fn base_url(&self) -> &str {
        &self.inner.base_url
    }

    /// 获取 HTTP 客户端引用 (内部使用)
    pub(crate) fn http_client(&self) -> &HttpClient {
        &self.inner.http_client
    }

    /// 获取淘宝平台 API
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let result = client.taobao().convert_by_item_id(request).await?;
    /// ```
    #[cfg(feature = "taobao")]
    pub fn taobao(&self) -> crate::taobao::TaobaoApi<'_> {
        crate::taobao::TaobaoApi::new(self)
    }

    /// 获取京东平台 API
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let result = client.jd().convert(request).await?;
    /// ```
    #[cfg(feature = "jd")]
    pub fn jd(&self) -> crate::jd::JdApi<'_> {
        crate::jd::JdApi::new(self)
    }

    /// 获取拼多多平台 API
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let result = client.pdd().convert(request).await?;
    /// ```
    #[cfg(feature = "pdd")]
    pub fn pdd(&self) -> crate::pdd::PddApi<'_> {
        crate::pdd::PddApi::new(self)
    }

    /// 获取唯品会平台 API
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let result = client.vip().convert(request).await?;
    /// ```
    #[cfg(feature = "vip")]
    pub fn vip(&self) -> crate::vip::VipApi<'_> {
        crate::vip::VipApi::new(self)
    }

    /// 获取美团平台 API
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let result = client.meituan().convert(request).await?;
    /// ```
    #[cfg(feature = "meituan")]
    pub fn meituan(&self) -> crate::meituan::MeituanApi<'_> {
        crate::meituan::MeituanApi::new(self)
    }

    /// 获取考拉平台 API
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let result = client.kaola().convert(request).await?;
    /// ```
    #[cfg(feature = "kaola")]
    pub fn kaola(&self) -> crate::kaola::KaolaApi<'_> {
        crate::kaola::KaolaApi::new(self)
    }

    /// 获取饿了么平台 API
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let result = client.eleme().convert(request).await?;
    /// ```
    #[cfg(feature = "eleme")]
    pub fn eleme(&self) -> crate::eleme::ElemeApi<'_> {
        crate::eleme::ElemeApi::new(self)
    }

    /// 获取抖音平台 API
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let result = client.douyin().convert_goods(request).await?;
    /// ```
    #[cfg(feature = "douyin")]
    pub fn douyin(&self) -> crate::douyin::DouyinApi<'_> {
        crate::douyin::DouyinApi::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::http::BACKUP_BASE_URL;

    #[test]
    fn test_builder_new() {
        let builder = ZtkClientBuilder::new("test_appkey");
        assert_eq!(builder.appkey, "test_appkey");
        assert!(builder.base_url.is_none());
        assert!(builder.timeout.is_none());
    }

    #[test]
    fn test_builder_base_url() {
        let builder = ZtkClientBuilder::new("test_appkey").base_url("https://custom.api.com");
        assert_eq!(builder.base_url, Some("https://custom.api.com".to_string()));
    }

    #[test]
    fn test_builder_timeout() {
        let builder = ZtkClientBuilder::new("test_appkey").timeout(Duration::from_secs(60));
        assert_eq!(builder.timeout, Some(Duration::from_secs(60)));
    }

    #[test]
    fn test_builder_chain() {
        let builder = ZtkClientBuilder::new("test_appkey")
            .base_url("https://custom.api.com")
            .timeout(Duration::from_secs(60));

        assert_eq!(builder.appkey, "test_appkey");
        assert_eq!(builder.base_url, Some("https://custom.api.com".to_string()));
        assert_eq!(builder.timeout, Some(Duration::from_secs(60)));
    }

    #[test]
    fn test_build_with_defaults() {
        let client = ZtkClient::new("test_appkey").build().unwrap();

        assert_eq!(client.appkey(), "test_appkey");
        assert_eq!(client.base_url(), DEFAULT_BASE_URL);
    }

    #[test]
    fn test_build_with_custom_base_url() {
        let client = ZtkClient::new("test_appkey")
            .base_url(BACKUP_BASE_URL)
            .build()
            .unwrap();

        assert_eq!(client.appkey(), "test_appkey");
        assert_eq!(client.base_url(), BACKUP_BASE_URL);
    }

    #[test]
    fn test_build_empty_appkey_fails() {
        let result = ZtkClient::new("").build();
        assert!(result.is_err());

        match result {
            Err(ZtkError::Validation(msg)) => {
                assert!(msg.contains("appkey"));
            }
            _ => panic!("Expected Validation error"),
        }
    }

    #[test]
    fn test_client_clone() {
        let client = ZtkClient::new("test_appkey").build().unwrap();
        let cloned = client.clone();

        assert_eq!(client.appkey(), cloned.appkey());
        assert_eq!(client.base_url(), cloned.base_url());
    }

    #[test]
    fn test_client_new_returns_builder() {
        let builder = ZtkClient::new("test_appkey");
        // 验证返回的是 ZtkClientBuilder
        let _client = builder.build().unwrap();
    }
}
