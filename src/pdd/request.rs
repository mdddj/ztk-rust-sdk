//! 拼多多平台请求参数结构体
//!
//! 定义拼多多平台 API 的请求参数结构体

use serde::Serialize;

/// 拼多多转链请求
///
/// 将商品 ID 或推广短链转换为推广链接
///
/// # Example
///
/// ```rust,ignore
/// let request = PddConvertRequest::new("pdd_app_key", "pdd_app_secret", "pid", "453581732819")
///     .custom_parameters(r#"{"uid":"user123"}"#);
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct PddConvertRequest {
    /// 拼多多开放平台应用的 appkey
    pub pdd_app_key: String,
    /// 拼多多开放平台应用的 appsecret
    pub pdd_app_secret: String,
    /// 拼多多平台的推广位
    pub pid: String,
    /// 商品内容，支持纯数字 ID 或拼多多推广短链
    pub content: String,
    /// 自定义参数，为链接打上自定义标签 (可选)
    /// 格式为 JSON: {"uid":"15611448080","sid":""}
    /// 用于返利场景，需要 URL 编码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_parameters: Option<String>,
}

impl PddConvertRequest {
    /// 创建新的拼多多转链请求
    ///
    /// # Arguments
    ///
    /// * `pdd_app_key` - 拼多多开放平台应用的 appkey
    /// * `pdd_app_secret` - 拼多多开放平台应用的 appsecret
    /// * `pid` - 拼多多平台的推广位
    /// * `content` - 商品 ID 或推广短链
    pub fn new(
        pdd_app_key: impl Into<String>,
        pdd_app_secret: impl Into<String>,
        pid: impl Into<String>,
        content: impl Into<String>,
    ) -> Self {
        Self {
            pdd_app_key: pdd_app_key.into(),
            pdd_app_secret: pdd_app_secret.into(),
            pid: pid.into(),
            content: content.into(),
            custom_parameters: None,
        }
    }

    /// 设置自定义参数 (用于返利场景)
    pub fn custom_parameters(mut self, custom_parameters: impl Into<String>) -> Self {
        self.custom_parameters = Some(custom_parameters.into());
        self
    }
}

/// 拼多多商品详情请求 (简版)
///
/// 获取拼多多商品的基本详情信息
///
/// # Example
///
/// ```rust,ignore
/// let request = PddGoodsDetailSimpleRequest::new("pdd_app_key", "pdd_app_secret", "pid", "453581732819");
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct PddGoodsDetailSimpleRequest {
    /// 拼多多开放平台应用的 appkey
    pub pdd_app_key: String,
    /// 拼多多开放平台应用的 appsecret
    pub pdd_app_secret: String,
    /// 拼多多平台的推广位
    pub pid: String,
    /// 商品 ID (纯数字)
    pub content: String,
}

impl PddGoodsDetailSimpleRequest {
    /// 创建新的拼多多商品详情请求 (简版)
    ///
    /// # Arguments
    ///
    /// * `pdd_app_key` - 拼多多开放平台应用的 appkey
    /// * `pdd_app_secret` - 拼多多开放平台应用的 appsecret
    /// * `pid` - 拼多多平台的推广位
    /// * `content` - 商品 ID
    pub fn new(
        pdd_app_key: impl Into<String>,
        pdd_app_secret: impl Into<String>,
        pid: impl Into<String>,
        content: impl Into<String>,
    ) -> Self {
        Self {
            pdd_app_key: pdd_app_key.into(),
            pdd_app_secret: pdd_app_secret.into(),
            pid: pid.into(),
            content: content.into(),
        }
    }
}

/// 拼多多商品详情请求 (详版)
///
/// 获取拼多多商品的详细信息，支持多种筛选条件
///
/// # Example
///
/// ```rust,ignore
/// let request = PddGoodsDetailFullRequest::new("pdd_app_key", "pdd_app_secret", "pid", "453581732819")
///     .custom_parameters(r#"{"uid":"user123"}"#)
///     .sort_type(2);
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct PddGoodsDetailFullRequest {
    /// 拼多多开放平台应用的 appkey
    pub pdd_app_key: String,
    /// 拼多多开放平台应用的 appsecret
    pub pdd_app_secret: String,
    /// 拼多多平台的推广位
    pub pid: String,
    /// 商品关键词，支持 goods_id、拼多多商品链接、推广长链和推广短链
    pub keyword: String,
    /// 自定义参数 (可选，用于返利场景)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_parameters: Option<String>,
    /// 商品类目 ID (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cat_id: Option<String>,
    /// 活动商品标记数组 (可选)
    /// 例: [4,7]，4-秒杀，7-百亿补贴，10851-千万补贴
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_tags: Option<String>,
    /// 屏蔽商品类目包 (可选)
    /// 1-拼多多小程序屏蔽的类目&关键词，2-虚拟类目，3-医疗器械等
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_cat_packages: Option<String>,
    /// 自定义屏蔽类目 ID (可选，最多 20 个)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_cats: Option<String>,
    /// 是否为品牌商品 (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_brand_goods: Option<bool>,
    /// 店铺类型 (可选)
    /// 1-个人，2-企业，3-旗舰店，4-专卖店，5-专营店，6-普通店
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_type: Option<u32>,
    /// 排序方式 (可选)
    /// 0-综合排序，1-按佣金比率升序，2-按佣金比例降序，3-按价格升序，4-按价格降序等
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_type: Option<u32>,
    /// 是否使用个性化推荐 (可选，默认 true)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_customized: Option<bool>,
    /// 是否只返回有优惠券的商品 (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_coupon: Option<bool>,
}

impl PddGoodsDetailFullRequest {
    /// 创建新的拼多多商品详情请求 (详版)
    pub fn new(
        pdd_app_key: impl Into<String>,
        pdd_app_secret: impl Into<String>,
        pid: impl Into<String>,
        keyword: impl Into<String>,
    ) -> Self {
        Self {
            pdd_app_key: pdd_app_key.into(),
            pdd_app_secret: pdd_app_secret.into(),
            pid: pid.into(),
            keyword: keyword.into(),
            custom_parameters: None,
            cat_id: None,
            activity_tags: None,
            block_cat_packages: None,
            block_cats: None,
            is_brand_goods: None,
            merchant_type: None,
            sort_type: None,
            use_customized: None,
            with_coupon: None,
        }
    }

    /// 设置自定义参数
    pub fn custom_parameters(mut self, custom_parameters: impl Into<String>) -> Self {
        self.custom_parameters = Some(custom_parameters.into());
        self
    }

    /// 设置商品类目 ID
    pub fn cat_id(mut self, cat_id: impl Into<String>) -> Self {
        self.cat_id = Some(cat_id.into());
        self
    }

    /// 设置活动商品标记
    pub fn activity_tags(mut self, activity_tags: impl Into<String>) -> Self {
        self.activity_tags = Some(activity_tags.into());
        self
    }

    /// 设置屏蔽商品类目包
    pub fn block_cat_packages(mut self, block_cat_packages: impl Into<String>) -> Self {
        self.block_cat_packages = Some(block_cat_packages.into());
        self
    }

    /// 设置自定义屏蔽类目
    pub fn block_cats(mut self, block_cats: impl Into<String>) -> Self {
        self.block_cats = Some(block_cats.into());
        self
    }

    /// 设置是否为品牌商品
    pub fn is_brand_goods(mut self, is_brand_goods: bool) -> Self {
        self.is_brand_goods = Some(is_brand_goods);
        self
    }

    /// 设置店铺类型
    pub fn merchant_type(mut self, merchant_type: u32) -> Self {
        self.merchant_type = Some(merchant_type);
        self
    }

    /// 设置排序方式
    pub fn sort_type(mut self, sort_type: u32) -> Self {
        self.sort_type = Some(sort_type);
        self
    }

    /// 设置是否使用个性化推荐
    pub fn use_customized(mut self, use_customized: bool) -> Self {
        self.use_customized = Some(use_customized);
        self
    }

    /// 设置是否只返回有优惠券的商品
    pub fn with_coupon(mut self, with_coupon: bool) -> Self {
        self.with_coupon = Some(with_coupon);
        self
    }
}

/// 拼多多订单查询请求
///
/// 查询拼多多推广订单
///
/// # Example
///
/// ```rust,ignore
/// let request = PddOrderQueryRequest::new("pdd_app_key", "pdd_app_secret", 1699200000, 1699286400)
///     .page(1)
///     .page_size(50);
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct PddOrderQueryRequest {
    /// 拼多多开放平台应用的 appkey
    pub pdd_app_key: String,
    /// 拼多多开放平台应用的 appsecret
    pub pdd_app_secret: String,
    /// 查询开始时间 (10 位秒级时间戳)
    pub start_update_time: i64,
    /// 查询结束时间 (10 位秒级时间戳，与开始时间相差不超过 24 小时)
    pub end_update_time: i64,
    /// 订单类型 (可选): 1-推广订单，2-直播间订单
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_order_type: Option<u32>,
    /// 页码 (可选，从 1 到 10000，默认 1)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    /// 每页数量 (可选，默认 100，范围 10-100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<u32>,
    /// 是否为礼金订单 (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cash_gift_order: Option<bool>,
}

impl PddOrderQueryRequest {
    /// 创建新的拼多多订单查询请求
    ///
    /// # Arguments
    ///
    /// * `pdd_app_key` - 拼多多开放平台应用的 appkey
    /// * `pdd_app_secret` - 拼多多开放平台应用的 appsecret
    /// * `start_update_time` - 查询开始时间 (10 位秒级时间戳)
    /// * `end_update_time` - 查询结束时间 (10 位秒级时间戳)
    pub fn new(
        pdd_app_key: impl Into<String>,
        pdd_app_secret: impl Into<String>,
        start_update_time: i64,
        end_update_time: i64,
    ) -> Self {
        Self {
            pdd_app_key: pdd_app_key.into(),
            pdd_app_secret: pdd_app_secret.into(),
            start_update_time,
            end_update_time,
            query_order_type: None,
            page: None,
            page_size: None,
            cash_gift_order: None,
        }
    }

    /// 设置订单类型
    pub fn query_order_type(mut self, query_order_type: u32) -> Self {
        self.query_order_type = Some(query_order_type);
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

    /// 设置是否为礼金订单
    pub fn cash_gift_order(mut self, cash_gift_order: bool) -> Self {
        self.cash_gift_order = Some(cash_gift_order);
        self
    }
}

/// 拼多多授权备案请求
///
/// 获取授权备案 URL，用于在服务器上进行 PID 授权备案
///
/// # Example
///
/// ```rust,ignore
/// let request = PddAuthorizeRequest::new("pdd_app_key", "pdd_app_secret", "pid");
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct PddAuthorizeRequest {
    /// 拼多多开放平台应用的 appkey
    pub pdd_app_key: String,
    /// 拼多多开放平台应用的 appsecret
    pub pdd_app_secret: String,
    /// 拼多多平台的推广位
    pub pid: String,
    /// 自定义参数 (可选，用于返利场景)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_parameters: Option<String>,
    /// 是否生成 QQ 小程序 (可选，默认 false)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generate_qq_app: Option<bool>,
    /// 是否生成微信小程序推广信息 (可选，默认 false)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generate_we_app: Option<bool>,
}

impl PddAuthorizeRequest {
    /// 创建新的拼多多授权备案请求
    ///
    /// # Arguments
    ///
    /// * `pdd_app_key` - 拼多多开放平台应用的 appkey
    /// * `pdd_app_secret` - 拼多多开放平台应用的 appsecret
    /// * `pid` - 拼多多平台的推广位
    pub fn new(
        pdd_app_key: impl Into<String>,
        pdd_app_secret: impl Into<String>,
        pid: impl Into<String>,
    ) -> Self {
        Self {
            pdd_app_key: pdd_app_key.into(),
            pdd_app_secret: pdd_app_secret.into(),
            pid: pid.into(),
            custom_parameters: None,
            generate_qq_app: None,
            generate_we_app: None,
        }
    }

    /// 设置自定义参数
    pub fn custom_parameters(mut self, custom_parameters: impl Into<String>) -> Self {
        self.custom_parameters = Some(custom_parameters.into());
        self
    }

    /// 设置是否生成 QQ 小程序
    pub fn generate_qq_app(mut self, generate_qq_app: bool) -> Self {
        self.generate_qq_app = Some(generate_qq_app);
        self
    }

    /// 设置是否生成微信小程序推广信息
    pub fn generate_we_app(mut self, generate_we_app: bool) -> Self {
        self.generate_we_app = Some(generate_we_app);
        self
    }
}

/// 拼多多授权备案查询请求
///
/// 查询 PID 授权备案状态
///
/// # Example
///
/// ```rust,ignore
/// let request = PddAuthorizeQueryRequest::new("pdd_app_key", "pdd_app_secret", "pid");
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct PddAuthorizeQueryRequest {
    /// 拼多多开放平台应用的 appkey
    pub pdd_app_key: String,
    /// 拼多多开放平台应用的 appsecret
    pub pdd_app_secret: String,
    /// 拼多多平台的推广位
    pub pid: String,
    /// 自定义参数 (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_parameters: Option<String>,
}

impl PddAuthorizeQueryRequest {
    /// 创建新的拼多多授权备案查询请求
    pub fn new(
        pdd_app_key: impl Into<String>,
        pdd_app_secret: impl Into<String>,
        pid: impl Into<String>,
    ) -> Self {
        Self {
            pdd_app_key: pdd_app_key.into(),
            pdd_app_secret: pdd_app_secret.into(),
            pid: pid.into(),
            custom_parameters: None,
        }
    }

    /// 设置自定义参数
    pub fn custom_parameters(mut self, custom_parameters: impl Into<String>) -> Self {
        self.custom_parameters = Some(custom_parameters.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pdd_convert_request_serialize() {
        let request = PddConvertRequest::new("app_key", "app_secret", "pid123", "453581732819");
        let json = serde_json::to_value(&request).unwrap();

        assert_eq!(json["pdd_app_key"], "app_key");
        assert_eq!(json["pdd_app_secret"], "app_secret");
        assert_eq!(json["pid"], "pid123");
        assert_eq!(json["content"], "453581732819");
        assert!(json.get("custom_parameters").is_none());
    }

    #[test]
    fn test_pdd_convert_request_with_custom_parameters() {
        let request = PddConvertRequest::new("app_key", "app_secret", "pid123", "453581732819")
            .custom_parameters(r#"{"uid":"user123"}"#);

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["custom_parameters"], r#"{"uid":"user123"}"#);
    }

    #[test]
    fn test_pdd_goods_detail_simple_request_serialize() {
        let request =
            PddGoodsDetailSimpleRequest::new("app_key", "app_secret", "pid123", "453581732819");
        let json = serde_json::to_value(&request).unwrap();

        assert_eq!(json["pdd_app_key"], "app_key");
        assert_eq!(json["content"], "453581732819");
    }

    #[test]
    fn test_pdd_goods_detail_full_request_serialize() {
        let request =
            PddGoodsDetailFullRequest::new("app_key", "app_secret", "pid123", "453581732819")
                .sort_type(2)
                .merchant_type(3)
                .with_coupon(true);

        let json = serde_json::to_value(&request).unwrap();

        assert_eq!(json["keyword"], "453581732819");
        assert_eq!(json["sort_type"], 2);
        assert_eq!(json["merchant_type"], 3);
        assert_eq!(json["with_coupon"], true);
    }

    #[test]
    fn test_pdd_order_query_request_serialize() {
        let request = PddOrderQueryRequest::new("app_key", "app_secret", 1699200000, 1699286400)
            .page(1)
            .page_size(50);

        let json = serde_json::to_value(&request).unwrap();

        assert_eq!(json["start_update_time"], 1699200000);
        assert_eq!(json["end_update_time"], 1699286400);
        assert_eq!(json["page"], 1);
        assert_eq!(json["page_size"], 50);
    }

    #[test]
    fn test_pdd_authorize_request_serialize() {
        let request =
            PddAuthorizeRequest::new("app_key", "app_secret", "pid123").generate_we_app(true);

        let json = serde_json::to_value(&request).unwrap();

        assert_eq!(json["pdd_app_key"], "app_key");
        assert_eq!(json["pid"], "pid123");
        assert_eq!(json["generate_we_app"], true);
    }

    #[test]
    fn test_optional_fields_not_serialized() {
        let request = PddConvertRequest::new("app_key", "app_secret", "pid", "content");
        let json_str = serde_json::to_string(&request).unwrap();

        assert!(!json_str.contains("custom_parameters"));
    }
}
