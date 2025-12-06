//! 京东平台请求参数结构体
//!
//! 定义京东平台 API 的请求参数结构体

use serde::Serialize;

use super::enums::{JdChainType, JdEliteId, JdOrderQueryType, JdSortField};
use crate::common::types::SortDirection;

/// 京东转链请求
///
/// 将商品链接或 SKU ID 转换为推广链接
///
/// # Example
///
/// ```rust,ignore
/// let request = JdConvertRequest::new("https://item.jd.com/123456.html", "your_union_id")
///     .position_id("12345")
///     .chain_type(JdChainType::Short);
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct JdConvertRequest {
    /// 推广物料 URL 或 SKU ID (需要 URL 编码)
    #[serde(rename = "materialId")]
    pub material_id: String,
    /// 京东联盟 ID，为一串数字
    #[serde(rename = "unionId")]
    pub union_id: String,
    /// 自定义推广位 ID (可选，用于返利场景)
    /// 可以自定义数字，自己在本地跟用户做好关联
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "positionId")]
    pub position_id: Option<String>,
    /// 联盟子推客身份标识 (可选)
    /// 格式: 联盟id_应用id_推广位id，三段式
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pid: Option<String>,
    /// 优惠券领取链接 (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "couponUrl")]
    pub coupon_url: Option<String>,
    /// 子渠道标识 (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "subUnionId")]
    pub sub_union_id: Option<String>,
    /// 转链类型: 1-长链, 2-短链, 3-长链+短链
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "chainType")]
    pub chain_type: Option<JdChainType>,
    /// 礼金批次号 (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "giftCouponKey")]
    pub gift_coupon_key: Option<String>,
}

impl JdConvertRequest {
    /// 创建新的京东转链请求
    ///
    /// # Arguments
    ///
    /// * `material_id` - 推广物料 URL 或 SKU ID
    /// * `union_id` - 京东联盟 ID
    pub fn new(material_id: impl Into<String>, union_id: impl Into<String>) -> Self {
        Self {
            material_id: material_id.into(),
            union_id: union_id.into(),
            position_id: None,
            pid: None,
            coupon_url: None,
            sub_union_id: None,
            chain_type: None,
            gift_coupon_key: None,
        }
    }

    /// 设置自定义推广位 ID
    pub fn position_id(mut self, position_id: impl Into<String>) -> Self {
        self.position_id = Some(position_id.into());
        self
    }

    /// 设置联盟子推客身份标识
    pub fn pid(mut self, pid: impl Into<String>) -> Self {
        self.pid = Some(pid.into());
        self
    }

    /// 设置优惠券领取链接
    pub fn coupon_url(mut self, coupon_url: impl Into<String>) -> Self {
        self.coupon_url = Some(coupon_url.into());
        self
    }

    /// 设置子渠道标识
    pub fn sub_union_id(mut self, sub_union_id: impl Into<String>) -> Self {
        self.sub_union_id = Some(sub_union_id.into());
        self
    }

    /// 设置转链类型
    pub fn chain_type(mut self, chain_type: JdChainType) -> Self {
        self.chain_type = Some(chain_type);
        self
    }

    /// 设置礼金批次号
    pub fn gift_coupon_key(mut self, gift_coupon_key: impl Into<String>) -> Self {
        self.gift_coupon_key = Some(gift_coupon_key.into());
        self
    }
}

/// 京粉精选商品请求
///
/// 获取京粉精选商品列表
///
/// # Example
///
/// ```rust,ignore
/// let request = JingfenGoodsRequest::new(JdEliteId::HotSale)
///     .page_index(1)
///     .page_size(20);
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct JingfenGoodsRequest {
    /// 频道 ID
    #[serde(rename = "eliteId")]
    pub elite_id: JdEliteId,
    /// 页码 (从 1 开始)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "pageIndex")]
    pub page_index: Option<u32>,
    /// 每页数量 (最大 20)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "pageSize")]
    pub page_size: Option<u32>,
    /// 排序字段
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "sortName")]
    pub sort_name: Option<JdSortField>,
    /// 排序方向
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<SortDirection>,
    /// 联盟子推客身份标识 (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pid: Option<String>,
    /// 支持出参数据筛选 (可选)
    /// 逗号分隔，可用: videoInfo, hotWords, similar, documentInfo, skuLabelInfo, promotionLabelInfo
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<String>,
    /// 禁售类型 (可选)
    /// 10-微信京东购物小程序禁售, 11-微信京喜小程序禁售
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "forbidTypes")]
    pub forbid_types: Option<String>,
}

impl JingfenGoodsRequest {
    /// 创建新的京粉精选商品请求
    ///
    /// # Arguments
    ///
    /// * `elite_id` - 频道 ID
    pub fn new(elite_id: JdEliteId) -> Self {
        Self {
            elite_id,
            page_index: None,
            page_size: None,
            sort_name: None,
            sort: None,
            pid: None,
            fields: None,
            forbid_types: None,
        }
    }

    /// 设置页码
    pub fn page_index(mut self, page_index: u32) -> Self {
        self.page_index = Some(page_index);
        self
    }

    /// 设置每页数量
    pub fn page_size(mut self, page_size: u32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置排序字段
    pub fn sort_name(mut self, sort_name: JdSortField) -> Self {
        self.sort_name = Some(sort_name);
        self
    }

    /// 设置排序方向
    pub fn sort(mut self, sort: SortDirection) -> Self {
        self.sort = Some(sort);
        self
    }

    /// 设置联盟子推客身份标识
    pub fn pid(mut self, pid: impl Into<String>) -> Self {
        self.pid = Some(pid.into());
        self
    }

    /// 设置出参数据筛选
    pub fn fields(mut self, fields: impl Into<String>) -> Self {
        self.fields = Some(fields.into());
        self
    }

    /// 设置禁售类型
    pub fn forbid_types(mut self, forbid_types: impl Into<String>) -> Self {
        self.forbid_types = Some(forbid_types.into());
        self
    }
}

/// 京东订单查询请求
///
/// 查询京东联盟订单
///
/// # Example
///
/// ```rust,ignore
/// let request = JdOrderQueryRequest::new("2024-01-01 00:00:00")
///     .end_time("2024-01-31 23:59:59")
///     .query_type(JdOrderQueryType::OrderTime)
///     .page_no(1);
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct JdOrderQueryRequest {
    /// 查询开始时间 (格式: yyyy-MM-dd HH:mm:ss)
    #[serde(rename = "startTime")]
    pub start_time: String,
    /// 查询结束时间 (格式: yyyy-MM-dd HH:mm:ss)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "endTime")]
    pub end_time: Option<String>,
    /// 查询类型: 1-按下单时间, 2-按完成时间, 3-按更新时间
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub query_type: Option<JdOrderQueryType>,
    /// 页码 (从 1 开始)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "pageNo")]
    pub page_no: Option<u32>,
    /// 每页数量 (最大 500)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "pageSize")]
    pub page_size: Option<u32>,
    /// 子渠道标识 (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "childUnionId")]
    pub child_union_id: Option<String>,
}

impl JdOrderQueryRequest {
    /// 创建新的京东订单查询请求
    ///
    /// # Arguments
    ///
    /// * `start_time` - 查询开始时间
    pub fn new(start_time: impl Into<String>) -> Self {
        Self {
            start_time: start_time.into(),
            end_time: None,
            query_type: None,
            page_no: None,
            page_size: None,
            child_union_id: None,
        }
    }

    /// 设置查询结束时间
    pub fn end_time(mut self, end_time: impl Into<String>) -> Self {
        self.end_time = Some(end_time.into());
        self
    }

    /// 设置查询类型
    pub fn query_type(mut self, query_type: JdOrderQueryType) -> Self {
        self.query_type = Some(query_type);
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

    /// 设置子渠道标识
    pub fn child_union_id(mut self, child_union_id: impl Into<String>) -> Self {
        self.child_union_id = Some(child_union_id.into());
        self
    }
}

/// 京东商品详情请求
///
/// 获取京东商品详情信息
///
/// # Example
///
/// ```rust,ignore
/// let request = JdGoodsDetailRequest::new("10025768652616");
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct JdGoodsDetailRequest {
    /// 商品 SKU ID 列表 (多个用逗号分隔，最多 100 个)
    #[serde(rename = "skuIds")]
    pub sku_ids: String,
    /// 联盟子推客身份标识 (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pid: Option<String>,
    /// 支持出参数据筛选 (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<String>,
}

impl JdGoodsDetailRequest {
    /// 创建新的京东商品详情请求
    ///
    /// # Arguments
    ///
    /// * `sku_ids` - 商品 SKU ID (多个用逗号分隔)
    pub fn new(sku_ids: impl Into<String>) -> Self {
        Self {
            sku_ids: sku_ids.into(),
            pid: None,
            fields: None,
        }
    }

    /// 设置联盟子推客身份标识
    pub fn pid(mut self, pid: impl Into<String>) -> Self {
        self.pid = Some(pid.into());
        self
    }

    /// 设置出参数据筛选
    pub fn fields(mut self, fields: impl Into<String>) -> Self {
        self.fields = Some(fields.into());
        self
    }
}

/// 京东朋友圈火爆商品请求
///
/// 获取京东朋友圈火爆商品列表
///
/// # Example
///
/// ```rust,ignore
/// let request = JdHotGoodsRequest::new()
///     .page_index(1)
///     .page_size(20)
///     .keyword("手机");
/// ```
#[derive(Debug, Clone, Serialize, Default)]
pub struct JdHotGoodsRequest {
    /// 页码 (从 1 开始)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "pageIndex")]
    pub page_index: Option<u32>,
    /// 每页数量 (最大 20)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "pageSize")]
    pub page_size: Option<u32>,
    /// 商品 ID (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub goods_id: Option<String>,
    /// 关键词 (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyword: Option<String>,
    /// 一级类目 (可选)
    /// 0-全部, 1-居家日用, 2-食品, 3-生鲜, 4-图书, 5-美妆个护,
    /// 6-母婴, 7-数码家电, 8-内衣, 9-配饰, 10-女装, 11-男装,
    /// 12-鞋品, 13-家装家纺, 14-文娱车品, 15-箱包, 16-户外运动
    #[serde(skip_serializing_if = "Option::is_none")]
    pub goods_type: Option<u32>,
    /// 佣金比例起始值 (可选)
    /// 空-全部, 20-佣金比例>=20%, 50-佣金比例>=50%
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commission_rate_start: Option<u32>,
}

impl JdHotGoodsRequest {
    /// 创建新的京东朋友圈火爆商品请求
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置页码
    pub fn page_index(mut self, page_index: u32) -> Self {
        self.page_index = Some(page_index);
        self
    }

    /// 设置每页数量
    pub fn page_size(mut self, page_size: u32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置商品 ID
    pub fn goods_id(mut self, goods_id: impl Into<String>) -> Self {
        self.goods_id = Some(goods_id.into());
        self
    }

    /// 设置关键词
    pub fn keyword(mut self, keyword: impl Into<String>) -> Self {
        self.keyword = Some(keyword.into());
        self
    }

    /// 设置一级类目
    pub fn goods_type(mut self, goods_type: u32) -> Self {
        self.goods_type = Some(goods_type);
        self
    }

    /// 设置佣金比例起始值
    pub fn commission_rate_start(mut self, commission_rate_start: u32) -> Self {
        self.commission_rate_start = Some(commission_rate_start);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jd_convert_request_serialize() {
        let request = JdConvertRequest::new("https://item.jd.com/123456.html", "1000000001");
        let json = serde_json::to_value(&request).unwrap();

        assert_eq!(json["materialId"], "https://item.jd.com/123456.html");
        assert_eq!(json["unionId"], "1000000001");
        assert!(json.get("positionId").is_none());
    }

    #[test]
    fn test_jd_convert_request_with_options() {
        let request = JdConvertRequest::new("https://item.jd.com/123456.html", "1000000001")
            .position_id("12345")
            .chain_type(JdChainType::Short);

        let json = serde_json::to_value(&request).unwrap();

        assert_eq!(json["positionId"], "12345");
        assert_eq!(json["chainType"], 2);
    }

    #[test]
    fn test_jingfen_goods_request_serialize() {
        let request = JingfenGoodsRequest::new(JdEliteId::HotSale)
            .page_index(1)
            .page_size(20);

        let json = serde_json::to_value(&request).unwrap();

        assert_eq!(json["eliteId"], 22);
        assert_eq!(json["pageIndex"], 1);
        assert_eq!(json["pageSize"], 20);
    }

    #[test]
    fn test_jingfen_goods_request_with_sort() {
        let request = JingfenGoodsRequest::new(JdEliteId::HighCommission)
            .sort_name(JdSortField::CommissionShare)
            .sort(SortDirection::Desc);

        let json = serde_json::to_value(&request).unwrap();

        assert_eq!(json["eliteId"], 129);
        assert_eq!(json["sortName"], "commissionShare");
        assert_eq!(json["sort"], "desc");
    }

    #[test]
    fn test_jd_order_query_request_serialize() {
        let request = JdOrderQueryRequest::new("2024-01-01 00:00:00")
            .end_time("2024-01-31 23:59:59")
            .query_type(JdOrderQueryType::OrderTime)
            .page_no(1)
            .page_size(100);

        let json = serde_json::to_value(&request).unwrap();

        assert_eq!(json["startTime"], "2024-01-01 00:00:00");
        assert_eq!(json["endTime"], "2024-01-31 23:59:59");
        assert_eq!(json["type"], 1);
        assert_eq!(json["pageNo"], 1);
        assert_eq!(json["pageSize"], 100);
    }

    #[test]
    fn test_jd_goods_detail_request_serialize() {
        let request = JdGoodsDetailRequest::new("10025768652616,10025768652617");
        let json = serde_json::to_value(&request).unwrap();

        assert_eq!(json["skuIds"], "10025768652616,10025768652617");
    }

    #[test]
    fn test_jd_hot_goods_request_serialize() {
        let request = JdHotGoodsRequest::new()
            .page_index(1)
            .page_size(20)
            .keyword("手机")
            .goods_type(7);

        let json = serde_json::to_value(&request).unwrap();

        assert_eq!(json["pageIndex"], 1);
        assert_eq!(json["pageSize"], 20);
        assert_eq!(json["keyword"], "手机");
        assert_eq!(json["goods_type"], 7);
    }

    #[test]
    fn test_optional_fields_not_serialized() {
        let request = JdConvertRequest::new("material", "union");
        let json_str = serde_json::to_string(&request).unwrap();

        assert!(!json_str.contains("positionId"));
        assert!(!json_str.contains("couponUrl"));
        assert!(!json_str.contains("chainType"));
    }
}
