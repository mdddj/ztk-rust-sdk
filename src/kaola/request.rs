//! 考拉平台请求参数结构体
//!
//! 定义考拉平台 API 的请求参数结构体

use serde::Serialize;

/// 考拉转链请求
///
/// 根据考拉商品 ID 或链接生成推广链接
///
/// # Example
///
/// ```rust,ignore
/// let request = KaolaConvertRequest::new("sid", "6919173790682972930")
///     .tracking_code2("12345");
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct KaolaConvertRequest {
    /// 考拉账号授权 SID
    pub sid: String,
    /// 考拉商品 ID 或链接
    /// 商品 ID 例子: 6919173790682972930
    /// 链接例子: http://lu.kaola.com/5DiMFU
    #[serde(rename = "targetUrl")]
    pub target_url: String,
    /// 自定义参数 (可选)
    /// 只允许数字，最大长度 11 位
    /// 此字段可以用来区分用户，进行下级返利
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "trackingCode2")]
    pub tracking_code2: Option<String>,
}

impl KaolaConvertRequest {
    /// 创建新的考拉转链请求
    ///
    /// # Arguments
    ///
    /// * `sid` - 考拉账号授权 SID
    /// * `target_url` - 考拉商品 ID 或链接
    pub fn new(sid: impl Into<String>, target_url: impl Into<String>) -> Self {
        Self {
            sid: sid.into(),
            target_url: target_url.into(),
            tracking_code2: None,
        }
    }

    /// 设置自定义参数 (用于返利)
    pub fn tracking_code2(mut self, tracking_code2: impl Into<String>) -> Self {
        self.tracking_code2 = Some(tracking_code2.into());
        self
    }
}

/// 考拉精选商品列表请求
///
/// 获取考拉精选商品列表，只返回商品 ID 列表
///
/// # Example
///
/// ```rust,ignore
/// let request = KaolaGoodsListRequest::new(KaolaPoolName::DailyBargain)
///     .page_no(1)
///     .page_size(20);
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct KaolaGoodsListRequest {
    /// 精选商品池名称
    /// 1-每日平价商品 2-高佣必推商品 3-新人专享商品 4-会员专属商品
    /// 5-低价包邮商品 6-考拉自营爆款 7-考拉商家爆款 8-黑卡用户最爱买商品
    /// 9-美妆个护热销品 10-食品保健热销品 11-母婴热销品 12-时尚热销品
    /// 13-家居宠物热销品 14-每日秒杀商品 15-黑卡好价商品 16-高转化好物商品
    /// 17-开卡一单省回商品 18-会员专属促销选品池 19-好券商品
    #[serde(rename = "poolName")]
    pub pool_name: String,
    /// 页码 (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "pageNo")]
    pub page_no: Option<u32>,
    /// 每页数量 (可选，默认 20，范围 1-200)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "pageSize")]
    pub page_size: Option<u32>,
}

impl KaolaGoodsListRequest {
    /// 创建新的考拉精选商品列表请求
    ///
    /// # Arguments
    ///
    /// * `pool_name` - 精选商品池名称 (1-19)
    pub fn new(pool_name: impl Into<String>) -> Self {
        Self {
            pool_name: pool_name.into(),
            page_no: None,
            page_size: None,
        }
    }

    /// 设置页码
    pub fn page_no(mut self, page_no: u32) -> Self {
        self.page_no = Some(page_no);
        self
    }

    /// 设置每页数量
    pub fn page_size(mut self, page_size: u32) -> Self {
        self.page_size = Some(page_size);
        self
    }
}

/// 考拉商品搜索请求
///
/// 根据关键词搜索考拉商品列表
///
/// # Example
///
/// ```rust,ignore
/// let request = KaolaSearchGoodsRequest::new("球鞋")
///     .sort_type(KaolaSortType::Sales)
///     .page_no(1)
///     .page_size(20);
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct KaolaSearchGoodsRequest {
    /// 搜索关键词
    #[serde(rename = "keyWord")]
    pub keyword: String,
    /// 排序方式 (可选)
    /// 0-综合排序 1-价格排序 2-销量排序 10-佣金比例排序
    /// 默认 0 综合排序
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub sort_type: Option<String>,
    /// 是否降序 (可选，只对价格排序方式有效)
    /// true-降序 false-升序，默认降序
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
    /// 页码 (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "pageNo")]
    pub page_no: Option<u32>,
    /// 每页数量 (可选，默认 20，范围 1-50)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "pageSize")]
    pub page_size: Option<u32>,
}

impl KaolaSearchGoodsRequest {
    /// 创建新的考拉商品搜索请求
    ///
    /// # Arguments
    ///
    /// * `keyword` - 搜索关键词
    pub fn new(keyword: impl Into<String>) -> Self {
        Self {
            keyword: keyword.into(),
            sort_type: None,
            desc: None,
            page_no: None,
            page_size: None,
        }
    }

    /// 设置排序方式
    /// 0-综合排序 1-价格排序 2-销量排序 10-佣金比例排序
    pub fn sort_type(mut self, sort_type: impl Into<String>) -> Self {
        self.sort_type = Some(sort_type.into());
        self
    }

    /// 设置是否降序 (只对价格排序有效)
    pub fn desc(mut self, desc: bool) -> Self {
        self.desc = Some(if desc {
            "true".to_string()
        } else {
            "false".to_string()
        });
        self
    }

    /// 设置页码
    pub fn page_no(mut self, page_no: u32) -> Self {
        self.page_no = Some(page_no);
        self
    }

    /// 设置每页数量
    pub fn page_size(mut self, page_size: u32) -> Self {
        self.page_size = Some(page_size);
        self
    }
}

/// 考拉商品详情请求
///
/// 根据商品 ID 获取商品详情信息
///
/// # Example
///
/// ```rust,ignore
/// let request = KaolaGoodsDetailRequest::new("2480622");
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct KaolaGoodsDetailRequest {
    /// 考拉商品 ID
    /// 商品 ID 例子: 6919173790682972930
    #[serde(rename = "goodsIds")]
    pub goods_ids: String,
}

impl KaolaGoodsDetailRequest {
    /// 创建新的考拉商品详情请求
    ///
    /// # Arguments
    ///
    /// * `goods_ids` - 考拉商品 ID
    pub fn new(goods_ids: impl Into<String>) -> Self {
        Self {
            goods_ids: goods_ids.into(),
        }
    }
}

/// 考拉订单查询请求
///
/// 查询考拉推广订单列表
///
/// # Example
///
/// ```rust,ignore
/// let request = KaolaOrderQueryRequest::new("1")
///     .page(1)
///     .page_size(50)
///     .start_time("2023-01-01 00:00:00")
///     .end_time("2023-01-31 23:59:59");
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct KaolaOrderQueryRequest {
    /// 时间类型
    /// 1-按照订单支付时间获取
    /// 2-按照订单更新时间获取
    /// 3-按照订单编号获取
    #[serde(rename = "type")]
    pub query_type: String,
    /// 页码
    pub page: u32,
    /// 每页数量 (单页数最大 50，默认 50)
    pub page_size: u32,
    /// 开始时间 (可选，格式: yyyy-MM-dd HH:mm:ss)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "startTime")]
    pub start_time: Option<String>,
    /// 结束时间 (可选，格式: yyyy-MM-dd HH:mm:ss)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "endTime")]
    pub end_time: Option<String>,
    /// 订单编号 (可选，当 type=3 时使用)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "orderSn")]
    pub order_sn: Option<String>,
}

impl KaolaOrderQueryRequest {
    /// 创建新的考拉订单查询请求
    ///
    /// # Arguments
    ///
    /// * `query_type` - 时间类型 (1-支付时间, 2-更新时间, 3-订单编号)
    pub fn new(query_type: impl Into<String>) -> Self {
        Self {
            query_type: query_type.into(),
            page: 1,
            page_size: 50,
            start_time: None,
            end_time: None,
            order_sn: None,
        }
    }

    /// 设置页码
    pub fn page(mut self, page: u32) -> Self {
        self.page = page;
        self
    }

    /// 设置每页数量
    pub fn page_size(mut self, page_size: u32) -> Self {
        self.page_size = page_size;
        self
    }

    /// 设置开始时间
    pub fn start_time(mut self, start_time: impl Into<String>) -> Self {
        self.start_time = Some(start_time.into());
        self
    }

    /// 设置结束时间
    pub fn end_time(mut self, end_time: impl Into<String>) -> Self {
        self.end_time = Some(end_time.into());
        self
    }

    /// 设置订单编号 (当 type=3 时使用)
    pub fn order_sn(mut self, order_sn: impl Into<String>) -> Self {
        self.order_sn = Some(order_sn.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kaola_convert_request_serialize() {
        let request = KaolaConvertRequest::new("sid123", "6919173790682972930");
        let json = serde_json::to_value(&request).unwrap();

        assert_eq!(json["sid"], "sid123");
        assert_eq!(json["targetUrl"], "6919173790682972930");
        assert!(json.get("trackingCode2").is_none());
    }

    #[test]
    fn test_kaola_convert_request_with_tracking_code() {
        let request = KaolaConvertRequest::new("sid123", "http://lu.kaola.com/5DiMFU")
            .tracking_code2("12345678901");

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["sid"], "sid123");
        assert_eq!(json["targetUrl"], "http://lu.kaola.com/5DiMFU");
        assert_eq!(json["trackingCode2"], "12345678901");
    }

    #[test]
    fn test_kaola_goods_list_request_serialize() {
        let request = KaolaGoodsListRequest::new("1").page_no(1).page_size(20);

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["poolName"], "1");
        assert_eq!(json["pageNo"], 1);
        assert_eq!(json["pageSize"], 20);
    }

    #[test]
    fn test_kaola_search_goods_request_serialize() {
        let request = KaolaSearchGoodsRequest::new("球鞋")
            .sort_type("2")
            .desc(true)
            .page_no(1)
            .page_size(20);

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["keyWord"], "球鞋");
        assert_eq!(json["type"], "2");
        assert_eq!(json["desc"], "true");
        assert_eq!(json["pageNo"], 1);
        assert_eq!(json["pageSize"], 20);
    }

    #[test]
    fn test_kaola_goods_detail_request_serialize() {
        let request = KaolaGoodsDetailRequest::new("2480622");
        let json = serde_json::to_value(&request).unwrap();

        assert_eq!(json["goodsIds"], "2480622");
    }

    #[test]
    fn test_kaola_order_query_request_serialize() {
        let request = KaolaOrderQueryRequest::new("1")
            .page(1)
            .page_size(50)
            .start_time("2023-01-01 00:00:00")
            .end_time("2023-01-31 23:59:59");

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["type"], "1");
        assert_eq!(json["page"], 1);
        assert_eq!(json["page_size"], 50);
        assert_eq!(json["startTime"], "2023-01-01 00:00:00");
        assert_eq!(json["endTime"], "2023-01-31 23:59:59");
    }

    #[test]
    fn test_kaola_order_query_request_by_order_sn() {
        let request = KaolaOrderQueryRequest::new("3").order_sn("ORDER123456");

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["type"], "3");
        assert_eq!(json["orderSn"], "ORDER123456");
    }

    #[test]
    fn test_optional_fields_not_serialized() {
        let request = KaolaConvertRequest::new("sid", "url");
        let json_str = serde_json::to_string(&request).unwrap();

        assert!(!json_str.contains("trackingCode2"));
    }
}
