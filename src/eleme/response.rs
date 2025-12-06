//! 饿了么平台响应结构体
//!
//! 定义饿了么平台 API 的响应结构体

use serde::Deserialize;

/// 饿了么转链响应
///
/// convert 接口的响应
#[derive(Debug, Clone, Deserialize)]
pub struct ElemeConvertResponse {
    /// 阿里本地生活联盟饿了么推广响应
    #[serde(rename = "alibaba_alsc_union_eleme_promotion_officialactivity_get_response")]
    pub response: Option<ElemePromotionResponse>,
}

/// 饿了么推广活动响应
#[derive(Debug, Clone, Deserialize)]
pub struct ElemePromotionResponse {
    /// 活动数据
    #[serde(default)]
    pub data: Option<ElemeActivityData>,
    /// 响应消息
    #[serde(default)]
    pub message: Option<String>,
    /// 请求 ID
    #[serde(default)]
    pub request_id: Option<String>,
    /// 结果码，0 表示成功
    #[serde(default)]
    pub result_code: Option<i32>,
}

/// 饿了么活动数据
#[derive(Debug, Clone, Deserialize)]
pub struct ElemeActivityData {
    /// 活动描述
    #[serde(default)]
    pub description: Option<String>,
    /// 结束时间（秒）
    #[serde(default)]
    pub end_time: Option<i64>,
    /// 活动 ID
    #[serde(default)]
    pub id: Option<String>,
    /// 推广链接信息
    #[serde(default)]
    pub link: Option<ElemeLink>,
    /// 活动创意图片
    #[serde(default)]
    pub picture: Option<String>,
    /// 起始时间（秒）
    #[serde(default)]
    pub start_time: Option<i64>,
    /// 活动标题
    #[serde(default)]
    pub title: Option<String>,
}

/// 饿了么推广链接信息
#[derive(Debug, Clone, Deserialize)]
pub struct ElemeLink {
    /// 支付宝小程序推广链接
    #[serde(default)]
    pub alipay_mini_url: Option<String>,
    /// 饿了么唤端链接
    #[serde(default)]
    pub ele_scheme_url: Option<String>,
    /// H5 推广地址短链
    #[serde(default)]
    pub h5_short_link: Option<String>,
    /// H5 推广地址
    #[serde(default)]
    pub h5_url: Option<String>,
    /// 微信独立二维码
    #[serde(default)]
    pub mini_qrcode: Option<String>,
    /// 推广图片地址
    #[serde(default)]
    pub picture: Option<String>,
    /// 淘宝独立二维码
    #[serde(default)]
    pub tb_mini_qrcode: Option<String>,
    /// 淘宝二维码图片地址
    #[serde(default)]
    pub tb_qr_code: Option<String>,
    /// 小程序 appId
    #[serde(default)]
    pub wx_appid: Option<String>,
    /// 微信小程序 path 链接
    #[serde(default)]
    pub wx_path: Option<String>,
}

/// 饿了么订单查询响应
///
/// query_orders 接口的响应
#[derive(Debug, Clone, Deserialize)]
pub struct ElemeOrderResponse {
    /// 状态码，200 表示成功
    #[serde(default)]
    pub status: Option<i32>,
    /// 订单列表
    #[serde(default)]
    pub content: Option<Vec<ElemeOrderInfo>>,
}

/// 饿了么订单信息
#[derive(Debug, Clone, Deserialize)]
pub struct ElemeOrderInfo {
    /// 折淘客自动增长列
    #[serde(default)]
    pub code: Option<String>,
    /// 折淘客账号
    #[serde(default)]
    pub account: Option<String>,
    /// 折淘客 appkey
    #[serde(default)]
    pub appkey_ztk: Option<String>,
    /// 原始推广位 SID
    #[serde(default)]
    pub sid: Option<String>,
    /// 折淘客是否结算: 0-未结算, 1-已结算
    #[serde(default)]
    pub is_jiesuan: Option<String>,
    /// 折淘客结算时间
    #[serde(default)]
    pub jiesuan_time: Option<String>,
    /// 折淘客结算金额
    #[serde(default)]
    pub jiesuan_profit: Option<String>,
    /// 订单编号
    #[serde(default)]
    pub orderid: Option<String>,
    /// 订单支付时间
    #[serde(default)]
    pub paytime: Option<String>,
    /// 订单实际支付金额
    #[serde(default)]
    pub payprice: Option<String>,
    /// 预估佣金
    #[serde(default)]
    pub profit: Option<f64>,
    /// 订单标题
    #[serde(default)]
    pub smstitle: Option<String>,
    /// 退款金额
    #[serde(default)]
    pub refundprice: Option<f64>,
    /// 退款时间
    #[serde(default)]
    pub refundtime: Option<String>,
    /// 订单状态: 1-已付款, 8-已完成, 9-已退款或风控
    #[serde(default)]
    pub status: Option<String>,
    /// 数据最后更新时间
    #[serde(default)]
    pub update_time: Option<String>,
    /// 订单类型
    #[serde(default, rename = "type")]
    pub order_type: Option<String>,
    /// 折淘客授权 SID
    #[serde(default)]
    pub sid_ztk: Option<String>,
    /// 推广者自定义编号
    #[serde(default)]
    pub customer_id_ztk: Option<String>,
    /// 推广者平台类型，默认 zhetaoke
    #[serde(default)]
    pub platform_ztk: Option<String>,
    /// 活动 ID
    #[serde(default, rename = "actId")]
    pub act_id: Option<String>,
    /// 订单所属平台名称
    #[serde(default)]
    pub san_pingtai: Option<String>,
    /// 订单所属平台 ID
    /// 1美团、2考拉、3苏宁、4淘宝、5京东、6拼多多、7唯品会、8饿了么、9抖音
    #[serde(default)]
    pub san_pingtai_id: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eleme_convert_response_deserialize() {
        let json = r#"{
            "alibaba_alsc_union_eleme_promotion_officialactivity_get_response": {
                "data": {
                    "description": "最高抢66元大额红包",
                    "end_time": 1714492799,
                    "id": "10144",
                    "link": {
                        "alipay_mini_url": "alipays://platformapi/startapp?appId=2021001110676437",
                        "ele_scheme_url": "eleme://web?action=ali.open.nav",
                        "h5_short_link": "https://u.ele.me/Rel9JsB9",
                        "h5_url": "https://fc.ele.me/a/test",
                        "wx_appid": "wxece3a9a4c82f58c9",
                        "wx_path": "commercialize/pages/taoke-guide/index"
                    },
                    "picture": "https://img.alicdn.com/test.png",
                    "start_time": 1649865600,
                    "title": "饿了么天天领红包"
                },
                "message": "success",
                "request_id": "15s3jznf0in4t",
                "result_code": 0
            }
        }"#;

        let response: ElemeConvertResponse = serde_json::from_str(json).unwrap();
        assert!(response.response.is_some());
        let inner = response.response.unwrap();
        assert_eq!(inner.message, Some("success".to_string()));
        assert_eq!(inner.result_code, Some(0));

        let data = inner.data.unwrap();
        assert_eq!(data.id, Some("10144".to_string()));
        assert_eq!(data.title, Some("饿了么天天领红包".to_string()));

        let link = data.link.unwrap();
        assert_eq!(
            link.h5_short_link,
            Some("https://u.ele.me/Rel9JsB9".to_string())
        );
    }

    #[test]
    fn test_eleme_order_response_deserialize() {
        let json = r#"{
            "status": 200,
            "content": [
                {
                    "code": "15",
                    "account": "test@example.com",
                    "appkey_ztk": "a2a33cacfb1f4005b818ffd0ab7c951d",
                    "sid": "16ztk",
                    "is_jiesuan": "0",
                    "jiesuan_time": "",
                    "jiesuan_profit": "1.16",
                    "orderid": "105518514197820013",
                    "paytime": "2021/04/27 12:22:09",
                    "payprice": "21.50",
                    "profit": 1.29,
                    "smstitle": "饿了么外卖订单",
                    "refundprice": 0,
                    "refundtime": "",
                    "status": "8",
                    "update_time": "",
                    "type": "4",
                    "sid_ztk": "16",
                    "customer_id_ztk": "100000",
                    "platform_ztk": "zhetaoke",
                    "actId": "10144",
                    "san_pingtai": "饿了么",
                    "san_pingtai_id": "8"
                }
            ]
        }"#;

        let response: ElemeOrderResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.status, Some(200));
        assert!(response.content.is_some());
        let orders = response.content.unwrap();
        assert_eq!(orders.len(), 1);
        assert_eq!(orders[0].orderid, Some("105518514197820013".to_string()));
        assert_eq!(orders[0].profit, Some(1.29));
        assert_eq!(orders[0].san_pingtai_id, Some("8".to_string()));
    }

    #[test]
    fn test_eleme_convert_response_with_missing_fields() {
        let json = r#"{
            "alibaba_alsc_union_eleme_promotion_officialactivity_get_response": {
                "message": "success",
                "result_code": 0
            }
        }"#;

        let response: ElemeConvertResponse = serde_json::from_str(json).unwrap();
        assert!(response.response.is_some());
        let inner = response.response.unwrap();
        assert!(inner.data.is_none());
    }

    #[test]
    fn test_eleme_order_response_empty_content() {
        let json = r#"{"status": 200, "content": []}"#;

        let response: ElemeOrderResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.status, Some(200));
        assert!(response.content.is_some());
        assert_eq!(response.content.unwrap().len(), 0);
    }

    #[test]
    fn test_eleme_order_info_optional_fields() {
        let json = r#"{
            "orderid": "123456",
            "status": "1",
            "san_pingtai_id": "8"
        }"#;

        let order: ElemeOrderInfo = serde_json::from_str(json).unwrap();
        assert_eq!(order.orderid, Some("123456".to_string()));
        assert_eq!(order.status, Some("1".to_string()));
        assert_eq!(order.san_pingtai_id, Some("8".to_string()));
        assert!(order.profit.is_none());
        assert!(order.act_id.is_none());
    }
}
