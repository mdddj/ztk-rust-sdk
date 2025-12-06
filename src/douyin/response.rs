//! 抖音平台响应结构体
//!
//! 定义抖音平台 API 的响应结构体

use serde::Deserialize;

/// 抖音商品转链响应
///
/// convert_goods 接口的响应
#[derive(Debug, Clone, Deserialize)]
pub struct DouyinGoodsConvertResponse {
    /// 状态码，10000 表示成功
    #[serde(default)]
    pub code: i32,
    /// 状态消息
    #[serde(default)]
    pub msg: Option<String>,
    /// 子错误码
    #[serde(default)]
    pub sub_code: Option<String>,
    /// 子错误消息
    #[serde(default)]
    pub sub_msg: Option<String>,
    /// 响应数据
    #[serde(default)]
    pub data: Option<DouyinGoodsConvertData>,
}

/// 抖音商品转链数据
#[derive(Debug, Clone, Deserialize)]
pub struct DouyinGoodsConvertData {
    /// 转链数据
    #[serde(default)]
    pub data: Option<DouyinConvertResult>,
}

/// 抖音转链结果
#[derive(Debug, Clone, Deserialize)]
pub struct DouyinConvertResult {
    /// 抖音 deeplink
    #[serde(default)]
    pub dy_deeplink: Option<String>,
    /// 抖音口令
    #[serde(default)]
    pub dy_password: Option<String>,
    /// 抖音短链接
    #[serde(default)]
    pub dy_zlink: Option<String>,
    /// 二维码信息
    #[serde(default)]
    pub qr_code: Option<DouyinQrCode>,
    /// 分享链接
    #[serde(default)]
    pub share_link: Option<String>,
    /// 是否使用活动
    #[serde(default)]
    pub use_ins_activity: Option<String>,
    /// 优惠券链接信息
    #[serde(default)]
    pub coupon_link: Option<DouyinCouponLink>,
}

/// 抖音二维码信息
#[derive(Debug, Clone, Deserialize)]
pub struct DouyinQrCode {
    /// 二维码高度
    #[serde(default)]
    pub height: Option<String>,
    /// 二维码 URL
    #[serde(default)]
    pub url: Option<String>,
    /// 二维码宽度
    #[serde(default)]
    pub width: Option<String>,
}

/// 抖音优惠券链接信息
#[derive(Debug, Clone, Deserialize)]
pub struct DouyinCouponLink {
    /// 优惠券状态
    #[serde(default)]
    pub coupon_status: Option<String>,
    /// deeplink
    #[serde(default)]
    pub deeplink: Option<String>,
    /// 二维码
    #[serde(default)]
    pub qrcode: Option<DouyinQrCode>,
    /// 分享口令
    #[serde(default)]
    pub share_command: Option<String>,
    /// 分享链接
    #[serde(default)]
    pub share_link: Option<String>,
}

/// 抖音直播间转链响应
///
/// convert_live 接口的响应
#[derive(Debug, Clone, Deserialize)]
pub struct DouyinLiveConvertResponse {
    /// 状态码，10000 表示成功
    #[serde(default)]
    pub code: i32,
    /// 状态消息
    #[serde(default)]
    pub msg: Option<String>,
    /// 子错误码
    #[serde(default)]
    pub sub_code: Option<String>,
    /// 子错误消息
    #[serde(default)]
    pub sub_msg: Option<String>,
    /// 响应数据
    #[serde(default)]
    pub data: Option<DouyinLiveConvertData>,
}

/// 抖音直播间转链数据
#[derive(Debug, Clone, Deserialize)]
pub struct DouyinLiveConvertData {
    /// 转链数据
    #[serde(default)]
    pub data: Option<DouyinLiveConvertResult>,
}

/// 抖音直播间转链结果
#[derive(Debug, Clone, Deserialize)]
pub struct DouyinLiveConvertResult {
    /// 抖音 deeplink
    #[serde(default)]
    pub dy_deeplink: Option<String>,
    /// 抖音口令
    #[serde(default)]
    pub dy_password: Option<String>,
    /// 抖音短链接
    #[serde(default)]
    pub dy_zlink: Option<String>,
    /// 二维码信息
    #[serde(default)]
    pub qr_code: Option<DouyinQrCode>,
}

/// 抖音活动转链响应
///
/// convert_activity 接口的响应
#[derive(Debug, Clone, Deserialize)]
pub struct DouyinActivityConvertResponse {
    /// 状态码，10000 表示成功
    #[serde(default)]
    pub code: i32,
    /// 状态消息
    #[serde(default)]
    pub msg: Option<String>,
    /// 子错误码
    #[serde(default)]
    pub sub_code: Option<String>,
    /// 子错误消息
    #[serde(default)]
    pub sub_msg: Option<String>,
    /// 响应数据
    #[serde(default)]
    pub data: Option<DouyinActivityConvertData>,
}

/// 抖音活动转链数据
#[derive(Debug, Clone, Deserialize)]
pub struct DouyinActivityConvertData {
    /// 转链数据
    #[serde(default)]
    pub data: Option<DouyinLiveConvertResult>,
}

/// 抖音商品详情响应
///
/// goods_detail 接口的响应
#[derive(Debug, Clone, Deserialize)]
pub struct DouyinGoodsDetailResponse {
    /// 状态码，10000 表示成功
    #[serde(default)]
    pub code: i32,
    /// 状态消息
    #[serde(default)]
    pub msg: Option<String>,
    /// 子错误码
    #[serde(default)]
    pub sub_code: Option<String>,
    /// 子错误消息
    #[serde(default)]
    pub sub_msg: Option<String>,
    /// 响应数据
    #[serde(default)]
    pub data: Option<DouyinGoodsDetailData>,
}

/// 抖音商品详情数据
#[derive(Debug, Clone, Deserialize)]
pub struct DouyinGoodsDetailData {
    /// 商品列表
    #[serde(default)]
    pub products: Option<Vec<DouyinProductDetail>>,
}

/// 抖音商品详情
#[derive(Debug, Clone, Deserialize)]
pub struct DouyinProductDetail {
    /// 基础信息
    #[serde(default)]
    pub base_info: Option<DouyinProductBaseInfo>,
    /// 推广信息
    #[serde(default)]
    pub promotion_info: Option<DouyinPromotionInfo>,
    /// 店铺信息
    #[serde(default)]
    pub shop_info: Option<DouyinShopInfo>,
    /// 优惠券信息
    #[serde(default)]
    pub coupon_info: Option<DouyinCouponInfo>,
    /// 评论信息
    #[serde(default)]
    pub comment_info: Option<DouyinCommentInfo>,
    /// 品牌信息
    #[serde(default)]
    pub brand_info: Option<DouyinBrandInfo>,
    /// 物流信息
    #[serde(default)]
    pub logistics_info: Option<DouyinLogisticsInfo>,
    /// 标签信息
    #[serde(default)]
    pub tags: Option<DouyinProductTags>,
}

/// 抖音商品基础信息
#[derive(Debug, Clone, Deserialize)]
pub struct DouyinProductBaseInfo {
    /// 商品 ID
    #[serde(default)]
    pub product_id: Option<String>,
    /// 商品标题
    #[serde(default)]
    pub title: Option<String>,
    /// 商品封面图
    #[serde(default)]
    pub cover: Option<String>,
    /// 商品价格 (单位: 分)
    #[serde(default)]
    pub price: Option<String>,
    /// 销量
    #[serde(default)]
    pub sales: Option<String>,
    /// 是否有库存
    #[serde(default)]
    pub in_stock: Option<String>,
    /// 详情页 URL
    #[serde(default)]
    pub detail_url: Option<String>,
    /// 一级类目 ID
    #[serde(default)]
    pub first_cid: Option<String>,
    /// 二级类目 ID
    #[serde(default)]
    pub second_cid: Option<String>,
    /// 三级类目 ID
    #[serde(default)]
    pub third_cid: Option<String>,
    /// 类目 ID
    #[serde(default)]
    pub category_id: Option<String>,
    /// 类目名称
    #[serde(default)]
    pub category_name: Option<String>,
    /// 轮播图列表
    #[serde(default)]
    pub imgs: Option<String>,
    /// 是否成功
    #[serde(default)]
    pub success: Option<String>,
}

/// 抖音推广信息
#[derive(Debug, Clone, Deserialize)]
pub struct DouyinPromotionInfo {
    /// 佣金金额 (单位: 分)
    #[serde(default)]
    pub cos_fee: Option<String>,
    /// 佣金比例 (乘100)
    #[serde(default)]
    pub cos_ratio: Option<String>,
    /// 是否成功
    #[serde(default)]
    pub success: Option<String>,
}

/// 抖音店铺信息
#[derive(Debug, Clone, Deserialize)]
pub struct DouyinShopInfo {
    /// 店铺 ID
    #[serde(default)]
    pub shop_id: Option<String>,
    /// 店铺名称
    #[serde(default)]
    pub shop_name: Option<String>,
    /// 店铺评分
    #[serde(default)]
    pub shop_total_score: Option<DouyinShopScore>,
    /// 是否成功
    #[serde(default)]
    pub success: Option<String>,
}

/// 抖音店铺评分
#[derive(Debug, Clone, Deserialize)]
pub struct DouyinShopScore {
    /// 物流体验分
    #[serde(default)]
    pub logistics_score: Option<DouyinScoreItem>,
    /// 商品体验分
    #[serde(default)]
    pub product_score: Option<DouyinScoreItem>,
    /// 商家服务分
    #[serde(default)]
    pub service_score: Option<DouyinScoreItem>,
    /// 商家体验分
    #[serde(default)]
    pub shop_score: Option<DouyinScoreItem>,
}

/// 抖音评分项
#[derive(Debug, Clone, Deserialize)]
pub struct DouyinScoreItem {
    /// 等级
    #[serde(default)]
    pub level: Option<String>,
    /// 分数
    #[serde(default)]
    pub score: Option<String>,
    /// 描述文本
    #[serde(default)]
    pub text: Option<String>,
}

/// 抖音优惠券信息
#[derive(Debug, Clone, Deserialize)]
pub struct DouyinCouponInfo {
    /// 券后价 (单位: 分)
    #[serde(default)]
    pub coupon_price: Option<String>,
    /// 可用优惠券列表
    #[serde(default)]
    pub available_coupons: Option<Vec<DouyinCoupon>>,
    /// 是否成功
    #[serde(default)]
    pub success: Option<String>,
}

/// 抖音优惠券
#[derive(Debug, Clone, Deserialize)]
pub struct DouyinCoupon {
    /// 优惠券类型
    #[serde(default)]
    pub coupon_type: Option<String>,
    /// 优惠描述
    #[serde(default)]
    pub discount_desc: Option<String>,
    /// 类型描述
    #[serde(default)]
    pub type_desc: Option<String>,
    /// 申请开始时间
    #[serde(default)]
    pub apply_start_time: Option<String>,
    /// 申请结束时间
    #[serde(default)]
    pub apply_end_time: Option<String>,
    /// 使用开始时间
    #[serde(default)]
    pub use_start_time: Option<String>,
    /// 使用结束时间
    #[serde(default)]
    pub use_end_time: Option<String>,
    /// 有效期 (秒)
    #[serde(default)]
    pub valid_period: Option<String>,
    /// 有效期类型
    #[serde(default)]
    pub validity_type: Option<String>,
}

/// 抖音评论信息
#[derive(Debug, Clone, Deserialize)]
pub struct DouyinCommentInfo {
    /// 评论数量
    #[serde(default)]
    pub comment_num: Option<String>,
    /// 评论评分
    #[serde(default)]
    pub comment_score: Option<String>,
    /// 是否成功
    #[serde(default)]
    pub success: Option<String>,
}

/// 抖音品牌信息
#[derive(Debug, Clone, Deserialize)]
pub struct DouyinBrandInfo {
    /// 品牌 ID
    #[serde(default)]
    pub brand_id: Option<String>,
    /// 品牌中文名
    #[serde(default)]
    pub brand_name_cn: Option<String>,
    /// 品牌英文名
    #[serde(default)]
    pub brand_name_en: Option<String>,
    /// 是否成功
    #[serde(default)]
    pub success: Option<String>,
}

/// 抖音物流信息
#[derive(Debug, Clone, Deserialize)]
pub struct DouyinLogisticsInfo {
    /// 是否包邮
    #[serde(default)]
    pub post_free: Option<String>,
    /// 物流描述
    #[serde(default)]
    pub text: Option<String>,
    /// 是否成功
    #[serde(default)]
    pub success: Option<String>,
}

/// 抖音商品标签
#[derive(Debug, Clone, Deserialize)]
pub struct DouyinProductTags {
    /// 是否有抖音商品标签
    #[serde(default)]
    pub has_douin_goods_tag: Option<String>,
    /// 是否有店铺品牌标签
    #[serde(default)]
    pub has_shop_brand_tag: Option<String>,
    /// 是否有补贴标签
    #[serde(default)]
    pub has_subsidy_tag: Option<String>,
    /// 是否有超市标签
    #[serde(default)]
    pub has_supermarket_tag: Option<String>,
    /// 是否成功
    #[serde(default)]
    pub success: Option<String>,
}

/// 抖音商品搜索响应
///
/// search_goods 接口的响应
#[derive(Debug, Clone, Deserialize)]
pub struct DouyinSearchGoodsResponse {
    /// 状态码，10000 表示成功
    #[serde(default)]
    pub code: i32,
    /// 状态消息
    #[serde(default)]
    pub msg: Option<String>,
    /// 子错误码
    #[serde(default)]
    pub sub_code: Option<String>,
    /// 子错误消息
    #[serde(default)]
    pub sub_msg: Option<String>,
    /// 响应数据
    #[serde(default)]
    pub data: Option<DouyinSearchGoodsData>,
}

/// 抖音商品搜索数据
#[derive(Debug, Clone, Deserialize)]
pub struct DouyinSearchGoodsData {
    /// 商品列表
    #[serde(default)]
    pub products: Option<Vec<DouyinSearchProduct>>,
    /// 总数量
    #[serde(default)]
    pub total: Option<String>,
}

/// 抖音搜索商品
#[derive(Debug, Clone, Deserialize)]
pub struct DouyinSearchProduct {
    /// 商品 ID
    #[serde(default)]
    pub product_id: Option<String>,
    /// 商品标题
    #[serde(default)]
    pub title: Option<String>,
    /// 商品封面图
    #[serde(default)]
    pub cover: Option<String>,
    /// 商品价格 (单位: 分)
    #[serde(default)]
    pub price: Option<String>,
    /// 券后价 (单位: 分)
    #[serde(default)]
    pub coupon_price: Option<String>,
    /// 销量
    #[serde(default)]
    pub sales: Option<String>,
    /// 佣金金额 (单位: 分)
    #[serde(default)]
    pub cos_fee: Option<String>,
    /// 佣金比例 (乘100)
    #[serde(default)]
    pub cos_ratio: Option<String>,
    /// 详情页 URL
    #[serde(default)]
    pub detail_url: Option<String>,
    /// 一级类目 ID
    #[serde(default)]
    pub first_cid: Option<String>,
    /// 二级类目 ID
    #[serde(default)]
    pub second_cid: Option<String>,
    /// 三级类目 ID
    #[serde(default)]
    pub third_cid: Option<String>,
    /// 是否有库存
    #[serde(default)]
    pub in_stock: Option<String>,
    /// 是否可分销
    #[serde(default)]
    pub sharable: Option<String>,
    /// 是否包邮
    #[serde(default)]
    pub post_free: Option<String>,
    /// 店铺 ID
    #[serde(default)]
    pub shop_id: Option<String>,
    /// 店铺名称
    #[serde(default)]
    pub shop_name: Option<String>,
    /// 活动 ID
    #[serde(default)]
    pub activity_id: Option<String>,
    /// 可用优惠券列表
    #[serde(default)]
    pub available_coupons: Option<Vec<DouyinCoupon>>,
}

/// 抖音口令解析响应
///
/// parse_command 接口的响应
#[derive(Debug, Clone, Deserialize)]
pub struct DouyinParseCommandResponse {
    /// 状态码，10000 表示成功
    #[serde(default)]
    pub code: i32,
    /// 状态消息
    #[serde(default)]
    pub msg: Option<String>,
    /// 子错误码
    #[serde(default)]
    pub sub_code: Option<String>,
    /// 子错误消息
    #[serde(default)]
    pub sub_msg: Option<String>,
    /// 响应数据
    #[serde(default)]
    pub data: Option<DouyinParseCommandData>,
}

/// 抖音口令解析数据
#[derive(Debug, Clone, Deserialize)]
pub struct DouyinParseCommandData {
    /// 口令信息
    #[serde(default)]
    pub command_info: Option<DouyinCommandInfo>,
}

/// 抖音口令信息
#[derive(Debug, Clone, Deserialize)]
pub struct DouyinCommandInfo {
    /// 口令类型
    #[serde(default)]
    pub command_type: Option<String>,
    /// 商品信息
    #[serde(default)]
    pub product_info: Option<DouyinCommandProductInfo>,
    /// 直播间信息
    #[serde(default)]
    pub live_info: Option<DouyinCommandLiveInfo>,
    /// 活动信息
    #[serde(default)]
    pub activity_info: Option<DouyinCommandActivityInfo>,
    /// 红包信息
    #[serde(default)]
    pub redpack_info: Option<DouyinCommandRedpackInfo>,
    /// 直播预约信息
    #[serde(default)]
    pub live_appoint_info: Option<DouyinCommandLiveAppointInfo>,
}

/// 抖音口令商品信息
#[derive(Debug, Clone, Deserialize)]
pub struct DouyinCommandProductInfo {
    /// 商品 ID
    #[serde(default)]
    pub product_id: Option<String>,
    /// 活动参数
    #[serde(default)]
    pub ins_activity_param: Option<String>,
    /// 分享信息
    #[serde(default)]
    pub share_info: Option<DouyinShareInfo>,
}

/// 抖音口令直播间信息
#[derive(Debug, Clone, Deserialize)]
pub struct DouyinCommandLiveInfo {
    /// 主播百应 ID
    #[serde(default)]
    pub author_buyin_id: Option<String>,
    /// 商品 ID
    #[serde(default)]
    pub product_id: Option<String>,
    /// 分享信息
    #[serde(default)]
    pub share_info: Option<DouyinShareInfo>,
}

/// 抖音口令活动信息
#[derive(Debug, Clone, Deserialize)]
pub struct DouyinCommandActivityInfo {
    /// 物料 ID
    #[serde(default)]
    pub material_id: Option<String>,
    /// 额外参数
    #[serde(default)]
    pub extra_params: Option<String>,
    /// 分享信息
    #[serde(default)]
    pub share_info: Option<DouyinShareInfo>,
}

/// 抖音口令红包信息
#[derive(Debug, Clone, Deserialize)]
pub struct DouyinCommandRedpackInfo {
    /// 红包 ID
    #[serde(default)]
    pub redpack_id: Option<String>,
    /// 分享信息
    #[serde(default)]
    pub share_info: Option<DouyinShareInfo>,
}

/// 抖音口令直播预约信息
#[derive(Debug, Clone, Deserialize)]
pub struct DouyinCommandLiveAppointInfo {
    /// 主播百应 ID
    #[serde(default)]
    pub author_buyin_id: Option<String>,
    /// 直播预约 ID
    #[serde(default)]
    pub live_appointment_id: Option<String>,
    /// 分享信息
    #[serde(default)]
    pub share_info: Option<DouyinShareInfo>,
}

/// 抖音分享信息
#[derive(Debug, Clone, Deserialize)]
pub struct DouyinShareInfo {
    /// deeplink
    #[serde(default)]
    pub deeplink: Option<String>,
    /// 短链接
    #[serde(default)]
    pub zlink: Option<String>,
    /// 分享口令
    #[serde(default)]
    pub share_command: Option<String>,
    /// 分享链接
    #[serde(default)]
    pub share_link: Option<String>,
    /// 二维码
    #[serde(default)]
    pub qrcode: Option<DouyinQrCode>,
}

/// 抖音订单查询响应
///
/// query_orders 接口的响应
#[derive(Debug, Clone, Deserialize)]
pub struct DouyinOrderQueryResponse {
    /// 状态码，200 表示成功
    #[serde(default)]
    pub status: i32,
    /// 订单列表
    #[serde(default)]
    pub content: Option<Vec<DouyinOrderInfo>>,
}

/// 抖音订单信息
#[derive(Debug, Clone, Deserialize)]
pub struct DouyinOrderInfo {
    /// 折淘客自动增长列
    #[serde(default)]
    pub code: Option<String>,
    /// 折淘客账号
    #[serde(default)]
    pub account: Option<String>,
    /// 折淘客 appkey
    #[serde(default)]
    pub appkey_ztk: Option<String>,
    /// 原始推广位 sid
    #[serde(default)]
    pub sid: Option<String>,
    /// 折淘客是否结算 (0未结算，1已结算)
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
    /// 店铺标题
    #[serde(default)]
    pub smstitle: Option<String>,
    /// 商品名称
    #[serde(default)]
    pub smstitle2: Option<String>,
    /// 商品 ID
    #[serde(default)]
    pub item_id: Option<String>,
    /// 商品主图
    #[serde(default)]
    pub item_pic: Option<String>,
    /// 退款金额
    #[serde(default)]
    pub refundprice: Option<f64>,
    /// 退款时间
    #[serde(default)]
    pub refundtime: Option<String>,
    /// 订单状态 (1已付款，8已完成，9已退款或风控)
    #[serde(default)]
    pub status: Option<String>,
    /// 数据最后更新时间
    #[serde(default)]
    pub update_time: Option<String>,
    /// 订单类型
    #[serde(default, rename = "type")]
    pub order_type: Option<String>,
    /// 折淘客授权 sid
    #[serde(default)]
    pub sid_ztk: Option<String>,
    /// 推广者自定义编号
    #[serde(default)]
    pub customer_id_ztk: Option<String>,
    /// 推广者平台类型
    #[serde(default)]
    pub platform_ztk: Option<String>,
    /// 活动 ID
    #[serde(default, rename = "actId")]
    pub act_id: Option<String>,
    /// 订单所属平台
    #[serde(default)]
    pub san_pingtai: Option<String>,
    /// 订单所属平台 ID
    #[serde(default)]
    pub san_pingtai_id: Option<String>,
}

/// 抖音活动列表响应
///
/// activity_list 接口的响应
#[derive(Debug, Clone, Deserialize)]
pub struct DouyinActivityListResponse {
    /// 状态码，10000 表示成功
    #[serde(default)]
    pub code: i32,
    /// 状态消息
    #[serde(default)]
    pub msg: Option<String>,
    /// 子错误码
    #[serde(default)]
    pub sub_code: Option<String>,
    /// 子错误消息
    #[serde(default)]
    pub sub_msg: Option<String>,
    /// 响应数据
    #[serde(default)]
    pub data: Option<DouyinActivityListData>,
}

/// 抖音活动列表数据
#[derive(Debug, Clone, Deserialize)]
pub struct DouyinActivityListData {
    /// 活动物料列表
    #[serde(default)]
    pub activity_material_list: Option<Vec<DouyinActivityMaterial>>,
    /// 总数量
    #[serde(default)]
    pub total: Option<String>,
}

/// 抖音活动物料
#[derive(Debug, Clone, Deserialize)]
pub struct DouyinActivityMaterial {
    /// 活动 ID
    #[serde(default)]
    pub activity_id: Option<String>,
    /// 活动名称
    #[serde(default)]
    pub activity_name: Option<String>,
    /// 活动主封面
    #[serde(default)]
    pub activity_main_cover: Option<String>,
    /// 活动预览封面
    #[serde(default)]
    pub activity_preview_cover: Option<String>,
    /// 活动推广素材
    #[serde(default)]
    pub activity_promotion_asset: Option<String>,
    /// 活动推广封面
    #[serde(default)]
    pub activity_promotion_cover: Option<String>,
    /// 活动规则
    #[serde(default)]
    pub activity_rule: Option<String>,
    /// 活动规则链接
    #[serde(default)]
    pub activity_rule_link: Option<String>,
    /// 活动状态
    #[serde(default)]
    pub activity_status: Option<String>,
    /// 活动类型
    #[serde(default)]
    pub activity_type: Option<String>,
    /// 活动有效期类型
    #[serde(default)]
    pub activity_validity_type: Option<String>,
    /// 物料 ID
    #[serde(default)]
    pub material_id: Option<String>,
    /// 开始时间
    #[serde(default)]
    pub start_time: Option<String>,
    /// 结束时间
    #[serde(default)]
    pub end_time: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_goods_convert_response_deserialize() {
        let json = r#"{
            "code": 10000,
            "msg": "success",
            "data": {
                "data": {
                    "dy_deeplink": "snssdk1128://xxx",
                    "dy_password": "口令内容",
                    "dy_zlink": "https://z.douyin.com/xxx"
                }
            }
        }"#;

        let response: DouyinGoodsConvertResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.code, 10000);
        assert!(response.data.is_some());
        let data = response.data.unwrap().data.unwrap();
        assert_eq!(data.dy_deeplink, Some("snssdk1128://xxx".to_string()));
        assert_eq!(data.dy_password, Some("口令内容".to_string()));
    }

    #[test]
    fn test_goods_detail_response_deserialize() {
        let json = r#"{
            "code": 10000,
            "msg": "success",
            "data": {
                "products": [
                    {
                        "base_info": {
                            "product_id": "123456",
                            "title": "测试商品",
                            "price": "1000"
                        },
                        "promotion_info": {
                            "cos_fee": "100",
                            "cos_ratio": "10"
                        }
                    }
                ]
            }
        }"#;

        let response: DouyinGoodsDetailResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.code, 10000);
        let products = response.data.unwrap().products.unwrap();
        assert_eq!(products.len(), 1);
        let base_info = products[0].base_info.as_ref().unwrap();
        assert_eq!(base_info.product_id, Some("123456".to_string()));
        assert_eq!(base_info.title, Some("测试商品".to_string()));
    }

    #[test]
    fn test_search_goods_response_deserialize() {
        let json = r#"{
            "code": 10000,
            "msg": "success",
            "data": {
                "products": [
                    {
                        "product_id": "123456",
                        "title": "测试商品",
                        "price": "1000",
                        "cos_fee": "100"
                    }
                ],
                "total": "100"
            }
        }"#;

        let response: DouyinSearchGoodsResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.code, 10000);
        let data = response.data.unwrap();
        assert_eq!(data.total, Some("100".to_string()));
        let products = data.products.unwrap();
        assert_eq!(products.len(), 1);
        assert_eq!(products[0].product_id, Some("123456".to_string()));
    }

    #[test]
    fn test_order_query_response_deserialize() {
        let json = r#"{
            "status": 200,
            "content": [
                {
                    "orderid": "123456789",
                    "paytime": "2024/01/01 12:00:00",
                    "payprice": "100.00",
                    "profit": 10.5,
                    "status": "8",
                    "san_pingtai": "抖音",
                    "san_pingtai_id": "9"
                }
            ]
        }"#;

        let response: DouyinOrderQueryResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.status, 200);
        let orders = response.content.unwrap();
        assert_eq!(orders.len(), 1);
        assert_eq!(orders[0].orderid, Some("123456789".to_string()));
        assert_eq!(orders[0].san_pingtai_id, Some("9".to_string()));
    }

    #[test]
    fn test_parse_command_response_deserialize() {
        let json = r#"{
            "code": 10000,
            "msg": "success",
            "data": {
                "command_info": {
                    "command_type": "1",
                    "product_info": {
                        "product_id": "123456",
                        "share_info": {
                            "deeplink": "snssdk1128://xxx",
                            "share_command": "口令内容"
                        }
                    }
                }
            }
        }"#;

        let response: DouyinParseCommandResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.code, 10000);
        let command_info = response.data.unwrap().command_info.unwrap();
        assert_eq!(command_info.command_type, Some("1".to_string()));
        let product_info = command_info.product_info.unwrap();
        assert_eq!(product_info.product_id, Some("123456".to_string()));
    }

    #[test]
    fn test_response_with_missing_fields() {
        let json = r#"{"code": 10000}"#;

        let response: DouyinGoodsConvertResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.code, 10000);
        assert!(response.data.is_none());
    }
}
