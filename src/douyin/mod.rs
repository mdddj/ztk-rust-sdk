//! 抖音平台模块
//!
//! 提供抖音平台相关的 API 调用功能，包括：
//! - 商品转链
//! - 直播间转链
//! - 活动转链
//! - 商品详情和搜索
//! - 口令解析
//! - 订单查询

pub mod api;
pub mod enums;
pub mod request;
pub mod response;

pub use api::*;
pub use enums::*;
pub use request::*;
pub use response::*;
