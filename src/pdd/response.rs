//! 拼多多平台响应结构体
//!
//! 定义拼多多平台 API 的响应结构体

use serde::Deserialize;

/// 拼多多转链响应
///
/// convert 接口的响应
#[derive(Debug, Clone, Deserialize)]
pub struct PddConvertResponse {
    /// 转链结果
    #[serde(default, rename = "goods_zs_unit_generate_response")]
    pub response: Option<PddConvertResult>,
}

/// 拼多多转链结果
#[derive(Debug, Clone, Deserialize)]
pub struct PddConvertResult {
    /// 推广短链接 (唤起拼多多 APP)
    #[serde(default)]
    pub multi_group_mobile_short_url: Option<String>,
    /// 双人团推广长链接
    #[serde(default)]
    pub multi_group_url: Option<String>,
    /// 普通长链，微信环境下进入领券页点领券拉起小程序
    #[serde(default)]
    pub mobile_url: Option<String>,
    /// 双人团推广短链接
    #[serde(default)]
    pub multi_group_short_url: Option<String>,
    /// 对应 mobile_url 的短链接
    #[serde(default)]
    pub mobile_short_url: Option<String>,
    /// 推广长链接 (可唤起拼多多 APP)
    #[serde(default)]
    pub multi_group_mobile_url: Option<String>,
    /// 请求 ID
    #[serde(default)]
    pub request_id: Option<String>,
    /// 普通长链，微信环境下优先拉起微信小程序
    #[serde(default)]
    pub url: Option<String>,
    /// 对应 url 的短链接
    #[serde(default)]
    pub short_url: Option<String>,
}

/// 拼多多商品详情响应 (简版)
///
/// goods_detail_simple 接口的响应
#[derive(Debug, Clone, Deserialize)]
pub struct PddGoodsDetailSimpleResponse {
    /// 商品详情结果
    #[serde(default, rename = "goods_basic_detail_response")]
    pub response: Option<PddGoodsBasicDetailResult>,
}

/// 拼多多商品基本详情结果
#[derive(Debug, Clone, Deserialize)]
pub struct PddGoodsBasicDetailResult {
    /// 商品列表
    #[serde(default)]
    pub goods_list: Option<Vec<PddGoodsBasicItem>>,
    /// 请求 ID
    #[serde(default)]
    pub request_id: Option<String>,
}

/// 拼多多商品基本信息
#[derive(Debug, Clone, Deserialize)]
pub struct PddGoodsBasicItem {
    /// 商品名称
    #[serde(default)]
    pub goods_name: Option<String>,
    /// 商品主图
    #[serde(default)]
    pub goods_pic: Option<String>,
    /// 商品原价 (单位: 分)
    #[serde(default)]
    pub min_normal_price: Option<i64>,
    /// 商品 ID
    #[serde(default)]
    pub goods_id: Option<i64>,
    /// 拼团价 (单位: 分)
    #[serde(default)]
    pub min_group_price: Option<i64>,
}

/// 拼多多商品详情响应 (详版)
///
/// goods_detail_full 接口的响应
#[derive(Debug, Clone, Deserialize)]
pub struct PddGoodsDetailFullResponse {
    /// 商品详情结果
    #[serde(default, rename = "goods_search_response")]
    pub response: Option<PddGoodsSearchResult>,
}

/// 拼多多商品搜索结果
#[derive(Debug, Clone, Deserialize)]
pub struct PddGoodsSearchResult {
    /// 商品列表
    #[serde(default)]
    pub goods_list: Option<Vec<PddGoodsDetailItem>>,
    /// 请求 ID
    #[serde(default)]
    pub request_id: Option<String>,
    /// 总数量
    #[serde(default)]
    pub total_count: Option<i64>,
}

/// 拼多多商品详细信息
#[derive(Debug, Clone, Deserialize)]
pub struct PddGoodsDetailItem {
    /// 商品 ID
    #[serde(default)]
    pub goods_id: Option<i64>,
    /// 商品名称
    #[serde(default)]
    pub goods_name: Option<String>,
    /// 商品描述
    #[serde(default)]
    pub goods_desc: Option<String>,
    /// 商品主图
    #[serde(default)]
    pub goods_thumbnail_url: Option<String>,
    /// 商品轮播图
    #[serde(default)]
    pub goods_image_url: Option<String>,
    /// 商品详情图列表
    #[serde(default)]
    pub goods_gallery_urls: Option<Vec<String>>,
    /// 商品原价 (单位: 分)
    #[serde(default)]
    pub min_normal_price: Option<i64>,
    /// 拼团价 (单位: 分)
    #[serde(default)]
    pub min_group_price: Option<i64>,
    /// 已售数量
    #[serde(default)]
    pub sales_tip: Option<String>,
    /// 佣金比例 (千分比)
    #[serde(default)]
    pub promotion_rate: Option<i64>,
    /// 优惠券面额 (单位: 分)
    #[serde(default)]
    pub coupon_discount: Option<i64>,
    /// 优惠券门槛 (单位: 分)
    #[serde(default)]
    pub coupon_min_order_amount: Option<i64>,
    /// 优惠券剩余数量
    #[serde(default)]
    pub coupon_remain_quantity: Option<i64>,
    /// 优惠券总数量
    #[serde(default)]
    pub coupon_total_quantity: Option<i64>,
    /// 优惠券开始时间 (秒级时间戳)
    #[serde(default)]
    pub coupon_start_time: Option<i64>,
    /// 优惠券结束时间 (秒级时间戳)
    #[serde(default)]
    pub coupon_end_time: Option<i64>,
    /// 店铺 ID
    #[serde(default)]
    pub mall_id: Option<i64>,
    /// 店铺名称
    #[serde(default)]
    pub mall_name: Option<String>,
    /// 店铺类型: 1-个人，2-企业，3-旗舰店，4-专卖店，5-专营店，6-普通店
    #[serde(default)]
    pub merchant_type: Option<i32>,
    /// 商品类目 ID
    #[serde(default)]
    pub cat_id: Option<i64>,
    /// 商品类目名称
    #[serde(default)]
    pub cat_name: Option<String>,
    /// 一级类目 ID
    #[serde(default, rename = "cat_id_1")]
    pub cat_id1: Option<i64>,
    /// 二级类目 ID
    #[serde(default, rename = "cat_id_2")]
    pub cat_id2: Option<i64>,
    /// 三级类目 ID
    #[serde(default, rename = "cat_id_3")]
    pub cat_id3: Option<i64>,
    /// 一级类目名称
    #[serde(default, rename = "cat_name_1")]
    pub cat_name1: Option<String>,
    /// 二级类目名称
    #[serde(default, rename = "cat_name_2")]
    pub cat_name2: Option<String>,
    /// 三级类目名称
    #[serde(default, rename = "cat_name_3")]
    pub cat_name3: Option<String>,
    /// 是否有优惠券
    #[serde(default)]
    pub has_coupon: Option<bool>,
    /// 是否品牌商品
    #[serde(default)]
    pub is_brand_goods: Option<bool>,
    /// 描述评分
    #[serde(default)]
    pub desc_txt: Option<String>,
    /// 物流评分
    #[serde(default)]
    pub lgst_txt: Option<String>,
    /// 服务评分
    #[serde(default)]
    pub serv_txt: Option<String>,
    /// 活动标签
    #[serde(default)]
    pub activity_tags: Option<Vec<i32>>,
}

/// 拼多多订单查询响应
///
/// query_orders 接口的响应
#[derive(Debug, Clone, Deserialize)]
pub struct PddOrderResponse {
    /// 订单查询结果
    #[serde(default, rename = "order_list_get_response")]
    pub response: Option<PddOrderListResult>,
}

/// 拼多多订单列表结果
#[derive(Debug, Clone, Deserialize)]
pub struct PddOrderListResult {
    /// 订单总数
    #[serde(default)]
    pub total_count: Option<i64>,
    /// 订单列表
    #[serde(default)]
    pub order_list: Option<Vec<PddOrderInfo>>,
    /// 请求 ID
    #[serde(default)]
    pub request_id: Option<String>,
}

/// 拼多多订单信息
#[derive(Debug, Clone, Deserialize)]
pub struct PddOrderInfo {
    /// 订单号
    #[serde(default)]
    pub order_sn: Option<String>,
    /// 商品 ID
    #[serde(default)]
    pub goods_id: Option<i64>,
    /// 商品名称
    #[serde(default)]
    pub goods_name: Option<String>,
    /// 商品主图
    #[serde(default)]
    pub goods_thumbnail_url: Option<String>,
    /// 商品数量
    #[serde(default)]
    pub goods_quantity: Option<i32>,
    /// 商品价格 (单位: 分)
    #[serde(default)]
    pub goods_price: Option<i64>,
    /// 订单金额 (单位: 分)
    #[serde(default)]
    pub order_amount: Option<i64>,
    /// 佣金金额 (单位: 分)
    #[serde(default)]
    pub promotion_amount: Option<i64>,
    /// 佣金比例 (千分比)
    #[serde(default)]
    pub promotion_rate: Option<i64>,
    /// 订单状态: -1-未支付，0-已支付，1-已成团，2-确认收货，3-审核成功，4-审核失败，5-已经结算，8-非多多进宝商品
    #[serde(default)]
    pub order_status: Option<i32>,
    /// 订单状态描述
    #[serde(default)]
    pub order_status_desc: Option<String>,
    /// 订单创建时间 (秒级时间戳)
    #[serde(default)]
    pub order_create_time: Option<i64>,
    /// 订单支付时间 (秒级时间戳)
    #[serde(default)]
    pub order_pay_time: Option<i64>,
    /// 订单成团时间 (秒级时间戳)
    #[serde(default)]
    pub order_group_success_time: Option<i64>,
    /// 订单确认收货时间 (秒级时间戳)
    #[serde(default)]
    pub order_receive_time: Option<i64>,
    /// 订单结算时间 (秒级时间戳)
    #[serde(default)]
    pub order_settle_time: Option<i64>,
    /// 订单更新时间 (秒级时间戳)
    #[serde(default)]
    pub order_modify_at: Option<i64>,
    /// 自定义参数
    #[serde(default)]
    pub custom_parameters: Option<String>,
    /// 推广位 ID
    #[serde(default)]
    pub p_id: Option<String>,
    /// 是否直推: 1-直推，0-非直推
    #[serde(default)]
    pub type_field: Option<i32>,
    /// 订单类型: 0-普通订单，1-礼金订单
    #[serde(default)]
    pub order_type: Option<i32>,
    /// 店铺 ID
    #[serde(default)]
    pub mall_id: Option<i64>,
    /// 店铺名称
    #[serde(default)]
    pub mall_name: Option<String>,
    /// 类目 ID
    #[serde(default)]
    pub cat_ids: Option<Vec<i64>>,
}

/// 拼多多授权备案响应
///
/// authorize 接口的响应
#[derive(Debug, Clone, Deserialize)]
pub struct PddAuthorizeResponse {
    /// 授权备案结果
    #[serde(default, rename = "rp_promotion_url_generate_response")]
    pub response: Option<PddAuthorizeResult>,
}

/// 拼多多授权备案结果
#[derive(Debug, Clone, Deserialize)]
pub struct PddAuthorizeResult {
    /// URL 列表
    #[serde(default)]
    pub url_list: Option<Vec<PddAuthorizeUrl>>,
    /// 请求 ID
    #[serde(default)]
    pub request_id: Option<String>,
}

/// 拼多多授权备案 URL
#[derive(Debug, Clone, Deserialize)]
pub struct PddAuthorizeUrl {
    /// 移动版授权备案地址
    #[serde(default)]
    pub mobile_url: Option<String>,
    /// 电脑版授权备案地址
    #[serde(default)]
    pub url: Option<String>,
}

/// 拼多多授权备案查询响应
///
/// authorize_query 接口的响应
#[derive(Debug, Clone, Deserialize)]
pub struct PddAuthorizeQueryResponse {
    /// 授权查询结果
    #[serde(default, rename = "authority_query_response")]
    pub response: Option<PddAuthorizeQueryResult>,
}

/// 拼多多授权备案查询结果
#[derive(Debug, Clone, Deserialize)]
pub struct PddAuthorizeQueryResult {
    /// 绑定状态: 1 表示授权备案成功，其他值表示失败
    #[serde(default)]
    pub bind: Option<i32>,
    /// 请求 ID
    #[serde(default)]
    pub request_id: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pdd_convert_response_deserialize() {
        let json = r#"{
            "goods_zs_unit_generate_response": {
                "multi_group_mobile_short_url": "https://p.pinduoduo.com/2sB0eGpK",
                "mobile_url": "https://mobile.yangkeduo.com/duo_coupon_landing.html",
                "short_url": "https://p.pinduoduo.com/bOL0pBPH",
                "request_id": "16942627387113959"
            }
        }"#;

        let response: PddConvertResponse = serde_json::from_str(json).unwrap();
        assert!(response.response.is_some());
        let result = response.response.unwrap();
        assert_eq!(
            result.multi_group_mobile_short_url,
            Some("https://p.pinduoduo.com/2sB0eGpK".to_string())
        );
        assert_eq!(
            result.short_url,
            Some("https://p.pinduoduo.com/bOL0pBPH".to_string())
        );
    }

    #[test]
    fn test_pdd_goods_detail_simple_response_deserialize() {
        let json = r#"{
            "goods_basic_detail_response": {
                "goods_list": [{
                    "goods_name": "测试商品",
                    "goods_pic": "https://img.pddpic.com/test.jpeg",
                    "min_normal_price": 1900,
                    "goods_id": 453581732819,
                    "min_group_price": 990
                }],
                "request_id": "16942480364163726"
            }
        }"#;

        let response: PddGoodsDetailSimpleResponse = serde_json::from_str(json).unwrap();
        assert!(response.response.is_some());
        let result = response.response.unwrap();
        assert!(result.goods_list.is_some());
        let goods = result.goods_list.unwrap();
        assert_eq!(goods.len(), 1);
        assert_eq!(goods[0].goods_name, Some("测试商品".to_string()));
        assert_eq!(goods[0].goods_id, Some(453581732819));
    }

    #[test]
    fn test_pdd_order_response_deserialize() {
        let json = r#"{
            "order_list_get_response": {
                "total_count": 1,
                "order_list": [{
                    "order_sn": "123456789",
                    "goods_id": 453581732819,
                    "goods_name": "测试商品",
                    "order_status": 2,
                    "order_amount": 9900,
                    "promotion_amount": 990
                }],
                "request_id": "16942626445817516"
            }
        }"#;

        let response: PddOrderResponse = serde_json::from_str(json).unwrap();
        assert!(response.response.is_some());
        let result = response.response.unwrap();
        assert_eq!(result.total_count, Some(1));
        assert!(result.order_list.is_some());
        let orders = result.order_list.unwrap();
        assert_eq!(orders.len(), 1);
        assert_eq!(orders[0].order_sn, Some("123456789".to_string()));
        assert_eq!(orders[0].order_status, Some(2));
    }

    #[test]
    fn test_pdd_authorize_response_deserialize() {
        let json = r#"{
            "rp_promotion_url_generate_response": {
                "url_list": [{
                    "mobile_url": "https://mobile.yangkeduo.com/duo_coupon_landing.html_",
                    "url": "https://mobile.yangkeduo.com/duo_coupon_landing.html"
                }],
                "request_id": "16942480613190789"
            }
        }"#;

        let response: PddAuthorizeResponse = serde_json::from_str(json).unwrap();
        assert!(response.response.is_some());
        let result = response.response.unwrap();
        assert!(result.url_list.is_some());
        let urls = result.url_list.unwrap();
        assert_eq!(urls.len(), 1);
        assert!(urls[0].mobile_url.is_some());
    }

    #[test]
    fn test_pdd_authorize_query_response_deserialize() {
        let json = r#"{
            "authority_query_response": {
                "bind": 1,
                "request_id": "16942467231374196"
            }
        }"#;

        let response: PddAuthorizeQueryResponse = serde_json::from_str(json).unwrap();
        assert!(response.response.is_some());
        let result = response.response.unwrap();
        assert_eq!(result.bind, Some(1));
    }

    #[test]
    fn test_response_with_missing_fields() {
        let json = r#"{"order_list_get_response": {"total_count": 0}}"#;

        let response: PddOrderResponse = serde_json::from_str(json).unwrap();
        assert!(response.response.is_some());
        let result = response.response.unwrap();
        assert_eq!(result.total_count, Some(0));
        assert!(result.order_list.is_none());
    }
}
