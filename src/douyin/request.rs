//! 抖音平台请求参数结构体
//!
//! 定义抖音平台 API 的请求参数结构体

use serde::Serialize;

use super::enums::{
    DouyinActivityStatus, DouyinAuthorSortBy, DouyinAuthorType, DouyinLiveStatus,
    DouyinOrderQueryType, DouyinProductTag, DouyinSearchType, DouyinShareStatus, DouyinSortType,
};

/// 抖音商品转链请求
///
/// 将商品 URL/口令/短链接转换为推广链接
///
/// # Example
///
/// ```rust,ignore
/// let request = DouyinGoodsConvertRequest::new("sid", "https://xxx.douyin.com/xxx");
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct DouyinGoodsConvertRequest {
    /// 淘客账号授权 SID
    pub sid: String,
    /// 商品 URL/口令/短链接 (需要 URL 编码)
    pub product_url: String,
    /// 自定义字段，只允许数字，限制长度为11位，返利专用字段
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_info: Option<String>,
    /// 是否需要二维码，true 或 false，默认 false
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_qr_code: Option<String>,
    /// 是否返回商品惠后价领券链接，true 或 false，默认 false
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_coupon: Option<String>,
    /// 是否返回站外 H5 链接，true 或 false，默认 false
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_share_link: Option<String>,
}

impl DouyinGoodsConvertRequest {
    /// 创建新的商品转链请求
    pub fn new(sid: impl Into<String>, product_url: impl Into<String>) -> Self {
        Self {
            sid: sid.into(),
            product_url: product_url.into(),
            external_info: None,
            need_qr_code: None,
            use_coupon: None,
            need_share_link: None,
        }
    }

    /// 设置自定义字段
    pub fn external_info(mut self, external_info: impl Into<String>) -> Self {
        self.external_info = Some(external_info.into());
        self
    }

    /// 设置是否需要二维码
    pub fn need_qr_code(mut self, need_qr_code: bool) -> Self {
        self.need_qr_code = Some(need_qr_code.to_string());
        self
    }

    /// 设置是否返回惠后价领券链接
    pub fn use_coupon(mut self, use_coupon: bool) -> Self {
        self.use_coupon = Some(use_coupon.to_string());
        self
    }

    /// 设置是否返回站外 H5 链接
    pub fn need_share_link(mut self, need_share_link: bool) -> Self {
        self.need_share_link = Some(need_share_link.to_string());
        self
    }
}

/// 抖音直播间转链请求
///
/// 将直播间转换为推广链接
#[derive(Debug, Clone, Serialize)]
pub struct DouyinLiveConvertRequest {
    /// 淘客账号授权 SID
    pub sid: String,
    /// 主播百应 ID (buyin_id 和 dy_code 最少填一项)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyin_id: Option<String>,
    /// 直播间口令或短链接 (需要 URL 编码)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dy_code: Option<String>,
    /// 自定义字段，只允许数字，限制长度为11位，返利专用字段
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_info: Option<String>,
}

impl DouyinLiveConvertRequest {
    /// 创建新的直播间转链请求 (使用百应 ID)
    pub fn with_buyin_id(sid: impl Into<String>, buyin_id: impl Into<String>) -> Self {
        Self {
            sid: sid.into(),
            buyin_id: Some(buyin_id.into()),
            dy_code: None,
            external_info: None,
        }
    }

    /// 创建新的直播间转链请求 (使用口令/短链接)
    pub fn with_dy_code(sid: impl Into<String>, dy_code: impl Into<String>) -> Self {
        Self {
            sid: sid.into(),
            buyin_id: None,
            dy_code: Some(dy_code.into()),
            external_info: None,
        }
    }

    /// 设置自定义字段
    pub fn external_info(mut self, external_info: impl Into<String>) -> Self {
        self.external_info = Some(external_info.into());
        self
    }
}

/// 抖音活动转链请求
///
/// 将活动页转换为推广链接
#[derive(Debug, Clone, Serialize)]
pub struct DouyinActivityConvertRequest {
    /// 淘客账号授权 SID
    pub sid: String,
    /// 活动页物料 ID
    pub material_id: String,
    /// 自定义字段，只允许数字，限制长度为11位，返利专用字段
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_info: Option<String>,
    /// 是否需要二维码，true 或 false，默认 false
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_qr_code: Option<String>,
}

impl DouyinActivityConvertRequest {
    /// 创建新的活动转链请求
    pub fn new(sid: impl Into<String>, material_id: impl Into<String>) -> Self {
        Self {
            sid: sid.into(),
            material_id: material_id.into(),
            external_info: None,
            need_qr_code: None,
        }
    }

    /// 设置自定义字段
    pub fn external_info(mut self, external_info: impl Into<String>) -> Self {
        self.external_info = Some(external_info.into());
        self
    }

    /// 设置是否需要二维码
    pub fn need_qr_code(mut self, need_qr_code: bool) -> Self {
        self.need_qr_code = Some(need_qr_code.to_string());
        self
    }
}

/// 抖音商品详情请求
///
/// 获取商品详细信息
#[derive(Debug, Clone, Serialize)]
pub struct DouyinGoodsDetailRequest {
    /// 淘客账号授权 SID
    pub sid: String,
    /// 商品 ID 列表 (最多20个，用英文逗号分割)
    pub product_ids: String,
    /// 需要返回的字段，多个用英文逗号隔开
    /// 可选值: base_info, promotion_info
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<String>,
}

impl DouyinGoodsDetailRequest {
    /// 创建新的商品详情请求
    pub fn new(sid: impl Into<String>, product_ids: impl Into<String>) -> Self {
        Self {
            sid: sid.into(),
            product_ids: product_ids.into(),
            fields: None,
        }
    }

    /// 设置返回字段
    pub fn fields(mut self, fields: impl Into<String>) -> Self {
        self.fields = Some(fields.into());
        self
    }

    /// 设置返回基础信息和推广信息
    pub fn with_all_fields(mut self) -> Self {
        self.fields = Some("base_info,promotion_info".to_string());
        self
    }
}

/// 抖音商品搜索请求
///
/// 搜索抖音商品
#[derive(Debug, Clone, Serialize)]
pub struct DouyinSearchGoodsRequest {
    /// 淘客账号授权 SID
    pub sid: String,
    /// 商品标题关键词 (需要 URL 编码)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 筛选商品一级类目
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_cids: Option<String>,
    /// 筛选商品二级类目
    #[serde(skip_serializing_if = "Option::is_none")]
    pub second_cids: Option<String>,
    /// 筛选商品三级类目
    #[serde(skip_serializing_if = "Option::is_none")]
    pub third_cids: Option<String>,
    /// 筛选价格区间-最小值 (单位为分)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_min: Option<String>,
    /// 筛选价格区间-最大值 (单位为分)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_max: Option<String>,
    /// 筛选历史销量区间-最小值
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sell_num_min: Option<String>,
    /// 筛选历史销量区间-最大值
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sell_num_max: Option<String>,
    /// 召回结果排序条件
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_type: Option<DouyinSearchType>,
    /// 排序顺序
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_type: Option<DouyinSortType>,
    /// 筛选普通佣金区间-最小值 (单位为分)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cos_fee_min: Option<String>,
    /// 筛选普通佣金区间-最大值 (单位为分)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cos_fee_max: Option<String>,
    /// 筛选普通佣金率区间-最小值 (乘100，例如1.1%为110)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cos_ratio_min: Option<String>,
    /// 筛选普通佣金率区间-最大值 (乘100，例如1.1%为110)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cos_ratio_max: Option<String>,
    /// 分页 (从1开始)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    /// 每页的数量 (小于等于20)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<u32>,
    /// 获取商品分销状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_status: Option<DouyinShareStatus>,
    /// 商品标签
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<DouyinProductTag>,
}

impl DouyinSearchGoodsRequest {
    /// 创建新的商品搜索请求
    pub fn new(sid: impl Into<String>) -> Self {
        Self {
            sid: sid.into(),
            title: None,
            first_cids: None,
            second_cids: None,
            third_cids: None,
            price_min: None,
            price_max: None,
            sell_num_min: None,
            sell_num_max: None,
            search_type: None,
            sort_type: None,
            cos_fee_min: None,
            cos_fee_max: None,
            cos_ratio_min: None,
            cos_ratio_max: None,
            page: None,
            page_size: None,
            share_status: None,
            tag: None,
        }
    }

    /// 设置搜索关键词
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// 设置一级类目
    pub fn first_cids(mut self, first_cids: impl Into<String>) -> Self {
        self.first_cids = Some(first_cids.into());
        self
    }

    /// 设置价格区间 (单位为分)
    pub fn price_range(mut self, min: u64, max: u64) -> Self {
        self.price_min = Some(min.to_string());
        self.price_max = Some(max.to_string());
        self
    }

    /// 设置销量区间
    pub fn sell_num_range(mut self, min: u64, max: u64) -> Self {
        self.sell_num_min = Some(min.to_string());
        self.sell_num_max = Some(max.to_string());
        self
    }

    /// 设置排序方式
    pub fn search_type(mut self, search_type: DouyinSearchType) -> Self {
        self.search_type = Some(search_type);
        self
    }

    /// 设置排序方向
    pub fn sort_type(mut self, sort_type: DouyinSortType) -> Self {
        self.sort_type = Some(sort_type);
        self
    }

    /// 设置分页
    pub fn page(mut self, page: u32) -> Self {
        self.page = Some(page);
        self
    }

    /// 设置每页数量
    pub fn page_size(mut self, page_size: u32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置分销状态筛选
    pub fn share_status(mut self, share_status: DouyinShareStatus) -> Self {
        self.share_status = Some(share_status);
        self
    }

    /// 设置商品标签筛选
    pub fn tag(mut self, tag: DouyinProductTag) -> Self {
        self.tag = Some(tag);
        self
    }
}

/// 抖音口令解析请求
///
/// 解析抖音口令/短链接
#[derive(Debug, Clone, Serialize)]
pub struct DouyinParseCommandRequest {
    /// 淘客账号授权 SID
    pub sid: String,
    /// 口令/短链接 (需要 URL 编码)
    pub command: String,
}

impl DouyinParseCommandRequest {
    /// 创建新的口令解析请求
    pub fn new(sid: impl Into<String>, command: impl Into<String>) -> Self {
        Self {
            sid: sid.into(),
            command: command.into(),
        }
    }
}

/// 抖音订单查询请求
///
/// 查询抖音订单
#[derive(Debug, Clone, Serialize)]
pub struct DouyinOrderQueryRequest {
    /// 时间类型
    #[serde(rename = "type")]
    pub query_type: DouyinOrderQueryType,
    /// 页码
    pub page: u32,
    /// 每页数量 (最大50，默认50)
    pub page_size: u32,
    /// 订单开始时间 (格式: yyyy-MM-dd HH:mm:ss)
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 订单结束时间 (格式: yyyy-MM-dd HH:mm:ss)
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 订单编号 (query_type=3 时必填)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orderid: Option<String>,
    /// 折淘客授权 SID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,
    /// 订单所属平台 ID (9=抖音)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub san_pingtai_id: Option<String>,
}

impl DouyinOrderQueryRequest {
    /// 创建按支付时间查询的订单请求
    pub fn by_pay_time(start_time: impl Into<String>, end_time: impl Into<String>) -> Self {
        Self {
            query_type: DouyinOrderQueryType::PayTime,
            page: 1,
            page_size: 50,
            start_time: Some(start_time.into()),
            end_time: Some(end_time.into()),
            orderid: None,
            sid: None,
            san_pingtai_id: Some("9".to_string()), // 抖音平台 ID
        }
    }

    /// 创建按更新时间查询的订单请求
    pub fn by_update_time(start_time: impl Into<String>, end_time: impl Into<String>) -> Self {
        Self {
            query_type: DouyinOrderQueryType::UpdateTime,
            page: 1,
            page_size: 50,
            start_time: Some(start_time.into()),
            end_time: Some(end_time.into()),
            orderid: None,
            sid: None,
            san_pingtai_id: Some("9".to_string()),
        }
    }

    /// 创建按订单编号查询的订单请求
    pub fn by_order_id(orderid: impl Into<String>) -> Self {
        Self {
            query_type: DouyinOrderQueryType::OrderId,
            page: 1,
            page_size: 50,
            start_time: None,
            end_time: None,
            orderid: Some(orderid.into()),
            sid: None,
            san_pingtai_id: Some("9".to_string()),
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

    /// 设置授权 SID
    pub fn sid(mut self, sid: impl Into<String>) -> Self {
        self.sid = Some(sid.into());
        self
    }
}

/// 抖音活动列表请求
///
/// 获取抖音活动列表
#[derive(Debug, Clone, Serialize)]
pub struct DouyinActivityListRequest {
    /// 淘客账号授权 SID
    pub sid: String,
    /// 分页 (从1开始)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    /// 每页的数量 (小于等于20)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<u32>,
    /// 活动状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_status: Option<DouyinActivityStatus>,
}

impl DouyinActivityListRequest {
    /// 创建新的活动列表请求
    pub fn new(sid: impl Into<String>) -> Self {
        Self {
            sid: sid.into(),
            page: None,
            page_size: None,
            activity_status: None,
        }
    }

    /// 设置分页
    pub fn page(mut self, page: u32) -> Self {
        self.page = Some(page);
        self
    }

    /// 设置每页数量
    pub fn page_size(mut self, page_size: u32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置活动状态筛选
    pub fn activity_status(mut self, activity_status: DouyinActivityStatus) -> Self {
        self.activity_status = Some(activity_status);
        self
    }
}

/// 抖音达人列表请求
///
/// 获取抖音直播间达人列表
#[derive(Debug, Clone, Serialize)]
pub struct DouyinAuthorListRequest {
    /// 淘客账号授权 SID
    pub sid: String,
    /// 达人类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_type: Option<DouyinAuthorType>,
    /// 作者电商等级 (0-7级)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_levels: Option<String>,
    /// 商品行业类目
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frist_cids: Option<String>,
    /// 达人昵称或账号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_info: Option<String>,
    /// 分页 (从1开始)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    /// 分页大小
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<u32>,
    /// 排序字段
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<DouyinAuthorSortBy>,
    /// 排序方式
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_type: Option<DouyinSortType>,
    /// 开关播状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub live_status: Option<DouyinLiveStatus>,
}

impl DouyinAuthorListRequest {
    /// 创建新的达人列表请求
    pub fn new(sid: impl Into<String>) -> Self {
        Self {
            sid: sid.into(),
            author_type: None,
            author_levels: None,
            frist_cids: None,
            author_info: None,
            page: None,
            page_size: None,
            sort_by: None,
            sort_type: None,
            live_status: None,
        }
    }

    /// 设置达人类型
    pub fn author_type(mut self, author_type: DouyinAuthorType) -> Self {
        self.author_type = Some(author_type);
        self
    }

    /// 设置达人昵称或账号搜索
    pub fn author_info(mut self, author_info: impl Into<String>) -> Self {
        self.author_info = Some(author_info.into());
        self
    }

    /// 设置分页
    pub fn page(mut self, page: u32) -> Self {
        self.page = Some(page);
        self
    }

    /// 设置每页数量
    pub fn page_size(mut self, page_size: u32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置排序字段
    pub fn sort_by(mut self, sort_by: DouyinAuthorSortBy) -> Self {
        self.sort_by = Some(sort_by);
        self
    }

    /// 设置直播状态筛选
    pub fn live_status(mut self, live_status: DouyinLiveStatus) -> Self {
        self.live_status = Some(live_status);
        self
    }
}

/// 抖音直播间商品列表请求
///
/// 获取直播间商品列表
#[derive(Debug, Clone, Serialize)]
pub struct DouyinLiveProductListRequest {
    /// 淘客账号授权 SID
    pub sid: String,
    /// 主播的百应 ID
    pub author_buyin_id: String,
}

impl DouyinLiveProductListRequest {
    /// 创建新的直播间商品列表请求
    pub fn new(sid: impl Into<String>, author_buyin_id: impl Into<String>) -> Self {
        Self {
            sid: sid.into(),
            author_buyin_id: author_buyin_id.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_goods_convert_request_serialize() {
        let request = DouyinGoodsConvertRequest::new("sid123", "https://xxx.douyin.com/xxx");
        let json = serde_json::to_value(&request).unwrap();

        assert_eq!(json["sid"], "sid123");
        assert_eq!(json["product_url"], "https://xxx.douyin.com/xxx");
        assert!(json.get("external_info").is_none());
    }

    #[test]
    fn test_goods_convert_request_with_options() {
        let request = DouyinGoodsConvertRequest::new("sid123", "https://xxx.douyin.com/xxx")
            .external_info("12345678901")
            .need_qr_code(true);

        let json = serde_json::to_value(&request).unwrap();

        assert_eq!(json["external_info"], "12345678901");
        assert_eq!(json["need_qr_code"], "true");
    }

    #[test]
    fn test_live_convert_request_with_buyin_id() {
        let request = DouyinLiveConvertRequest::with_buyin_id("sid123", "buyin123");
        let json = serde_json::to_value(&request).unwrap();

        assert_eq!(json["sid"], "sid123");
        assert_eq!(json["buyin_id"], "buyin123");
        assert!(json.get("dy_code").is_none());
    }

    #[test]
    fn test_activity_convert_request_serialize() {
        let request = DouyinActivityConvertRequest::new("sid123", "material123");
        let json = serde_json::to_value(&request).unwrap();

        assert_eq!(json["sid"], "sid123");
        assert_eq!(json["material_id"], "material123");
    }

    #[test]
    fn test_goods_detail_request_serialize() {
        let request = DouyinGoodsDetailRequest::new("sid123", "123,456,789").with_all_fields();
        let json = serde_json::to_value(&request).unwrap();

        assert_eq!(json["product_ids"], "123,456,789");
        assert_eq!(json["fields"], "base_info,promotion_info");
    }

    #[test]
    fn test_search_goods_request_serialize() {
        let request = DouyinSearchGoodsRequest::new("sid123")
            .title("测试商品")
            .page(1)
            .page_size(20)
            .search_type(DouyinSearchType::Sales);

        let json = serde_json::to_value(&request).unwrap();

        assert_eq!(json["title"], "测试商品");
        assert_eq!(json["page"], 1);
        assert_eq!(json["page_size"], 20);
        assert_eq!(json["search_type"], 1);
    }

    #[test]
    fn test_parse_command_request_serialize() {
        let request = DouyinParseCommandRequest::new("sid123", "口令内容");
        let json = serde_json::to_value(&request).unwrap();

        assert_eq!(json["sid"], "sid123");
        assert_eq!(json["command"], "口令内容");
    }

    #[test]
    fn test_order_query_request_by_pay_time() {
        let request =
            DouyinOrderQueryRequest::by_pay_time("2024-01-01 00:00:00", "2024-01-31 23:59:59");
        let json = serde_json::to_value(&request).unwrap();

        assert_eq!(json["type"], 1);
        assert_eq!(json["startTime"], "2024-01-01 00:00:00");
        assert_eq!(json["endTime"], "2024-01-31 23:59:59");
        assert_eq!(json["san_pingtai_id"], "9");
    }

    #[test]
    fn test_order_query_request_by_order_id() {
        let request = DouyinOrderQueryRequest::by_order_id("123456789");
        let json = serde_json::to_value(&request).unwrap();

        assert_eq!(json["type"], 3);
        assert_eq!(json["orderid"], "123456789");
    }

    #[test]
    fn test_activity_list_request_serialize() {
        let request = DouyinActivityListRequest::new("sid123")
            .page(1)
            .page_size(20)
            .activity_status(DouyinActivityStatus::Active);

        let json = serde_json::to_value(&request).unwrap();

        assert_eq!(json["page"], 1);
        assert_eq!(json["activity_status"], 2);
    }

    #[test]
    fn test_optional_fields_not_serialized() {
        let request = DouyinGoodsConvertRequest::new("sid", "url");
        let json_str = serde_json::to_string(&request).unwrap();

        assert!(!json_str.contains("external_info"));
        assert!(!json_str.contains("need_qr_code"));
    }
}
