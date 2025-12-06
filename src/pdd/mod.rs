//! 拼多多平台模块
//!
//! 提供拼多多平台相关的 API 调用功能，包括：
//! - 商品转链
//! - 商品详情 (简版/详版)
//! - 订单查询
//! - 授权备案

pub mod api;
pub mod request;
pub mod response;

pub use api::*;
pub use request::*;
pub use response::*;
