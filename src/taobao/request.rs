//! 淘宝平台请求参数结构体
//!
//! 定义淘宝平台 API 的请求参数结构体

use serde::Serialize;

use super::enums::{TaobaoOrderQueryType, TaobaoOrderStatus, TaobaoSignUrlType};

/// 高佣转链请求 (商品ID)
///
/// 将商品 ID 转换为带有高佣金的推广链接
///
/// # Example
///
/// ```rust,ignore
/// let request = ConvertByItemIdRequest {
///     sid: "your_sid".to_string(),
///     pid: "mm_xxx_xxx_xxx".to_string(),
///     num_iid: "123456789".to_string(),
///     relation_id: Some("your_relation_id".to_string()),
///     special_id: None,
///     signurl: Some(TaobaoSignUrlType::WithFullDetail),
/// };
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct ConvertByItemIdRequest {
    /// 淘客账号授权 ID
    pub sid: String,
    /// 淘客 PID，格式: mm_xxx_xxx_xxx
    pub pid: String,
    /// 商品 ID
    pub num_iid: String,
    /// 渠道关系 ID (可选，用于返利场景)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relation_id: Option<String>,
    /// 会员运营 ID (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub special_id: Option<String>,
    /// 返回结果类型
    /// - 0/1/2: 官方结果
    /// - 3: 整合高佣转链+解析商品编号
    /// - 4: 整合+简版详情
    /// - 5: 整合+全网详情+淘口令
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signurl: Option<TaobaoSignUrlType>,
}

impl ConvertByItemIdRequest {
    /// 创建新的高佣转链请求
    ///
    /// # Arguments
    ///
    /// * `sid` - 淘客账号授权 ID
    /// * `pid` - 淘客 PID
    /// * `num_iid` - 商品 ID
    pub fn new(sid: impl Into<String>, pid: impl Into<String>, num_iid: impl Into<String>) -> Self {
        Self {
            sid: sid.into(),
            pid: pid.into(),
            num_iid: num_iid.into(),
            relation_id: None,
            special_id: None,
            signurl: None,
        }
    }

    /// 设置渠道关系 ID
    pub fn relation_id(mut self, relation_id: impl Into<String>) -> Self {
        self.relation_id = Some(relation_id.into());
        self
    }

    /// 设置会员运营 ID
    pub fn special_id(mut self, special_id: impl Into<String>) -> Self {
        self.special_id = Some(special_id.into());
        self
    }

    /// 设置返回结果类型
    pub fn signurl(mut self, signurl: TaobaoSignUrlType) -> Self {
        self.signurl = Some(signurl);
        self
    }
}

/// 高佣转链请求 (淘口令)
///
/// 将淘口令转换为自己的推广淘口令
///
/// # Example
///
/// ```rust,ignore
/// let request = ConvertByTklRequest {
///     sid: "your_sid".to_string(),
///     pid: "mm_xxx_xxx_xxx".to_string(),
///     tkl: "淘口令内容".to_string(),
///     relation_id: None,
///     union_id: None,
///     position_id: None,
///     custom_parameters: None,
///     chan_tag: None,
///     customer_id: None,
/// };
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct ConvertByTklRequest {
    /// 淘客账号授权 ID
    pub sid: String,
    /// 淘客 PID
    pub pid: String,
    /// 淘口令文案 (会自动 URL 编码)
    pub tkl: String,
    /// 淘宝渠道关系 ID (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relation_id: Option<String>,
    /// 京东联盟 ID (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub union_id: Option<String>,
    /// 京东渠道关系 ID (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_id: Option<String>,
    /// 拼多多自定义参数 (可选，JSON 格式)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_parameters: Option<String>,
    /// 唯品会渠道标识 (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chan_tag: Option<String>,
    /// 美团渠道关系 ID (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
}

impl ConvertByTklRequest {
    /// 创建新的淘口令转链请求
    ///
    /// # Arguments
    ///
    /// * `sid` - 淘客账号授权 ID
    /// * `pid` - 淘客 PID
    /// * `tkl` - 淘口令文案
    pub fn new(sid: impl Into<String>, pid: impl Into<String>, tkl: impl Into<String>) -> Self {
        Self {
            sid: sid.into(),
            pid: pid.into(),
            tkl: tkl.into(),
            relation_id: None,
            union_id: None,
            position_id: None,
            custom_parameters: None,
            chan_tag: None,
            customer_id: None,
        }
    }

    /// 设置淘宝渠道关系 ID
    pub fn relation_id(mut self, relation_id: impl Into<String>) -> Self {
        self.relation_id = Some(relation_id.into());
        self
    }

    /// 设置京东联盟 ID
    pub fn union_id(mut self, union_id: impl Into<String>) -> Self {
        self.union_id = Some(union_id.into());
        self
    }

    /// 设置京东渠道关系 ID
    pub fn position_id(mut self, position_id: impl Into<String>) -> Self {
        self.position_id = Some(position_id.into());
        self
    }

    /// 设置拼多多自定义参数
    pub fn custom_parameters(mut self, custom_parameters: impl Into<String>) -> Self {
        self.custom_parameters = Some(custom_parameters.into());
        self
    }

    /// 设置唯品会渠道标识
    pub fn chan_tag(mut self, chan_tag: impl Into<String>) -> Self {
        self.chan_tag = Some(chan_tag.into());
        self
    }

    /// 设置美团渠道关系 ID
    pub fn customer_id(mut self, customer_id: impl Into<String>) -> Self {
        self.customer_id = Some(customer_id.into());
        self
    }
}

/// 批量高佣转链请求
///
/// 同时获取多个渠道 ID 的转链结果
#[derive(Debug, Clone, Serialize)]
pub struct BatchConvertRequest {
    /// 淘客账号授权 ID
    pub sid: String,
    /// 淘客 PID
    pub pid: String,
    /// 商品 ID
    pub num_iid: String,
    /// 渠道关系 ID 列表 (用逗号分隔)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relation_ids: Option<String>,
    /// 返回结果类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signurl: Option<TaobaoSignUrlType>,
}

impl BatchConvertRequest {
    /// 创建新的批量转链请求
    ///
    /// # Arguments
    ///
    /// * `sid` - 淘客账号授权 ID
    /// * `pid` - 淘客 PID
    /// * `num_iid` - 商品 ID
    pub fn new(sid: impl Into<String>, pid: impl Into<String>, num_iid: impl Into<String>) -> Self {
        Self {
            sid: sid.into(),
            pid: pid.into(),
            num_iid: num_iid.into(),
            relation_ids: None,
            signurl: None,
        }
    }

    /// 设置渠道关系 ID 列表
    pub fn relation_ids(mut self, relation_ids: impl Into<String>) -> Self {
        self.relation_ids = Some(relation_ids.into());
        self
    }

    /// 设置返回结果类型
    pub fn signurl(mut self, signurl: TaobaoSignUrlType) -> Self {
        self.signurl = Some(signurl);
        self
    }
}

/// 订单查询请求
///
/// 查询淘宝联盟订单
#[derive(Debug, Clone, Serialize)]
pub struct QueryOrdersRequest {
    /// 淘客账号授权 ID
    pub sid: String,
    /// 查询开始时间 (格式: yyyy-MM-dd HH:mm:ss)
    pub start_time: String,
    /// 查询结束时间 (格式: yyyy-MM-dd HH:mm:ss)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 查询类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_type: Option<TaobaoOrderQueryType>,
    /// 订单状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tk_status: Option<TaobaoOrderStatus>,
    /// 页码 (从 1 开始)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_no: Option<u32>,
    /// 每页数量 (最大 100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<u32>,
    /// 渠道关系 ID (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relation_id: Option<String>,
    /// 会员运营 ID (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub special_id: Option<String>,
}

impl QueryOrdersRequest {
    /// 创建新的订单查询请求
    ///
    /// # Arguments
    ///
    /// * `sid` - 淘客账号授权 ID
    /// * `start_time` - 查询开始时间
    pub fn new(sid: impl Into<String>, start_time: impl Into<String>) -> Self {
        Self {
            sid: sid.into(),
            start_time: start_time.into(),
            end_time: None,
            query_type: None,
            tk_status: None,
            page_no: None,
            page_size: None,
            relation_id: None,
            special_id: None,
        }
    }

    /// 设置查询结束时间
    pub fn end_time(mut self, end_time: impl Into<String>) -> Self {
        self.end_time = Some(end_time.into());
        self
    }

    /// 设置查询类型
    pub fn query_type(mut self, query_type: TaobaoOrderQueryType) -> Self {
        self.query_type = Some(query_type);
        self
    }

    /// 设置订单状态
    pub fn tk_status(mut self, tk_status: TaobaoOrderStatus) -> Self {
        self.tk_status = Some(tk_status);
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

    /// 设置渠道关系 ID
    pub fn relation_id(mut self, relation_id: impl Into<String>) -> Self {
        self.relation_id = Some(relation_id.into());
        self
    }

    /// 设置会员运营 ID
    pub fn special_id(mut self, special_id: impl Into<String>) -> Self {
        self.special_id = Some(special_id.into());
        self
    }
}

/// 创建淘口令请求
///
/// 生成淘口令
#[derive(Debug, Clone, Serialize)]
pub struct CreateTklRequest {
    /// 淘客账号授权 ID
    pub sid: String,
    /// 淘客 PID
    pub pid: String,
    /// 商品 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_iid: Option<String>,
    /// 推广链接 URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// 淘口令文案
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// 淘口令 logo 图片 URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo: Option<String>,
    /// 渠道关系 ID (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relation_id: Option<String>,
}

impl CreateTklRequest {
    /// 创建新的淘口令生成请求
    ///
    /// # Arguments
    ///
    /// * `sid` - 淘客账号授权 ID
    /// * `pid` - 淘客 PID
    pub fn new(sid: impl Into<String>, pid: impl Into<String>) -> Self {
        Self {
            sid: sid.into(),
            pid: pid.into(),
            num_iid: None,
            url: None,
            text: None,
            logo: None,
            relation_id: None,
        }
    }

    /// 设置商品 ID
    pub fn num_iid(mut self, num_iid: impl Into<String>) -> Self {
        self.num_iid = Some(num_iid.into());
        self
    }

    /// 设置推广链接 URL
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    /// 设置淘口令文案
    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    /// 设置淘口令 logo 图片 URL
    pub fn logo(mut self, logo: impl Into<String>) -> Self {
        self.logo = Some(logo.into());
        self
    }

    /// 设置渠道关系 ID
    pub fn relation_id(mut self, relation_id: impl Into<String>) -> Self {
        self.relation_id = Some(relation_id.into());
        self
    }
}

/// 解析商品编号请求
///
/// 从淘口令或链接中提取商品 ID
#[derive(Debug, Clone, Serialize)]
pub struct ParseItemIdRequest {
    /// 淘口令或商品链接
    pub content: String,
}

impl ParseItemIdRequest {
    /// 创建新的解析商品编号请求
    ///
    /// # Arguments
    ///
    /// * `content` - 淘口令或商品链接
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
        }
    }
}

/// 全网搜索商品请求
///
/// 根据关键字搜索商品，返回动态描述分≥4.6的商品列表
///
/// # Example
///
/// ```rust,ignore
/// let request = SearchGoodsRequest::new("sid123", "mm_xxx_xxx_xxx")
///     .keyword("内存条")
///     .page(1)
///     .page_size(20)
///     .sort("sale_num_desc");
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct SearchGoodsRequest {
    /// 淘客账号授权 ID
    pub sid: String,
    /// 淘客 PID，格式: mm_xxx_xxx_xxx
    pub pid: String,
    /// 搜索关键词
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q: Option<String>,
    /// 页码 (从 1 开始)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    /// 每页数量 (1-50，默认 20)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<u32>,
    /// 排序方式
    /// - new: 综合排序
    /// - total_sale_num_desc: 总销量从大到小
    /// - sale_num_desc: 年销量从大到小
    /// - commission_rate_desc: 佣金比例从大到小
    /// - price_asc: 价格从小到大
    /// - price_desc: 价格从大到小
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
    /// 是否有券，1 为有券
    #[serde(skip_serializing_if = "Option::is_none")]
    pub youquan: Option<String>,
    /// 是否天猫商品，tmall 为天猫
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tj: Option<String>,
    /// 是否海外商品，1 为海外
    #[serde(skip_serializing_if = "Option::is_none")]
    pub haiwai: Option<String>,
    /// 商品所在地 (城市名称，如：北京、上海、广州)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub itemloc: Option<String>,
    /// 淘客佣金比率下限 (如：20 表示 >=20%)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_tk_rate: Option<String>,
    /// 淘客佣金比率上限 (如：50 表示 <=50%)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_tk_rate: Option<String>,
    /// 折扣价格下限 (如：100 表示 >=100 元)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_price: Option<String>,
    /// 折扣价格上限 (如：200 表示 <=200 元)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_price: Option<String>,
    /// 过滤值: 0-不过滤, 1-轻度过滤, 2-中度过滤 (推荐)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub filter_type: Option<String>,
    /// 后台类目 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cat: Option<String>,
}

impl SearchGoodsRequest {
    /// 创建新的全网搜索商品请求
    ///
    /// # Arguments
    ///
    /// * `sid` - 淘客账号授权 ID
    /// * `pid` - 淘客 PID
    pub fn new(sid: impl Into<String>, pid: impl Into<String>) -> Self {
        Self {
            sid: sid.into(),
            pid: pid.into(),
            q: None,
            page: None,
            page_size: None,
            sort: None,
            youquan: None,
            tj: None,
            haiwai: None,
            itemloc: None,
            start_tk_rate: None,
            end_tk_rate: None,
            start_price: None,
            end_price: None,
            filter_type: None,
            cat: None,
        }
    }

    /// 设置搜索关键词
    pub fn keyword(mut self, keyword: impl Into<String>) -> Self {
        self.q = Some(keyword.into());
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

    /// 设置排序方式
    pub fn sort(mut self, sort: impl Into<String>) -> Self {
        self.sort = Some(sort.into());
        self
    }

    /// 只返回有券商品
    pub fn with_coupon(mut self) -> Self {
        self.youquan = Some("1".to_string());
        self
    }

    /// 只返回天猫商品
    pub fn tmall_only(mut self) -> Self {
        self.tj = Some("tmall".to_string());
        self
    }

    /// 只返回海外商品
    pub fn overseas_only(mut self) -> Self {
        self.haiwai = Some("1".to_string());
        self
    }

    /// 设置商品所在地
    pub fn location(mut self, location: impl Into<String>) -> Self {
        self.itemloc = Some(location.into());
        self
    }

    /// 设置佣金比率范围
    pub fn commission_rate_range(mut self, start: u32, end: u32) -> Self {
        self.start_tk_rate = Some(start.to_string());
        self.end_tk_rate = Some(end.to_string());
        self
    }

    /// 设置价格范围
    pub fn price_range(mut self, start: f64, end: f64) -> Self {
        self.start_price = Some(start.to_string());
        self.end_price = Some(end.to_string());
        self
    }

    /// 设置过滤级别 (推荐使用 2)
    pub fn filter(mut self, level: u32) -> Self {
        self.filter_type = Some(level.to_string());
        self
    }

    /// 设置后台类目 ID
    pub fn category(mut self, cat: impl Into<String>) -> Self {
        self.cat = Some(cat.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_by_item_id_request_serialize() {
        let request = ConvertByItemIdRequest::new("sid123", "mm_123_456_789", "987654321");
        let json = serde_json::to_value(&request).unwrap();

        assert_eq!(json["sid"], "sid123");
        assert_eq!(json["pid"], "mm_123_456_789");
        assert_eq!(json["num_iid"], "987654321");
        assert!(json.get("relation_id").is_none());
    }

    #[test]
    fn test_convert_by_item_id_request_with_options() {
        let request = ConvertByItemIdRequest::new("sid123", "mm_123_456_789", "987654321")
            .relation_id("rel123")
            .signurl(TaobaoSignUrlType::WithFullDetail);

        let json = serde_json::to_value(&request).unwrap();

        assert_eq!(json["relation_id"], "rel123");
        assert_eq!(json["signurl"], 5);
    }

    #[test]
    fn test_convert_by_tkl_request_serialize() {
        let request = ConvertByTklRequest::new("sid123", "mm_123_456_789", "淘口令内容");
        let json = serde_json::to_value(&request).unwrap();

        assert_eq!(json["sid"], "sid123");
        assert_eq!(json["pid"], "mm_123_456_789");
        assert_eq!(json["tkl"], "淘口令内容");
    }

    #[test]
    fn test_batch_convert_request_serialize() {
        let request = BatchConvertRequest::new("sid123", "mm_123_456_789", "987654321")
            .relation_ids("rel1,rel2,rel3");

        let json = serde_json::to_value(&request).unwrap();

        assert_eq!(json["relation_ids"], "rel1,rel2,rel3");
    }

    #[test]
    fn test_query_orders_request_serialize() {
        let request = QueryOrdersRequest::new("sid123", "2024-01-01 00:00:00")
            .end_time("2024-01-31 23:59:59")
            .query_type(TaobaoOrderQueryType::CreateTime)
            .page_no(1)
            .page_size(20);

        let json = serde_json::to_value(&request).unwrap();

        assert_eq!(json["start_time"], "2024-01-01 00:00:00");
        assert_eq!(json["end_time"], "2024-01-31 23:59:59");
        assert_eq!(json["query_type"], 1);
        assert_eq!(json["page_no"], 1);
        assert_eq!(json["page_size"], 20);
    }

    #[test]
    fn test_create_tkl_request_serialize() {
        let request = CreateTklRequest::new("sid123", "mm_123_456_789")
            .num_iid("987654321")
            .text("推荐好物");

        let json = serde_json::to_value(&request).unwrap();

        assert_eq!(json["num_iid"], "987654321");
        assert_eq!(json["text"], "推荐好物");
    }

    #[test]
    fn test_parse_item_id_request_serialize() {
        let request = ParseItemIdRequest::new("https://item.taobao.com/item.htm?id=123456");
        let json = serde_json::to_value(&request).unwrap();

        assert_eq!(
            json["content"],
            "https://item.taobao.com/item.htm?id=123456"
        );
    }

    #[test]
    fn test_optional_fields_not_serialized() {
        let request = ConvertByItemIdRequest::new("sid", "pid", "num_iid");
        let json_str = serde_json::to_string(&request).unwrap();

        assert!(!json_str.contains("relation_id"));
        assert!(!json_str.contains("special_id"));
        assert!(!json_str.contains("signurl"));
    }
}
