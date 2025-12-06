//! 美团平台请求参数结构体
//!
//! 定义美团平台 API 的请求参数结构体

use serde::Serialize;

/// 美团转链请求
///
/// 根据美团活动 ID 或链接生成联盟推广链接
///
/// # Example
///
/// ```rust,ignore
/// let request = MeituanConvertRequest::new("sid", "7")
///     .link_type(MeituanLinkType::H5Short)
///     .customer_id("100000");
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct MeituanConvertRequest {
    /// 淘客账号授权 SID
    pub sid: String,
    /// 美团活动 ID 或活动链接
    /// 活动ID：7，美团外卖节，帮你吃更好，佣金比例3%起
    /// 活动ID：272，【商超果蔬】超值爆款鲜花全场3折起
    /// 活动ID：273，【美团闪购】红包天天领
    /// 活动ID：284，【玩乐变美】天天可领红包
    /// 活动ID：309，【美食团购】红包天天领
    /// 活动ID：442，【高佣专场】吃喝玩乐全品类好货
    /// 活动ID：529，【特惠券包】4.9元购180元神券包
    /// 活动ID：689，【美团拼好饭】爆品一口价会场
    #[serde(rename = "actId")]
    pub act_id: String,
    /// 链接类型
    /// 1: H5长链接
    /// 2: H5短链接
    /// 3: deeplink(唤起)链接
    /// 4: 微信小程序唤起路径
    /// 5: 团口令
    #[serde(rename = "linkType")]
    pub link_type: u32,
    /// 是否生成小程序二维码 (可选)
    /// 1: 生成，如果不需要此图，请忽略该参数，会降低转链性能
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "miniCode")]
    pub mini_code: Option<String>,
    /// 是否生成长链接二维码推广大图 (可选)
    /// 1: 生成，如果不需要此图，请忽略该参数，会降低转链性能
    /// 如果需要此图，miniCode 参数必须等于 1
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "miniCode2")]
    pub mini_code2: Option<String>,
    /// 是否生成小程序二维码推广大图 (可选)
    /// 1: 生成，如果不需要此图，请忽略该参数，会降低转链性能
    /// 如果需要此图，miniCode 参数必须等于 1
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "miniCode3")]
    pub mini_code3: Option<String>,
    /// 自定义平台名称 (可选)
    /// 此参数需要联系折淘客官方进行备案审核
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    /// 自定义参数 (可选)
    /// 只允许数字，最大长度11位
    /// 此字段可以用来区分用户，进行下级返利
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
}

impl MeituanConvertRequest {
    /// 创建新的美团转链请求
    ///
    /// # Arguments
    ///
    /// * `sid` - 淘客账号授权 SID
    /// * `act_id` - 美团活动 ID 或活动链接
    pub fn new(sid: impl Into<String>, act_id: impl Into<String>) -> Self {
        Self {
            sid: sid.into(),
            act_id: act_id.into(),
            link_type: 1, // 默认 H5 长链接
            mini_code: None,
            mini_code2: None,
            mini_code3: None,
            platform: None,
            customer_id: None,
        }
    }

    /// 设置链接类型
    /// 1: H5长链接, 2: H5短链接, 3: deeplink, 4: 小程序路径, 5: 团口令
    pub fn link_type(mut self, link_type: u32) -> Self {
        self.link_type = link_type;
        self
    }

    /// 设置是否生成小程序二维码
    pub fn mini_code(mut self, mini_code: bool) -> Self {
        self.mini_code = if mini_code {
            Some("1".to_string())
        } else {
            None
        };
        self
    }

    /// 设置是否生成长链接二维码推广大图
    pub fn mini_code2(mut self, mini_code2: bool) -> Self {
        self.mini_code2 = if mini_code2 {
            Some("1".to_string())
        } else {
            None
        };
        self
    }

    /// 设置是否生成小程序二维码推广大图
    pub fn mini_code3(mut self, mini_code3: bool) -> Self {
        self.mini_code3 = if mini_code3 {
            Some("1".to_string())
        } else {
            None
        };
        self
    }

    /// 设置自定义平台名称
    pub fn platform(mut self, platform: impl Into<String>) -> Self {
        self.platform = Some(platform.into());
        self
    }

    /// 设置自定义参数 (用于用户区分和返利)
    pub fn customer_id(mut self, customer_id: impl Into<String>) -> Self {
        self.customer_id = Some(customer_id.into());
        self
    }
}

/// 美团订单查询请求
///
/// 查询美团推广订单列表
///
/// # Example
///
/// ```rust,ignore
/// let request = MeituanOrderQueryRequest::new(1, 1, 50)
///     .start_time("2021-04-30 08:00:00")
///     .end_time("2021-04-30 09:00:00");
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct MeituanOrderQueryRequest {
    /// 时间类型
    /// 1: 按照订单支付时间获取
    /// 2: 按照订单更新时间获取
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
    /// 无值表示获取 appkey 下所有美团订单
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,
}

impl MeituanOrderQueryRequest {
    /// 创建新的美团订单查询请求
    ///
    /// # Arguments
    ///
    /// * `query_type` - 时间类型: 1-按支付时间, 2-按更新时间
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

/// 美团链接类型枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MeituanLinkType {
    /// H5 长链接
    H5Long = 1,
    /// H5 短链接
    H5Short = 2,
    /// Deeplink (唤起) 链接
    Deeplink = 3,
    /// 微信小程序唤起路径
    MiniProgram = 4,
    /// 团口令
    Command = 5,
}

impl From<MeituanLinkType> for u32 {
    fn from(link_type: MeituanLinkType) -> Self {
        link_type as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_meituan_convert_request_serialize() {
        let request = MeituanConvertRequest::new("sid123", "7");
        let json = serde_json::to_value(&request).unwrap();

        assert_eq!(json["sid"], "sid123");
        assert_eq!(json["actId"], "7");
        assert_eq!(json["linkType"], 1);
        assert!(json.get("miniCode").is_none());
        assert!(json.get("customer_id").is_none());
    }

    #[test]
    fn test_meituan_convert_request_with_options() {
        let request = MeituanConvertRequest::new("sid123", "7")
            .link_type(2)
            .mini_code(true)
            .customer_id("100000");

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["linkType"], 2);
        assert_eq!(json["miniCode"], "1");
        assert_eq!(json["customer_id"], "100000");
    }

    #[test]
    fn test_meituan_order_query_request_serialize() {
        let request = MeituanOrderQueryRequest::new(1, 1, 50);
        let json = serde_json::to_value(&request).unwrap();

        assert_eq!(json["type"], 1);
        assert_eq!(json["page"], 1);
        assert_eq!(json["page_size"], 50);
        assert!(json.get("startTime").is_none());
    }

    #[test]
    fn test_meituan_order_query_request_with_time_range() {
        let request = MeituanOrderQueryRequest::new(1, 1, 50)
            .time_range("2021-04-30 08:00:00", "2021-04-30 09:00:00")
            .sid("sid123");

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["startTime"], "2021-04-30 08:00:00");
        assert_eq!(json["endTime"], "2021-04-30 09:00:00");
        assert_eq!(json["sid"], "sid123");
    }

    #[test]
    fn test_meituan_order_query_page_size_limit() {
        let request = MeituanOrderQueryRequest::new(1, 1, 100);
        assert_eq!(request.page_size, 50); // Should be capped at 50
    }

    #[test]
    fn test_optional_fields_not_serialized() {
        let request = MeituanConvertRequest::new("sid", "7");
        let json_str = serde_json::to_string(&request).unwrap();

        assert!(!json_str.contains("miniCode"));
        assert!(!json_str.contains("customer_id"));
        assert!(!json_str.contains("platform"));
    }

    #[test]
    fn test_meituan_link_type_conversion() {
        assert_eq!(u32::from(MeituanLinkType::H5Long), 1);
        assert_eq!(u32::from(MeituanLinkType::H5Short), 2);
        assert_eq!(u32::from(MeituanLinkType::Deeplink), 3);
        assert_eq!(u32::from(MeituanLinkType::MiniProgram), 4);
        assert_eq!(u32::from(MeituanLinkType::Command), 5);
    }
}
