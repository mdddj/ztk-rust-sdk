//! 饿了么平台请求参数结构体
//!
//! 定义饿了么平台 API 的请求参数结构体

use serde::Serialize;

/// 饿了么转链请求
///
/// 根据饿了么活动 ID 生成联盟推广链接
///
/// # Example
///
/// ```rust,ignore
/// let request = ElemeConvertRequest::new("sid", "10144")
///     .customer_id("100000");
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct ElemeConvertRequest {
    /// 淘客账号授权 SID
    pub sid: String,
    /// 饿了么活动 ID
    /// 活动ID：10144，饿了么天天领红包，最高抢66元大额红包，预估佣金收益6%
    /// 活动ID：11862，限时快闪，最高领18元红包
    /// 活动ID：12819，闪购天天领红包，预估佣金收益6%
    /// 活动ID：12628，淘宝闪购，领最高15元红包，预估佣金收益6%
    /// 活动ID：12688，来闪购一下 可赢免单福利，闪购大额满减红包，预估佣金收益6%
    /// 活动ID：12918，淘宝闪购惊喜福利，1分钱喝奶茶，预估佣金收益6%
    #[serde(rename = "activity_id")]
    pub activity_id: String,
    /// 自定义参数 (可选)
    /// 只允许数字，最大长度11位
    /// 此字段可以用来区分用户，进行下级返利
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
}

impl ElemeConvertRequest {
    /// 创建新的饿了么转链请求
    ///
    /// # Arguments
    ///
    /// * `sid` - 淘客账号授权 SID
    /// * `activity_id` - 饿了么活动 ID
    pub fn new(sid: impl Into<String>, activity_id: impl Into<String>) -> Self {
        Self {
            sid: sid.into(),
            activity_id: activity_id.into(),
            customer_id: None,
        }
    }

    /// 设置自定义参数 (用于用户区分和返利)
    pub fn customer_id(mut self, customer_id: impl Into<String>) -> Self {
        self.customer_id = Some(customer_id.into());
        self
    }
}

/// 饿了么订单查询请求
///
/// 查询饿了么推广订单列表（通过折淘客联盟订单接口）
///
/// # Example
///
/// ```rust,ignore
/// let request = ElemeOrderQueryRequest::new(1, 1, 50)
///     .start_time("2021-04-30 08:00:00")
///     .end_time("2021-04-30 09:00:00");
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct ElemeOrderQueryRequest {
    /// 时间类型
    /// 1: 按照订单支付时间获取
    /// 2: 按照订单更新时间获取
    /// 3: 按照订单编号获取
    #[serde(rename = "type")]
    pub query_type: u32,
    /// 页码
    pub page: u32,
    /// 每页数量，单页数最大 50，默认 50
    pub page_size: u32,
    /// 订单开始时间 (可选)
    /// 格式: 2021-04-30 08:00:00
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "startTime")]
    pub start_time: Option<String>,
    /// 订单结束时间 (可选)
    /// 格式: 2021-04-30 09:00:00
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "endTime")]
    pub end_time: Option<String>,
    /// 订单编号 (可选)
    /// 必须输入完整的订单编号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orderid: Option<String>,
    /// 折淘客授权 SID (可选)
    /// 无值表示获取 appkey 下所有饿了么订单
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,
    /// 订单所属平台 ID (固定为 8 表示饿了么)
    /// 1美团、2考拉、3苏宁、4淘宝、5京东、6拼多多、7唯品会、8饿了么、9抖音
    #[serde(rename = "san_pingtai_id")]
    pub san_pingtai_id: String,
}

impl ElemeOrderQueryRequest {
    /// 创建新的饿了么订单查询请求
    ///
    /// # Arguments
    ///
    /// * `query_type` - 时间类型: 1-按支付时间, 2-按更新时间, 3-按订单编号
    /// * `page` - 页码
    /// * `page_size` - 每页数量 (最大 50)
    pub fn new(query_type: u32, page: u32, page_size: u32) -> Self {
        Self {
            query_type,
            page,
            page_size: page_size.min(50),
            start_time: None,
            end_time: None,
            orderid: None,
            sid: None,
            san_pingtai_id: "8".to_string(), // 固定为饿了么平台
        }
    }

    /// 设置订单开始时间
    pub fn start_time(mut self, start_time: impl Into<String>) -> Self {
        self.start_time = Some(start_time.into());
        self
    }

    /// 设置订单结束时间
    pub fn end_time(mut self, end_time: impl Into<String>) -> Self {
        self.end_time = Some(end_time.into());
        self
    }

    /// 设置时间范围
    pub fn time_range(mut self, start: impl Into<String>, end: impl Into<String>) -> Self {
        self.start_time = Some(start.into());
        self.end_time = Some(end.into());
        self
    }

    /// 设置订单编号
    pub fn orderid(mut self, orderid: impl Into<String>) -> Self {
        self.orderid = Some(orderid.into());
        self
    }

    /// 设置折淘客授权 SID
    pub fn sid(mut self, sid: impl Into<String>) -> Self {
        self.sid = Some(sid.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eleme_convert_request_serialize() {
        let request = ElemeConvertRequest::new("sid123", "10144");
        let json = serde_json::to_value(&request).unwrap();

        assert_eq!(json["sid"], "sid123");
        assert_eq!(json["activity_id"], "10144");
        assert!(json.get("customer_id").is_none());
    }

    #[test]
    fn test_eleme_convert_request_with_customer_id() {
        let request = ElemeConvertRequest::new("sid123", "10144").customer_id("100000");

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["customer_id"], "100000");
    }

    #[test]
    fn test_eleme_order_query_request_serialize() {
        let request = ElemeOrderQueryRequest::new(1, 1, 50);
        let json = serde_json::to_value(&request).unwrap();

        assert_eq!(json["type"], 1);
        assert_eq!(json["page"], 1);
        assert_eq!(json["page_size"], 50);
        assert_eq!(json["san_pingtai_id"], "8");
        assert!(json.get("startTime").is_none());
    }

    #[test]
    fn test_eleme_order_query_request_with_time_range() {
        let request = ElemeOrderQueryRequest::new(1, 1, 50)
            .time_range("2021-04-30 08:00:00", "2021-04-30 09:00:00")
            .sid("sid123");

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["startTime"], "2021-04-30 08:00:00");
        assert_eq!(json["endTime"], "2021-04-30 09:00:00");
        assert_eq!(json["sid"], "sid123");
    }

    #[test]
    fn test_eleme_order_query_page_size_limit() {
        let request = ElemeOrderQueryRequest::new(1, 1, 100);
        assert_eq!(request.page_size, 50); // Should be capped at 50
    }

    #[test]
    fn test_optional_fields_not_serialized() {
        let request = ElemeConvertRequest::new("sid", "10144");
        let json_str = serde_json::to_string(&request).unwrap();

        assert!(!json_str.contains("customer_id"));
    }
}
