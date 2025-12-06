//! 淘宝平台枚举类型
//!
//! 定义淘宝平台特有的枚举类型

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// 淘宝返回结果类型
///
/// 控制淘宝 API 返回的数据详细程度
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum TaobaoSignUrlType {
    /// 官方结果 (默认)
    Official = 0,
    /// 官方结果 (同 0)
    Official1 = 1,
    /// 官方结果 (同 0)
    Official2 = 2,
    /// 整合高佣转链+解析商品编号
    WithParse = 3,
    /// 整合+简版详情
    WithSimpleDetail = 4,
    /// 整合+全网详情+淘口令
    WithFullDetail = 5,
}

impl Default for TaobaoSignUrlType {
    fn default() -> Self {
        TaobaoSignUrlType::Official
    }
}

/// 淘宝平台类型
///
/// 区分淘宝、天猫、天猫超市
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaobaoPlatformType {
    /// 淘宝
    #[serde(rename = "1")]
    Taobao,
    /// 天猫
    #[serde(rename = "2")]
    Tmall,
    /// 天猫超市
    #[serde(rename = "3")]
    TmallSupermarket,
}

impl Default for TaobaoPlatformType {
    fn default() -> Self {
        TaobaoPlatformType::Taobao
    }
}

/// 淘宝店铺类型
///
/// 区分淘宝店和天猫店
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaobaoShopType {
    /// 淘宝店
    #[serde(rename = "0")]
    Taobao,
    /// 天猫店
    #[serde(rename = "1")]
    Tmall,
}

impl Default for TaobaoShopType {
    fn default() -> Self {
        TaobaoShopType::Taobao
    }
}

/// 淘宝订单查询类型
///
/// 控制订单查询的时间维度
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum TaobaoOrderQueryType {
    /// 按创建时间查询
    CreateTime = 1,
    /// 按结算时间查询
    SettleTime = 2,
    /// 按付款时间查询
    PayTime = 3,
}

impl Default for TaobaoOrderQueryType {
    fn default() -> Self {
        TaobaoOrderQueryType::CreateTime
    }
}

/// 淘宝订单状态
///
/// 订单的当前状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(i8)]
pub enum TaobaoOrderStatus {
    /// 全部
    All = -1,
    /// 已付款
    Paid = 3,
    /// 已收货
    Received = 12,
    /// 已结算
    Settled = 14,
    /// 已失效
    Invalid = 13,
}

impl Default for TaobaoOrderStatus {
    fn default() -> Self {
        TaobaoOrderStatus::All
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_taobao_sign_url_type_serialize() {
        let value = TaobaoSignUrlType::WithFullDetail;
        let json = serde_json::to_string(&value).unwrap();
        assert_eq!(json, "5");
    }

    #[test]
    fn test_taobao_sign_url_type_deserialize() {
        let value: TaobaoSignUrlType = serde_json::from_str("3").unwrap();
        assert_eq!(value, TaobaoSignUrlType::WithParse);
    }

    #[test]
    fn test_taobao_platform_type_serialize() {
        let value = TaobaoPlatformType::Tmall;
        let json = serde_json::to_string(&value).unwrap();
        assert_eq!(json, "\"2\"");
    }

    #[test]
    fn test_taobao_shop_type_serialize() {
        let value = TaobaoShopType::Tmall;
        let json = serde_json::to_string(&value).unwrap();
        assert_eq!(json, "\"1\"");
    }

    #[test]
    fn test_taobao_order_query_type_serialize() {
        let value = TaobaoOrderQueryType::SettleTime;
        let json = serde_json::to_string(&value).unwrap();
        assert_eq!(json, "2");
    }

    #[test]
    fn test_taobao_order_status_serialize() {
        let value = TaobaoOrderStatus::Settled;
        let json = serde_json::to_string(&value).unwrap();
        assert_eq!(json, "14");
    }
}
