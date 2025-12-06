//! 拼多多平台 API 实现
//!
//! 提供拼多多平台相关的 API 调用功能，包括：
//! - 商品转链
//! - 商品详情 (简版/详版)
//! - 订单查询
//! - 授权备案

use crate::client::ZtkClient;
use crate::common::http::build_params_with_appkey;
use crate::error::ZtkResult;

use super::request::{
    PddAuthorizeQueryRequest, PddAuthorizeRequest, PddConvertRequest, PddGoodsDetailFullRequest,
    PddGoodsDetailSimpleRequest, PddOrderQueryRequest,
};
use super::response::{
    PddAuthorizeQueryResponse, PddAuthorizeResponse, PddConvertResponse,
    PddGoodsDetailFullResponse, PddGoodsDetailSimpleResponse, PddOrderResponse,
};

/// 拼多多平台 API
///
/// 提供拼多多平台相关的 API 调用方法
///
/// # Example
///
/// ```rust,ignore
/// use ztk_rust_sdk::ZtkClient;
/// use ztk_rust_sdk::pdd::PddConvertRequest;
///
/// #[tokio::main]
/// async fn main() -> ztk_rust_sdk::ZtkResult<()> {
///     let client = ZtkClient::new("your_appkey").build()?;
///     
///     let request = PddConvertRequest::new(
///         "pdd_app_key",
///         "pdd_app_secret",
///         "pid",
///         "453581732819"
///     );
///     
///     let result = client.pdd().convert(request).await?;
///     println!("转链结果: {:?}", result);
///     
///     Ok(())
/// }
/// ```
pub struct PddApi<'a> {
    /// 客户端引用
    client: &'a ZtkClient,
}

impl<'a> PddApi<'a> {
    /// 创建拼多多 API 实例
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

    /// 拼多多转链
    ///
    /// 将商品 ID 或推广短链转换为推广链接
    ///
    /// # Arguments
    ///
    /// * `request` - 转链请求参数
    ///
    /// # Returns
    ///
    /// 返回转链结果，包含多种推广链接
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let request = PddConvertRequest::new("pdd_app_key", "pdd_app_secret", "pid", "453581732819")
    ///     .custom_parameters(r#"{"uid":"user123"}"#);
    /// let result = client.pdd().convert(request).await?;
    /// ```
    pub async fn convert(&self, request: PddConvertRequest) -> ZtkResult<PddConvertResponse> {
        let params = build_params_with_appkey(self.client.appkey(), &request)?;
        self.client
            .http_client()
            .get("/api/open_pdd_zhuanlian_new.ashx", &params)
            .await
    }

    /// 商品详情 (简版)
    ///
    /// 获取拼多多商品的基本详情信息
    ///
    /// # Arguments
    ///
    /// * `request` - 商品详情请求参数
    ///
    /// # Returns
    ///
    /// 返回商品基本信息
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let request = PddGoodsDetailSimpleRequest::new("pdd_app_key", "pdd_app_secret", "pid", "453581732819");
    /// let result = client.pdd().goods_detail_simple(request).await?;
    /// ```
    pub async fn goods_detail_simple(
        &self,
        request: PddGoodsDetailSimpleRequest,
    ) -> ZtkResult<PddGoodsDetailSimpleResponse> {
        let params = build_params_with_appkey(self.client.appkey(), &request)?;
        self.client
            .http_client()
            .get("/api/open_pdd_goods_detail_get_new.ashx", &params)
            .await
    }

    /// 商品详情 (详版)
    ///
    /// 获取拼多多商品的详细信息，支持多种筛选条件
    ///
    /// # Arguments
    ///
    /// * `request` - 商品详情请求参数
    ///
    /// # Returns
    ///
    /// 返回商品详细信息
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let request = PddGoodsDetailFullRequest::new("pdd_app_key", "pdd_app_secret", "pid", "453581732819")
    ///     .sort_type(2)
    ///     .with_coupon(true);
    /// let result = client.pdd().goods_detail_full(request).await?;
    /// ```
    pub async fn goods_detail_full(
        &self,
        request: PddGoodsDetailFullRequest,
    ) -> ZtkResult<PddGoodsDetailFullResponse> {
        let params = build_params_with_appkey(self.client.appkey(), &request)?;
        self.client
            .http_client()
            .get("/api/open_pdd_goods_detail_search_new.ashx", &params)
            .await
    }

    /// 订单查询
    ///
    /// 查询拼多多推广订单
    ///
    /// # Arguments
    ///
    /// * `request` - 订单查询请求参数
    ///
    /// # Returns
    ///
    /// 返回订单列表数据
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let request = PddOrderQueryRequest::new("pdd_app_key", "pdd_app_secret", 1699200000, 1699286400)
    ///     .page(1)
    ///     .page_size(50);
    /// let result = client.pdd().query_orders(request).await?;
    /// ```
    pub async fn query_orders(&self, request: PddOrderQueryRequest) -> ZtkResult<PddOrderResponse> {
        let params = build_params_with_appkey(self.client.appkey(), &request)?;
        self.client
            .http_client()
            .get("/api/open_pdd_dingdan_new.ashx", &params)
            .await
    }

    /// 授权备案
    ///
    /// 获取授权备案 URL，用于在服务器上进行 PID 授权备案
    ///
    /// # Arguments
    ///
    /// * `request` - 授权备案请求参数
    ///
    /// # Returns
    ///
    /// 返回授权备案 URL
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let request = PddAuthorizeRequest::new("pdd_app_key", "pdd_app_secret", "pid");
    /// let result = client.pdd().authorize(request).await?;
    /// // 使用返回的 URL 在服务器浏览器中打开进行授权备案
    /// ```
    pub async fn authorize(&self, request: PddAuthorizeRequest) -> ZtkResult<PddAuthorizeResponse> {
        let params = build_params_with_appkey(self.client.appkey(), &request)?;
        self.client
            .http_client()
            .get("/api/open_pdd_shouquan_new.ashx", &params)
            .await
    }

    /// 授权备案查询
    ///
    /// 查询 PID 授权备案状态
    ///
    /// # Arguments
    ///
    /// * `request` - 授权备案查询请求参数
    ///
    /// # Returns
    ///
    /// 返回授权备案状态
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let request = PddAuthorizeQueryRequest::new("pdd_app_key", "pdd_app_secret", "pid");
    /// let result = client.pdd().authorize_query(request).await?;
    /// if result.response.map(|r| r.bind == Some(1)).unwrap_or(false) {
    ///     println!("授权备案成功");
    /// }
    /// ```
    pub async fn authorize_query(
        &self,
        request: PddAuthorizeQueryRequest,
    ) -> ZtkResult<PddAuthorizeQueryResponse> {
        let params = build_params_with_appkey(self.client.appkey(), &request)?;
        self.client
            .http_client()
            .get("/api/open_pdd_shouquan_query_new.ashx", &params)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pdd_api_creation() {
        let client = ZtkClient::new("test_appkey").build().unwrap();
        let pdd = client.pdd();
        assert_eq!(pdd.client().appkey(), "test_appkey");
    }
}
