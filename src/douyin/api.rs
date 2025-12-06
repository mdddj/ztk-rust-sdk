//! 抖音平台 API 实现
//!
//! 提供抖音平台相关的 API 调用功能，包括：
//! - 商品转链
//! - 直播间转链
//! - 活动转链
//! - 商品详情和搜索
//! - 口令解析
//! - 订单查询

use crate::client::ZtkClient;
use crate::common::http::{build_params_with_appkey, url_encode};
use crate::error::ZtkResult;

use super::request::{
    DouyinActivityConvertRequest, DouyinActivityListRequest, DouyinAuthorListRequest,
    DouyinGoodsConvertRequest, DouyinGoodsDetailRequest, DouyinLiveConvertRequest,
    DouyinLiveProductListRequest, DouyinOrderQueryRequest, DouyinParseCommandRequest,
    DouyinSearchGoodsRequest,
};
use super::response::{
    DouyinActivityConvertResponse, DouyinActivityListResponse, DouyinGoodsConvertResponse,
    DouyinGoodsDetailResponse, DouyinLiveConvertResponse, DouyinOrderQueryResponse,
    DouyinParseCommandResponse, DouyinSearchGoodsResponse,
};

/// 抖音平台 API
///
/// 提供抖音平台相关的 API 调用方法
///
/// # Example
///
/// ```rust,ignore
/// use ztk_rust_sdk::ZtkClient;
/// use ztk_rust_sdk::douyin::DouyinGoodsConvertRequest;
///
/// #[tokio::main]
/// async fn main() -> ztk_rust_sdk::ZtkResult<()> {
///     let client = ZtkClient::new("your_appkey").build()?;
///     
///     let request = DouyinGoodsConvertRequest::new("sid", "https://xxx.douyin.com/xxx");
///     let result = client.douyin().convert_goods(request).await?;
///     println!("转链结果: {:?}", result);
///     
///     Ok(())
/// }
/// ```
pub struct DouyinApi<'a> {
    /// 客户端引用
    client: &'a ZtkClient,
}

impl<'a> DouyinApi<'a> {
    /// 创建抖音 API 实例
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

    /// 商品转链
    ///
    /// 将商品 URL/口令/短链接转换为推广链接
    ///
    /// # Arguments
    ///
    /// * `request` - 商品转链请求参数
    ///
    /// # Returns
    ///
    /// 返回转链结果，包含推广链接和口令
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let request = DouyinGoodsConvertRequest::new("sid", "https://xxx.douyin.com/xxx")
    ///     .need_qr_code(true);
    /// let result = client.douyin().convert_goods(request).await?;
    /// ```
    pub async fn convert_goods(
        &self,
        request: DouyinGoodsConvertRequest,
    ) -> ZtkResult<DouyinGoodsConvertResponse> {
        // 对 product_url 进行 URL 编码
        let mut encoded_request = request.clone();
        encoded_request.product_url = url_encode(&request.product_url);

        let params = build_params_with_appkey(self.client.appkey(), &encoded_request)?;
        self.client
            .http_client()
            .get("/api/open_douyin_zhuanlian.ashx", &params)
            .await
    }

    /// 直播间转链
    ///
    /// 将直播间转换为推广链接
    ///
    /// # Arguments
    ///
    /// * `request` - 直播间转链请求参数
    ///
    /// # Returns
    ///
    /// 返回转链结果，包含推广链接和口令
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let request = DouyinLiveConvertRequest::with_buyin_id("sid", "buyin123");
    /// let result = client.douyin().convert_live(request).await?;
    /// ```
    pub async fn convert_live(
        &self,
        request: DouyinLiveConvertRequest,
    ) -> ZtkResult<DouyinLiveConvertResponse> {
        // 对 dy_code 进行 URL 编码 (如果存在)
        let mut encoded_request = request.clone();
        if let Some(ref dy_code) = request.dy_code {
            encoded_request.dy_code = Some(url_encode(dy_code));
        }

        let params = build_params_with_appkey(self.client.appkey(), &encoded_request)?;
        self.client
            .http_client()
            .get("/api/open_douyin_zhuanlian_zhibojian.ashx", &params)
            .await
    }

    /// 活动转链
    ///
    /// 将活动页转换为推广链接
    ///
    /// # Arguments
    ///
    /// * `request` - 活动转链请求参数
    ///
    /// # Returns
    ///
    /// 返回转链结果，包含推广链接和口令
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let request = DouyinActivityConvertRequest::new("sid", "material123");
    /// let result = client.douyin().convert_activity(request).await?;
    /// ```
    pub async fn convert_activity(
        &self,
        request: DouyinActivityConvertRequest,
    ) -> ZtkResult<DouyinActivityConvertResponse> {
        let params = build_params_with_appkey(self.client.appkey(), &request)?;
        self.client
            .http_client()
            .get("/api/open_douyin_zhuanlian_activity.ashx", &params)
            .await
    }

    /// 商品详情
    ///
    /// 获取商品详细信息
    ///
    /// # Arguments
    ///
    /// * `request` - 商品详情请求参数
    ///
    /// # Returns
    ///
    /// 返回商品详情列表
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let request = DouyinGoodsDetailRequest::new("sid", "123,456,789")
    ///     .with_all_fields();
    /// let result = client.douyin().goods_detail(request).await?;
    /// ```
    pub async fn goods_detail(
        &self,
        request: DouyinGoodsDetailRequest,
    ) -> ZtkResult<DouyinGoodsDetailResponse> {
        let params = build_params_with_appkey(self.client.appkey(), &request)?;
        self.client
            .http_client()
            .get("/api/open_douyin_product_detail.ashx", &params)
            .await
    }

    /// 商品搜索
    ///
    /// 搜索抖音商品
    ///
    /// # Arguments
    ///
    /// * `request` - 商品搜索请求参数
    ///
    /// # Returns
    ///
    /// 返回搜索结果列表
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let request = DouyinSearchGoodsRequest::new("sid")
    ///     .title("测试商品")
    ///     .page(1)
    ///     .page_size(20);
    /// let result = client.douyin().search_goods(request).await?;
    /// ```
    pub async fn search_goods(
        &self,
        request: DouyinSearchGoodsRequest,
    ) -> ZtkResult<DouyinSearchGoodsResponse> {
        // 对 title 进行 URL 编码 (如果存在)
        let mut encoded_request = request.clone();
        if let Some(ref title) = request.title {
            encoded_request.title = Some(url_encode(title));
        }

        let params = build_params_with_appkey(self.client.appkey(), &encoded_request)?;
        self.client
            .http_client()
            .get("/api/open_douyin_product_search.ashx", &params)
            .await
    }

    /// 口令解析
    ///
    /// 解析抖音口令/短链接
    ///
    /// # Arguments
    ///
    /// * `request` - 口令解析请求参数
    ///
    /// # Returns
    ///
    /// 返回解析结果，包含口令类型和相关信息
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let request = DouyinParseCommandRequest::new("sid", "口令内容");
    /// let result = client.douyin().parse_command(request).await?;
    /// ```
    pub async fn parse_command(
        &self,
        request: DouyinParseCommandRequest,
    ) -> ZtkResult<DouyinParseCommandResponse> {
        // 对 command 进行 URL 编码
        let encoded_command = url_encode(&request.command);
        let encoded_request = DouyinParseCommandRequestEncoded {
            sid: request.sid,
            command: encoded_command,
        };

        let params = build_params_with_appkey(self.client.appkey(), &encoded_request)?;
        self.client
            .http_client()
            .get("/api/open_douyin_kouling_jiexi.ashx", &params)
            .await
    }

    /// 订单查询
    ///
    /// 查询抖音订单
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
    /// let request = DouyinOrderQueryRequest::by_pay_time(
    ///     "2024-01-01 00:00:00",
    ///     "2024-01-31 23:59:59"
    /// );
    /// let result = client.douyin().query_orders(request).await?;
    /// ```
    pub async fn query_orders(
        &self,
        request: DouyinOrderQueryRequest,
    ) -> ZtkResult<DouyinOrderQueryResponse> {
        let params = build_params_with_appkey(self.client.appkey(), &request)?;
        self.client
            .http_client()
            .get("/api/open_lianmeng_orderList.ashx", &params)
            .await
    }

    /// 活动列表
    ///
    /// 获取抖音活动列表
    ///
    /// # Arguments
    ///
    /// * `request` - 活动列表请求参数
    ///
    /// # Returns
    ///
    /// 返回活动列表数据
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let request = DouyinActivityListRequest::new("sid")
    ///     .page(1)
    ///     .page_size(20);
    /// let result = client.douyin().activity_list(request).await?;
    /// ```
    pub async fn activity_list(
        &self,
        request: DouyinActivityListRequest,
    ) -> ZtkResult<DouyinActivityListResponse> {
        let params = build_params_with_appkey(self.client.appkey(), &request)?;
        self.client
            .http_client()
            .get("/api/open_douyin_activity_list.ashx", &params)
            .await
    }

    /// 达人列表
    ///
    /// 获取抖音直播间达人列表
    ///
    /// # Arguments
    ///
    /// * `request` - 达人列表请求参数
    ///
    /// # Returns
    ///
    /// 返回达人列表数据 (使用商品转链响应格式)
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let request = DouyinAuthorListRequest::new("sid")
    ///     .page(1)
    ///     .page_size(20);
    /// let result = client.douyin().author_list(request).await?;
    /// ```
    pub async fn author_list(
        &self,
        request: DouyinAuthorListRequest,
    ) -> ZtkResult<DouyinGoodsConvertResponse> {
        let params = build_params_with_appkey(self.client.appkey(), &request)?;
        self.client
            .http_client()
            .get("/api/open_douyin_zhibojian_dare_list.ashx", &params)
            .await
    }

    /// 直播间商品列表
    ///
    /// 获取直播间商品列表
    ///
    /// # Arguments
    ///
    /// * `request` - 直播间商品列表请求参数
    ///
    /// # Returns
    ///
    /// 返回直播间商品列表数据 (使用商品转链响应格式)
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let request = DouyinLiveProductListRequest::new("sid", "buyin123");
    /// let result = client.douyin().live_product_list(request).await?;
    /// ```
    pub async fn live_product_list(
        &self,
        request: DouyinLiveProductListRequest,
    ) -> ZtkResult<DouyinGoodsConvertResponse> {
        let params = build_params_with_appkey(self.client.appkey(), &request)?;
        self.client
            .http_client()
            .get(
                "/api/open_douyin_distributionLive_productlist.ashx",
                &params,
            )
            .await
    }
}

/// 内部使用的编码后请求结构
#[derive(serde::Serialize)]
struct DouyinParseCommandRequestEncoded {
    sid: String,
    command: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_douyin_api_creation() {
        let client = ZtkClient::new("test_appkey").build().unwrap();
        let douyin = client.douyin();
        assert_eq!(douyin.client().appkey(), "test_appkey");
    }
}
