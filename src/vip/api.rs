//! 唯品会平台 API 实现
//!
//! 提供唯品会平台相关的 API 调用功能，包括：
//! - 商品转链
//! - 账号授权
//! - 订单查询
//! - 商品详情
//! - 商品搜索

use crate::client::ZtkClient;
use crate::common::http::build_params_with_appkey;
use crate::error::ZtkResult;

use super::request::{
    VipAuthorizeListRequest, VipAuthorizeRequest, VipConvertRequest, VipGoodsDetailRequest,
    VipOrderQueryRequest, VipSearchGoodsRequest,
};
use super::response::{
    VipAuthorizeListResponse, VipAuthorizeResponse, VipConvertResponse, VipGoodsDetailResponse,
    VipOrderResponse, VipSearchGoodsResponse,
};

/// 唯品会平台 API
pub struct VipApi<'a> {
    client: &'a ZtkClient,
}

impl<'a> VipApi<'a> {
    /// 创建唯品会 API 实例
    pub fn new(client: &'a ZtkClient) -> Self {
        Self { client }
    }

    /// 获取客户端引用
    #[allow(dead_code)]
    pub(crate) fn client(&self) -> &ZtkClient {
        self.client
    }

    /// 唯品会转链
    pub async fn convert(&self, request: VipConvertRequest) -> ZtkResult<VipConvertResponse> {
        let params = build_params_with_appkey(self.client.appkey(), &request)?;
        self.client
            .http_client()
            .get("/api/open_vip_genByVIPUrlWithOauth.ashx", &params)
            .await
    }

    /// 唯品会账号授权
    pub async fn authorize(&self, request: VipAuthorizeRequest) -> ZtkResult<VipAuthorizeResponse> {
        let params = build_params_with_appkey(self.client.appkey(), &request)?;
        self.client
            .http_client()
            .get("/api/vip_shouquan.ashx", &params)
            .await
    }

    /// 唯品会授权列表查询
    pub async fn authorize_list(
        &self,
        request: VipAuthorizeListRequest,
    ) -> ZtkResult<VipAuthorizeListResponse> {
        let params = build_params_with_appkey(self.client.appkey(), &request)?;
        self.client
            .http_client()
            .get("/api/vip_shouquaninfo.ashx", &params)
            .await
    }

    /// 唯品会订单查询
    pub async fn query_orders(&self, request: VipOrderQueryRequest) -> ZtkResult<VipOrderResponse> {
        let params = build_params_with_appkey(self.client.appkey(), &request)?;
        self.client
            .http_client()
            .get("/api/open_vip_orderListWithOauth.ashx", &params)
            .await
    }

    /// 唯品会商品详情
    pub async fn goods_detail(
        &self,
        request: VipGoodsDetailRequest,
    ) -> ZtkResult<VipGoodsDetailResponse> {
        let params = build_params_with_appkey(self.client.appkey(), &request)?;
        self.client
            .http_client()
            .get("/api/open_vip_getByGoodsIdsV2WithOauth.ashx", &params)
            .await
    }

    /// 唯品会商品搜索
    pub async fn search_goods(
        &self,
        request: VipSearchGoodsRequest,
    ) -> ZtkResult<VipSearchGoodsResponse> {
        let params = build_params_with_appkey(self.client.appkey(), &request)?;
        self.client
            .http_client()
            .get("/api/open_vip_queryWithOauth.ashx", &params)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vip_api_creation() {
        let client = ZtkClient::new("test_appkey").build().unwrap();
        let vip = client.vip();
        assert_eq!(vip.client().appkey(), "test_appkey");
    }
}
