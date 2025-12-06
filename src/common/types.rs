//! 公共类型定义
//!
//! 定义 SDK 中使用的公共枚举和类型

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// 返回结果类型
///
/// 控制 API 返回的数据详细程度
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum SignUrlType {
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

impl Default for SignUrlType {
    fn default() -> Self {
        SignUrlType::Official
    }
}

/// 转链类型 (京东)
///
/// 控制京东转链返回的链接类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum ChainType {
    /// 长链
    Long = 1,
    /// 短链
    Short = 2,
    /// 长链+短链
    Both = 3,
}

impl Default for ChainType {
    fn default() -> Self {
        ChainType::Long
    }
}

/// 排序方向
///
/// 控制查询结果的排序顺序
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SortDirection {
    /// 升序
    Asc,
    /// 降序
    Desc,
}

impl Default for SortDirection {
    fn default() -> Self {
        SortDirection::Desc
    }
}

/// 京粉频道 ID
///
/// 京东京粉精选商品的频道分类
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(u32)]
pub enum JdEliteId {
    /// 好券商品
    GoodCoupon = 1,
    /// 精选卖场
    Selected = 2,
    /// 9.9 包邮
    NineNine = 10,
    /// 京东配送
    JdDelivery = 15,
    /// 实时热销榜
    HotSale = 22,
    /// 为你推荐
    Recommend = 23,
    /// 数码家电
    Digital = 24,
    /// 超市
    Supermarket = 25,
    /// 母婴玩具
    Baby = 26,
    /// 家具日用
    Home = 27,
    /// 美妆穿搭
    Beauty = 28,
    /// 图书文具
    Book = 30,
    /// 今日必推
    TodayMust = 31,
    /// 京东好物
    JdGood = 32,
    /// 京东秒杀
    Seckill = 33,
    /// 拼购商品
    Pingou = 34,
    /// 高收益榜
    HighProfit = 40,
    /// 自营热卖榜
    SelfHot = 41,
    /// 秒杀进行中
    SeckillNow = 108,
    /// 新品首发
    NewProduct = 109,
    /// 自营
    SelfOperated = 110,
    /// 京东爆品
    JdHot = 112,
    /// 首购商品
    FirstBuy = 125,
    /// 高佣榜单
    HighCommission = 129,
    /// 视频商品
    Video = 130,
    /// 历史最低价商品榜
    LowestPrice = 153,
}

impl Default for JdEliteId {
    fn default() -> Self {
        JdEliteId::GoodCoupon
    }
}

/// 京东排序字段
///
/// 京东商品查询的排序字段选项
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum JdSortField {
    /// 单价
    Price,
    /// 佣金比例
    CommissionShare,
    /// 佣金
    Commission,
    /// 30天引单量
    InOrderCount30DaysSku,
    /// 评论数
    Comments,
    /// 好评数
    GoodComments,
}

impl Default for JdSortField {
    fn default() -> Self {
        JdSortField::InOrderCount30DaysSku
    }
}

/// 订单查询类型
///
/// 控制订单查询的时间维度
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum OrderQueryType {
    /// 按下单时间
    OrderTime = 1,
    /// 按完成时间
    FinishTime = 2,
    /// 按更新时间
    UpdateTime = 3,
}

impl Default for OrderQueryType {
    fn default() -> Self {
        OrderQueryType::OrderTime
    }
}

/// 店铺类型
///
/// 区分淘宝店和天猫店
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum ShopType {
    /// 淘宝店
    Taobao = 0,
    /// 天猫店
    Tmall = 1,
}

impl Default for ShopType {
    fn default() -> Self {
        ShopType::Taobao
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sign_url_type_serialize() {
        let value = SignUrlType::WithFullDetail;
        let json = serde_json::to_string(&value).unwrap();
        assert_eq!(json, "5");
    }

    #[test]
    fn test_sign_url_type_deserialize() {
        let value: SignUrlType = serde_json::from_str("3").unwrap();
        assert_eq!(value, SignUrlType::WithParse);
    }

    #[test]
    fn test_chain_type_serialize() {
        let value = ChainType::Short;
        let json = serde_json::to_string(&value).unwrap();
        assert_eq!(json, "2");
    }

    #[test]
    fn test_chain_type_deserialize() {
        let value: ChainType = serde_json::from_str("1").unwrap();
        assert_eq!(value, ChainType::Long);
    }

    #[test]
    fn test_sort_direction_serialize() {
        let value = SortDirection::Asc;
        let json = serde_json::to_string(&value).unwrap();
        assert_eq!(json, "\"asc\"");
    }

    #[test]
    fn test_sort_direction_deserialize() {
        let value: SortDirection = serde_json::from_str("\"desc\"").unwrap();
        assert_eq!(value, SortDirection::Desc);
    }

    #[test]
    fn test_jd_elite_id_serialize() {
        let value = JdEliteId::HotSale;
        let json = serde_json::to_string(&value).unwrap();
        assert_eq!(json, "22");
    }

    #[test]
    fn test_jd_elite_id_deserialize() {
        let value: JdEliteId = serde_json::from_str("33").unwrap();
        assert_eq!(value, JdEliteId::Seckill);
    }

    #[test]
    fn test_jd_sort_field_serialize() {
        let value = JdSortField::CommissionShare;
        let json = serde_json::to_string(&value).unwrap();
        assert_eq!(json, "\"commissionShare\"");
    }

    #[test]
    fn test_order_query_type_serialize() {
        let value = OrderQueryType::UpdateTime;
        let json = serde_json::to_string(&value).unwrap();
        assert_eq!(json, "3");
    }

    #[test]
    fn test_shop_type_serialize() {
        let value = ShopType::Tmall;
        let json = serde_json::to_string(&value).unwrap();
        assert_eq!(json, "1");
    }
}
