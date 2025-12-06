//! 折淘客 (ZheTaoKe/ZTK) Rust SDK
//!
//! 为 Rust 开发者提供一个类型安全、易用且功能完整的折淘客 API 客户端库，
//! 支持淘宝、京东、拼多多、唯品会、美团、考拉、饿了么、抖音等多个电商平台的 API 调用。
//!
//! # 特性
//!
//! - **类型安全** - 利用 Rust 类型系统在编译时捕获错误
//! - **模块化** - 各平台独立，支持按需编译
//! - **易用性** - 链式调用，Builder 模式，清晰的 API
//! - **异步优先** - 基于 async/await 的异步 API
//!
//! # Cargo Features
//!
//! SDK 支持通过 Cargo features 按需编译各平台模块：
//!
//! - `taobao` - 淘宝平台模块 (默认启用)
//! - `jd` - 京东平台模块 (默认启用)
//! - `pdd` - 拼多多平台模块 (默认启用)
//! - `vip` - 唯品会平台模块
//! - `meituan` - 美团平台模块
//! - `kaola` - 考拉平台模块
//! - `eleme` - 饿了么平台模块
//! - `douyin` - 抖音平台模块
//! - `full` - 启用所有平台模块
//!
//! # 使用示例
//!
//! ```rust,ignore
//! use ztk_rust_sdk::{ZtkClient, ZtkResult};
//!
//! #[tokio::main]
//! async fn main() -> ZtkResult<()> {
//!     // 创建客户端
//!     let client = ZtkClient::new("your_appkey")
//!         .base_url("https://api.zhetaoke.com:10001")
//!         .build()?;
//!     
//!     // 调用淘宝 API (需要启用 taobao feature)
//!     #[cfg(feature = "taobao")]
//!     {
//!         // let result = client.taobao().convert_by_item_id(request).await?;
//!     }
//!     
//!     // 调用京东 API (需要启用 jd feature)
//!     #[cfg(feature = "jd")]
//!     {
//!         // let result = client.jd().convert(request).await?;
//!     }
//!     
//!     Ok(())
//! }
//! ```

// ============================================================================
// 公共模块 (始终可用)
// ============================================================================

/// 核心客户端模块
pub mod client;

/// 公共类型和工具模块
pub mod common;

/// 错误类型模块
pub mod error;

// ============================================================================
// 核心类型导出
// ============================================================================

/// 折淘客 SDK 客户端
pub use client::ZtkClient;

/// 客户端构建器
pub use client::ZtkClientBuilder;

/// SDK 错误类型
pub use error::ZtkError;

/// SDK Result 类型别名
pub use error::ZtkResult;

// ============================================================================
// 公共类型导出
// ============================================================================

/// 公共枚举和类型
pub use common::types::{SignUrlType, SortDirection};

// ============================================================================
// 平台模块 (条件编译)
// ============================================================================

/// 淘宝平台模块
///
/// 提供淘宝平台相关的 API 调用功能，包括：
/// - 高佣转链 (商品ID/淘口令)
/// - 批量转链
/// - 订单查询
/// - 淘口令生成和解析
///
/// 需要启用 `taobao` feature
#[cfg(feature = "taobao")]
pub mod taobao;

/// 京东平台模块
///
/// 提供京东平台相关的 API 调用功能，包括：
/// - 商品转链
/// - 京粉精选商品
/// - 订单查询
/// - 商品详情
/// - 朋友圈火爆商品
///
/// 需要启用 `jd` feature
#[cfg(feature = "jd")]
pub mod jd;

/// 拼多多平台模块
///
/// 提供拼多多平台相关的 API 调用功能，包括：
/// - 商品转链
/// - 商品详情 (简版/详版)
/// - 订单查询
/// - 授权备案
///
/// 需要启用 `pdd` feature
#[cfg(feature = "pdd")]
pub mod pdd;

/// 唯品会平台模块
///
/// 提供唯品会平台相关的 API 调用功能，包括：
/// - 商品转链
/// - 授权
/// - 订单查询
/// - 商品详情和搜索
///
/// 需要启用 `vip` feature
#[cfg(feature = "vip")]
pub mod vip;

/// 美团平台模块
///
/// 提供美团平台相关的 API 调用功能，包括：
/// - 商品转链
/// - 订单查询
///
/// 需要启用 `meituan` feature
#[cfg(feature = "meituan")]
pub mod meituan;

/// 考拉平台模块
///
/// 提供考拉平台相关的 API 调用功能，包括：
/// - 商品转链
/// - 精选商品列表
/// - 商品搜索
/// - 订单查询
///
/// 需要启用 `kaola` feature
#[cfg(feature = "kaola")]
pub mod kaola;

/// 饿了么平台模块
///
/// 提供饿了么平台相关的 API 调用功能，包括：
/// - 商品转链
/// - 订单查询
///
/// 需要启用 `eleme` feature
#[cfg(feature = "eleme")]
pub mod eleme;

/// 抖音平台模块
///
/// 提供抖音平台相关的 API 调用功能，包括：
/// - 商品转链
/// - 直播间转链
/// - 活动转链
/// - 商品详情和搜索
/// - 口令解析
/// - 订单查询
///
/// 需要启用 `douyin` feature
#[cfg(feature = "douyin")]
pub mod douyin;

// ============================================================================
// 平台类型便捷导出 (条件编译)
// ============================================================================

// 淘宝平台类型导出
#[cfg(feature = "taobao")]
pub use taobao::{
    BatchConvertRequest,
    BatchConvertResponse,
    // 请求类型
    ConvertByItemIdRequest,
    ConvertByTklRequest,
    ConvertByTklResponse,
    // 响应类型
    ConvertResponse,
    CreateTklRequest,
    CreateTklResponse,
    GoodsDetail,
    ParseItemIdRequest,
    ParseItemIdResponse,
    QueryOrdersRequest,
    QueryOrdersResponse,
    // 全网搜索
    SearchGoodsItem,
    SearchGoodsRequest,
    SearchGoodsResponse,
    TaobaoApi,
    TaobaoSignUrlType,
};

// 京东平台类型导出
#[cfg(feature = "jd")]
pub use jd::{
    JdApi,
    // 枚举类型
    JdChainType,
    // 请求类型
    JdConvertRequest,
    // 响应类型
    JdConvertResponse,
    JdEliteId,
    JdGoodsDetailRequest,
    JdGoodsDetailResponse,
    JdHotGoodsRequest,
    JdHotGoodsResponse,
    JdOrderQueryRequest,
    JdOrderQueryType,
    JdOrderResponse,
    JdSortField,
    JingfenGoodsRequest,
    JingfenGoodsResponse,
};

// 拼多多平台类型导出
#[cfg(feature = "pdd")]
pub use pdd::{
    PddApi,
    PddAuthorizeRequest,
    PddAuthorizeResponse,
    // 请求类型
    PddConvertRequest,
    // 响应类型
    PddConvertResponse,
    PddGoodsDetailFullRequest,
    PddGoodsDetailFullResponse,
    PddGoodsDetailSimpleRequest,
    PddGoodsDetailSimpleResponse,
    PddOrderQueryRequest,
    PddOrderResponse,
};

// 唯品会平台类型导出
#[cfg(feature = "vip")]
pub use vip::{
    VipApi,
    VipAuthorizeRequest,
    VipAuthorizeResponse,
    // 请求类型
    VipConvertRequest,
    // 响应类型
    VipConvertResponse,
    VipGoodsDetailRequest,
    VipGoodsDetailResponse,
    VipOrderQueryRequest,
    VipOrderResponse,
    VipSearchGoodsRequest,
    VipSearchGoodsResponse,
};

// 美团平台类型导出
#[cfg(feature = "meituan")]
pub use meituan::{
    MeituanApi,
    // 请求类型
    MeituanConvertRequest,
    // 响应类型
    MeituanConvertResponse,
    MeituanOrderQueryRequest,
    MeituanOrderResponse,
};

// 考拉平台类型导出
#[cfg(feature = "kaola")]
pub use kaola::{
    KaolaApi,
    // 请求类型
    KaolaConvertRequest,
    // 响应类型
    KaolaConvertResponse,
    KaolaGoodsListRequest,
    KaolaGoodsListResponse,
    KaolaOrderQueryRequest,
    KaolaOrderResponse,
    KaolaSearchGoodsRequest,
    KaolaSearchGoodsResponse,
};

// 饿了么平台类型导出
#[cfg(feature = "eleme")]
pub use eleme::{
    ElemeApi,
    // 请求类型
    ElemeConvertRequest,
    // 响应类型
    ElemeConvertResponse,
    ElemeOrderQueryRequest,
    ElemeOrderResponse,
};

// 抖音平台类型导出
#[cfg(feature = "douyin")]
pub use douyin::{
    DouyinActivityConvertRequest,
    DouyinActivityConvertResponse,
    DouyinActivityStatus,
    DouyinApi,
    DouyinAuthorSortBy,
    DouyinAuthorType,
    DouyinCommandType,
    // 请求类型
    DouyinGoodsConvertRequest,
    // 响应类型
    DouyinGoodsConvertResponse,
    DouyinGoodsDetailRequest,
    DouyinGoodsDetailResponse,
    DouyinLiveConvertRequest,
    DouyinLiveConvertResponse,
    DouyinLiveStatus,
    DouyinOrderQueryRequest,
    DouyinOrderQueryResponse,
    DouyinOrderQueryType,
    DouyinParseCommandRequest,
    DouyinParseCommandResponse,
    DouyinProductTag,
    DouyinSearchGoodsRequest,
    DouyinSearchGoodsResponse,
    // 枚举类型
    DouyinSearchType,
    DouyinShareStatus,
    DouyinSortType,
};
