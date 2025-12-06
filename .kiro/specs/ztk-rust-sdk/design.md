# Design Document

## Overview

本设计文档描述折淘客 (ZTK) Rust SDK 的技术架构和实现方案。SDK 采用模块化设计，每个电商平台独立成模块，通过统一的客户端入口访问。

### 设计目标

1. **类型安全** - 利用 Rust 类型系统在编译时捕获错误
2. **模块化** - 各平台独立，支持按需编译
3. **易用性** - 链式调用，Builder 模式，清晰的 API
4. **异步优先** - 基于 async/await 的异步 API
5. **完善文档** - 中文注释，示例代码

## Architecture

```
ztk-sdk/
├── src/
│   ├── lib.rs              # 库入口，导出公共 API
│   ├── client.rs           # ZtkClient 核心客户端
│   ├── error.rs            # 错误类型定义
│   ├── common/             # 公共模块
│   │   ├── mod.rs
│   │   ├── types.rs        # 公共类型定义
│   │   └── http.rs         # HTTP 请求封装
│   ├── taobao/             # 淘宝平台模块
│   │   ├── mod.rs
│   │   ├── api.rs          # TaobaoApi 实现
│   │   ├── request.rs      # 请求参数结构体
│   │   ├── response.rs     # 响应结构体
│   │   └── enums.rs        # 淘宝特有枚举
│   ├── jd/                 # 京东平台模块
│   │   ├── mod.rs
│   │   ├── api.rs          # JdApi 实现
│   │   ├── request.rs
│   │   ├── response.rs
│   │   └── enums.rs
│   ├── pdd/                # 拼多多平台模块
│   │   ├── mod.rs
│   │   ├── api.rs
│   │   ├── request.rs
│   │   ├── response.rs
│   │   └── enums.rs
│   ├── vip/                # 唯品会平台模块
│   │   ├── mod.rs
│   │   ├── api.rs
│   │   ├── request.rs
│   │   └── response.rs
│   ├── meituan/            # 美团平台模块
│   │   ├── mod.rs
│   │   ├── api.rs
│   │   ├── request.rs
│   │   └── response.rs
│   ├── kaola/              # 考拉平台模块
│   │   ├── mod.rs
│   │   ├── api.rs
│   │   ├── request.rs
│   │   └── response.rs
│   ├── eleme/              # 饿了么平台模块
│   │   ├── mod.rs
│   │   ├── api.rs
│   │   ├── request.rs
│   │   └── response.rs
│   └── douyin/             # 抖音平台模块
│       ├── mod.rs
│       ├── api.rs
│       ├── request.rs
│       ├── response.rs
│       └── enums.rs
└── Cargo.toml
```

## Components and Interfaces

### 1. ZtkClient (核心客户端)

```rust
/// 折淘客 SDK 客户端
/// 
/// # Example
/// ```rust
/// use ztk_sdk::ZtkClient;
/// 
/// let client = ZtkClient::new("your_appkey")
///     .base_url("https://api.zhetaoke.com:10001")
///     .build();
/// 
/// // 调用淘宝 API
/// let result = client.taobao().convert_by_item_id(request).await?;
/// 
/// // 调用京东 API
/// let result = client.jd().convert(request).await?;
/// ```
pub struct ZtkClient {
    /// HTTP 客户端
    http_client: reqwest::Client,
    /// API 基础地址
    base_url: String,
    /// 折淘客 AppKey
    appkey: String,
}

impl ZtkClient {
    /// 创建新的客户端构建器
    pub fn new(appkey: impl Into<String>) -> ZtkClientBuilder;
    
    /// 获取淘宝平台 API
    pub fn taobao(&self) -> TaobaoApi;
    
    /// 获取京东平台 API
    pub fn jd(&self) -> JdApi;
    
    /// 获取拼多多平台 API
    pub fn pdd(&self) -> PddApi;
    
    /// 获取唯品会平台 API
    pub fn vip(&self) -> VipApi;
    
    /// 获取美团平台 API
    pub fn meituan(&self) -> MeituanApi;
    
    /// 获取考拉平台 API
    pub fn kaola(&self) -> KaolaApi;
    
    /// 获取饿了么平台 API
    pub fn eleme(&self) -> ElemeApi;
    
    /// 获取抖音平台 API
    pub fn douyin(&self) -> DouyinApi;
}
```

### 2. ZtkClientBuilder (构建器)

```rust
/// 客户端构建器
pub struct ZtkClientBuilder {
    appkey: String,
    base_url: Option<String>,
    timeout: Option<Duration>,
}

impl ZtkClientBuilder {
    /// 设置 API 基础地址
    pub fn base_url(mut self, url: impl Into<String>) -> Self;
    
    /// 设置请求超时时间
    pub fn timeout(mut self, timeout: Duration) -> Self;
    
    /// 构建客户端实例
    pub fn build(self) -> Result<ZtkClient, ZtkError>;
}
```

### 3. 平台 API 接口示例 (TaobaoApi)

```rust
/// 淘宝平台 API
pub struct TaobaoApi<'a> {
    client: &'a ZtkClient,
}

impl<'a> TaobaoApi<'a> {
    /// 高佣转链 (商品ID)
    /// 
    /// 将商品 ID 转换为带有高佣金的推广链接
    pub async fn convert_by_item_id(
        &self,
        request: ConvertByItemIdRequest,
    ) -> Result<ConvertResponse, ZtkError>;
    
    /// 高佣转链 (淘口令)
    /// 
    /// 将淘口令转换为自己的推广淘口令
    pub async fn convert_by_tkl(
        &self,
        request: ConvertByTklRequest,
    ) -> Result<ConvertByTklResponse, ZtkError>;
    
    /// 批量高佣转链
    /// 
    /// 同时获取多个渠道 ID 的转链结果
    pub async fn batch_convert(
        &self,
        request: BatchConvertRequest,
    ) -> Result<BatchConvertResponse, ZtkError>;
    
    /// 订单查询
    pub async fn query_orders(
        &self,
        request: QueryOrdersRequest,
    ) -> Result<QueryOrdersResponse, ZtkError>;
    
    /// 创建淘口令
    pub async fn create_tkl(
        &self,
        request: CreateTklRequest,
    ) -> Result<CreateTklResponse, ZtkError>;
    
    /// 解析商品编号
    pub async fn parse_item_id(
        &self,
        request: ParseItemIdRequest,
    ) -> Result<ParseItemIdResponse, ZtkError>;
}
```

### 4. 错误类型

```rust
/// SDK 错误类型
#[derive(Debug, thiserror::Error)]
pub enum ZtkError {
    /// 网络请求错误
    #[error("网络请求失败: {0}")]
    Network(#[from] reqwest::Error),
    
    /// API 业务错误
    #[error("API 错误 [{code}]: {message}")]
    Api {
        /// 错误码
        code: i32,
        /// 错误消息
        message: String,
        /// 子错误码
        sub_code: Option<String>,
        /// 子错误消息
        sub_msg: Option<String>,
    },
    
    /// JSON 解析错误
    #[error("JSON 解析失败: {0}")]
    Parse(#[from] serde_json::Error),
    
    /// 参数验证错误
    #[error("参数验证失败: {0}")]
    Validation(String),
    
    /// URL 编码错误
    #[error("URL 编码失败: {0}")]
    UrlEncode(String),
}
```

## Data Models

### 淘宝平台数据模型

#### 请求模型

```rust
/// 高佣转链请求 (商品ID)
#[derive(Debug, Clone, Serialize)]
pub struct ConvertByItemIdRequest {
    /// 淘客账号授权 ID
    pub sid: String,
    /// 淘客 PID，格式: mm_xxx_xxx_xxx
    pub pid: String,
    /// 商品 ID
    pub num_iid: String,
    /// 渠道关系 ID (可选，用于返利场景)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relation_id: Option<String>,
    /// 会员运营 ID (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub special_id: Option<String>,
    /// 返回结果类型
    /// - 0/1/2: 官方结果
    /// - 3: 整合高佣转链+解析商品编号
    /// - 4: 整合+简版详情
    /// - 5: 整合+全网详情+淘口令
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signurl: Option<SignUrlType>,
}

/// 高佣转链请求 (淘口令)
#[derive(Debug, Clone, Serialize)]
pub struct ConvertByTklRequest {
    /// 淘客账号授权 ID
    pub sid: String,
    /// 淘客 PID
    pub pid: String,
    /// 淘口令文案 (会自动 URL 编码)
    pub tkl: String,
    /// 淘宝渠道关系 ID (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relation_id: Option<String>,
    /// 京东联盟 ID (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub union_id: Option<String>,
    /// 京东渠道关系 ID (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_id: Option<String>,
    /// 拼多多自定义参数 (可选，JSON 格式)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_parameters: Option<String>,
    /// 唯品会渠道标识 (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chan_tag: Option<String>,
    /// 美团渠道关系 ID (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
}
```

#### 响应模型

```rust
/// 高佣转链响应 (淘口令)
#[derive(Debug, Clone, Deserialize)]
pub struct ConvertByTklResponse {
    /// 状态码，200 表示成功
    pub status: i32,
    /// 转换后的文案
    pub content: String,
    /// 商品 ID
    pub tao_id: Option<String>,
    /// 平台类型: 1-淘宝, 2-京东, 3-拼多多, 4-苏宁, 5-唯品会, 6-美团
    pub plat: Option<String>,
    /// 淘宝子平台: 1-淘宝, 2-天猫, 3-天猫超市
    pub plat2: Option<String>,
    /// 是否有优惠券
    pub is_youquan: Option<String>,
    /// 商品主图
    pub pic: Option<String>,
    /// 一级分类 ID
    pub cid1: Option<String>,
    /// 一级分类名称
    pub cid1_name: Option<String>,
    /// 二级分类 ID
    pub cid2: Option<String>,
    /// 二级分类名称
    pub cid2_name: Option<String>,
    /// 总销量
    pub volume: Option<String>,
    /// 评论数量
    #[serde(rename = "commentCount")]
    pub comment_count: Option<String>,
    /// 商品短标题
    pub title: Option<String>,
    /// 商品长标题
    pub tao_title: Option<String>,
    /// 原始价格
    pub size: Option<String>,
    /// 券后价格
    pub quanhou_jiage: Option<String>,
    /// 佣金比例
    pub tkrate3: Option<String>,
    /// 佣金金额
    pub tkfee3: Option<String>,
    /// 优惠券金额
    pub coupon_info_money: Option<String>,
    /// 店铺昵称
    pub nick: Option<String>,
    /// 生成的推广淘口令
    pub result_tkl: Option<String>,
    /// 生成的推广链接
    pub result_url: Option<String>,
    /// 文案是否纯文字: 0-否, 1-是
    pub chunwenzi: Option<String>,
}

/// 商品详情 (signurl=5 返回)
#[derive(Debug, Clone, Deserialize)]
pub struct GoodsDetail {
    /// 折淘客编号
    pub code: Option<String>,
    /// 分类 ID
    pub type_one_id: Option<String>,
    /// 商品 ID
    pub tao_id: String,
    /// 商品短标题
    pub title: String,
    /// 商品简介
    pub jianjie: Option<String>,
    /// 商品主图
    pub pict_url: Option<String>,
    /// 是否天猫: 0-淘宝, 1-天猫
    pub user_type: Option<String>,
    /// 卖家 ID
    pub seller_id: Option<String>,
    /// 商品描述分
    pub shop_dsr: Option<String>,
    /// 年销量
    pub volume: Option<String>,
    /// 折扣价
    pub size: Option<String>,
    /// 券后价
    pub quanhou_jiage: Option<String>,
    /// 佣金比率
    pub tkrate3: Option<String>,
    /// 佣金类型
    pub yongjin_type: Option<String>,
    /// 优惠券 ID
    pub coupon_id: Option<String>,
    /// 优惠券开始时间
    pub coupon_start_time: Option<String>,
    /// 优惠券结束时间
    pub coupon_end_time: Option<String>,
    /// 优惠券金额
    pub coupon_info_money: Option<String>,
    /// 优惠券总数量
    pub coupon_total_count: Option<String>,
    /// 优惠券剩余数量
    pub coupon_remain_count: Option<String>,
    /// 优惠券信息
    pub coupon_info: Option<String>,
    /// 是否聚划算
    pub juhuasuan: Option<String>,
    /// 是否淘抢购
    pub taoqianggou: Option<String>,
    /// 是否海淘
    pub haitao: Option<String>,
    /// 是否极有家
    pub jiyoujia: Option<String>,
    /// 是否金牌卖家
    pub jinpaimaijia: Option<String>,
    /// 是否精选品牌
    pub pinpai: Option<String>,
    /// 品牌名称
    pub pinpai_name: Option<String>,
    /// 是否有运费险
    pub yunfeixian: Option<String>,
    /// 卖家昵称
    pub nick: Option<String>,
    /// 商品小图列表 (用 | 分隔)
    pub small_images: Option<String>,
    /// 商品白底图
    pub white_image: Option<String>,
    /// 商品长标题
    pub tao_title: Option<String>,
    /// 宝贝所在地
    pub provcity: Option<String>,
    /// 店铺名称
    pub shop_title: Option<String>,
    /// 视频地址
    pub zhibo_url: Option<String>,
    /// 实时总销量
    #[serde(rename = "sellCount")]
    pub sell_count: Option<String>,
    /// 评论数量
    #[serde(rename = "commentCount")]
    pub comment_count: Option<String>,
    /// 收藏数量
    pub favcount: Option<String>,
    /// 宝贝描述分
    pub score1: Option<String>,
    /// 卖家服务分
    pub score2: Option<String>,
    /// 物流服务分
    pub score3: Option<String>,
    /// 店铺等级 (1-20)
    #[serde(rename = "creditLevel")]
    pub credit_level: Option<String>,
    /// 店铺 logo
    #[serde(rename = "shopIcon")]
    pub shop_icon: Option<String>,
    /// 商品 URL
    pub taobao_url: Option<String>,
    /// 叶子类目 ID
    pub category_id: Option<String>,
    /// 叶子类目名称
    pub category_name: Option<String>,
    /// 一级类目 ID
    pub level_one_category_id: Option<String>,
    /// 一级类目名称
    pub level_one_category_name: Option<String>,
    /// 返佣金额
    pub tkfee3: Option<String>,
    /// 二合一推广链接
    pub coupon_click_url: Option<String>,
    /// 推广长链接
    pub item_url: Option<String>,
    /// 淘宝短链接
    pub shorturl: Option<String>,
    /// 淘宝短链接2 (可拉起手淘)
    pub shorturl2: Option<String>,
    /// 淘口令
    pub tkl: Option<String>,
}
```

### 京东平台数据模型

```rust
/// 京东转链请求
#[derive(Debug, Clone, Serialize)]
pub struct JdConvertRequest {
    /// 推广物料 URL 或 SKU ID
    #[serde(rename = "materialId")]
    pub material_id: String,
    /// 京东联盟 ID
    #[serde(rename = "unionId")]
    pub union_id: String,
    /// 自定义推广位 ID (可选，用于返利)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "positionId")]
    pub position_id: Option<String>,
    /// 联盟子推客身份标识 (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pid: Option<String>,
    /// 优惠券领取链接 (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "couponUrl")]
    pub coupon_url: Option<String>,
    /// 子渠道标识 (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "subUnionId")]
    pub sub_union_id: Option<String>,
    /// 转链类型
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "chainType")]
    pub chain_type: Option<ChainType>,
    /// 礼金批次号 (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "giftCouponKey")]
    pub gift_coupon_key: Option<String>,
    /// 返回结果类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signurl: Option<SignUrlType>,
}

/// 京粉精选商品请求
#[derive(Debug, Clone, Serialize)]
pub struct JingfenGoodsRequest {
    /// 频道 ID
    #[serde(rename = "eliteId")]
    pub elite_id: JdEliteId,
    /// 页码
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "pageIndex")]
    pub page_index: Option<u32>,
    /// 每页数量 (最大 20)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "pageSize")]
    pub page_size: Option<u32>,
    /// 排序字段
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "sortName")]
    pub sort_name: Option<JdSortField>,
    /// 排序方向
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<SortDirection>,
}
```

### 公共枚举类型

```rust
/// 返回结果类型
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[repr(u8)]
pub enum SignUrlType {
    /// 官方结果
    Official = 0,
    /// 整合高佣转链+解析商品编号
    WithParse = 3,
    /// 整合+简版详情
    WithSimpleDetail = 4,
    /// 整合+全网详情+淘口令
    WithFullDetail = 5,
}

/// 转链类型 (京东)
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[repr(u8)]
pub enum ChainType {
    /// 长链
    Long = 1,
    /// 短链
    Short = 2,
    /// 长链+短链
    Both = 3,
}

/// 排序方向
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SortDirection {
    /// 升序
    Asc,
    /// 降序
    Desc,
}

/// 京粉频道 ID
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
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

/// 京东排序字段
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
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

/// 订单查询类型
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[repr(u8)]
pub enum OrderQueryType {
    /// 按下单时间
    OrderTime = 1,
    /// 按完成时间
    FinishTime = 2,
    /// 按更新时间
    UpdateTime = 3,
}
```

## Error Handling

SDK 使用 `thiserror` 库定义错误类型，所有错误实现 `std::error::Error` trait。

```rust
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ZtkError {
    #[error("网络请求失败: {0}")]
    Network(#[from] reqwest::Error),
    
    #[error("API 错误 [{code}]: {message}")]
    Api {
        code: i32,
        message: String,
        sub_code: Option<String>,
        sub_msg: Option<String>,
    },
    
    #[error("JSON 解析失败: {0}")]
    Parse(#[from] serde_json::Error),
    
    #[error("参数验证失败: {0}")]
    Validation(String),
}

/// Result 类型别名
pub type ZtkResult<T> = Result<T, ZtkError>;
```

## Testing Strategy

### 测试方法

1. **单元测试** - 测试各模块的独立功能
2. **集成测试** - 测试完整的 API 调用流程
3. **属性测试** - 使用 proptest 验证数据模型的序列化/反序列化

### 测试框架

- 单元测试: Rust 内置测试框架
- 属性测试: proptest
- Mock HTTP: wiremock



## Correctness Properties

*A property is a characteristic or behavior that should hold true across all valid executions of a system-essentially, a formal statement about what the system should do. Properties serve as the bridge between human-readable specifications and machine-verifiable correctness guarantees.*

基于需求分析，以下是需要通过属性测试验证的正确性属性：

### Property 1: 客户端配置保持一致性

*For any* appkey 字符串和 base_url 字符串，创建 ZtkClient 后，通过 getter 方法获取的配置值应与传入值相等。

**Validates: Requirements 1.1, 1.3**

### Property 2: URL 编码 Round-Trip

*For any* 包含特殊字符的淘口令字符串，URL 编码后再解码应得到原始字符串。

**Validates: Requirements 2.2**

### Property 3: 请求参数序列化 Round-Trip

*For any* 有效的请求参数结构体实例，序列化为 JSON 后再反序列化应得到等价的结构体。

**Validates: Requirements 10.5**

### Property 4: 响应数据反序列化 Round-Trip

*For any* 有效的响应结构体实例，序列化为 JSON 后再反序列化应得到等价的结构体。

**Validates: Requirements 11.2, 11.5**

### Property 5: 枚举序列化 Round-Trip

*For any* 枚举值（SignUrlType, ChainType, JdEliteId 等），序列化为 JSON 后再反序列化应得到相同的枚举值。

**Validates: Requirements 14.1-14.7**

### Property 6: 错误类型完整性

*For any* ZtkError 实例，调用 std::error::Error trait 方法应返回有意义的错误信息，且错误链应正确传播。

**Validates: Requirements 12.1-12.5**

### Property 7: Builder 模式链式调用

*For any* 有效的配置参数序列，通过 Builder 模式链式调用设置后，构建的客户端应包含所有设置的配置。

**Validates: Requirements 10.4**

### Property 8: 可选参数序列化

*For any* 包含 None 值的可选字段的请求结构体，序列化后的 JSON 不应包含该字段。

**Validates: Requirements 10.3**

## Cargo Features 设计

```toml
[features]
default = ["taobao", "jd", "pdd"]

# 各平台独立 feature
taobao = []
jd = []
pdd = []
vip = []
meituan = []
kaola = []
eleme = []
douyin = []

# 全部平台
full = ["taobao", "jd", "pdd", "vip", "meituan", "kaola", "eleme", "douyin"]
```

## 依赖项

```toml
[dependencies]
reqwest = { version = "0.11", features = ["json"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
thiserror = "1.0"
tokio = { version = "1.0", features = ["rt-multi-thread", "macros"] }
url = "2.0"

[dev-dependencies]
proptest = "1.0"
wiremock = "0.5"
tokio-test = "0.4"
```

## 使用示例

```rust
use ztk_sdk::{ZtkClient, ZtkResult};
use ztk_sdk::taobao::{ConvertByItemIdRequest, SignUrlType};
use ztk_sdk::jd::{JdConvertRequest, ChainType};

#[tokio::main]
async fn main() -> ZtkResult<()> {
    // 创建客户端
    let client = ZtkClient::new("your_appkey")
        .base_url("https://api.zhetaoke.com:10001")
        .build()?;
    
    // 淘宝高佣转链
    let taobao_request = ConvertByItemIdRequest {
        sid: "your_sid".to_string(),
        pid: "mm_xxx_xxx_xxx".to_string(),
        num_iid: "123456789".to_string(),
        relation_id: Some("your_relation_id".to_string()),
        special_id: None,
        signurl: Some(SignUrlType::WithFullDetail),
    };
    
    let result = client.taobao().convert_by_item_id(taobao_request).await?;
    println!("淘宝转链结果: {:?}", result);
    
    // 京东转链
    let jd_request = JdConvertRequest {
        material_id: "https://item.jd.com/123456.html".to_string(),
        union_id: "your_union_id".to_string(),
        position_id: Some("12345".to_string()),
        chain_type: Some(ChainType::Short),
        ..Default::default()
    };
    
    let result = client.jd().convert(jd_request).await?;
    println!("京东转链结果: {:?}", result);
    
    Ok(())
}
```
