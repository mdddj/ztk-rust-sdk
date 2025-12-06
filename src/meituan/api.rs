//! 美团平台 API 实现
//!
//! 提供美团平台相关的 API 调用功能，包括：
//! - 商品转链
//! - 订单查询

use crate::client::ZtkClient;
use crate::common::http::build_params_with_appkey;
use crate::error::ZtkResult;

use super::request::{MeituanConvertRequest, MeituanOrderQueryRequest};
use super::response::{MeituanConvertResponse, MeituanOrderResponse};

/// 美团平台 API
///
/// 提供美团平台相关的 API 调用方法
///
/// # Example
///
/// ```rust,ignore
/// use ztk_sdk::ZtkClient;
/// use ztk_sdk::meituan::{MeituanConvertRequest, MeituanOrderQueryRequest};
///
/// let client = ZtkClient::new("your_appkey").build()?;
/// let meituan = client.meituan();
///
/// // 转链
/// let request = MeituanConvertRequest::new("sid", "7")
///     .link_type(2)
///     .customer_id("100000");
/// let result = meituan.convert(request).await?;
///
/// // 查询订单
/// let request = MeituanOrderQueryRequest::new(1, 1, 50)
///     .time_range("2021-04-30 08:00:00", "2021-04-30 09:00:00");
/// let orders = meituan.query_orders(request).await?;
/// ```
pub struct MeituanApi<'a> {
    /// 客户端引用
    client: &'a ZtkClient,
}

impl<'a> MeituanApi<'a> {
    /// 创建美团 API 实例
    ///
    /// # Arguments
    ///
    /// * `client` - ZtkClient 引用
    pub fn new(client: &'a ZtkClient) -> Self {
        Self { client }
    }

    /// 获取客户端引用
    #[allow(dead_code)]
    pub(crate) fn client(&self) -> &ZtkClient {
        self.client
    }

    /// 美团转链
    ///
    /// 根据美团活动 ID 或链接生成联盟推广链接
    ///
    /// # Arguments
    ///
    /// * `request` - 转链请求参数
    ///
    /// # Returns
    ///
    /// 返回包含推广链接和二维码图片的响应
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let request = MeituanConvertRequest::new("sid", "7")
    ///     .link_type(2)
    ///     .customer_id("100000");
    /// let result = meituan.convert(request).await?;
    /// println!("推广链接: {:?}", result.data);
    /// ```
    pub async fn convert(
        &self,
        request: MeituanConvertRequest,
    ) -> ZtkResult<MeituanConvertResponse> {
        let params = build_params_with_appkey(self.client.appkey(), &request)?;
        self.client
            .http_client()
            .get("/api/open_meituan_generateLink.ashx", &params)
            .await
    }

    /// 美团订单查询
    ///
    /// 根据订单支付时间或更新时间获取最近 90 天内的美团订单数据
    ///
    /// # Arguments
    ///
    /// * `request` - 订单查询请求参数
    ///
    /// # Returns
    ///
    /// 返回订单列表
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let request = MeituanOrderQueryRequest::new(1, 1, 50)
    ///     .time_range("2021-04-30 08:00:00", "2021-04-30 09:00:00");
    /// let result = meituan.query_orders(request).await?;
    /// if let Some(orders) = result.content {
    ///     for order in orders {
    ///         println!("订单: {:?}, 佣金: {:?}", order.orderid, order.profit);
    ///     }
    /// }
    /// ```
    pub async fn query_orders(
        &self,
        request: MeituanOrderQueryRequest,
    ) -> ZtkResult<MeituanOrderResponse> {
        let params = build_params_with_appkey(self.client.appkey(), &request)?;
        self.client
            .http_client()
            .get("/api/open_meituan_orderList2.ashx", &params)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_meituan_api_creation() {
        let client = ZtkClient::new("test_appkey").build().unwrap();
        let meituan = client.meituan();
        assert_eq!(meituan.client().appkey(), "test_appkey");
    }
}
