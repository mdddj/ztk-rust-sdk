//! 唯品会平台响应结构体
//!
//! 定义唯品会平台 API 的响应结构体

use serde::Deserialize;

/// 唯品会转链响应
///
/// convert 接口的响应
#[derive(Debug, Clone, Deserialize)]
pub struct VipConvertResponse {
    /// 返回码，0 表示成功
    #[serde(default, rename = "returnCode")]
    pub return_code: Option<String>,
    /// 转链结果
    #[serde(default)]
    pub result: Option<VipConvertResult>,
}

/// 唯品会转链结果
#[derive(Debug, Clone, Deserialize)]
pub struct VipConvertResult {
    /// URL 信息列表
    #[serde(default, rename = "urlInfoList")]
    pub url_info_list: Option<Vec<VipUrlInfo>>,
}

/// 唯品会 URL 信息
#[derive(Debug, Clone, Deserialize)]
pub struct VipUrlInfo {
    /// CPS 短链接
    #[serde(default)]
    pub url: Option<String>,
    /// CPS 长链接
    #[serde(default, rename = "longUrl")]
    pub long_url: Option<String>,
    /// CPS 短链接 (不唤起快应用)
    #[serde(default, rename = "noEvokeUrl")]
    pub no_evoke_url: Option<String>,
    /// CPS 长链接 (不唤起快应用)
    #[serde(default, rename = "noEvokeLongUrl")]
    pub no_evoke_long_url: Option<String>,
    /// 唯品会快应用链接
    #[serde(default, rename = "vipQuickAppUrl")]
    pub vip_quick_app_url: Option<String>,
    /// 唯品会小程序链接 (仅在根据商品 ID 获取时返回)
    #[serde(default, rename = "vipWxUrl")]
    pub vip_wx_url: Option<String>,
    /// CPS Deeplink 链接
    #[serde(default, rename = "deeplinkUrl")]
    pub deeplink_url: Option<String>,
    /// CPS 通用链接
    #[serde(default, rename = "ulUrl")]
    pub ul_url: Option<String>,
    /// 原始链接或商品 ID
    #[serde(default)]
    pub source: Option<String>,
    /// 小程序 CPS 参数 (通用小程序跟单参数)
    #[serde(default, rename = "traFrom")]
    pub tra_from: Option<String>,
}

/// 唯品会账号授权响应
///
/// authorize 接口的响应 (返回授权 URL)
#[derive(Debug, Clone, Deserialize)]
pub struct VipAuthorizeResponse {
    /// 状态码
    #[serde(default)]
    pub status: Option<i32>,
    /// 授权 URL
    #[serde(default)]
    pub url: Option<String>,
    /// 错误信息
    #[serde(default)]
    pub msg: Option<String>,
}

/// 唯品会授权列表响应
///
/// authorize_list 接口的响应
#[derive(Debug, Clone, Deserialize)]
pub struct VipAuthorizeListResponse {
    /// 状态码，200 表示成功
    #[serde(default)]
    pub status: Option<i32>,
    /// 授权账号列表
    #[serde(default)]
    pub data: Option<Vec<VipAuthorizeInfo>>,
}

/// 唯品会授权账号信息
#[derive(Debug, Clone, Deserialize)]
pub struct VipAuthorizeInfo {
    /// 唯品会账号授权后在折淘客生成的 ID
    #[serde(default)]
    pub sid: Option<String>,
    /// 折淘客账号
    #[serde(default)]
    pub account: Option<String>,
    /// 唯品会账号 (编号形式)
    #[serde(default)]
    pub user_id: Option<String>,
    /// 用户昵称 (暂时无用)
    #[serde(default)]
    pub user_nick: Option<String>,
    /// 授权时间
    #[serde(default)]
    pub auth_time: Option<String>,
    /// 授权过期时间
    #[serde(default)]
    pub expire_time: Option<String>,
    /// access_token
    #[serde(default)]
    pub access_token: Option<String>,
}

/// 唯品会订单查询响应
///
/// query_orders 接口的响应
#[derive(Debug, Clone, Deserialize)]
pub struct VipOrderResponse {
    /// 返回码，0 表示成功
    #[serde(default, rename = "returnCode")]
    pub return_code: Option<String>,
    /// 订单结果
    #[serde(default)]
    pub result: Option<VipOrderResult>,
}

/// 唯品会订单结果
#[derive(Debug, Clone, Deserialize)]
pub struct VipOrderResult {
    /// 订单列表
    #[serde(default, rename = "orderInfoList")]
    pub order_info_list: Option<Vec<VipOrderInfo>>,
    /// 总数量
    #[serde(default)]
    pub total: Option<i64>,
    /// 页码
    #[serde(default)]
    pub page: Option<i32>,
    /// 每页数量
    #[serde(default, rename = "pageSize")]
    pub page_size: Option<i32>,
}

/// 唯品会订单信息
#[derive(Debug, Clone, Deserialize)]
pub struct VipOrderInfo {
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
    /// 订单状态: 0-不合格，1-待定，2-已完结
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
    /// 渠道标识
    #[serde(default, rename = "chanTag")]
    pub chan_tag: Option<String>,
    /// 自定义参数
    #[serde(default, rename = "statParam")]
    pub stat_param: Option<String>,
}

/// 唯品会商品详情响应 (V2 版本)
///
/// goods_detail 接口的响应
#[derive(Debug, Clone, Deserialize)]
pub struct VipGoodsDetailResponse {
    /// 返回码，0 表示成功
    #[serde(default, rename = "returnCode")]
    pub return_code: Option<String>,
    /// 商品详情列表
    #[serde(default)]
    pub result: Option<Vec<VipGoodsDetail>>,
}

/// 唯品会商品详情
#[derive(Debug, Clone, Deserialize)]
pub struct VipGoodsDetail {
    /// 商品 ID
    #[serde(default, rename = "goodsId")]
    pub goods_id: Option<String>,
    /// 商品名称
    #[serde(default, rename = "goodsName")]
    pub goods_name: Option<String>,
    /// 市场价 (元)
    #[serde(default, rename = "marketPrice")]
    pub market_price: Option<String>,
    /// 唯品价 (元)
    #[serde(default, rename = "vipPrice")]
    pub vip_price: Option<String>,
    /// 折扣 (唯品价/市场价)
    #[serde(default)]
    pub discount: Option<String>,
    /// 佣金比例 (%)
    #[serde(default, rename = "commissionRate")]
    pub commission_rate: Option<String>,
    /// 佣金金额 (元)
    #[serde(default)]
    pub commission: Option<String>,
    /// 商品主图
    #[serde(default, rename = "goodsMainPicture")]
    pub goods_main_picture: Option<String>,
    /// 商品缩略图
    #[serde(default, rename = "goodsThumbUrl")]
    pub goods_thumb_url: Option<String>,
    /// 商品轮播图 (queryDetail=true 时返回)
    #[serde(default, rename = "goodsCarouselPictures")]
    pub goods_carousel_pictures: Option<Vec<String>>,
    /// 商品详情图 (queryDetail=true 时返回)
    #[serde(default, rename = "goodsDetailPictures")]
    pub goods_detail_pictures: Option<Vec<String>>,
    /// 商品落地页
    #[serde(default, rename = "destUrl")]
    pub dest_url: Option<String>,
    /// 品牌名称
    #[serde(default, rename = "brandName")]
    pub brand_name: Option<String>,
    /// 品牌 ID
    #[serde(default, rename = "brandId")]
    pub brand_id: Option<i64>,
    /// 品牌 Logo
    #[serde(default, rename = "brandLogoFull")]
    pub brand_logo_full: Option<String>,
    /// 品牌店铺 SN
    #[serde(default, rename = "brandStoreSn")]
    pub brand_store_sn: Option<String>,
    /// 一级分类 ID
    #[serde(default, rename = "cat1stId")]
    pub cat1st_id: Option<i64>,
    /// 一级分类名称
    #[serde(default, rename = "cat1stName")]
    pub cat1st_name: Option<String>,
    /// 二级分类 ID
    #[serde(default, rename = "cat2ndId")]
    pub cat2nd_id: Option<i64>,
    /// 二级分类名称
    #[serde(default, rename = "cat2ndName")]
    pub cat2nd_name: Option<String>,
    /// 三级分类 ID
    #[serde(default, rename = "categoryId")]
    pub category_id: Option<i64>,
    /// 三级分类名称
    #[serde(default, rename = "categoryName")]
    pub category_name: Option<String>,
    /// 是否海淘商品: 1-是，0-否
    #[serde(default, rename = "haiTao")]
    pub hai_tao: Option<i32>,
    /// 商品类型: 0-自营，1-MP
    #[serde(default, rename = "sourceType")]
    pub source_type: Option<i32>,
    /// 商品状态: 0-下架，1-上架
    #[serde(default)]
    pub status: Option<i32>,
    /// SPU ID
    #[serde(default, rename = "spuId")]
    pub spu_id: Option<String>,
    /// 商品售卖开始时间 (毫秒时间戳)
    #[serde(default, rename = "sellTimeFrom")]
    pub sell_time_from: Option<i64>,
    /// 商品售卖结束时间 (毫秒时间戳)
    #[serde(default, rename = "sellTimeTo")]
    pub sell_time_to: Option<i64>,
    /// 推广方案开始时间 (毫秒时间戳)
    #[serde(default, rename = "schemeStartTime")]
    pub scheme_start_time: Option<i64>,
    /// 推广方案结束时间 (毫秒时间戳)
    #[serde(default, rename = "schemeEndTime")]
    pub scheme_end_time: Option<i64>,
    /// 店铺信息
    #[serde(default, rename = "storeInfo")]
    pub store_info: Option<VipStoreInfo>,
}

/// 唯品会店铺信息
#[derive(Debug, Clone, Deserialize)]
pub struct VipStoreInfo {
    /// 店铺 ID
    #[serde(default, rename = "storeId")]
    pub store_id: Option<String>,
    /// 店铺名称
    #[serde(default, rename = "storeName")]
    pub store_name: Option<String>,
}

/// 唯品会商品搜索响应
///
/// search_goods 接口的响应
#[derive(Debug, Clone, Deserialize)]
pub struct VipSearchGoodsResponse {
    /// 返回码，0 表示成功
    #[serde(default, rename = "returnCode")]
    pub return_code: Option<String>,
    /// 搜索结果
    #[serde(default)]
    pub result: Option<VipSearchResult>,
}

/// 唯品会商品搜索结果
#[derive(Debug, Clone, Deserialize)]
pub struct VipSearchResult {
    /// 商品总数
    #[serde(default)]
    pub total: Option<i64>,
    /// 商品列表
    #[serde(default, rename = "goodsInfoList")]
    pub goods_info_list: Option<Vec<VipGoodsInfo>>,
    /// 页码
    #[serde(default)]
    pub page: Option<i32>,
    /// 每页数量
    #[serde(default, rename = "pageSize")]
    pub page_size: Option<i32>,
    /// 可用排序字段
    #[serde(default, rename = "sortFields")]
    pub sort_fields: Option<Vec<VipSortField>>,
}

/// 唯品会商品信息 (搜索结果)
#[derive(Debug, Clone, Deserialize)]
pub struct VipGoodsInfo {
    /// 商品 ID
    #[serde(default, rename = "goodsId")]
    pub goods_id: Option<String>,
    /// 商品名称
    #[serde(default, rename = "goodsName")]
    pub goods_name: Option<String>,
    /// 市场价 (元)
    #[serde(default, rename = "marketPrice")]
    pub market_price: Option<String>,
    /// 唯品价 (元)
    #[serde(default, rename = "vipPrice")]
    pub vip_price: Option<String>,
    /// 折扣 (唯品价/市场价)
    #[serde(default)]
    pub discount: Option<String>,
    /// 佣金比例 (%)
    #[serde(default, rename = "commissionRate")]
    pub commission_rate: Option<String>,
    /// 佣金金额 (元)
    #[serde(default)]
    pub commission: Option<String>,
    /// 商品主图
    #[serde(default, rename = "goodsMainPicture")]
    pub goods_main_picture: Option<String>,
    /// 商品缩略图
    #[serde(default, rename = "goodsThumbUrl")]
    pub goods_thumb_url: Option<String>,
    /// 商品落地页
    #[serde(default, rename = "destUrl")]
    pub dest_url: Option<String>,
    /// 品牌名称
    #[serde(default, rename = "brandName")]
    pub brand_name: Option<String>,
    /// 品牌 ID
    #[serde(default, rename = "brandId")]
    pub brand_id: Option<i64>,
    /// 品牌 Logo
    #[serde(default, rename = "brandLogoFull")]
    pub brand_logo_full: Option<String>,
    /// 品牌店铺 SN
    #[serde(default, rename = "brandStoreSn")]
    pub brand_store_sn: Option<String>,
    /// 一级分类 ID
    #[serde(default, rename = "cat1stId")]
    pub cat1st_id: Option<i64>,
    /// 一级分类名称
    #[serde(default, rename = "cat1stName")]
    pub cat1st_name: Option<String>,
    /// 二级分类 ID
    #[serde(default, rename = "cat2ndId")]
    pub cat2nd_id: Option<i64>,
    /// 二级分类名称
    #[serde(default, rename = "cat2ndName")]
    pub cat2nd_name: Option<String>,
    /// 三级分类 ID
    #[serde(default, rename = "categoryId")]
    pub category_id: Option<i64>,
    /// 三级分类名称
    #[serde(default, rename = "categoryName")]
    pub category_name: Option<String>,
    /// 是否海淘商品: 1-是，0-否
    #[serde(default, rename = "haiTao")]
    pub hai_tao: Option<i32>,
    /// 商品类型: 0-自营，1-MP
    #[serde(default, rename = "sourceType")]
    pub source_type: Option<i32>,
    /// 商品状态: 0-下架，1-上架
    #[serde(default)]
    pub status: Option<i32>,
    /// SPU ID
    #[serde(default, rename = "spuId")]
    pub spu_id: Option<String>,
    /// 商品售卖开始时间 (毫秒时间戳)
    #[serde(default, rename = "sellTimeFrom")]
    pub sell_time_from: Option<i64>,
    /// 商品售卖结束时间 (毫秒时间戳)
    #[serde(default, rename = "sellTimeTo")]
    pub sell_time_to: Option<i64>,
    /// 推广方案开始时间 (毫秒时间戳)
    #[serde(default, rename = "schemeStartTime")]
    pub scheme_start_time: Option<i64>,
    /// 推广方案结束时间 (毫秒时间戳)
    #[serde(default, rename = "schemeEndTime")]
    pub scheme_end_time: Option<i64>,
    /// 店铺信息
    #[serde(default, rename = "storeInfo")]
    pub store_info: Option<VipStoreInfo>,
}

/// 唯品会排序字段
#[derive(Debug, Clone, Deserialize)]
pub struct VipSortField {
    /// 字段名称
    #[serde(default, rename = "fieldName")]
    pub field_name: Option<String>,
    /// 字段描述
    #[serde(default, rename = "fieldDesc")]
    pub field_desc: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vip_convert_response_deserialize() {
        let json = r#"{
            "returnCode": "0",
            "result": {
                "urlInfoList": [
                    {
                        "url": "https://t.vip.com/mlCYWuKuVNA",
                        "longUrl": "https://t.vip.com/redirect.php?url=xxx",
                        "noEvokeUrl": "https://t.vip.com/mlCYWuKuVNA?wq=1",
                        "vipWxUrl": "pages/productDetail/productDetail?goodsId=123",
                        "deeplinkUrl": "vipshop://showGoodsDetail?pid=123",
                        "source": "https://m.vip.com/product-123.html"
                    }
                ]
            }
        }"#;

        let response: VipConvertResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.return_code, Some("0".to_string()));
        assert!(response.result.is_some());
        let result = response.result.unwrap();
        assert!(result.url_info_list.is_some());
        let urls = result.url_info_list.unwrap();
        assert_eq!(urls.len(), 1);
        assert_eq!(
            urls[0].url,
            Some("https://t.vip.com/mlCYWuKuVNA".to_string())
        );
    }

    #[test]
    fn test_vip_authorize_list_response_deserialize() {
        let json = r#"{
            "status": 200,
            "data": [
                {
                    "sid": "123",
                    "account": "test@example.com",
                    "user_id": "456",
                    "auth_time": "2023/11/12 15:55:49",
                    "expire_time": "2024/11/12 15:55:49",
                    "access_token": "token123"
                }
            ]
        }"#;

        let response: VipAuthorizeListResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.status, Some(200));
        assert!(response.data.is_some());
        let data = response.data.unwrap();
        assert_eq!(data.len(), 1);
        assert_eq!(data[0].sid, Some("123".to_string()));
    }

    #[test]
    fn test_vip_order_response_deserialize() {
        let json = r#"{
            "returnCode": "0",
            "result": {
                "orderInfoList": [
                    {
                        "orderSn": "123456789",
                        "goodsId": "6919173790682972930",
                        "goodsName": "测试商品",
                        "status": 2,
                        "orderAmount": "99.00",
                        "commission": "4.95"
                    }
                ],
                "total": 1,
                "page": 1,
                "pageSize": 20
            }
        }"#;

        let response: VipOrderResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.return_code, Some("0".to_string()));
        assert!(response.result.is_some());
        let result = response.result.unwrap();
        assert_eq!(result.total, Some(1));
        assert!(result.order_info_list.is_some());
        let orders = result.order_info_list.unwrap();
        assert_eq!(orders.len(), 1);
        assert_eq!(orders[0].order_sn, Some("123456789".to_string()));
    }

    #[test]
    fn test_vip_goods_detail_response_deserialize() {
        let json = r#"{
            "returnCode": "0",
            "result": [
                {
                    "goodsId": "6919173790682972930",
                    "goodsName": "测试商品",
                    "marketPrice": "539.00",
                    "vipPrice": "179.00",
                    "discount": "0.33",
                    "commissionRate": "5",
                    "commission": "8.95",
                    "brandName": "测试品牌",
                    "cat1stName": "男装",
                    "status": 1
                }
            ]
        }"#;

        let response: VipGoodsDetailResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.return_code, Some("0".to_string()));
        assert!(response.result.is_some());
        let result = response.result.unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].goods_id, Some("6919173790682972930".to_string()));
        assert_eq!(result[0].market_price, Some("539.00".to_string()));
    }

    #[test]
    fn test_vip_search_goods_response_deserialize() {
        let json = r#"{
            "returnCode": "0",
            "result": {
                "total": 3250,
                "goodsInfoList": [
                    {
                        "goodsId": "6918901004295698011",
                        "goodsName": "测试球鞋",
                        "marketPrice": "339.00",
                        "vipPrice": "179.00",
                        "discount": "0.68",
                        "commissionRate": "3",
                        "commission": "5.37",
                        "brandName": "乔丹",
                        "cat1stName": "运动户外",
                        "status": 1
                    }
                ],
                "page": 1,
                "pageSize": 20,
                "sortFields": [
                    {"fieldName": "PRICE", "fieldDesc": "价格排序"},
                    {"fieldName": "DISCOUNT", "fieldDesc": "折扣排序"}
                ]
            }
        }"#;

        let response: VipSearchGoodsResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.return_code, Some("0".to_string()));
        assert!(response.result.is_some());
        let result = response.result.unwrap();
        assert_eq!(result.total, Some(3250));
        assert!(result.goods_info_list.is_some());
        let goods = result.goods_info_list.unwrap();
        assert_eq!(goods.len(), 1);
        assert_eq!(goods[0].goods_name, Some("测试球鞋".to_string()));
    }

    #[test]
    fn test_response_with_missing_fields() {
        let json = r#"{"returnCode": "0"}"#;

        let response: VipConvertResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.return_code, Some("0".to_string()));
        assert!(response.result.is_none());
    }
}
