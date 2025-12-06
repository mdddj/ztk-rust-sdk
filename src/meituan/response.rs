//! 美团平台响应结构体
//!
//! 定义美团平台 API 的响应结构体

use serde::Deserialize;

/// 美团转链响应
///
/// convert 接口的响应
#[derive(Debug, Clone, Deserialize)]
pub struct MeituanConvertResponse {
    /// 状态码，0 表示成功
    #[serde(default)]
    pub status: Option<i32>,
    /// 状态描述
    #[serde(default)]
    pub des: Option<String>,
    /// 推广链接
    #[serde(default)]
    pub data: Option<String>,
    /// 小程序二维码推广小图
    #[serde(default, rename = "wx_mini_pic")]
    pub wx_mini_pic: Option<String>,
    /// 长链接二维码推广大图
    #[serde(default, rename = "qrcode_chang_pic")]
    pub qrcode_chang_pic: Option<String>,
    /// 小程序二维码推广大图
    #[serde(default, rename = "qrcode_wx_pic")]
    pub qrcode_wx_pic: Option<String>,
}

/// 美团订单查询响应
///
/// query_orders 接口的响应
#[derive(Debug, Clone, Deserialize)]
pub struct MeituanOrderResponse {
    /// 状态码，200 表示成功
    #[serde(default)]
    pub status: Option<i32>,
    /// 订单列表
    #[serde(default)]
    pub content: Option<Vec<MeituanOrderInfo>>,
}

/// 美团订单信息
#[derive(Debug, Clone, Deserialize)]
pub struct MeituanOrderInfo {
    /// 折淘客自动增长列
    #[serde(default)]
    pub code: Option<String>,
    /// 折淘客账号
    #[serde(default)]
    pub account: Option<String>,
    /// 折淘客 appkey
    #[serde(default)]
    pub appkey_ztk: Option<String>,
    /// 美团原始推广位 SID
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
    /// 0: 团购订单
    /// 2: 酒店订单
    /// 4: 外卖订单
    /// 5: 话费&团好货订单
    /// 6: 闪购订单
    /// 8: 优选订单
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
    /// 业务线类型
    /// 1: 平台
    /// 2: 外卖/闪购
    /// 3: 酒店
    /// 4: 优选
    /// 5: 团APP拉新
    #[serde(default, rename = "businessLine")]
    pub business_line: Option<String>,
    /// 子业务线
    /// 当是外卖订单时，subBusinessLine=1
    /// 当是闪购订单时，subBusinessLine=2
    #[serde(default, rename = "subBusinessLine")]
    pub sub_business_line: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_meituan_convert_response_deserialize() {
        let json = r#"{
            "status": 0,
            "des": "推广链接生成成功",
            "data": "https://click.meituan.com/t?t=1&c=1&p=OWMpZ-uzIFOVe6JyOONs3dXuqV0qcAf-r-KCvH",
            "wx_mini_pic": "http://p0.meituan.net/poiqrcode/b407b0c4346e854c4468a704ab42f156103581.jpg",
            "qrcode_chang_pic": "http://www.zhetaoke.com/uploads_qrcode/20210429/20210429449433.jpg",
            "qrcode_wx_pic": "http://www.zhetaoke.com/uploads_qrcode/20210429/2021047444942.jpg"
        }"#;

        let response: MeituanConvertResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.status, Some(0));
        assert_eq!(response.des, Some("推广链接生成成功".to_string()));
        assert!(response.data.is_some());
        assert!(response.wx_mini_pic.is_some());
    }

    #[test]
    fn test_meituan_order_response_deserialize() {
        let json = r#"{
            "status": 200,
            "content": [
                {
                    "code": "15",
                    "account": "test@example.com",
                    "appkey_ztk": "a2a33cacfb1f4005b818ffd0ab7c951d",
                    "sid": "16ztkzheta",
                    "is_jiesuan": "0",
                    "jiesuan_time": "",
                    "jiesuan_profit": "1.16",
                    "orderid": "105518514197820013",
                    "paytime": "2021/04/27 12:22:09",
                    "payprice": "21.50",
                    "profit": 1.29,
                    "smstitle": "霸碗盖码饭（凤凰大厦店）",
                    "refundprice": 0,
                    "refundtime": "",
                    "status": "8",
                    "update_time": "",
                    "type": "4",
                    "sid_ztk": "16",
                    "customer_id_ztk": "100000",
                    "platform_ztk": "zhetaoke",
                    "actId": "33",
                    "businessLine": "2",
                    "subBusinessLine": "1"
                }
            ]
        }"#;

        let response: MeituanOrderResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.status, Some(200));
        assert!(response.content.is_some());
        let orders = response.content.unwrap();
        assert_eq!(orders.len(), 1);
        assert_eq!(orders[0].orderid, Some("105518514197820013".to_string()));
        assert_eq!(orders[0].profit, Some(1.29));
        assert_eq!(orders[0].status, Some("8".to_string()));
        assert_eq!(orders[0].order_type, Some("4".to_string()));
    }

    #[test]
    fn test_meituan_convert_response_with_missing_fields() {
        let json = r#"{"status": 0, "des": "成功"}"#;

        let response: MeituanConvertResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.status, Some(0));
        assert!(response.data.is_none());
        assert!(response.wx_mini_pic.is_none());
    }

    #[test]
    fn test_meituan_order_response_empty_content() {
        let json = r#"{"status": 200, "content": []}"#;

        let response: MeituanOrderResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.status, Some(200));
        assert!(response.content.is_some());
        assert_eq!(response.content.unwrap().len(), 0);
    }

    #[test]
    fn test_meituan_order_info_optional_fields() {
        let json = r#"{
            "orderid": "123456",
            "status": "1"
        }"#;

        let order: MeituanOrderInfo = serde_json::from_str(json).unwrap();
        assert_eq!(order.orderid, Some("123456".to_string()));
        assert_eq!(order.status, Some("1".to_string()));
        assert!(order.profit.is_none());
        assert!(order.act_id.is_none());
    }
}
