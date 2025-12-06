//! 京东平台 API 实现
//!
//! 提供京东平台相关的 API 调用功能，包括：
//! - 商品转链
//! - 京粉精选商品
//! - 订单查询
//! - 商品详情
//! - 朋友圈火爆商品

use crate::client::ZtkClient;
use crate::common::http::{build_params_with_appkey, url_encode};
use crate::error::ZtkResult;

use super::request::{
    JdConvertRequest, JdGoodsDetailRequest, JdHotGoodsRequest, JdOrderQueryRequest,
    JingfenGoodsRequest,
};
use super::response::{
    JdConvertResponse, JdGoodsDetailResponse, JdHotGoodsResponse, JdOrderResponse,
    JingfenGoodsResponse,
};

/// 京东平台 API
///
/// 提供京东平台相关的 API 调用方法
///
/// # Example
///
/// ```rust,ignore
/// use ztk_rust_sdk::ZtkClient;
/// use ztk_rust_sdk::jd::{JdConvertRequest, JdChainType};
///
/// #[tokio::main]
/// async fn main() -> ztk_rust_sdk::ZtkResult<()> {
///     let client = ZtkClient::new("your_appkey").build()?;
///     
///     let request = JdConvertRequest::new("https://item.jd.com/123456.html", "your_union_id")
///         .chain_type(JdChainType::Short);
///     
///     let result = client.jd().convert(request).await?;
///     println!("转链结果: {:?}", result);
///     
///     Ok(())
/// }
/// ```
pub struct JdApi<'a> {
    /// 客户端引用
    client: &'a ZtkClient,
}

impl<'a> JdApi<'a> {
    /// 创建京东 API 实例
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

    /// 京东转链
    ///
    /// 将商品链接或 SKU ID 转换为推广链接
    ///
    /// # Arguments
    ///
    /// * `request` - 转链请求参数
    ///
    /// # Returns
    ///
    /// 返回转链结果，包含推广链接
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let request = JdConvertRequest::new("https://item.jd.com/123456.html", "your_union_id")
    ///     .position_id("12345")
    ///     .chain_type(JdChainType::Short);
    /// let result = client.jd().convert(request).await?;
    /// ```
    pub async fn convert(&self, request: JdConvertRequest) -> ZtkResult<JdConvertResponse> {
        // 对 material_id 进行 URL 编码
        let mut encoded_request = request.clone();
        encoded_request.material_id = url_encode(&request.material_id);

        // 如果有 coupon_url，也需要编码
        if let Some(ref coupon_url) = request.coupon_url {
            encoded_request.coupon_url = Some(url_encode(coupon_url));
        }

        let params = build_params_with_appkey(self.client.appkey(), &encoded_request)?;
        self.client
            .http_client()
            .get(
                "/api/open_jing_union_open_promotion_byunionid_get.ashx",
                &params,
            )
            .await
    }

    /// 京粉精选商品
    ///
    /// 获取京粉精选商品列表
    ///
    /// # Arguments
    ///
    /// * `request` - 京粉精选商品请求参数
    ///
    /// # Returns
    ///
    /// 返回京粉精选商品列表
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let request = JingfenGoodsRequest::new(JdEliteId::HotSale)
    ///     .page_index(1)
    ///     .page_size(20);
    /// let result = client.jd().jingfen_goods(request).await?;
    /// ```
    pub async fn jingfen_goods(
        &self,
        request: JingfenGoodsRequest,
    ) -> ZtkResult<JingfenGoodsResponse> {
        let params = build_params_with_appkey(self.client.appkey(), &request)?;
        self.client
            .http_client()
            .get(
                "/api/open_jing_union_open_goods_jingfen_query.ashx",
                &params,
            )
            .await
    }

    /// 订单查询
    ///
    /// 查询京东联盟订单
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
    /// let request = JdOrderQueryRequest::new("2024-01-01 00:00:00")
    ///     .end_time("2024-01-31 23:59:59")
    ///     .query_type(JdOrderQueryType::OrderTime)
    ///     .page_no(1);
    /// let result = client.jd().query_orders(request).await?;
    /// ```
    pub async fn query_orders(&self, request: JdOrderQueryRequest) -> ZtkResult<JdOrderResponse> {
        let params = build_params_with_appkey(self.client.appkey(), &request)?;
        self.client
            .http_client()
            .get("/api/open_jing_union_open_order_query.ashx", &params)
            .await
    }

    /// 商品详情
    ///
    /// 获取京东商品详情信息
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
    /// let request = JdGoodsDetailRequest::new("10025768652616");
    /// let result = client.jd().goods_detail(request).await?;
    /// ```
    pub async fn goods_detail(
        &self,
        request: JdGoodsDetailRequest,
    ) -> ZtkResult<JdGoodsDetailResponse> {
        let params = build_params_with_appkey(self.client.appkey(), &request)?;
        self.client
            .http_client()
            .get("/api/open_jing_union_open_goods_query.ashx", &params)
            .await
    }

    /// 朋友圈火爆商品
    ///
    /// 获取京东朋友圈火爆商品列表，包含精编社群推广文案
    ///
    /// # Arguments
    ///
    /// * `request` - 朋友圈火爆商品请求参数
    ///
    /// # Returns
    ///
    /// 返回朋友圈火爆商品列表
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let request = JdHotGoodsRequest::new()
    ///     .page_index(1)
    ///     .page_size(20)
    ///     .keyword("手机");
    /// let result = client.jd().hot_goods(request).await?;
    /// ```
    pub async fn hot_goods(&self, request: JdHotGoodsRequest) -> ZtkResult<JdHotGoodsResponse> {
        let params = build_params_with_appkey(self.client.appkey(), &request)?;
        self.client
            .http_client()
            .get("/api/open_jing_goods_list.ashx", &params)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jd_api_creation() {
        let client = ZtkClient::new("test_appkey").build().unwrap();
        let jd = client.jd();
        assert_eq!(jd.client().appkey(), "test_appkey");
    }
}
