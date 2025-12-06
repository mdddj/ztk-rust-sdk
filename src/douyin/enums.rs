//! 抖音平台枚举类型
//!
//! 定义抖音平台特有的枚举类型

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// 抖音搜索排序类型
///
/// 控制商品搜索结果的排序方式
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum DouyinSearchType {
    /// 默认排序
    Default = 0,
    /// 历史销量排序
    Sales = 1,
    /// 价格排序
    Price = 2,
    /// 佣金金额排序
    Commission = 3,
    /// 佣金比例排序
    CommissionRatio = 4,
}

impl Default for DouyinSearchType {
    fn default() -> Self {
        DouyinSearchType::Default
    }
}

/// 抖音排序方向
///
/// 控制排序的升降序
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum DouyinSortType {
    /// 升序
    Asc = 0,
    /// 降序
    Desc = 1,
}

impl Default for DouyinSortType {
    fn default() -> Self {
        DouyinSortType::Desc
    }
}

/// 抖音商品标签
///
/// 筛选不同类型的商品
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum DouyinProductTag {
    /// 返回全量商品
    All = 0,
    /// 返回超值购商品
    SuperValue = 1,
    /// 返回抖音超市(次日达)商品
    Supermarket = 2,
}

impl Default for DouyinProductTag {
    fn default() -> Self {
        DouyinProductTag::All
    }
}

/// 抖音分销状态
///
/// 筛选商品的分销状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum DouyinShareStatus {
    /// 返回全量商品
    All = 0,
    /// 仅返回可分销商品
    Sharable = 1,
}

impl Default for DouyinShareStatus {
    fn default() -> Self {
        DouyinShareStatus::All
    }
}

/// 抖音订单查询类型
///
/// 控制订单查询的时间维度
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum DouyinOrderQueryType {
    /// 按订单支付时间获取
    PayTime = 1,
    /// 按订单更新时间获取
    UpdateTime = 2,
    /// 按订单编号获取
    OrderId = 3,
}

impl Default for DouyinOrderQueryType {
    fn default() -> Self {
        DouyinOrderQueryType::PayTime
    }
}

/// 抖音活动状态
///
/// 筛选活动的状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum DouyinActivityStatus {
    /// 待开始
    Pending = 1,
    /// 进行中
    Active = 2,
    /// 已结束
    Ended = 3,
}

impl Default for DouyinActivityStatus {
    fn default() -> Self {
        DouyinActivityStatus::Active
    }
}

/// 抖音口令类型
///
/// 口令解析返回的类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DouyinCommandType {
    /// 商品口令
    #[serde(rename = "1")]
    Product,
    /// 直播间口令
    #[serde(rename = "2")]
    Live,
    /// 活动口令
    #[serde(rename = "3")]
    Activity,
    /// 红包口令
    #[serde(rename = "4")]
    Redpack,
}

impl Default for DouyinCommandType {
    fn default() -> Self {
        DouyinCommandType::Product
    }
}

/// 抖音达人类型
///
/// 筛选达人的类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum DouyinAuthorType {
    /// 品牌
    Brand = 1,
    /// 达人
    Creator = 2,
}

impl Default for DouyinAuthorType {
    fn default() -> Self {
        DouyinAuthorType::Creator
    }
}

/// 抖音达人排序字段
///
/// 控制达人列表的排序方式
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum DouyinAuthorSortBy {
    /// 综合排序
    Comprehensive = 1,
    /// 销量排序
    Sales = 2,
    /// 佣金率排序
    CommissionRatio = 3,
    /// 粉丝数排序
    Followers = 4,
}

impl Default for DouyinAuthorSortBy {
    fn default() -> Self {
        DouyinAuthorSortBy::Comprehensive
    }
}

/// 抖音直播状态
///
/// 筛选主播的开播状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum DouyinLiveStatus {
    /// 所有
    All = 0,
    /// 开播中
    Live = 1,
    /// 未开播
    Offline = 2,
}

impl Default for DouyinLiveStatus {
    fn default() -> Self {
        DouyinLiveStatus::All
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_douyin_search_type_serialize() {
        let value = DouyinSearchType::Sales;
        let json = serde_json::to_string(&value).unwrap();
        assert_eq!(json, "1");
    }

    #[test]
    fn test_douyin_search_type_deserialize() {
        let value: DouyinSearchType = serde_json::from_str("2").unwrap();
        assert_eq!(value, DouyinSearchType::Price);
    }

    #[test]
    fn test_douyin_sort_type_serialize() {
        let value = DouyinSortType::Desc;
        let json = serde_json::to_string(&value).unwrap();
        assert_eq!(json, "1");
    }

    #[test]
    fn test_douyin_product_tag_serialize() {
        let value = DouyinProductTag::Supermarket;
        let json = serde_json::to_string(&value).unwrap();
        assert_eq!(json, "2");
    }

    #[test]
    fn test_douyin_share_status_serialize() {
        let value = DouyinShareStatus::Sharable;
        let json = serde_json::to_string(&value).unwrap();
        assert_eq!(json, "1");
    }

    #[test]
    fn test_douyin_order_query_type_serialize() {
        let value = DouyinOrderQueryType::UpdateTime;
        let json = serde_json::to_string(&value).unwrap();
        assert_eq!(json, "2");
    }

    #[test]
    fn test_douyin_activity_status_serialize() {
        let value = DouyinActivityStatus::Active;
        let json = serde_json::to_string(&value).unwrap();
        assert_eq!(json, "2");
    }

    #[test]
    fn test_douyin_command_type_serialize() {
        let value = DouyinCommandType::Live;
        let json = serde_json::to_string(&value).unwrap();
        assert_eq!(json, "\"2\"");
    }

    #[test]
    fn test_douyin_author_type_serialize() {
        let value = DouyinAuthorType::Brand;
        let json = serde_json::to_string(&value).unwrap();
        assert_eq!(json, "1");
    }

    #[test]
    fn test_douyin_author_sort_by_serialize() {
        let value = DouyinAuthorSortBy::Followers;
        let json = serde_json::to_string(&value).unwrap();
        assert_eq!(json, "4");
    }

    #[test]
    fn test_douyin_live_status_serialize() {
        let value = DouyinLiveStatus::Live;
        let json = serde_json::to_string(&value).unwrap();
        assert_eq!(json, "1");
    }
}
