//! 淘宝平台模块
//!
//! 提供淘宝平台相关的 API 调用功能，包括：
//! - 高佣转链
//! - 订单查询
//! - 淘口令生成和解析

pub mod api;
pub mod enums;
pub mod request;
pub mod response;

pub use api::TaobaoApi;
pub use enums::*;
pub use request::*;
pub use response::*;
