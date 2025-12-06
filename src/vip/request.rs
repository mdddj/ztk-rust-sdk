//! 唯品会平台请求参数结构体
//!
//! 定义唯品会平台 API 的请求参数结构体

use serde::Serialize;

/// 唯品会转链请求
///
/// 根据唯品会商品 ID 或链接生成联盟推广链接
///
/// # Example
///
/// ```rust,ignore
/// let request = VipConvertRequest::new("sid", "6919173790682972930")
///     .chan_tag("your_pid")
///     .stat_param("custom_param");
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct VipConvertRequest {
    /// 唯品会账号授权 SID
    pub sid: String,
    /// 唯品会商品 ID 或链接
    /// 商品 ID 例子: 6919173790682972930
    /// 链接例子: https://t.vip.com/URKiMYwbrW6
    pub url: String,
    /// 渠道标识，即推广位 PID (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "chanTag")]
    pub chan_tag: Option<String>,
    /// 自定义渠道统计参数 (可选，最大长度 256 字符)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "statParam")]
    pub stat_param: Option<String>,
}

impl VipConvertRequest {
    /// 创建新的唯品会转链请求
    ///
    /// # Arguments
    ///
    /// * `sid` - 唯品会账号授权 SID
    /// * `url` - 唯品会商品 ID 或链接
    pub fn new(sid: impl Into<String>, url: impl Into<String>) -> Self {
        Self {
            sid: sid.into(),
            url: url.into(),
            chan_tag: None,
            stat_param: None,
        }
    }

    /// 设置渠道标识 (推广位 PID)
    pub fn chan_tag(mut self, chan_tag: impl Into<String>) -> Self {
        self.chan_tag = Some(chan_tag.into());
        self
    }

    /// 设置自定义渠道统计参数
    pub fn stat_param(mut self, stat_param: impl Into<String>) -> Self {
        self.stat_param = Some(stat_param.into());
        self
    }
}

/// 唯品会账号授权请求
///
/// 获取唯品会账号授权 URL
///
/// # Example
///
/// ```rust,ignore
/// let request = VipAuthorizeRequest::new("https://www.example.com/callback");
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct VipAuthorizeRequest {
    /// 授权回调地址
    /// 唯品会账号授权后，将授权信息返回至此回调地址
    /// 回调地址必须以 http:// 或 https:// 开头
    pub backurl: String,
}

impl VipAuthorizeRequest {
    /// 创建新的唯品会账号授权请求
    ///
    /// # Arguments
    ///
    /// * `backurl` - 授权回调地址
    pub fn new(backurl: impl Into<String>) -> Self {
        Self {
            backurl: backurl.into(),
        }
    }
}

/// 唯品会订单查询请求
///
/// 查询唯品会推广订单列表
///
/// # Example
///
/// ```rust,ignore
/// let request = VipOrderQueryRequest::new("sid")
///     .page(1)
///     .page_size(20)
///     .status(2);
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct VipOrderQueryRequest {
    /// 唯品会账号授权 SID
    pub sid: String,
    /// 订单状态 (可选): 0-不合格，1-待定，2-已完结
    /// 不设置默认代表全部状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<u32>,
    /// 订单时间起始 (可选，13 位毫秒级时间戳)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "orderTimeStart")]
    pub order_time_start: Option<i64>,
    /// 订单时间结束 (可选，13 位毫秒级时间戳)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "orderTimeEnd")]
    pub order_time_end: Option<i64>,
    /// 更新时间起始 (可选，13 位毫秒级时间戳)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "updateTimeStart")]
    pub update_time_start: Option<i64>,
    /// 更新时间结束 (可选，13 位毫秒级时间戳)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "updateTimeEnd")]
    pub update_time_end: Option<i64>,
    /// 页码 (可选，从 1 开始)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    /// 每页数量 (可选，默认 20)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "pageSize")]
    pub page_size: Option<u32>,
    /// 订单号列表 (可选，多个订单号用英文逗号分隔)
    /// 传入订单号列表时，订单时间和更新时间区间可不传入
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "orderSnList")]
    pub order_sn_list: Option<String>,
    /// 渠道标识，即推广位 PID (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "chanTag")]
    pub chan_tag: Option<String>,
}

impl VipOrderQueryRequest {
    /// 创建新的唯品会订单查询请求
    ///
    /// # Arguments
    ///
    /// * `sid` - 唯品会账号授权 SID
    pub fn new(sid: impl Into<String>) -> Self {
        Self {
            sid: sid.into(),
            status: None,
            order_time_start: None,
            order_time_end: None,
            update_time_start: None,
            update_time_end: None,
            page: None,
            page_size: None,
            order_sn_list: None,
            chan_tag: None,
        }
    }

    /// 设置订单状态
    pub fn status(mut self, status: u32) -> Self {
        self.status = Some(status);
        self
    }

    /// 设置订单时间范围
    pub fn order_time_range(mut self, start: i64, end: i64) -> Self {
        self.order_time_start = Some(start);
        self.order_time_end = Some(end);
        self
    }

    /// 设置更新时间范围
    pub fn update_time_range(mut self, start: i64, end: i64) -> Self {
        self.update_time_start = Some(start);
        self.update_time_end = Some(end);
        self
    }

    /// 设置页码
    pub fn page(mut self, page: u32) -> Self {
        self.page = Some(page);
        self
    }

    /// 设置每页数量
    pub fn page_size(mut self, page_size: u32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置订单号列表
    pub fn order_sn_list(mut self, order_sn_list: impl Into<String>) -> Self {
        self.order_sn_list = Some(order_sn_list.into());
        self
    }

    /// 设置渠道标识
    pub fn chan_tag(mut self, chan_tag: impl Into<String>) -> Self {
        self.chan_tag = Some(chan_tag.into());
        self
    }
}

/// 唯品会商品详情请求 (V2 版本)
///
/// 根据商品 ID 或链接获取商品详情信息
///
/// # Example
///
/// ```rust,ignore
/// let request = VipGoodsDetailRequest::new("sid", "6919173790682972930")
///     .query_detail(true)
///     .extend_sku(true);
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct VipGoodsDetailRequest {
    /// 唯品会账号授权 SID
    pub sid: String,
    /// 唯品会商品 ID 或链接
    pub id: String,
    /// 是否查询详情信息 (可选，默认 false)
    /// 设为 true 将查询出商品轮播图和详情图
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "queryDetail")]
    pub query_detail: Option<bool>,
    /// 是否查询商品库存状态 (可选，默认 false)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "queryStock")]
    pub query_stock: Option<bool>,
    /// 是否查询商品评价信息 (可选，默认 false)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "queryReputation")]
    pub query_reputation: Option<bool>,
    /// 是否查询商品所属店铺服务能力信息 (可选，默认 false)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "queryStoreServiceCapability")]
    pub query_store_service_capability: Option<bool>,
    /// 是否查询商品活动信息 (可选，默认 false)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "queryPMSAct")]
    pub query_pms_act: Option<bool>,
    /// 是否按照同 SPU 扩展 (可选，默认 false)
    /// 设为 true 将根据传入的商品 ID 扩展查询出同 SPU 下的所有商品 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "extendBySpu")]
    pub extend_by_spu: Option<bool>,
    /// 是否查询渠道专属红包信息 (可选，默认不查询)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "queryExclusiveCoupon")]
    pub query_exclusive_coupon: Option<bool>,
    /// 是否扩展查询商品 SKU 信息 (可选，默认 false)
    /// 设为 true 将根据传入的商品 ID 扩展查询出对应的 SKU 信息
    /// 注意: 此参数查询较耗时，建议单个查询
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "extendSku")]
    pub extend_sku: Option<bool>,
}

impl VipGoodsDetailRequest {
    /// 创建新的唯品会商品详情请求
    ///
    /// # Arguments
    ///
    /// * `sid` - 唯品会账号授权 SID
    /// * `id` - 唯品会商品 ID 或链接
    pub fn new(sid: impl Into<String>, id: impl Into<String>) -> Self {
        Self {
            sid: sid.into(),
            id: id.into(),
            query_detail: None,
            query_stock: None,
            query_reputation: None,
            query_store_service_capability: None,
            query_pms_act: None,
            extend_by_spu: None,
            query_exclusive_coupon: None,
            extend_sku: None,
        }
    }

    /// 设置是否查询详情信息
    pub fn query_detail(mut self, query_detail: bool) -> Self {
        self.query_detail = Some(query_detail);
        self
    }

    /// 设置是否查询库存状态
    pub fn query_stock(mut self, query_stock: bool) -> Self {
        self.query_stock = Some(query_stock);
        self
    }

    /// 设置是否查询评价信息
    pub fn query_reputation(mut self, query_reputation: bool) -> Self {
        self.query_reputation = Some(query_reputation);
        self
    }

    /// 设置是否查询店铺服务能力信息
    pub fn query_store_service_capability(mut self, query: bool) -> Self {
        self.query_store_service_capability = Some(query);
        self
    }

    /// 设置是否查询活动信息
    pub fn query_pms_act(mut self, query_pms_act: bool) -> Self {
        self.query_pms_act = Some(query_pms_act);
        self
    }

    /// 设置是否按同 SPU 扩展
    pub fn extend_by_spu(mut self, extend_by_spu: bool) -> Self {
        self.extend_by_spu = Some(extend_by_spu);
        self
    }

    /// 设置是否查询渠道专属红包信息
    pub fn query_exclusive_coupon(mut self, query: bool) -> Self {
        self.query_exclusive_coupon = Some(query);
        self
    }

    /// 设置是否扩展查询 SKU 信息
    pub fn extend_sku(mut self, extend_sku: bool) -> Self {
        self.extend_sku = Some(extend_sku);
        self
    }
}

/// 唯品会商品搜索请求
///
/// 根据关键词搜索唯品会商品列表
///
/// # Example
///
/// ```rust,ignore
/// let request = VipSearchGoodsRequest::new("sid", "球鞋")
///     .page(1)
///     .page_size(20)
///     .field_name(VipSortField::Price)
///     .order(0);
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct VipSearchGoodsRequest {
    /// 唯品会账号授权 SID
    pub sid: String,
    /// 搜索关键词
    pub keyword: String,
    /// 排序字段 (可选): PRICE-价格，DISCOUNT-折扣，SALES-销量
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "fieldName")]
    pub field_name: Option<String>,
    /// 排序顺序 (可选): 0-正序，1-逆序，默认正序
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<u32>,
    /// 页码 (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    /// 每页数量 (可选，默认 20，范围 1-50)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "pageSize")]
    pub page_size: Option<u32>,
    /// 价格区间起始 (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "priceStart")]
    pub price_start: Option<String>,
    /// 价格区间结束 (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "priceEnd")]
    pub price_end: Option<String>,
}

impl VipSearchGoodsRequest {
    /// 创建新的唯品会商品搜索请求
    ///
    /// # Arguments
    ///
    /// * `sid` - 唯品会账号授权 SID
    /// * `keyword` - 搜索关键词
    pub fn new(sid: impl Into<String>, keyword: impl Into<String>) -> Self {
        Self {
            sid: sid.into(),
            keyword: keyword.into(),
            field_name: None,
            order: None,
            page: None,
            page_size: None,
            price_start: None,
            price_end: None,
        }
    }

    /// 设置排序字段
    /// PRICE-价格，DISCOUNT-折扣，SALES-销量
    pub fn field_name(mut self, field_name: impl Into<String>) -> Self {
        self.field_name = Some(field_name.into());
        self
    }

    /// 设置排序顺序: 0-正序，1-逆序
    pub fn order(mut self, order: u32) -> Self {
        self.order = Some(order);
        self
    }

    /// 设置页码
    pub fn page(mut self, page: u32) -> Self {
        self.page = Some(page);
        self
    }

    /// 设置每页数量
    pub fn page_size(mut self, page_size: u32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置价格区间
    pub fn price_range(mut self, start: impl Into<String>, end: impl Into<String>) -> Self {
        self.price_start = Some(start.into());
        self.price_end = Some(end.into());
        self
    }
}

/// 唯品会授权列表查询请求
///
/// 获取唯品会账户授权列表
///
/// # Example
///
/// ```rust,ignore
/// let request = VipAuthorizeListRequest::new()
///     .page(1)
///     .expire_day(3);
/// ```
#[derive(Debug, Clone, Serialize, Default)]
pub struct VipAuthorizeListRequest {
    /// 页码 (可选，每页最多返回 100 个)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    /// 还剩下几天授权过期 (可选，取值范围 0-7)
    /// 比如值为 3，表示获取还剩下 3 天授权过期的账号信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_day: Option<u32>,
    /// 唯品会账号授权 SID (可选，查询指定账号)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,
}

impl VipAuthorizeListRequest {
    /// 创建新的唯品会授权列表查询请求
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置页码
    pub fn page(mut self, page: u32) -> Self {
        self.page = Some(page);
        self
    }

    /// 设置过期天数筛选
    pub fn expire_day(mut self, expire_day: u32) -> Self {
        self.expire_day = Some(expire_day);
        self
    }

    /// 设置指定 SID 查询
    pub fn sid(mut self, sid: impl Into<String>) -> Self {
        self.sid = Some(sid.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vip_convert_request_serialize() {
        let request = VipConvertRequest::new("sid123", "6919173790682972930");
        let json = serde_json::to_value(&request).unwrap();

        assert_eq!(json["sid"], "sid123");
        assert_eq!(json["url"], "6919173790682972930");
        assert!(json.get("chanTag").is_none());
    }

    #[test]
    fn test_vip_convert_request_with_options() {
        let request = VipConvertRequest::new("sid123", "6919173790682972930")
            .chan_tag("pid123")
            .stat_param("custom_param");

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["chanTag"], "pid123");
        assert_eq!(json["statParam"], "custom_param");
    }

    #[test]
    fn test_vip_authorize_request_serialize() {
        let request = VipAuthorizeRequest::new("https://www.example.com/callback");
        let json = serde_json::to_value(&request).unwrap();

        assert_eq!(json["backurl"], "https://www.example.com/callback");
    }

    #[test]
    fn test_vip_order_query_request_serialize() {
        let request = VipOrderQueryRequest::new("sid123")
            .page(1)
            .page_size(20)
            .status(2);

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["sid"], "sid123");
        assert_eq!(json["page"], 1);
        assert_eq!(json["pageSize"], 20);
        assert_eq!(json["status"], 2);
    }

    #[test]
    fn test_vip_goods_detail_request_serialize() {
        let request = VipGoodsDetailRequest::new("sid123", "6919173790682972930")
            .query_detail(true)
            .extend_sku(true);

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["sid"], "sid123");
        assert_eq!(json["id"], "6919173790682972930");
        assert_eq!(json["queryDetail"], true);
        assert_eq!(json["extendSku"], true);
    }

    #[test]
    fn test_vip_search_goods_request_serialize() {
        let request = VipSearchGoodsRequest::new("sid123", "球鞋")
            .page(1)
            .page_size(20)
            .field_name("PRICE")
            .order(0);

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["sid"], "sid123");
        assert_eq!(json["keyword"], "球鞋");
        assert_eq!(json["page"], 1);
        assert_eq!(json["pageSize"], 20);
        assert_eq!(json["fieldName"], "PRICE");
        assert_eq!(json["order"], 0);
    }

    #[test]
    fn test_vip_authorize_list_request_serialize() {
        let request = VipAuthorizeListRequest::new().page(1).expire_day(3);

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["page"], 1);
        assert_eq!(json["expire_day"], 3);
    }

    #[test]
    fn test_optional_fields_not_serialized() {
        let request = VipConvertRequest::new("sid", "url");
        let json_str = serde_json::to_string(&request).unwrap();

        assert!(!json_str.contains("chanTag"));
        assert!(!json_str.contains("statParam"));
    }
}
