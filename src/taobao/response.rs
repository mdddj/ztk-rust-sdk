//! 淘宝平台响应结构体
//!
//! 定义淘宝平台 API 的响应结构体

use serde::Deserialize;

/// 高佣转链响应 (商品ID)
///
/// convert_by_item_id 接口的响应
#[derive(Debug, Clone, Deserialize)]
pub struct ConvertResponse {
    /// 状态码，200 表示成功
    #[serde(default)]
    pub status: i32,
    /// 推广长链接
    #[serde(default)]
    pub item_url: Option<String>,
    /// 二合一推广链接 (带优惠券)
    #[serde(default)]
    pub coupon_click_url: Option<String>,
    /// 淘宝短链接
    #[serde(default)]
    pub shorturl: Option<String>,
    /// 淘宝短链接2 (可拉起手淘)
    #[serde(default)]
    pub shorturl2: Option<String>,
    /// 淘口令
    #[serde(default)]
    pub tkl: Option<String>,
    /// 商品 ID
    #[serde(default)]
    pub tao_id: Option<String>,
    /// 商品详情 (signurl=5 时返回)
    #[serde(default)]
    pub goods_detail: Option<GoodsDetail>,
}

/// 高佣转链响应 (淘口令)
///
/// convert_by_tkl 接口的响应
#[derive(Debug, Clone, Deserialize)]
pub struct ConvertByTklResponse {
    /// 状态码，200 表示成功
    #[serde(default)]
    pub status: i32,
    /// 转换后的文案
    #[serde(default)]
    pub content: Option<String>,
    /// 商品 ID
    #[serde(default)]
    pub tao_id: Option<String>,
    /// 平台类型: 1-淘宝, 2-京东, 3-拼多多, 4-苏宁, 5-唯品会, 6-美团
    #[serde(default)]
    pub plat: Option<String>,
    /// 淘宝子平台: 1-淘宝, 2-天猫, 3-天猫超市
    #[serde(default)]
    pub plat2: Option<String>,
    /// 是否有优惠券
    #[serde(default)]
    pub is_youquan: Option<String>,
    /// 商品主图
    #[serde(default)]
    pub pic: Option<String>,
    /// 一级分类 ID
    #[serde(default)]
    pub cid1: Option<String>,
    /// 一级分类名称
    #[serde(default)]
    pub cid1_name: Option<String>,
    /// 二级分类 ID
    #[serde(default)]
    pub cid2: Option<String>,
    /// 二级分类名称
    #[serde(default)]
    pub cid2_name: Option<String>,
    /// 总销量
    #[serde(default)]
    pub volume: Option<String>,
    /// 评论数量
    #[serde(default, rename = "commentCount")]
    pub comment_count: Option<String>,
    /// 商品短标题
    #[serde(default)]
    pub title: Option<String>,
    /// 商品长标题
    #[serde(default)]
    pub tao_title: Option<String>,
    /// 原始价格
    #[serde(default)]
    pub size: Option<String>,
    /// 券后价格
    #[serde(default)]
    pub quanhou_jiage: Option<String>,
    /// 佣金比例
    #[serde(default)]
    pub tkrate3: Option<String>,
    /// 佣金金额
    #[serde(default)]
    pub tkfee3: Option<String>,
    /// 优惠券金额
    #[serde(default)]
    pub coupon_info_money: Option<String>,
    /// 店铺昵称
    #[serde(default)]
    pub nick: Option<String>,
    /// 生成的推广淘口令
    #[serde(default)]
    pub result_tkl: Option<String>,
    /// 生成的推广链接
    #[serde(default)]
    pub result_url: Option<String>,
    /// 文案是否纯文字: 0-否, 1-是
    #[serde(default)]
    pub chunwenzi: Option<String>,
}

/// 批量高佣转链响应
///
/// batch_convert 接口的响应
#[derive(Debug, Clone, Deserialize)]
pub struct BatchConvertResponse {
    /// 状态码，200 表示成功
    #[serde(default)]
    pub status: i32,
    /// 转链结果列表
    #[serde(default)]
    pub data: Option<Vec<BatchConvertItem>>,
}

/// 批量转链结果项
#[derive(Debug, Clone, Deserialize)]
pub struct BatchConvertItem {
    /// 渠道关系 ID
    #[serde(default)]
    pub relation_id: Option<String>,
    /// 推广长链接
    #[serde(default)]
    pub item_url: Option<String>,
    /// 二合一推广链接
    #[serde(default)]
    pub coupon_click_url: Option<String>,
    /// 淘口令
    #[serde(default)]
    pub tkl: Option<String>,
}

/// 订单查询响应
///
/// query_orders 接口的响应
#[derive(Debug, Clone, Deserialize)]
pub struct QueryOrdersResponse {
    /// 状态码，200 表示成功
    #[serde(default)]
    pub status: i32,
    /// 订单列表
    #[serde(default)]
    pub data: Option<QueryOrdersData>,
}

/// 订单查询数据
#[derive(Debug, Clone, Deserialize)]
pub struct QueryOrdersData {
    /// 是否有下一页
    #[serde(default)]
    pub has_next: Option<bool>,
    /// 订单列表
    #[serde(default)]
    pub results: Option<Vec<OrderInfo>>,
}

/// 订单信息
#[derive(Debug, Clone, Deserialize)]
pub struct OrderInfo {
    /// 订单编号
    #[serde(default)]
    pub trade_id: Option<String>,
    /// 父订单编号
    #[serde(default)]
    pub trade_parent_id: Option<String>,
    /// 商品 ID
    #[serde(default)]
    pub num_iid: Option<String>,
    /// 商品标题
    #[serde(default)]
    pub item_title: Option<String>,
    /// 商品图片
    #[serde(default)]
    pub item_img: Option<String>,
    /// 商品数量
    #[serde(default)]
    pub item_num: Option<i32>,
    /// 商品单价
    #[serde(default)]
    pub item_price: Option<String>,
    /// 付款金额
    #[serde(default)]
    pub alipay_total_price: Option<String>,
    /// 预估收入
    #[serde(default)]
    pub pub_share_pre_fee: Option<String>,
    /// 结算金额
    #[serde(default)]
    pub pub_share_fee: Option<String>,
    /// 佣金比例
    #[serde(default)]
    pub total_commission_rate: Option<String>,
    /// 订单状态
    #[serde(default)]
    pub tk_status: Option<i32>,
    /// 创建时间
    #[serde(default)]
    pub tk_create_time: Option<String>,
    /// 付款时间
    #[serde(default)]
    pub tk_paid_time: Option<String>,
    /// 结算时间
    #[serde(default)]
    pub tk_earning_time: Option<String>,
    /// 渠道关系 ID
    #[serde(default)]
    pub relation_id: Option<String>,
    /// 会员运营 ID
    #[serde(default)]
    pub special_id: Option<String>,
    /// 店铺名称
    #[serde(default)]
    pub seller_shop_title: Option<String>,
    /// 卖家昵称
    #[serde(default)]
    pub seller_nick: Option<String>,
}

/// 创建淘口令响应
///
/// create_tkl 接口的响应
#[derive(Debug, Clone, Deserialize)]
pub struct CreateTklResponse {
    /// 状态码，200 表示成功
    #[serde(default)]
    pub status: i32,
    /// 生成的淘口令
    #[serde(default)]
    pub tkl: Option<String>,
    /// 淘口令模型
    #[serde(default)]
    pub model: Option<String>,
}

/// 解析商品编号响应
///
/// parse_item_id 接口的响应
#[derive(Debug, Clone, Deserialize)]
pub struct ParseItemIdResponse {
    /// 状态码，200 表示成功
    #[serde(default)]
    pub status: i32,
    /// 商品 ID
    #[serde(default)]
    pub num_iid: Option<String>,
    /// 商品 ID (别名)
    #[serde(default)]
    pub tao_id: Option<String>,
    /// 商品标题
    #[serde(default)]
    pub title: Option<String>,
    /// 商品图片
    #[serde(default)]
    pub pic: Option<String>,
}

/// 商品详情 (signurl=5 返回)
///
/// 包含商品的完整详细信息
#[derive(Debug, Clone, Deserialize)]
pub struct GoodsDetail {
    /// 折淘客编号
    #[serde(default)]
    pub code: Option<String>,
    /// 分类 ID
    #[serde(default)]
    pub type_one_id: Option<String>,
    /// 商品 ID
    #[serde(default)]
    pub tao_id: Option<String>,
    /// 商品短标题
    #[serde(default)]
    pub title: Option<String>,
    /// 商品简介
    #[serde(default)]
    pub jianjie: Option<String>,
    /// 商品主图
    #[serde(default)]
    pub pict_url: Option<String>,
    /// 是否天猫: 0-淘宝, 1-天猫
    #[serde(default)]
    pub user_type: Option<String>,
    /// 卖家 ID
    #[serde(default)]
    pub seller_id: Option<String>,
    /// 商品描述分
    #[serde(default)]
    pub shop_dsr: Option<String>,
    /// 年销量
    #[serde(default)]
    pub volume: Option<String>,
    /// 折扣价
    #[serde(default)]
    pub size: Option<String>,
    /// 券后价
    #[serde(default)]
    pub quanhou_jiage: Option<String>,
    /// 佣金比率
    #[serde(default)]
    pub tkrate3: Option<String>,
    /// 佣金类型
    #[serde(default)]
    pub yongjin_type: Option<String>,
    /// 优惠券 ID
    #[serde(default)]
    pub coupon_id: Option<String>,
    /// 优惠券开始时间
    #[serde(default)]
    pub coupon_start_time: Option<String>,
    /// 优惠券结束时间
    #[serde(default)]
    pub coupon_end_time: Option<String>,
    /// 优惠券金额
    #[serde(default)]
    pub coupon_info_money: Option<String>,
    /// 优惠券总数量
    #[serde(default)]
    pub coupon_total_count: Option<String>,
    /// 优惠券剩余数量
    #[serde(default)]
    pub coupon_remain_count: Option<String>,
    /// 优惠券信息
    #[serde(default)]
    pub coupon_info: Option<String>,
    /// 是否聚划算
    #[serde(default)]
    pub juhuasuan: Option<String>,
    /// 是否淘抢购
    #[serde(default)]
    pub taoqianggou: Option<String>,
    /// 是否海淘
    #[serde(default)]
    pub haitao: Option<String>,
    /// 是否极有家
    #[serde(default)]
    pub jiyoujia: Option<String>,
    /// 是否金牌卖家
    #[serde(default)]
    pub jinpaimaijia: Option<String>,
    /// 是否精选品牌
    #[serde(default)]
    pub pinpai: Option<String>,
    /// 品牌名称
    #[serde(default)]
    pub pinpai_name: Option<String>,
    /// 是否有运费险
    #[serde(default)]
    pub yunfeixian: Option<String>,
    /// 卖家昵称
    #[serde(default)]
    pub nick: Option<String>,
    /// 商品小图列表 (用 | 分隔)
    #[serde(default)]
    pub small_images: Option<String>,
    /// 商品白底图
    #[serde(default)]
    pub white_image: Option<String>,
    /// 商品长标题
    #[serde(default)]
    pub tao_title: Option<String>,
    /// 宝贝所在地
    #[serde(default)]
    pub provcity: Option<String>,
    /// 店铺名称
    #[serde(default)]
    pub shop_title: Option<String>,
    /// 视频地址
    #[serde(default)]
    pub zhibo_url: Option<String>,
    /// 实时总销量
    #[serde(default, rename = "sellCount")]
    pub sell_count: Option<String>,
    /// 评论数量
    #[serde(default, rename = "commentCount")]
    pub comment_count: Option<String>,
    /// 收藏数量
    #[serde(default)]
    pub favcount: Option<String>,
    /// 宝贝描述分
    #[serde(default)]
    pub score1: Option<String>,
    /// 卖家服务分
    #[serde(default)]
    pub score2: Option<String>,
    /// 物流服务分
    #[serde(default)]
    pub score3: Option<String>,
    /// 店铺等级 (1-20)
    #[serde(default, rename = "creditLevel")]
    pub credit_level: Option<String>,
    /// 店铺 logo
    #[serde(default, rename = "shopIcon")]
    pub shop_icon: Option<String>,
    /// 商品 URL
    #[serde(default)]
    pub taobao_url: Option<String>,
    /// 叶子类目 ID
    #[serde(default)]
    pub category_id: Option<String>,
    /// 叶子类目名称
    #[serde(default)]
    pub category_name: Option<String>,
    /// 一级类目 ID
    #[serde(default)]
    pub level_one_category_id: Option<String>,
    /// 一级类目名称
    #[serde(default)]
    pub level_one_category_name: Option<String>,
    /// 返佣金额
    #[serde(default)]
    pub tkfee3: Option<String>,
    /// 二合一推广链接
    #[serde(default)]
    pub coupon_click_url: Option<String>,
    /// 推广长链接
    #[serde(default)]
    pub item_url: Option<String>,
    /// 淘宝短链接
    #[serde(default)]
    pub shorturl: Option<String>,
    /// 淘宝短链接2 (可拉起手淘)
    #[serde(default)]
    pub shorturl2: Option<String>,
    /// 淘口令
    #[serde(default)]
    pub tkl: Option<String>,
}

/// 全网搜索商品响应
///
/// search_goods 接口的响应
#[derive(Debug, Clone, Deserialize)]
pub struct SearchGoodsResponse {
    /// 状态码，200 表示成功
    #[serde(default)]
    pub status: i32,
    /// 商品列表
    #[serde(default)]
    pub content: Option<Vec<SearchGoodsItem>>,
}

/// 全网搜索商品项
#[derive(Debug, Clone, Deserialize)]
pub struct SearchGoodsItem {
    /// 折淘客编号
    #[serde(default)]
    pub code: Option<String>,
    /// 分类 ID
    #[serde(default)]
    pub type_one_id: Option<String>,
    /// 商品 ID
    #[serde(default)]
    pub tao_id: Option<String>,
    /// 商品短标题
    #[serde(default)]
    pub title: Option<String>,
    /// 商品简介
    #[serde(default)]
    pub jianjie: Option<String>,
    /// 商品主图
    #[serde(default)]
    pub pict_url: Option<String>,
    /// 是否天猫: 0-淘宝, 1-天猫
    #[serde(default)]
    pub user_type: Option<String>,
    /// 卖家 ID
    #[serde(default)]
    pub seller_id: Option<String>,
    /// 商品描述分
    #[serde(default)]
    pub shop_dsr: Option<String>,
    /// 年销量
    #[serde(default)]
    pub volume: Option<String>,
    /// 折扣价
    #[serde(default)]
    pub size: Option<String>,
    /// 券后价
    #[serde(default)]
    pub quanhou_jiage: Option<String>,
    /// 数据更新时间
    #[serde(default)]
    pub date_time_yongjin: Option<String>,
    /// 佣金比率
    #[serde(default)]
    pub tkrate3: Option<String>,
    /// 佣金类型
    #[serde(default)]
    pub yongjin_type: Option<String>,
    /// 优惠券 ID
    #[serde(default)]
    pub coupon_id: Option<String>,
    /// 优惠券开始时间
    #[serde(default)]
    pub coupon_start_time: Option<String>,
    /// 优惠券结束时间
    #[serde(default)]
    pub coupon_end_time: Option<String>,
    /// 优惠券金额
    #[serde(default)]
    pub coupon_info_money: Option<String>,
    /// 优惠券总数量
    #[serde(default)]
    pub coupon_total_count: Option<String>,
    /// 优惠券剩余数量
    #[serde(default)]
    pub coupon_remain_count: Option<String>,
    /// 优惠券信息
    #[serde(default)]
    pub coupon_info: Option<String>,
    /// 是否聚划算
    #[serde(default)]
    pub juhuasuan: Option<String>,
    /// 是否淘抢购
    #[serde(default)]
    pub taoqianggou: Option<String>,
    /// 是否海淘
    #[serde(default)]
    pub haitao: Option<String>,
    /// 是否极有家
    #[serde(default)]
    pub jiyoujia: Option<String>,
    /// 是否金牌卖家
    #[serde(default)]
    pub jinpaimaijia: Option<String>,
    /// 是否精选品牌
    #[serde(default)]
    pub pinpai: Option<String>,
    /// 品牌名称
    #[serde(default)]
    pub pinpai_name: Option<String>,
    /// 是否有运费险
    #[serde(default)]
    pub yunfeixian: Option<String>,
    /// 卖家昵称
    #[serde(default)]
    pub nick: Option<String>,
    /// 商品小图列表 (用 | 分隔)
    #[serde(default)]
    pub small_images: Option<String>,
    /// 商品白底图
    #[serde(default)]
    pub white_image: Option<String>,
    /// 商品长标题
    #[serde(default)]
    pub tao_title: Option<String>,
    /// 宝贝所在地
    #[serde(default)]
    pub provcity: Option<String>,
    /// 店铺名称
    #[serde(default)]
    pub shop_title: Option<String>,
    /// 视频地址
    #[serde(default)]
    pub zhibo_url: Option<String>,
    /// 实时总销量
    #[serde(default, rename = "sellCount")]
    pub sell_count: Option<String>,
    /// 评论数量
    #[serde(default, rename = "commentCount")]
    pub comment_count: Option<String>,
    /// 收藏数量
    #[serde(default)]
    pub favcount: Option<String>,
    /// 宝贝描述分
    #[serde(default)]
    pub score1: Option<String>,
    /// 卖家服务分
    #[serde(default)]
    pub score2: Option<String>,
    /// 物流服务分
    #[serde(default)]
    pub score3: Option<String>,
    /// 店铺等级 (1-20)
    #[serde(default, rename = "creditLevel")]
    pub credit_level: Option<String>,
    /// 店铺 logo
    #[serde(default, rename = "shopIcon")]
    pub shop_icon: Option<String>,
    /// 商品 URL
    #[serde(default)]
    pub item_url: Option<String>,
    /// 叶子类目 ID
    #[serde(default)]
    pub category_id: Option<String>,
    /// 叶子类目名称
    #[serde(default)]
    pub category_name: Option<String>,
    /// 一级类目 ID
    #[serde(default)]
    pub level_one_category_id: Option<String>,
    /// 一级类目名称
    #[serde(default)]
    pub level_one_category_name: Option<String>,
    /// 返佣金额
    #[serde(default)]
    pub tkfee3: Option<String>,
    /// 店铺活动
    #[serde(default)]
    pub biaoqian: Option<String>,
    /// 朋友圈文案
    #[serde(default)]
    pub tag: Option<String>,
    /// 数据添加时间
    #[serde(default)]
    pub date_time: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_response_deserialize() {
        let json = r#"{
            "status": 200,
            "item_url": "https://s.click.taobao.com/xxx",
            "tkl": "淘口令内容"
        }"#;

        let response: ConvertResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.status, 200);
        assert_eq!(
            response.item_url,
            Some("https://s.click.taobao.com/xxx".to_string())
        );
        assert_eq!(response.tkl, Some("淘口令内容".to_string()));
    }

    #[test]
    fn test_convert_by_tkl_response_deserialize() {
        let json = r#"{
            "status": 200,
            "content": "转换后的文案",
            "tao_id": "123456789",
            "plat": "1",
            "result_tkl": "新淘口令"
        }"#;

        let response: ConvertByTklResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.status, 200);
        assert_eq!(response.content, Some("转换后的文案".to_string()));
        assert_eq!(response.tao_id, Some("123456789".to_string()));
        assert_eq!(response.plat, Some("1".to_string()));
        assert_eq!(response.result_tkl, Some("新淘口令".to_string()));
    }

    #[test]
    fn test_query_orders_response_deserialize() {
        let json = r#"{
            "status": 200,
            "data": {
                "has_next": true,
                "results": [
                    {
                        "trade_id": "123456",
                        "item_title": "测试商品",
                        "tk_status": 3
                    }
                ]
            }
        }"#;

        let response: QueryOrdersResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.status, 200);
        assert!(response.data.is_some());
        let data = response.data.unwrap();
        assert_eq!(data.has_next, Some(true));
        assert!(data.results.is_some());
        let results = data.results.unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].trade_id, Some("123456".to_string()));
    }

    #[test]
    fn test_create_tkl_response_deserialize() {
        let json = r#"{
            "status": 200,
            "tkl": "新生成的淘口令"
        }"#;

        let response: CreateTklResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.status, 200);
        assert_eq!(response.tkl, Some("新生成的淘口令".to_string()));
    }

    #[test]
    fn test_parse_item_id_response_deserialize() {
        let json = r#"{
            "status": 200,
            "num_iid": "987654321",
            "title": "商品标题"
        }"#;

        let response: ParseItemIdResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.status, 200);
        assert_eq!(response.num_iid, Some("987654321".to_string()));
        assert_eq!(response.title, Some("商品标题".to_string()));
    }

    #[test]
    fn test_goods_detail_deserialize() {
        let json = r#"{
            "tao_id": "123456789",
            "title": "商品标题",
            "quanhou_jiage": "99.00",
            "tkrate3": "20.00",
            "tkl": "淘口令"
        }"#;

        let detail: GoodsDetail = serde_json::from_str(json).unwrap();
        assert_eq!(detail.tao_id, Some("123456789".to_string()));
        assert_eq!(detail.title, Some("商品标题".to_string()));
        assert_eq!(detail.quanhou_jiage, Some("99.00".to_string()));
        assert_eq!(detail.tkrate3, Some("20.00".to_string()));
        assert_eq!(detail.tkl, Some("淘口令".to_string()));
    }

    #[test]
    fn test_response_with_missing_fields() {
        let json = r#"{"status": 200}"#;

        let response: ConvertResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.status, 200);
        assert!(response.item_url.is_none());
        assert!(response.tkl.is_none());
    }
}
