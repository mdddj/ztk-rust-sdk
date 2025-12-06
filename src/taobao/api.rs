//! 淘宝平台 API 实现
//!
//! 提供淘宝平台相关的 API 调用功能，包括：
//! - 高佣转链 (商品ID/淘口令)
//! - 批量转链
//! - 订单查询
//! - 淘口令生成和解析

use crate::client::ZtkClient;
use crate::common::http::{build_params_with_appkey, url_encode};
use crate::error::ZtkResult;

use super::request::{
    BatchConvertRequest, ConvertByItemIdRequest, ConvertByTklRequest, CreateTklRequest,
    ParseItemIdRequest, QueryOrdersRequest, SearchGoodsRequest,
};
use super::response::{
    BatchConvertResponse, ConvertByTklResponse, ConvertResponse, CreateTklResponse,
    ParseItemIdResponse, QueryOrdersResponse, SearchGoodsResponse,
};

/// 淘宝平台 API
///
/// 提供淘宝平台相关的 API 调用方法
///
/// # Example
///
/// ```rust,ignore
/// use ztk_rust_sdk::ZtkClient;
/// use ztk_rust_sdk::taobao::{ConvertByItemIdRequest, TaobaoSignUrlType};
///
/// #[tokio::main]
/// async fn main() -> ztk_rust_sdk::ZtkResult<()> {
///     let client = ZtkClient::new("your_appkey").build()?;
///     
///     let request = ConvertByItemIdRequest::new("sid", "mm_xxx_xxx_xxx", "123456789")
///         .signurl(TaobaoSignUrlType::WithFullDetail);
///     
///     let result = client.taobao().convert_by_item_id(request).await?;
///     println!("转链结果: {:?}", result);
///     
///     Ok(())
/// }
/// ```
pub struct TaobaoApi<'a> {
    /// 客户端引用
    client: &'a ZtkClient,
}

impl<'a> TaobaoApi<'a> {
    /// 创建淘宝 API 实例
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

    /// 高佣转链 (商品ID)
    ///
    /// 将商品 ID 转换为带有高佣金的推广链接
    ///
    /// # Arguments
    ///
    /// * `request` - 转链请求参数
    ///
    /// # Returns
    ///
    /// 返回转链结果，包含推广链接和淘口令
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let request = ConvertByItemIdRequest::new("sid", "mm_xxx_xxx_xxx", "123456789");
    /// let result = client.taobao().convert_by_item_id(request).await?;
    /// ```
    pub async fn convert_by_item_id(
        &self,
        request: ConvertByItemIdRequest,
    ) -> ZtkResult<ConvertResponse> {
        let params = build_params_with_appkey(self.client.appkey(), &request)?;
        self.client
            .http_client()
            .get("/api/open_gaoyongzhuanlian.ashx", &params)
            .await
    }

    /// 高佣转链 (淘口令)
    ///
    /// 将淘口令转换为自己的推广淘口令，自动对淘口令进行 URL 编码
    ///
    /// # Arguments
    ///
    /// * `request` - 淘口令转链请求参数
    ///
    /// # Returns
    ///
    /// 返回转链结果，包含转换后的文案和推广信息
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let request = ConvertByTklRequest::new("sid", "mm_xxx_xxx_xxx", "淘口令内容");
    /// let result = client.taobao().convert_by_tkl(request).await?;
    /// ```
    pub async fn convert_by_tkl(
        &self,
        request: ConvertByTklRequest,
    ) -> ZtkResult<ConvertByTklResponse> {
        // 对淘口令进行 URL 编码
        let mut encoded_request = request.clone();
        encoded_request.tkl = url_encode(&request.tkl);

        let params = build_params_with_appkey(self.client.appkey(), &encoded_request)?;
        self.client
            .http_client()
            .get("/api/open_tkljm.ashx", &params)
            .await
    }

    /// 批量高佣转链
    ///
    /// 同时获取多个渠道 ID 的转链结果
    ///
    /// # Arguments
    ///
    /// * `request` - 批量转链请求参数
    ///
    /// # Returns
    ///
    /// 返回批量转链结果列表
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let request = BatchConvertRequest::new("sid", "mm_xxx_xxx_xxx", "123456789")
    ///     .relation_ids("rel1,rel2,rel3");
    /// let result = client.taobao().batch_convert(request).await?;
    /// ```
    pub async fn batch_convert(
        &self,
        request: BatchConvertRequest,
    ) -> ZtkResult<BatchConvertResponse> {
        let params = build_params_with_appkey(self.client.appkey(), &request)?;
        self.client
            .http_client()
            .get("/api/open_gaoyongzhuanlian_batch.ashx", &params)
            .await
    }

    /// 订单查询
    ///
    /// 查询淘宝联盟订单
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
    /// let request = QueryOrdersRequest::new("sid", "2024-01-01 00:00:00")
    ///     .end_time("2024-01-31 23:59:59")
    ///     .page_no(1)
    ///     .page_size(20);
    /// let result = client.taobao().query_orders(request).await?;
    /// ```
    pub async fn query_orders(
        &self,
        request: QueryOrdersRequest,
    ) -> ZtkResult<QueryOrdersResponse> {
        let params = build_params_with_appkey(self.client.appkey(), &request)?;
        self.client
            .http_client()
            .get("/api/open_dingdanchaxun.ashx", &params)
            .await
    }

    /// 创建淘口令
    ///
    /// 生成淘口令
    ///
    /// # Arguments
    ///
    /// * `request` - 创建淘口令请求参数
    ///
    /// # Returns
    ///
    /// 返回生成的淘口令
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let request = CreateTklRequest::new("sid", "mm_xxx_xxx_xxx")
    ///     .num_iid("123456789")
    ///     .text("推荐好物");
    /// let result = client.taobao().create_tkl(request).await?;
    /// ```
    pub async fn create_tkl(&self, request: CreateTklRequest) -> ZtkResult<CreateTklResponse> {
        let params = build_params_with_appkey(self.client.appkey(), &request)?;
        self.client
            .http_client()
            .get("/api/open_taokoujian.ashx", &params)
            .await
    }

    /// 解析商品编号
    ///
    /// 从淘口令或链接中提取商品 ID
    ///
    /// # Arguments
    ///
    /// * `request` - 解析请求参数
    ///
    /// # Returns
    ///
    /// 返回解析出的商品 ID 和基本信息
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let request = ParseItemIdRequest::new("https://item.taobao.com/item.htm?id=123456");
    /// let result = client.taobao().parse_item_id(request).await?;
    /// ```
    pub async fn parse_item_id(
        &self,
        request: ParseItemIdRequest,
    ) -> ZtkResult<ParseItemIdResponse> {
        // 对内容进行 URL 编码
        let encoded_content = url_encode(&request.content);
        let params = build_params_with_appkey(
            self.client.appkey(),
            &ParseItemIdRequestEncoded {
                content: encoded_content,
            },
        )?;
        self.client
            .http_client()
            .get("/api/open_shangpinbianhao.ashx", &params)
            .await
    }

    /// 全网搜索商品
    ///
    /// 根据关键字搜索商品，返回动态描述分≥4.6的商品列表
    ///
    /// # Arguments
    ///
    /// * `request` - 搜索请求参数
    ///
    /// # Returns
    ///
    /// 返回商品列表
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let request = SearchGoodsRequest::new("sid", "mm_xxx_xxx_xxx")
    ///     .keyword("内存条")
    ///     .page(1)
    ///     .page_size(20)
    ///     .sort("sale_num_desc");
    /// let result = client.taobao().search_goods(request).await?;
    /// ```
    pub async fn search_goods(
        &self,
        request: SearchGoodsRequest,
    ) -> ZtkResult<SearchGoodsResponse> {
        // 对关键词进行 URL 编码
        let mut encoded_request = request.clone();
        if let Some(ref q) = request.q {
            encoded_request.q = Some(url_encode(q));
        }

        let params = build_params_with_appkey(self.client.appkey(), &encoded_request)?;
        // 全网搜索使用不同的端口 10003
        self.client
            .http_client()
            .get_with_base_url(
                "https://api.zhetaoke.com:10003",
                "/api/api_quanwang.ashx",
                &params,
            )
            .await
    }
}

/// 内部使用的编码后请求结构
#[derive(serde::Serialize)]
struct ParseItemIdRequestEncoded {
    content: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_taobao_api_creation() {
        let client = ZtkClient::new("test_appkey").build().unwrap();
        let taobao = client.taobao();
        assert_eq!(taobao.client().appkey(), "test_appkey");
    }
}
