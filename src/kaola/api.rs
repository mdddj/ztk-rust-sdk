//! 考拉平台 API 实现
//!
//! 提供考拉平台相关的 API 调用功能，包括：
//! - 商品转链
//! - 精选商品列表
//! - 商品搜索
//! - 商品详情
//! - 订单查询

use crate::client::ZtkClient;
use crate::common::http::build_params_with_appkey;
use crate::error::ZtkResult;

use super::request::{
    KaolaConvertRequest, KaolaGoodsDetailRequest, KaolaGoodsListRequest, KaolaOrderQueryRequest,
    KaolaSearchGoodsRequest,
};
use super::response::{
    KaolaConvertResponse, KaolaGoodsDetailResponse, KaolaGoodsListResponse, KaolaOrderResponse,
    KaolaSearchGoodsResponse,
};

/// 考拉平台 API
///
/// 提供考拉平台相关的 API 调用方法
///
/// # Example
///
/// ```rust,ignore
/// use ztk_sdk::ZtkClient;
/// use ztk_sdk::kaola::{KaolaConvertRequest, KaolaSearchGoodsRequest};
///
/// let client = ZtkClient::new("your_appkey").build()?;
/// let kaola = client.kaola();
///
/// // 转链
/// let convert_request = KaolaConvertRequest::new("sid", "6919173790682972930");
/// let result = kaola.convert(convert_request).await?;
///
/// // 搜索商品
/// let search_request = KaolaSearchGoodsRequest::new("球鞋")
///     .page_no(1)
///     .page_size(20);
/// let result = kaola.search_goods(search_request).await?;
/// ```
pub struct KaolaApi<'a> {
    /// 客户端引用
    client: &'a ZtkClient,
}

impl<'a> KaolaApi<'a> {
    /// 创建考拉 API 实例
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

    /// 考拉转链
    ///
    /// 根据考拉商品 ID 或链接生成推广链接
    ///
    /// # Arguments
    ///
    /// * `request` - 转链请求参数
    ///
    /// # Returns
    ///
    /// 返回转链结果，包含原始链接、推广长链接和推广短链接
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let request = KaolaConvertRequest::new("sid", "6919173790682972930")
    ///     .tracking_code2("12345");
    /// let result = kaola.convert(request).await?;
    /// ```
    pub async fn convert(&self, request: KaolaConvertRequest) -> ZtkResult<KaolaConvertResponse> {
        let params = build_params_with_appkey(self.client.appkey(), &request)?;
        self.client
            .http_client()
            .get("/api/open_kaola_zhuanke_api_zhuanlian.ashx", &params)
            .await
    }

    /// 考拉精选商品列表
    ///
    /// 获取考拉精选商品列表，只返回商品 ID 列表
    ///
    /// # Arguments
    ///
    /// * `request` - 精选商品列表请求参数
    ///
    /// # Returns
    ///
    /// 返回商品 ID 列表
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let request = KaolaGoodsListRequest::new("1")  // 1-每日平价商品
    ///     .page_no(1)
    ///     .page_size(20);
    /// let result = kaola.goods_list(request).await?;
    /// ```
    pub async fn goods_list(
        &self,
        request: KaolaGoodsListRequest,
    ) -> ZtkResult<KaolaGoodsListResponse> {
        let params = build_params_with_appkey(self.client.appkey(), &request)?;
        self.client
            .http_client()
            .get(
                "/api/open_kaola_zhuanke_api_querySelectedGoods.ashx",
                &params,
            )
            .await
    }

    /// 考拉商品搜索
    ///
    /// 根据关键词搜索考拉商品列表
    ///
    /// # Arguments
    ///
    /// * `request` - 商品搜索请求参数
    ///
    /// # Returns
    ///
    /// 返回商品搜索结果列表
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let request = KaolaSearchGoodsRequest::new("球鞋")
    ///     .sort_type("2")  // 销量排序
    ///     .page_no(1)
    ///     .page_size(20);
    /// let result = kaola.search_goods(request).await?;
    /// ```
    pub async fn search_goods(
        &self,
        request: KaolaSearchGoodsRequest,
    ) -> ZtkResult<KaolaSearchGoodsResponse> {
        let params = build_params_with_appkey(self.client.appkey(), &request)?;
        self.client
            .http_client()
            .get("/api/open_kaola_zhuanke_api_searchGoods.ashx", &params)
            .await
    }

    /// 考拉商品详情
    ///
    /// 根据商品 ID 获取商品详情信息
    ///
    /// # Arguments
    ///
    /// * `request` - 商品详情请求参数
    ///
    /// # Returns
    ///
    /// 返回商品详情信息
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let request = KaolaGoodsDetailRequest::new("2480622");
    /// let result = kaola.goods_detail(request).await?;
    /// ```
    pub async fn goods_detail(
        &self,
        request: KaolaGoodsDetailRequest,
    ) -> ZtkResult<KaolaGoodsDetailResponse> {
        let params = build_params_with_appkey(self.client.appkey(), &request)?;
        self.client
            .http_client()
            .get("/api/open_kaola_zhuanke_api_queryGoodsInfo.ashx", &params)
            .await
    }

    /// 考拉订单查询
    ///
    /// 查询考拉推广订单列表
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
    /// let request = KaolaOrderQueryRequest::new("1")  // 按支付时间查询
    ///     .page(1)
    ///     .page_size(50)
    ///     .start_time("2023-01-01 00:00:00")
    ///     .end_time("2023-01-31 23:59:59");
    /// let result = kaola.query_orders(request).await?;
    /// ```
    pub async fn query_orders(
        &self,
        request: KaolaOrderQueryRequest,
    ) -> ZtkResult<KaolaOrderResponse> {
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
    fn test_kaola_api_creation() {
        let client = ZtkClient::new("test_appkey").build().unwrap();
        let kaola = client.kaola();
        assert_eq!(kaola.client().appkey(), "test_appkey");
    }
}
