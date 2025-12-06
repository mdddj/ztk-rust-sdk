# ZTK Rust SDK

折淘客 (ZheTaoKe/ZTK) Rust SDK - 多平台电商 API 客户端库

[![Crates.io](https://img.shields.io/crates/v/ztk-rust-sdk.svg)](https://crates.io/crates/ztk-rust-sdk)
[![Documentation](https://docs.rs/ztk-rust-sdk/badge.svg)](https://docs.rs/ztk-rust-sdk)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## 特性

- **类型安全** - 利用 Rust 类型系统在编译时捕获错误
- **模块化** - 各平台独立，支持按需编译
- **易用性** - 链式调用，Builder 模式，清晰的 API
- **异步优先** - 基于 async/await 的异步 API
- **完善文档** - 中文注释，示例代码

## 支持平台

| 平台 | Feature | 功能 |
|------|---------|------|
| 淘宝 | `taobao` | 高佣转链、批量转链、订单查询、淘口令生成/解析 |
| 京东 | `jd` | 商品转链、京粉精选、订单查询、商品详情、热销商品 |
| 拼多多 | `pdd` | 商品转链、商品详情、订单查询、授权备案 |
| 唯品会 | `vip` | 商品转链、授权、订单查询、商品搜索 |
| 美团 | `meituan` | 商品转链、订单查询 |
| 考拉 | `kaola` | 商品转链、精选商品、商品搜索、订单查询 |
| 饿了么 | `eleme` | 商品转链、订单查询 |
| 抖音 | `douyin` | 商品/直播/活动转链、商品搜索、口令解析、订单查询 |

## 安装

在 `Cargo.toml` 中添加依赖：

```toml
[dependencies]
ztk-rust-sdk = "0.1"
tokio = { version = "1.0", features = ["rt-multi-thread", "macros"] }
```

### 按需启用平台

默认启用 `taobao`、`jd`、`pdd` 三个平台。如需其他平台：

```toml
# 启用特定平台
[dependencies]
ztk-rust-sdk = { version = "0.1", features = ["taobao", "jd", "douyin"] }

# 启用所有平台
[dependencies]
ztk-rust-sdk = { version = "0.1", features = ["full"] }

# 只启用淘宝
[dependencies]
ztk-rust-sdk = { version = "0.1", default-features = false, features = ["taobao"] }
```

## 快速开始

```rust
use ztk_rust_sdk::{ZtkClient, ZtkResult};

#[tokio::main]
async fn main() -> ZtkResult<()> {
    // 创建客户端
    let client = ZtkClient::new("your_appkey")
        .build()?;
    
    println!("客户端创建成功！");
    Ok(())
}
```

## 使用示例

### 淘宝平台

#### 高佣转链 (商品ID)

```rust
use ztk_rust_sdk::{ZtkClient, ZtkResult, ConvertByItemIdRequest, TaobaoSignUrlType};

#[tokio::main]
async fn main() -> ZtkResult<()> {
    let client = ZtkClient::new("your_appkey").build()?;
    
    // 方式一：使用结构体
    let request = ConvertByItemIdRequest {
        sid: "your_sid".to_string(),
        pid: "mm_xxx_xxx_xxx".to_string(),
        num_iid: "123456789".to_string(),
        relation_id: Some("your_relation_id".to_string()),
        special_id: None,
        signurl: Some(TaobaoSignUrlType::WithFullDetail),
    };
    
    // 方式二：使用 Builder 模式
    let request = ConvertByItemIdRequest::new("your_sid", "mm_xxx_xxx_xxx", "123456789")
        .relation_id("your_relation_id")
        .signurl(TaobaoSignUrlType::WithFullDetail);
    
    let result = client.taobao().convert_by_item_id(request).await?;
    println!("转链结果: {:?}", result);
    
    Ok(())
}
```

#### 高佣转链 (淘口令)

```rust
use ztk_rust_sdk::{ZtkClient, ZtkResult, ConvertByTklRequest};

#[tokio::main]
async fn main() -> ZtkResult<()> {
    let client = ZtkClient::new("your_appkey").build()?;
    
    let request = ConvertByTklRequest::new(
        "your_sid",
        "mm_xxx_xxx_xxx",
        "【淘口令内容】"
    );
    
    let result = client.taobao().convert_by_tkl(request).await?;
    println!("转链结果: {:?}", result);
    
    Ok(())
}
```

#### 订单查询

```rust
use ztk_rust_sdk::{ZtkClient, ZtkResult, QueryOrdersRequest};

#[tokio::main]
async fn main() -> ZtkResult<()> {
    let client = ZtkClient::new("your_appkey").build()?;
    
    let request = QueryOrdersRequest::new("your_sid", "2024-01-01 00:00:00")
        .page(1)
        .page_size(50);
    
    let result = client.taobao().query_orders(request).await?;
    println!("订单列表: {:?}", result);
    
    Ok(())
}
```

### 京东平台

#### 商品转链

```rust
use ztk_rust_sdk::{ZtkClient, ZtkResult, JdConvertRequest, JdChainType};

#[tokio::main]
async fn main() -> ZtkResult<()> {
    let client = ZtkClient::new("your_appkey").build()?;
    
    let request = JdConvertRequest::new(
        "https://item.jd.com/123456.html",
        "your_union_id"
    )
    .position_id("12345")
    .chain_type(JdChainType::Short);
    
    let result = client.jd().convert(request).await?;
    println!("转链结果: {:?}", result);
    
    Ok(())
}
```

#### 京粉精选商品

```rust
use ztk_rust_sdk::{ZtkClient, ZtkResult, JingfenGoodsRequest, JdEliteId, JdSortField, SortDirection};

#[tokio::main]
async fn main() -> ZtkResult<()> {
    let client = ZtkClient::new("your_appkey").build()?;
    
    let request = JingfenGoodsRequest::new(JdEliteId::HotSale)
        .page_index(1)
        .page_size(20)
        .sort_name(JdSortField::CommissionShare)
        .sort(SortDirection::Desc);
    
    let result = client.jd().jingfen_goods(request).await?;
    println!("商品列表: {:?}", result);
    
    Ok(())
}
```

### 拼多多平台

#### 商品转链

```rust
use ztk_rust_sdk::{ZtkClient, ZtkResult, PddConvertRequest};

#[tokio::main]
async fn main() -> ZtkResult<()> {
    let client = ZtkClient::new("your_appkey").build()?;
    
    let request = PddConvertRequest::new(
        "pdd_app_key",
        "pdd_app_secret",
        "your_pid",
        "453581732819"  // 商品 ID
    )
    .custom_parameters(r#"{"uid":"user123"}"#);  // 用于返利场景
    
    let result = client.pdd().convert(request).await?;
    println!("转链结果: {:?}", result);
    
    Ok(())
}
```

### 抖音平台

#### 商品转链

```rust
use ztk_rust_sdk::{ZtkClient, ZtkResult, DouyinGoodsConvertRequest};

#[tokio::main]
async fn main() -> ZtkResult<()> {
    let client = ZtkClient::new("your_appkey").build()?;
    
    let request = DouyinGoodsConvertRequest::new("product_url_or_id");
    
    let result = client.douyin().convert_goods(request).await?;
    println!("转链结果: {:?}", result);
    
    Ok(())
}
```

#### 直播间转链

```rust
use ztk_rust_sdk::{ZtkClient, ZtkResult, DouyinLiveConvertRequest};

#[tokio::main]
async fn main() -> ZtkResult<()> {
    let client = ZtkClient::new("your_appkey").build()?;
    
    let request = DouyinLiveConvertRequest::new_with_buyin_id("buyin_id");
    
    let result = client.douyin().convert_live(request).await?;
    println!("直播间转链结果: {:?}", result);
    
    Ok(())
}
```

## 客户端配置

### 基础配置

```rust
use ztk_rust_sdk::ZtkClient;
use std::time::Duration;

let client = ZtkClient::new("your_appkey")
    .base_url("https://api.zhetaoke.com:10001")  // 主接口地址 (默认)
    .timeout(Duration::from_secs(60))             // 请求超时时间
    .build()?;
```

### 使用备用接口

```rust
let client = ZtkClient::new("your_appkey")
    .base_url("http://api.zhetaoke.cn:10000")  // 备用接口地址
    .build()?;
```

## 错误处理

SDK 定义了统一的错误类型 `ZtkError`：

```rust
use ztk_rust_sdk::{ZtkClient, ZtkError, ZtkResult};

async fn example() -> ZtkResult<()> {
    let client = ZtkClient::new("your_appkey").build()?;
    
    match client.taobao().convert_by_item_id(request).await {
        Ok(result) => {
            println!("成功: {:?}", result);
        }
        Err(ZtkError::Network(e)) => {
            eprintln!("网络错误: {}", e);
        }
        Err(ZtkError::Api { code, message, .. }) => {
            eprintln!("API 错误 [{}]: {}", code, message);
        }
        Err(ZtkError::Parse(e)) => {
            eprintln!("解析错误: {}", e);
        }
        Err(ZtkError::Validation(msg)) => {
            eprintln!("参数验证错误: {}", msg);
        }
        Err(e) => {
            eprintln!("其他错误: {}", e);
        }
    }
    
    Ok(())
}
```

## 枚举类型

SDK 提供了丰富的枚举类型，确保参数值的正确性：

### 通用枚举

- `SignUrlType` - 返回结果类型
- `SortDirection` - 排序方向 (Asc/Desc)

### 京东枚举

- `JdEliteId` - 京粉频道 ID (好券商品、精选卖场、9.9包邮、实时热销榜等)
- `JdChainType` - 转链类型 (长链/短链)
- `JdSortField` - 排序字段 (价格、佣金比例、销量等)
- `JdOrderQueryType` - 订单查询类型 (下单时间/完成时间/更新时间)

### 淘宝枚举

- `TaobaoSignUrlType` - 淘宝返回结果类型
- `TaobaoOrderQueryType` - 订单查询类型
- `TaobaoOrderStatus` - 订单状态

### 抖音枚举

- `DouyinSearchType` - 搜索类型
- `DouyinSortType` - 排序类型
- `DouyinOrderQueryType` - 订单查询类型

## API 参考

### ZtkClient 方法

| 方法 | Feature | 说明 |
|------|---------|------|
| `taobao()` | `taobao` | 获取淘宝平台 API |
| `jd()` | `jd` | 获取京东平台 API |
| `pdd()` | `pdd` | 获取拼多多平台 API |
| `vip()` | `vip` | 获取唯品会平台 API |
| `meituan()` | `meituan` | 获取美团平台 API |
| `kaola()` | `kaola` | 获取考拉平台 API |
| `eleme()` | `eleme` | 获取饿了么平台 API |
| `douyin()` | `douyin` | 获取抖音平台 API |

### TaobaoApi 方法

| 方法 | 说明 |
|------|------|
| `convert_by_item_id()` | 高佣转链 (商品ID) |
| `convert_by_tkl()` | 高佣转链 (淘口令) |
| `batch_convert()` | 批量高佣转链 |
| `query_orders()` | 订单查询 |
| `create_tkl()` | 创建淘口令 |
| `parse_item_id()` | 解析商品编号 |

### JdApi 方法

| 方法 | 说明 |
|------|------|
| `convert()` | 商品转链 |
| `jingfen_goods()` | 京粉精选商品 |
| `query_orders()` | 订单查询 |
| `goods_detail()` | 商品详情 |
| `hot_goods()` | 朋友圈火爆商品 |

### PddApi 方法

| 方法 | 说明 |
|------|------|
| `convert()` | 商品转链 |
| `goods_detail_simple()` | 商品详情 (简版) |
| `goods_detail_full()` | 商品详情 (详版) |
| `query_orders()` | 订单查询 |
| `authorize()` | 授权备案 |

### DouyinApi 方法

| 方法 | 说明 |
|------|------|
| `convert_goods()` | 商品转链 |
| `convert_live()` | 直播间转链 |
| `convert_activity()` | 活动转链 |
| `goods_detail()` | 商品详情 |
| `search_goods()` | 商品搜索 |
| `parse_command()` | 口令解析 |
| `query_orders()` | 订单查询 |

## 依赖项

- `reqwest` - HTTP 客户端
- `serde` / `serde_json` - JSON 序列化
- `thiserror` - 错误处理
- `tokio` - 异步运行时
- `url` - URL 处理

## License

MIT License

## 相关链接

- [折淘客官网](https://www.zhetaoke.com/)
- [折淘客 API 文档](https://www.zhetaoke.com/user/open/open_apilist.aspx)
