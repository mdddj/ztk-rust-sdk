//! 唯品会平台模块
//!
//! 提供唯品会平台相关的 API 调用功能，包括：
//! - 商品转链
//! - 授权
//! - 订单查询
//! - 商品详情和搜索

pub mod api;
pub mod request;
pub mod response;

pub use api::*;
pub use request::*;
pub use response::*;
