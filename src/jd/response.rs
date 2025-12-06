//! 京东平台响应结构体
//!
//! 定义京东平台 API 的响应结构体

use serde::Deserialize;

/// 京东转链响应
///
/// convert 接口的响应
#[derive(Debug, Clone, Deserialize)]
pub struct JdConvertResponse {
    /// 状态码，200 表示成功
    #[serde(default)]
    pub status: i32,
    /// 转链结果数据
    #[serde(default)]
    pub content: Option<Vec<JdConvertResult>>,
}

/// 京东转链结果
#[derive(Debug, Clone, Deserialize)]
pub struct JdConvertResult {
    /// 折淘客编号
    #[serde(default)]
    pub code: Option<String>,
    /// 分类 ID
    #[serde(default)]
    pub type_one_id: Option<String>,
    /// 商品 ID (SKU)
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
    /// 是否京东自营: 0-非自营, 1-自营
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
    /// 是否京东配送: 1-是
    #[serde(default)]
    pub juhuasuan: Option<String>,
    /// 是否京东拼购: 1-是
    #[serde(default)]
    pub taoqianggou: Option<String>,
    /// 是否海淘 (京东国际): 1-是
    #[serde(default)]
    pub haitao: Option<String>,
    /// 是否京喜商品: 1-是
    #[serde(default)]
    pub jiyoujia: Option<String>,
    /// 是否京东好店: 1-是
    #[serde(default)]
    pub jinpaimaijia: Option<String>,
    /// 是否精选品牌: 1-是
    #[serde(default)]
    pub pinpai: Option<String>,
    /// 品牌名称
    #[serde(default)]
    pub pinpai_name: Option<String>,
    /// 是否有运费险: 1-有
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
    /// 图文详情图片地址 (用 | 分隔)
    #[serde(default, rename = "pcDescContent")]
    pub pc_desc_content: Option<String>,
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
    /// 朋友圈文案
    #[serde(default)]
    pub tag: Option<String>,
    /// 数据添加时间
    #[serde(default)]
    pub date_time: Option<String>,
    /// 二合一推广链接
    #[serde(default)]
    pub coupon_click_url: Option<String>,
    /// 推广短链接
    #[serde(default)]
    pub shorturl: Option<String>,
}

/// 京粉精选商品响应
///
/// jingfen_goods 接口的响应
#[derive(Debug, Clone, Deserialize)]
pub struct JingfenGoodsResponse {
    /// 京东联盟响应
    #[serde(default, rename = "jd_union_open_goods_jingfen_query_response")]
    pub response: Option<JingfenQueryResponse>,
}

/// 京粉查询响应
#[derive(Debug, Clone, Deserialize)]
pub struct JingfenQueryResponse {
    /// 查询结果
    #[serde(default, rename = "queryResult")]
    pub query_result: Option<JingfenQueryResult>,
}

/// 京粉查询结果
#[derive(Debug, Clone, Deserialize)]
pub struct JingfenQueryResult {
    /// 状态码
    #[serde(default)]
    pub code: Option<String>,
    /// 消息
    #[serde(default)]
    pub message: Option<String>,
    /// 商品列表
    #[serde(default)]
    pub data: Option<Vec<JingfenGoodsItem>>,
    /// 总数量
    #[serde(default, rename = "totalCount")]
    pub total_count: Option<i64>,
}

/// 京粉商品项
#[derive(Debug, Clone, Deserialize)]
pub struct JingfenGoodsItem {
    /// 商品 SKU ID
    #[serde(default, rename = "skuId")]
    pub sku_id: Option<i64>,
    /// 商品名称
    #[serde(default, rename = "skuName")]
    pub sku_name: Option<String>,
    /// 商品落地页
    #[serde(default, rename = "materialUrl")]
    pub material_url: Option<String>,
    /// 商品主图
    #[serde(default, rename = "imageInfo")]
    pub image_info: Option<JdImageInfo>,
    /// 价格信息
    #[serde(default, rename = "priceInfo")]
    pub price_info: Option<JdPriceInfo>,
    /// 佣金信息
    #[serde(default, rename = "commissionInfo")]
    pub commission_info: Option<JdCommissionInfo>,
    /// 优惠券信息
    #[serde(default, rename = "couponInfo")]
    pub coupon_info: Option<JdCouponInfo>,
    /// 店铺信息
    #[serde(default, rename = "shopInfo")]
    pub shop_info: Option<JdShopInfo>,
    /// 分类信息
    #[serde(default, rename = "categoryInfo")]
    pub category_info: Option<JdCategoryInfo>,
    /// 拼购信息
    #[serde(default, rename = "pinGouInfo")]
    pub pingou_info: Option<JdPingouInfo>,
    /// 30天引单量
    #[serde(default, rename = "inOrderCount30Days")]
    pub in_order_count_30_days: Option<i64>,
    /// 30天引单金额
    #[serde(default, rename = "inOrderComm30Days")]
    pub in_order_comm_30_days: Option<f64>,
    /// 评论数
    #[serde(default)]
    pub comments: Option<i64>,
    /// 好评率
    #[serde(default, rename = "goodCommentsShare")]
    pub good_comments_share: Option<f64>,
    /// 品牌编码
    #[serde(default, rename = "brandCode")]
    pub brand_code: Option<String>,
    /// 品牌名称
    #[serde(default, rename = "brandName")]
    pub brand_name: Option<String>,
    /// 店铺性质: g-自营, p-pop
    #[serde(default)]
    pub owner: Option<String>,
    /// 是否京东自营
    #[serde(default, rename = "isJdSale")]
    pub is_jd_sale: Option<i32>,
}

/// 京东图片信息
#[derive(Debug, Clone, Deserialize)]
pub struct JdImageInfo {
    /// 图片列表
    #[serde(default, rename = "imageList")]
    pub image_list: Option<Vec<JdImage>>,
    /// 白底图
    #[serde(default, rename = "whiteImage")]
    pub white_image: Option<String>,
}

/// 京东图片
#[derive(Debug, Clone, Deserialize)]
pub struct JdImage {
    /// 图片 URL
    #[serde(default)]
    pub url: Option<String>,
}

/// 京东价格信息
#[derive(Debug, Clone, Deserialize)]
pub struct JdPriceInfo {
    /// 商品价格
    #[serde(default)]
    pub price: Option<f64>,
    /// 最低价格
    #[serde(default, rename = "lowestPrice")]
    pub lowest_price: Option<f64>,
    /// 最低券后价
    #[serde(default, rename = "lowestCouponPrice")]
    pub lowest_coupon_price: Option<f64>,
    /// 历史最低价
    #[serde(default, rename = "historyPriceDay")]
    pub history_price_day: Option<i32>,
}

/// 京东佣金信息
#[derive(Debug, Clone, Deserialize)]
pub struct JdCommissionInfo {
    /// 佣金
    #[serde(default)]
    pub commission: Option<f64>,
    /// 佣金比例
    #[serde(default, rename = "commissionShare")]
    pub commission_share: Option<f64>,
    /// 券后佣金比例
    #[serde(default, rename = "couponCommission")]
    pub coupon_commission: Option<f64>,
    /// 加入计划佣金比例
    #[serde(default, rename = "plusCommissionShare")]
    pub plus_commission_share: Option<f64>,
}

/// 京东优惠券信息
#[derive(Debug, Clone, Deserialize)]
pub struct JdCouponInfo {
    /// 优惠券列表
    #[serde(default, rename = "couponList")]
    pub coupon_list: Option<Vec<JdCoupon>>,
}

/// 京东优惠券
#[derive(Debug, Clone, Deserialize)]
pub struct JdCoupon {
    /// 券类型: 1-平台券, 2-店铺券
    #[serde(default, rename = "bindType")]
    pub bind_type: Option<i32>,
    /// 优惠券面额
    #[serde(default)]
    pub discount: Option<f64>,
    /// 优惠券链接
    #[serde(default)]
    pub link: Option<String>,
    /// 券使用平台: 0-全平台, 1-限平台券, 2-限移动端
    #[serde(default, rename = "platformType")]
    pub platform_type: Option<i32>,
    /// 券消费限额
    #[serde(default)]
    pub quota: Option<f64>,
    /// 券开始时间
    #[serde(default, rename = "getStartTime")]
    pub get_start_time: Option<i64>,
    /// 券结束时间
    #[serde(default, rename = "getEndTime")]
    pub get_end_time: Option<i64>,
    /// 是否最优券
    #[serde(default, rename = "isBest")]
    pub is_best: Option<i32>,
}

/// 京东店铺信息
#[derive(Debug, Clone, Deserialize)]
pub struct JdShopInfo {
    /// 店铺 ID
    #[serde(default, rename = "shopId")]
    pub shop_id: Option<i64>,
    /// 店铺名称
    #[serde(default, rename = "shopName")]
    pub shop_name: Option<String>,
    /// 店铺等级
    #[serde(default, rename = "shopLevel")]
    pub shop_level: Option<f64>,
    /// 店铺评分
    #[serde(default, rename = "shopLabel")]
    pub shop_label: Option<String>,
}

/// 京东分类信息
#[derive(Debug, Clone, Deserialize)]
pub struct JdCategoryInfo {
    /// 一级类目 ID
    #[serde(default)]
    pub cid1: Option<i64>,
    /// 一级类目名称
    #[serde(default, rename = "cid1Name")]
    pub cid1_name: Option<String>,
    /// 二级类目 ID
    #[serde(default)]
    pub cid2: Option<i64>,
    /// 二级类目名称
    #[serde(default, rename = "cid2Name")]
    pub cid2_name: Option<String>,
    /// 三级类目 ID
    #[serde(default)]
    pub cid3: Option<i64>,
    /// 三级类目名称
    #[serde(default, rename = "cid3Name")]
    pub cid3_name: Option<String>,
}

/// 京东拼购信息
#[derive(Debug, Clone, Deserialize)]
pub struct JdPingouInfo {
    /// 拼购价格
    #[serde(default, rename = "pingouPrice")]
    pub pingou_price: Option<f64>,
    /// 拼购成团人数
    #[serde(default, rename = "pingouTmCount")]
    pub pingou_tm_count: Option<i64>,
    /// 拼购链接
    #[serde(default, rename = "pingouUrl")]
    pub pingou_url: Option<String>,
    /// 拼购开始时间
    #[serde(default, rename = "pingouStartTime")]
    pub pingou_start_time: Option<i64>,
    /// 拼购结束时间
    #[serde(default, rename = "pingouEndTime")]
    pub pingou_end_time: Option<i64>,
}

/// 京东订单查询响应
///
/// query_orders 接口的响应
#[derive(Debug, Clone, Deserialize)]
pub struct JdOrderResponse {
    /// 状态码，200 表示成功
    #[serde(default)]
    pub status: i32,
    /// 订单数据
    #[serde(default)]
    pub data: Option<JdOrderData>,
}

/// 京东订单数据
#[derive(Debug, Clone, Deserialize)]
pub struct JdOrderData {
    /// 是否有下一页
    #[serde(default)]
    pub has_more: Option<bool>,
    /// 订单列表
    #[serde(default)]
    pub data: Option<Vec<JdOrderInfo>>,
}

/// 京东订单信息
#[derive(Debug, Clone, Deserialize)]
pub struct JdOrderInfo {
    /// 订单号
    #[serde(default, rename = "orderId")]
    pub order_id: Option<i64>,
    /// 父订单号
    #[serde(default, rename = "parentId")]
    pub parent_id: Option<i64>,
    /// 下单时间
    #[serde(default, rename = "orderTime")]
    pub order_time: Option<String>,
    /// 完成时间
    #[serde(default, rename = "finishTime")]
    pub finish_time: Option<String>,
    /// 更新时间
    #[serde(default, rename = "modifyTime")]
    pub modify_time: Option<String>,
    /// 订单状态: 1-待付款, 2-已付款, 3-已完成, 4-已取消
    #[serde(default, rename = "validCode")]
    pub valid_code: Option<i32>,
    /// 商品 SKU ID
    #[serde(default, rename = "skuId")]
    pub sku_id: Option<i64>,
    /// 商品名称
    #[serde(default, rename = "skuName")]
    pub sku_name: Option<String>,
    /// 商品数量
    #[serde(default, rename = "skuNum")]
    pub sku_num: Option<i32>,
    /// 商品单价
    #[serde(default)]
    pub price: Option<f64>,
    /// 实际支付金额
    #[serde(default, rename = "actualCosPrice")]
    pub actual_cos_price: Option<f64>,
    /// 实际佣金
    #[serde(default, rename = "actualFee")]
    pub actual_fee: Option<f64>,
    /// 预估佣金
    #[serde(default, rename = "estimateCosPrice")]
    pub estimate_cos_price: Option<f64>,
    /// 预估佣金金额
    #[serde(default, rename = "estimateFee")]
    pub estimate_fee: Option<f64>,
    /// 佣金比例
    #[serde(default, rename = "commissionRate")]
    pub commission_rate: Option<f64>,
    /// 子渠道标识
    #[serde(default, rename = "subUnionId")]
    pub sub_union_id: Option<String>,
    /// 推广位 ID
    #[serde(default, rename = "positionId")]
    pub position_id: Option<i64>,
    /// 站点 ID
    #[serde(default, rename = "siteId")]
    pub site_id: Option<i64>,
    /// 联盟 ID
    #[serde(default, rename = "unionId")]
    pub union_id: Option<i64>,
    /// 一级类目 ID
    #[serde(default)]
    pub cid1: Option<i64>,
    /// 二级类目 ID
    #[serde(default)]
    pub cid2: Option<i64>,
    /// 三级类目 ID
    #[serde(default)]
    pub cid3: Option<i64>,
    /// 是否 plus 会员
    #[serde(default)]
    pub plus: Option<i32>,
}

/// 京东商品详情响应
///
/// goods_detail 接口的响应
#[derive(Debug, Clone, Deserialize)]
pub struct JdGoodsDetailResponse {
    /// 状态码，200 表示成功
    #[serde(default)]
    pub status: i32,
    /// 商品详情列表
    #[serde(default)]
    pub data: Option<Vec<JingfenGoodsItem>>,
}

/// 京东朋友圈火爆商品响应
///
/// hot_goods 接口的响应
#[derive(Debug, Clone, Deserialize)]
pub struct JdHotGoodsResponse {
    /// 状态码，200 表示成功
    #[serde(default)]
    pub status: i32,
    /// 商品列表
    #[serde(default)]
    pub content: Option<Vec<JdHotGoodsItem>>,
}

/// 京东朋友圈火爆商品项
#[derive(Debug, Clone, Deserialize)]
pub struct JdHotGoodsItem {
    /// 商品 ID
    #[serde(default)]
    pub goods_id: Option<i64>,
    /// 商品状态: 1-正常, 2-下架
    #[serde(default)]
    pub state: Option<i32>,
    /// 京东商品状态: 2-正常
    #[serde(default, rename = "J_state")]
    pub j_state: Option<i32>,
    /// 商品精编标题
    #[serde(default)]
    pub short_name: Option<String>,
    /// 商品一级类目
    #[serde(default)]
    pub goods_type: Option<i32>,
    /// 商品主图
    #[serde(default)]
    pub goods_img: Option<String>,
    /// 商品原价
    #[serde(default)]
    pub goods_price: Option<f64>,
    /// 商品券后价
    #[serde(default)]
    pub discount_price_after: Option<f64>,
    /// 推荐理由
    #[serde(default)]
    pub goods_recommend: Option<String>,
    /// 商品佣金比例
    #[serde(default)]
    pub commissionshare: Option<f64>,
    /// 商品落地页
    #[serde(default)]
    pub goods_link: Option<String>,
    /// 优惠券面额
    #[serde(default)]
    pub discount_price: Option<f64>,
    /// 领券链接
    #[serde(default)]
    pub discount_link: Option<String>,
    /// 今日领券数
    #[serde(default)]
    pub today_num: Option<i32>,
    /// 券消费限额
    #[serde(default)]
    pub quota: Option<f64>,
    /// 券开始时间
    #[serde(default)]
    pub discount_start: Option<String>,
    /// 券结束时间
    #[serde(default)]
    pub discount_end: Option<String>,
    /// 店铺 ID
    #[serde(default)]
    pub shop_id: Option<i64>,
    /// 店铺名称
    #[serde(default)]
    pub shop_name: Option<String>,
    /// 近30天引入单量
    #[serde(default, rename = "inOrderCount30Days")]
    pub in_order_count_30_days: Option<i64>,
    /// 京东图片合集
    #[serde(default, rename = "imageList")]
    pub image_list: Option<String>,
    /// 商品细节图
    #[serde(default)]
    pub img_info: Option<String>,
    /// 商品更新时间
    #[serde(default)]
    pub update_time: Option<String>,
    /// 品牌 ID
    #[serde(default)]
    pub brand_id: Option<String>,
    /// 京东品牌 code
    #[serde(default, rename = "brandCode")]
    pub brand_code: Option<String>,
    /// 京东品牌名称
    #[serde(default, rename = "brandName")]
    pub brand_name: Option<String>,
    /// 店铺性质: g-自营, p-pop
    #[serde(default)]
    pub owner: Option<String>,
    /// 京东一级类目 ID
    #[serde(default)]
    pub cid1: Option<i64>,
    /// 京东一级类目名称
    #[serde(default, rename = "cid1Name")]
    pub cid1_name: Option<String>,
    /// 京东二级类目 ID
    #[serde(default)]
    pub cid2: Option<i64>,
    /// 京东二级类目名称
    #[serde(default, rename = "cid2Name")]
    pub cid2_name: Option<String>,
    /// 京东三级类目 ID
    #[serde(default)]
    pub cid3: Option<i64>,
    /// 京东三级类目名称
    #[serde(default, rename = "cid3Name")]
    pub cid3_name: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jd_convert_response_deserialize() {
        let json = r#"{
            "status": 200,
            "content": [{
                "tao_id": "10025768652616",
                "title": "测试商品",
                "quanhou_jiage": "99.00",
                "coupon_click_url": "https://union-click.jd.com/xxx",
                "shorturl": "https://u.jd.com/xxx"
            }]
        }"#;

        let response: JdConvertResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.status, 200);
        assert!(response.content.is_some());
        let content = response.content.unwrap();
        assert_eq!(content.len(), 1);
        assert_eq!(content[0].tao_id, Some("10025768652616".to_string()));
        assert_eq!(content[0].title, Some("测试商品".to_string()));
    }

    #[test]
    fn test_jingfen_goods_response_deserialize() {
        let json = r#"{
            "jd_union_open_goods_jingfen_query_response": {
                "queryResult": {
                    "code": "200",
                    "message": "success",
                    "data": [{
                        "skuId": 10025768652616,
                        "skuName": "测试商品",
                        "inOrderCount30Days": 100
                    }],
                    "totalCount": 1
                }
            }
        }"#;

        let response: JingfenGoodsResponse = serde_json::from_str(json).unwrap();
        assert!(response.response.is_some());
        let query_response = response.response.unwrap();
        assert!(query_response.query_result.is_some());
        let result = query_response.query_result.unwrap();
        assert_eq!(result.code, Some("200".to_string()));
        assert!(result.data.is_some());
        let data = result.data.unwrap();
        assert_eq!(data.len(), 1);
        assert_eq!(data[0].sku_id, Some(10025768652616));
    }

    #[test]
    fn test_jd_order_response_deserialize() {
        let json = r#"{
            "status": 200,
            "data": {
                "has_more": true,
                "data": [{
                    "orderId": 123456789,
                    "skuId": 10025768652616,
                    "skuName": "测试商品",
                    "validCode": 2,
                    "price": 99.00,
                    "actualFee": 9.90
                }]
            }
        }"#;

        let response: JdOrderResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.status, 200);
        assert!(response.data.is_some());
        let data = response.data.unwrap();
        assert_eq!(data.has_more, Some(true));
        assert!(data.data.is_some());
        let orders = data.data.unwrap();
        assert_eq!(orders.len(), 1);
        assert_eq!(orders[0].order_id, Some(123456789));
    }

    #[test]
    fn test_jd_hot_goods_response_deserialize() {
        let json = r#"{
            "status": 200,
            "content": [{
                "goods_id": 10025768652616,
                "short_name": "测试商品",
                "goods_price": 99.00,
                "discount_price_after": 79.00,
                "commissionshare": 20.0,
                "shop_name": "测试店铺"
            }]
        }"#;

        let response: JdHotGoodsResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.status, 200);
        assert!(response.content.is_some());
        let content = response.content.unwrap();
        assert_eq!(content.len(), 1);
        assert_eq!(content[0].goods_id, Some(10025768652616));
        assert_eq!(content[0].short_name, Some("测试商品".to_string()));
    }

    #[test]
    fn test_response_with_missing_fields() {
        let json = r#"{"status": 200}"#;

        let response: JdConvertResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.status, 200);
        assert!(response.content.is_none());
    }

    #[test]
    fn test_jd_price_info_deserialize() {
        let json = r#"{
            "price": 99.00,
            "lowestPrice": 89.00,
            "lowestCouponPrice": 79.00
        }"#;

        let price_info: JdPriceInfo = serde_json::from_str(json).unwrap();
        assert_eq!(price_info.price, Some(99.00));
        assert_eq!(price_info.lowest_price, Some(89.00));
        assert_eq!(price_info.lowest_coupon_price, Some(79.00));
    }

    #[test]
    fn test_jd_commission_info_deserialize() {
        let json = r#"{
            "commission": 9.90,
            "commissionShare": 10.0
        }"#;

        let commission_info: JdCommissionInfo = serde_json::from_str(json).unwrap();
        assert_eq!(commission_info.commission, Some(9.90));
        assert_eq!(commission_info.commission_share, Some(10.0));
    }
}
