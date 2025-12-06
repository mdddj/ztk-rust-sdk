//! 考拉平台响应结构体
//!
//! 定义考拉平台 API 的响应结构体

use serde::Deserialize;

/// 考拉转链响应
///
/// convert 接口的响应
#[derive(Debug, Clone, Deserialize)]
pub struct KaolaConvertResponse {
    /// 返回码，200 表示成功
    #[serde(default)]
    pub code: Option<i32>,
    /// 返回消息
    #[serde(default)]
    pub msg: Option<String>,
    /// 转链结果列表
    #[serde(default)]
    pub data: Option<Vec<KaolaConvertResult>>,
}

/// 考拉转链结果
#[derive(Debug, Clone, Deserialize)]
pub struct KaolaConvertResult {
    /// 原始链接
    #[serde(default, rename = "originalLink")]
    pub original_link: Option<String>,
    /// 推广长链接
    #[serde(default, rename = "shareLink")]
    pub share_link: Option<String>,
    /// 推广短链接
    #[serde(default, rename = "shortLink")]
    pub short_link: Option<String>,
}

/// 考拉精选商品列表响应
///
/// goods_list 接口的响应
#[derive(Debug, Clone, Deserialize)]
pub struct KaolaGoodsListResponse {
    /// 返回码，200 表示成功
    #[serde(default)]
    pub code: Option<i32>,
    /// 返回消息
    #[serde(default)]
    pub msg: Option<String>,
    /// 商品列表数据
    #[serde(default)]
    pub data: Option<KaolaGoodsListData>,
}

/// 考拉精选商品列表数据
#[derive(Debug, Clone, Deserialize)]
pub struct KaolaGoodsListData {
    /// 商品 ID 列表
    #[serde(default, rename = "goodsIdList")]
    pub goods_id_list: Option<Vec<i64>>,
    /// 页码
    #[serde(default, rename = "pageNo")]
    pub page_no: Option<i32>,
    /// 每页数量
    #[serde(default, rename = "pageSize")]
    pub page_size: Option<i32>,
    /// 总记录数
    #[serde(default, rename = "totalRecord")]
    pub total_record: Option<i64>,
}

/// 考拉商品搜索响应
///
/// search_goods 接口的响应
#[derive(Debug, Clone, Deserialize)]
pub struct KaolaSearchGoodsResponse {
    /// 返回码，200 表示成功
    #[serde(default)]
    pub code: Option<i32>,
    /// 返回消息
    #[serde(default)]
    pub msg: Option<String>,
    /// 搜索结果数据
    #[serde(default)]
    pub data: Option<KaolaSearchGoodsData>,
}

/// 考拉商品搜索数据
#[derive(Debug, Clone, Deserialize)]
pub struct KaolaSearchGoodsData {
    /// 商品列表
    #[serde(default, rename = "dataList")]
    pub data_list: Option<Vec<KaolaGoodsInfo>>,
    /// 页码
    #[serde(default, rename = "pageNo")]
    pub page_no: Option<i32>,
    /// 每页数量
    #[serde(default, rename = "pageSize")]
    pub page_size: Option<i32>,
    /// 总数量
    #[serde(default)]
    pub total: Option<i64>,
}

/// 考拉商品详情响应
///
/// goods_detail 接口的响应
#[derive(Debug, Clone, Deserialize)]
pub struct KaolaGoodsDetailResponse {
    /// 返回码，200 表示成功
    #[serde(default)]
    pub code: Option<i32>,
    /// 返回消息
    #[serde(default)]
    pub msg: Option<String>,
    /// 商品详情列表
    #[serde(default)]
    pub data: Option<Vec<KaolaGoodsInfo>>,
}

/// 考拉商品信息
#[derive(Debug, Clone, Deserialize)]
pub struct KaolaGoodsInfo {
    /// 商品 ID
    #[serde(default, rename = "goodsId")]
    pub goods_id: Option<i64>,
    /// 活动信息
    #[serde(default, rename = "activityInfo")]
    pub activity_info: Option<KaolaActivityInfo>,
    /// 基础信息
    #[serde(default, rename = "baseInfo")]
    pub base_info: Option<KaolaBaseInfo>,
    /// 分类信息
    #[serde(default, rename = "categoryInfo")]
    pub category_info: Option<Vec<KaolaCategoryInfo>>,
    /// 佣金信息
    #[serde(default, rename = "commissionInfo")]
    pub commission_info: Option<KaolaCommissionInfo>,
    /// 链接信息
    #[serde(default, rename = "linkInfo")]
    pub link_info: Option<KaolaLinkInfo>,
    /// 价格信息
    #[serde(default, rename = "priceInfo")]
    pub price_info: Option<KaolaPriceInfo>,
    /// 促销信息
    #[serde(default, rename = "promotionInfo")]
    pub promotion_info: Option<Vec<KaolaPromotionInfo>>,
}

/// 考拉活动信息
#[derive(Debug, Clone, Deserialize)]
pub struct KaolaActivityInfo {
    /// 活动标签
    #[serde(default, rename = "activityLable")]
    pub activity_label: Option<String>,
    /// 是否包邮: 0-否, 1-是
    #[serde(default, rename = "noPostage")]
    pub no_postage: Option<i32>,
}

/// 考拉商品基础信息
#[derive(Debug, Clone, Deserialize)]
pub struct KaolaBaseInfo {
    /// 品牌国家名称
    #[serde(default, rename = "brandCountryName")]
    pub brand_country_name: Option<String>,
    /// 品牌名称
    #[serde(default, rename = "brandName")]
    pub brand_name: Option<String>,
    /// 详情图列表
    #[serde(default, rename = "detailImgList")]
    pub detail_img_list: Option<Vec<String>>,
    /// 商品副标题
    #[serde(default, rename = "goodsSubTitle")]
    pub goods_sub_title: Option<String>,
    /// 商品标题
    #[serde(default, rename = "goodsTitle")]
    pub goods_title: Option<String>,
    /// 是否团购商品: 0-否, 1-是
    #[serde(default, rename = "groupBuyGoods")]
    pub group_buy_goods: Option<i32>,
    /// 商品图片列表
    #[serde(default, rename = "imageList")]
    pub image_list: Option<Vec<String>>,
    /// 进口类型
    #[serde(default, rename = "importType")]
    pub import_type: Option<i32>,
    /// 是否跨境购: 0-否, 1-是
    #[serde(default, rename = "interPurch")]
    pub inter_purch: Option<i32>,
    /// 上架状态: 0-下架, 1-上架
    #[serde(default, rename = "onlineStatus")]
    pub online_status: Option<i32>,
    /// 是否自营: 0-否, 1-是
    #[serde(default, rename = "self")]
    pub is_self: Option<i32>,
    /// 是否有库存: 0-否, 1-是
    #[serde(default)]
    pub store: Option<i32>,
}

/// 考拉分类信息
#[derive(Debug, Clone, Deserialize)]
pub struct KaolaCategoryInfo {
    /// 分类 ID
    #[serde(default, rename = "categoryId")]
    pub category_id: Option<i64>,
    /// 分类名称
    #[serde(default, rename = "categoryName")]
    pub category_name: Option<String>,
    /// 分类层级
    #[serde(default)]
    pub level: Option<i32>,
}

/// 考拉佣金信息
#[derive(Debug, Clone, Deserialize)]
pub struct KaolaCommissionInfo {
    /// 佣金比例 (小数形式，如 0.032 表示 3.2%)
    #[serde(default, rename = "commissionRate")]
    pub commission_rate: Option<f64>,
    /// 过期类型: 0-有时间限制, 1-无时间限制
    #[serde(default, rename = "expireType")]
    pub expire_type: Option<i32>,
    /// 开始时间
    #[serde(default, rename = "startTime")]
    pub start_time: Option<String>,
    /// 结束时间
    #[serde(default, rename = "endTime")]
    pub end_time: Option<String>,
}

/// 考拉链接信息
#[derive(Debug, Clone, Deserialize)]
pub struct KaolaLinkInfo {
    /// 商品详情页 URL (移动端)
    #[serde(default, rename = "goodsDetailUrl")]
    pub goods_detail_url: Option<String>,
    /// 商品详情页 URL (PC 端)
    #[serde(default, rename = "goodsPCUrl")]
    pub goods_pc_url: Option<String>,
    /// 小程序分享链接
    #[serde(default, rename = "miniShareUrl")]
    pub mini_share_url: Option<String>,
    /// 推广分享链接
    #[serde(default, rename = "shareUrl")]
    pub share_url: Option<String>,
    /// 推广短链接
    #[serde(default, rename = "shortShareUrl")]
    pub short_share_url: Option<String>,
}

/// 考拉价格信息
#[derive(Debug, Clone, Deserialize)]
pub struct KaolaPriceInfo {
    /// 当前价格 (元)
    #[serde(default, rename = "currentPrice")]
    pub current_price: Option<f64>,
    /// 市场价格 (元)
    #[serde(default, rename = "marketPrice")]
    pub market_price: Option<f64>,
    /// 会员当前价格 (元)
    #[serde(default, rename = "memberCurrentPrice")]
    pub member_current_price: Option<f64>,
    /// 会员价差 (元)
    #[serde(default, rename = "memberPriceSpread")]
    pub member_price_spread: Option<f64>,
}

/// 考拉促销信息
#[derive(Debug, Clone, Deserialize)]
pub struct KaolaPromotionInfo {
    /// 活动商品销量
    #[serde(default, rename = "activityGoodsSellCount")]
    pub activity_goods_sell_count: Option<i64>,
    /// 活动商品库存
    #[serde(default, rename = "activityGoodsStore")]
    pub activity_goods_store: Option<i64>,
    /// 适用用户类型
    #[serde(default, rename = "applyUserType")]
    pub apply_user_type: Option<i32>,
    /// 结束时间
    #[serde(default, rename = "endTime")]
    pub end_time: Option<String>,
    /// 商品 ID
    #[serde(default, rename = "goodsId")]
    pub goods_id: Option<i64>,
    /// 促销价格 (元)
    #[serde(default, rename = "promotionSalePrice")]
    pub promotion_sale_price: Option<f64>,
    /// SKU ID
    #[serde(default, rename = "skuId")]
    pub sku_id: Option<String>,
    /// 开始时间
    #[serde(default, rename = "startTime")]
    pub start_time: Option<String>,
    /// 是否限购
    #[serde(default, rename = "storeLimit")]
    pub store_limit: Option<bool>,
}

/// 考拉订单查询响应
///
/// query_orders 接口的响应
#[derive(Debug, Clone, Deserialize)]
pub struct KaolaOrderResponse {
    /// 返回码，200 表示成功
    #[serde(default)]
    pub code: Option<i32>,
    /// 返回消息
    #[serde(default)]
    pub msg: Option<String>,
    /// 订单数据
    #[serde(default)]
    pub data: Option<KaolaOrderData>,
}

/// 考拉订单数据
#[derive(Debug, Clone, Deserialize)]
pub struct KaolaOrderData {
    /// 订单列表
    #[serde(default, rename = "orderList")]
    pub order_list: Option<Vec<KaolaOrderInfo>>,
    /// 页码
    #[serde(default)]
    pub page: Option<i32>,
    /// 每页数量
    #[serde(default)]
    pub page_size: Option<i32>,
    /// 总数量
    #[serde(default)]
    pub total: Option<i64>,
}

/// 考拉订单信息
#[derive(Debug, Clone, Deserialize)]
pub struct KaolaOrderInfo {
    /// 订单号
    #[serde(default, rename = "orderSn")]
    pub order_sn: Option<String>,
    /// 商品 ID
    #[serde(default, rename = "goodsId")]
    pub goods_id: Option<String>,
    /// 商品名称
    #[serde(default, rename = "goodsName")]
    pub goods_name: Option<String>,
    /// 商品图片
    #[serde(default, rename = "goodsThumb")]
    pub goods_thumb: Option<String>,
    /// 商品数量
    #[serde(default, rename = "goodsCount")]
    pub goods_count: Option<i32>,
    /// 商品单价 (元)
    #[serde(default, rename = "goodsPrice")]
    pub goods_price: Option<String>,
    /// 订单金额 (元)
    #[serde(default, rename = "orderAmount")]
    pub order_amount: Option<String>,
    /// 佣金金额 (元)
    #[serde(default)]
    pub commission: Option<String>,
    /// 佣金比例 (%)
    #[serde(default, rename = "commissionRate")]
    pub commission_rate: Option<String>,
    /// 订单状态
    #[serde(default)]
    pub status: Option<i32>,
    /// 订单状态描述
    #[serde(default, rename = "statusName")]
    pub status_name: Option<String>,
    /// 订单时间
    #[serde(default, rename = "orderTime")]
    pub order_time: Option<String>,
    /// 更新时间
    #[serde(default, rename = "updateTime")]
    pub update_time: Option<String>,
    /// 自定义参数
    #[serde(default, rename = "trackingCode2")]
    pub tracking_code2: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kaola_convert_response_deserialize() {
        let json = r#"{
            "code": 200,
            "msg": "SUCCESS",
            "data": [
                {
                    "originalLink": "http://lu.kaola.com/50uT0o",
                    "shareLink": "https://cps.kaola.com/cps/zhuankeLogin?unionId=xxx",
                    "shortLink": "http://lu.kaola.com/1Sv5Fq"
                }
            ]
        }"#;

        let response: KaolaConvertResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.code, Some(200));
        assert_eq!(response.msg, Some("SUCCESS".to_string()));
        assert!(response.data.is_some());
        let data = response.data.unwrap();
        assert_eq!(data.len(), 1);
        assert_eq!(
            data[0].original_link,
            Some("http://lu.kaola.com/50uT0o".to_string())
        );
        assert_eq!(
            data[0].short_link,
            Some("http://lu.kaola.com/1Sv5Fq".to_string())
        );
    }

    #[test]
    fn test_kaola_goods_list_response_deserialize() {
        let json = r#"{
            "code": 200,
            "msg": "SUCCESS",
            "data": {
                "goodsIdList": [2480622, 5245545, 1631853],
                "pageNo": 1,
                "pageSize": 20,
                "totalRecord": 693
            }
        }"#;

        let response: KaolaGoodsListResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.code, Some(200));
        assert!(response.data.is_some());
        let data = response.data.unwrap();
        assert!(data.goods_id_list.is_some());
        let goods_ids = data.goods_id_list.unwrap();
        assert_eq!(goods_ids.len(), 3);
        assert_eq!(goods_ids[0], 2480622);
        assert_eq!(data.total_record, Some(693));
    }

    #[test]
    fn test_kaola_search_goods_response_deserialize() {
        let json = r#"{
            "code": 200,
            "msg": "SUCCESS",
            "data": {
                "dataList": [
                    {
                        "goodsId": 8434647,
                        "baseInfo": {
                            "brandName": "NIKE 耐克",
                            "goodsTitle": "NIKE 耐克小空一女子休闲鞋",
                            "onlineStatus": 1,
                            "self": 1
                        },
                        "priceInfo": {
                            "currentPrice": 399,
                            "marketPrice": 549
                        },
                        "commissionInfo": {
                            "commissionRate": 0.032,
                            "expireType": 1
                        }
                    }
                ],
                "pageNo": 1,
                "pageSize": 1,
                "total": 2195
            }
        }"#;

        let response: KaolaSearchGoodsResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.code, Some(200));
        assert!(response.data.is_some());
        let data = response.data.unwrap();
        assert_eq!(data.total, Some(2195));
        assert!(data.data_list.is_some());
        let goods = data.data_list.unwrap();
        assert_eq!(goods.len(), 1);
        assert_eq!(goods[0].goods_id, Some(8434647));
    }

    #[test]
    fn test_kaola_goods_detail_response_deserialize() {
        let json = r#"{
            "code": 200,
            "msg": "SUCCESS",
            "data": [
                {
                    "goodsId": 2480622,
                    "baseInfo": {
                        "brandCountryName": "美国",
                        "brandName": "vitafusion",
                        "goodsTitle": "vitafusion 褪黑素软糖 60粒/瓶*3瓶",
                        "onlineStatus": 1,
                        "self": 1,
                        "store": 1
                    },
                    "priceInfo": {
                        "currentPrice": 192,
                        "marketPrice": 429,
                        "memberCurrentPrice": 164.16,
                        "memberPriceSpread": 27.84
                    },
                    "commissionInfo": {
                        "commissionRate": 0.172,
                        "expireType": 0,
                        "startTime": "2022-01-07 00:00:00",
                        "endTime": "2022-01-15 23:59:59"
                    },
                    "linkInfo": {
                        "goodsDetailUrl": "https://m-goods.kaola.com/product/2480622.html",
                        "shortShareUrl": "http://lu.kaola.com/3tmWKg"
                    }
                }
            ]
        }"#;

        let response: KaolaGoodsDetailResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.code, Some(200));
        assert!(response.data.is_some());
        let data = response.data.unwrap();
        assert_eq!(data.len(), 1);
        assert_eq!(data[0].goods_id, Some(2480622));

        let base_info = data[0].base_info.as_ref().unwrap();
        assert_eq!(base_info.brand_name, Some("vitafusion".to_string()));

        let price_info = data[0].price_info.as_ref().unwrap();
        assert_eq!(price_info.current_price, Some(192.0));

        let commission_info = data[0].commission_info.as_ref().unwrap();
        assert_eq!(commission_info.commission_rate, Some(0.172));
    }

    #[test]
    fn test_kaola_order_response_deserialize() {
        let json = r#"{
            "code": 200,
            "msg": "SUCCESS",
            "data": {
                "orderList": [
                    {
                        "orderSn": "123456789",
                        "goodsId": "2480622",
                        "goodsName": "测试商品",
                        "goodsCount": 1,
                        "orderAmount": "192.00",
                        "commission": "33.02",
                        "commissionRate": "17.2",
                        "status": 2,
                        "statusName": "已完成",
                        "orderTime": "2023-01-15 10:30:00"
                    }
                ],
                "page": 1,
                "page_size": 50,
                "total": 1
            }
        }"#;

        let response: KaolaOrderResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.code, Some(200));
        assert!(response.data.is_some());
        let data = response.data.unwrap();
        assert_eq!(data.total, Some(1));
        assert!(data.order_list.is_some());
        let orders = data.order_list.unwrap();
        assert_eq!(orders.len(), 1);
        assert_eq!(orders[0].order_sn, Some("123456789".to_string()));
        assert_eq!(orders[0].commission, Some("33.02".to_string()));
    }

    #[test]
    fn test_response_with_missing_fields() {
        let json = r#"{"code": 200}"#;

        let response: KaolaConvertResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.code, Some(200));
        assert!(response.msg.is_none());
        assert!(response.data.is_none());
    }
}
