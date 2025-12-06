//! 饿了么平台 API 实现
//!
//! 提供饿了么平台相关的 API 调用功能，包括：
//! - 商品转链
//! - 订单查询

use crate::client::ZtkClient;
use crate::common::http::build_params_with_appkey;
use crate::error::ZtkResult;

use super::request::{ElemeConvertRequest, ElemeOrderQueryRequest};
use super::response::{ElemeConvertResponse, ElemeOrderResponse};

/// 饿了么平台 API
///
/// 提供饿了么平台相关的 API 调用方法
///
/// # Example
///
/// ```rust,ignore
/// use ztk_sdk::ZtkClient;
/// use ztk_sdk::eleme::{ElemeConvertRequest, ElemeOrderQueryRequest};
///
/// let client = ZtkClient::new("your_appkey").build()?;
/// let eleme = client.eleme();
///
/// // 转链
/// let request = ElemeConvertRequest::new("sid", "10144")
///     .customer_id("100000");
/// let result = eleme.convert(request).await?;
///
/// // 查询订单
/// let request = ElemeOrderQueryRequest::new(1, 1, 50)
///     .time_range("2021-04-30 08:00:00", "2021-04-30 09:00:00");
/// let orders = eleme.query_orders(request).await?;
/// ```
pub struct ElemeApi<'a> {
    /// 客户端引用
    client: &'a ZtkClient,
}

impl<'a> ElemeApi<'a> {
    /// 创建饿了么 API 实例
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

    /// 饿了么转链
    ///
    /// 根据饿了么活动 ID 生成联盟推广链接
    ///
    /// # Arguments
    ///
    /// * `request` - 转链请求参数
    ///
    /// # Returns
    ///
    /// 返回包含推广链接和活动信息的响应
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let request = ElemeConvertRequest::new("sid", "10144")
    ///     .customer_id("100000");
    /// let result = eleme.convert(request).await?;
    /// if let Some(response) = result.response {
    ///     if let Some(data) = response.data {
    ///         println!("活动标题: {:?}", data.title);
    ///         if let Some(link) = data.link {
    ///             println!("H5短链: {:?}", link.h5_short_link);
    ///         }
    ///     }
    /// }
    /// ```
    pub async fn convert(&self, request: ElemeConvertRequest) -> ZtkResult<ElemeConvertResponse> {
        let params = build_params_with_appkey(self.client.appkey(), &request)?;
        self.client
            .http_client()
            .get("/api/open_eleme_generateLink.ashx", &params)
            .await
    }

    /// 饿了么订单查询
    ///
    /// 根据订单支付时间或更新时间获取最近 90 天内的饿了么订单数据
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
    /// let request = ElemeOrderQueryRequest::new(1, 1, 50)
    ///     .time_range("2021-04-30 08:00:00", "2021-04-30 09:00:00");
    /// let result = eleme.query_orders(request).await?;
    /// if let Some(orders) = result.content {
    ///     for order in orders {
    ///         println!("订单: {:?}, 佣金: {:?}", order.orderid, order.profit);
    ///     }
    /// }
    /// ```
    pub async fn query_orders(
        &self,
        request: ElemeOrderQueryRequest,
    ) -> ZtkResult<ElemeOrderResponse> {
        let params = build_params_with_appkey(self.client.appkey(), &request)?;
        self.client
            .http_client()
            .get("/api/open_lianmeng_orderList.ashx", &params)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eleme_api_creation() {
        let client = ZtkClient::new("test_appkey").build().unwrap();
        let eleme = client.eleme();
        assert_eq!(eleme.client().appkey(), "test_appkey");
    }
}
