# Requirements Document

## Introduction

本文档定义了折淘客 (ZheTaoKe/ZTK) Rust SDK 的需求规范。该 SDK 旨在为 Rust 开发者提供一个类型安全、易用且功能完整的折淘客 API 客户端库，支持淘宝、京东、拼多多、唯品会、美团、考拉、饿了么、抖音等多个电商平台的 API 调用。

**核心设计原则：每个平台独立成模块，方便按需引入和维护。**

## Glossary

- **ZTK**: 折淘客 (ZheTaoKe) 的缩写，一个提供多平台电商 API 聚合服务的平台
- **ZtkClient**: SDK 的核心客户端结构体，用于配置和发起 API 请求
- **AppKey**: 折淘客平台分配的应用密钥，用于 API 认证
- **PID**: 淘客推广位 ID，格式为 mm_xxx_xxx_xxx
- **SID**: 淘客账号授权 ID
- **淘口令 (TKL)**: 淘宝口令，一种短文本形式的商品分享链接
- **高佣转链**: 将商品链接转换为带有高佣金的推广链接
- **TaobaoApi**: 淘宝平台 API 模块
- **JdApi**: 京东平台 API 模块
- **PddApi**: 拼多多平台 API 模块
- **VipApi**: 唯品会平台 API 模块
- **MeituanApi**: 美团平台 API 模块
- **KaolaApi**: 考拉平台 API 模块
- **ElemeApi**: 饿了么平台 API 模块
- **DouyinApi**: 抖音平台 API 模块

## Requirements

### Requirement 1: SDK 客户端初始化与模块化架构

**User Story:** As a Rust 开发者, I want to 创建一个可配置的 ZtkClient 实例并按平台访问独立模块, so that 我可以灵活地配置 API 并按需使用各平台功能。

#### Acceptance Criteria

1. WHEN 开发者创建 ZtkClient 实例时 THE ZtkClient SHALL 接受 base_url 和 appkey 作为必填配置参数
2. WHEN 开发者未指定 base_url 时 THE ZtkClient SHALL 使用默认的主接口地址 "https://api.zhetaoke.com:10001"
3. WHEN 开发者需要使用备用接口时 THE ZtkClient SHALL 支持配置备用地址 "http://api.zhetaoke.cn:10000"
4. WHEN ZtkClient 实例创建完成后 THE ZtkClient SHALL 提供 taobao()、jd()、pdd()、vip()、meituan()、kaola()、eleme()、douyin() 方法访问各平台独立模块
5. WHEN 开发者只需要某个平台功能时 THE SDK SHALL 支持通过 Cargo features 按需编译各平台模块

### Requirement 2: 淘宝平台模块 (TaobaoApi)

**User Story:** As a 淘客开发者, I want to 通过独立的淘宝模块调用 API, so that 我可以进行商品转链、订单查询等淘宝特有操作。

#### Acceptance Criteria

1. WHEN 开发者调用 client.taobao().convert_by_item_id() 时 THE TaobaoApi SHALL 封装高佣转链请求并返回类型化的转链结果
2. WHEN 开发者调用 client.taobao().convert_by_tkl() 时 THE TaobaoApi SHALL 自动对淘口令参数进行 URL 编码
3. WHEN 开发者调用 client.taobao().batch_convert() 时 THE TaobaoApi SHALL 支持同时处理多个渠道 ID 的转链请求
4. WHEN 开发者调用 client.taobao().query_orders() 时 THE TaobaoApi SHALL 返回包含订单详情的结构化数据
5. WHEN 开发者调用 client.taobao().create_tkl() 时 THE TaobaoApi SHALL 返回生成的淘口令字符串
6. WHEN 开发者调用 client.taobao().parse_item_id() 时 THE TaobaoApi SHALL 从淘口令或链接中提取商品 ID

### Requirement 3: 京东平台模块 (JdApi)

**User Story:** As a 京东联盟推广者, I want to 通过独立的京东模块调用 API, so that 我可以进行京东商品转链和订单管理。

#### Acceptance Criteria

1. WHEN 开发者调用 client.jd().convert() 时 THE JdApi SHALL 支持 unionId 和 positionId 参数配置
2. WHEN 开发者调用 client.jd().jingfen_goods() 时 THE JdApi SHALL 支持通过 eliteId 枚举筛选不同频道的商品
3. WHEN 开发者调用 client.jd().query_orders() 时 THE JdApi SHALL 支持按下单时间、完成时间或更新时间查询
4. WHEN 开发者调用 client.jd().goods_detail() 时 THE JdApi SHALL 返回包含详情图的完整商品信息
5. WHEN 开发者调用 client.jd().hot_goods() 时 THE JdApi SHALL 返回朋友圈火爆商品列表

### Requirement 4: 拼多多平台模块 (PddApi)

**User Story:** As a 拼多多推广者, I want to 通过独立的拼多多模块调用 API, so that 我可以进行商品推广和订单跟踪。

#### Acceptance Criteria

1. WHEN 开发者调用 client.pdd().convert() 时 THE PddApi SHALL 支持 custom_parameters 参数用于返利场景
2. WHEN 开发者调用 client.pdd().goods_detail_simple() 时 THE PddApi SHALL 返回简版商品详情
3. WHEN 开发者调用 client.pdd().goods_detail_full() 时 THE PddApi SHALL 返回详版商品详情
4. WHEN 开发者调用 client.pdd().query_orders() 时 THE PddApi SHALL 返回结构化的订单数据
5. WHEN 开发者调用 client.pdd().authorize() 时 THE PddApi SHALL 处理授权备案请求

### Requirement 5: 唯品会平台模块 (VipApi)

**User Story:** As a 唯品会推广者, I want to 通过独立的唯品会模块调用 API, so that 我可以进行唯品会商品推广。

#### Acceptance Criteria

1. WHEN 开发者调用 client.vip().convert() 时 THE VipApi SHALL 支持 chanTag 渠道标识参数
2. WHEN 开发者调用 client.vip().authorize() 时 THE VipApi SHALL 处理账号授权请求
3. WHEN 开发者调用 client.vip().query_orders() 时 THE VipApi SHALL 返回订单列表数据
4. WHEN 开发者调用 client.vip().goods_detail() 时 THE VipApi SHALL 返回商品详情信息
5. WHEN 开发者调用 client.vip().search_goods() 时 THE VipApi SHALL 支持关键词搜索商品

### Requirement 6: 美团平台模块 (MeituanApi)

**User Story:** As a 美团推广者, I want to 通过独立的美团模块调用 API, so that 我可以进行美团外卖等推广。

#### Acceptance Criteria

1. WHEN 开发者调用 client.meituan().convert() 时 THE MeituanApi SHALL 支持 customer_id 参数用于用户区分
2. WHEN 开发者调用 client.meituan().query_orders() 时 THE MeituanApi SHALL 返回美团订单数据

### Requirement 7: 考拉平台模块 (KaolaApi)

**User Story:** As a 考拉推广者, I want to 通过独立的考拉模块调用 API, so that 我可以进行考拉海购商品推广。

#### Acceptance Criteria

1. WHEN 开发者调用 client.kaola().convert() 时 THE KaolaApi SHALL 返回转链后的推广链接
2. WHEN 开发者调用 client.kaola().goods_list() 时 THE KaolaApi SHALL 返回精选商品列表
3. WHEN 开发者调用 client.kaola().search_goods() 时 THE KaolaApi SHALL 支持关键词搜索商品
4. WHEN 开发者调用 client.kaola().query_orders() 时 THE KaolaApi SHALL 返回订单数据

### Requirement 8: 饿了么平台模块 (ElemeApi)

**User Story:** As a 饿了么推广者, I want to 通过独立的饿了么模块调用 API, so that 我可以进行饿了么外卖推广。

#### Acceptance Criteria

1. WHEN 开发者调用 client.eleme().convert() 时 THE ElemeApi SHALL 处理饿了么特有的参数格式
2. WHEN 开发者调用 client.eleme().query_orders() 时 THE ElemeApi SHALL 返回饿了么订单数据

### Requirement 9: 抖音平台模块 (DouyinApi)

**User Story:** As a 抖音推广者, I want to 通过独立的抖音模块调用 API, so that 我可以进行抖音商品和直播推广。

#### Acceptance Criteria

1. WHEN 开发者调用 client.douyin().convert_goods() 时 THE DouyinApi SHALL 返回商品转链结果
2. WHEN 开发者调用 client.douyin().convert_live() 时 THE DouyinApi SHALL 返回直播间转链结果
3. WHEN 开发者调用 client.douyin().convert_activity() 时 THE DouyinApi SHALL 返回活动转链结果
4. WHEN 开发者调用 client.douyin().goods_detail() 时 THE DouyinApi SHALL 返回商品详情
5. WHEN 开发者调用 client.douyin().search_goods() 时 THE DouyinApi SHALL 支持商品搜索
6. WHEN 开发者调用 client.douyin().parse_command() 时 THE DouyinApi SHALL 解析抖音口令
7. WHEN 开发者调用 client.douyin().query_orders() 时 THE DouyinApi SHALL 返回订单数据

### Requirement 10: 请求参数模型封装

**User Story:** As a Rust 开发者, I want to 使用类型安全的请求参数结构体, so that 我可以在编译时发现参数错误。

#### Acceptance Criteria

1. WHEN 定义请求参数结构体时 THE SDK SHALL 为每个字段添加中文注释说明其用途
2. WHEN 参数只能取固定值时 THE SDK SHALL 使用枚举类型定义可选值
3. WHEN 参数为可选时 THE SDK SHALL 使用 Option<T> 类型包装
4. WHEN 参数需要特殊格式时 THE SDK SHALL 提供 Builder 模式或验证方法
5. WHEN 序列化请求参数时 THE SDK SHALL 使用 serde 进行 JSON 序列化
6. WHEN 定义各平台请求参数时 THE SDK SHALL 将参数结构体放在对应平台的子模块中

### Requirement 11: 响应结果模型封装

**User Story:** As a Rust 开发者, I want to 使用类型安全的响应结构体, so that 我可以方便地访问 API 返回的数据。

#### Acceptance Criteria

1. WHEN 定义响应结构体时 THE SDK SHALL 为每个字段添加中文注释说明其含义
2. WHEN API 返回成功时 THE SDK SHALL 将 JSON 响应反序列化为对应的结构体
3. WHEN API 返回错误时 THE SDK SHALL 将错误信息封装为 ZtkError 类型
4. WHEN 响应字段可能为空时 THE SDK SHALL 使用 Option<T> 类型处理
5. WHEN 反序列化响应数据时 THE SDK SHALL 使用 serde 进行 JSON 反序列化
6. WHEN 定义各平台响应结构时 THE SDK SHALL 将响应结构体放在对应平台的子模块中

### Requirement 12: 错误处理

**User Story:** As a Rust 开发者, I want to 获得清晰的错误信息, so that 我可以快速定位和处理问题。

#### Acceptance Criteria

1. WHEN 网络请求失败时 THE SDK SHALL 返回包含原始错误信息的 NetworkError
2. WHEN API 返回业务错误时 THE SDK SHALL 返回包含错误码和错误消息的 ApiError
3. WHEN JSON 解析失败时 THE SDK SHALL 返回包含解析错误详情的 ParseError
4. WHEN 参数验证失败时 THE SDK SHALL 返回包含验证失败原因的 ValidationError
5. IF 发生任何错误 THEN THE SDK SHALL 实现 std::error::Error trait 以支持错误链

### Requirement 13: 异步支持

**User Story:** As a Rust 开发者, I want to 使用异步方式调用 API, so that 我可以在异步运行时中高效地发起请求。

#### Acceptance Criteria

1. WHEN 开发者调用任何 API 方法时 THE SDK SHALL 返回 impl Future 类型
2. WHEN 发起 HTTP 请求时 THE SDK SHALL 使用 reqwest 库的异步客户端
3. WHEN 需要并发调用多个 API 时 THE SDK SHALL 支持同时发起多个请求

### Requirement 14: 枚举类型定义

**User Story:** As a Rust 开发者, I want to 使用枚举类型表示固定选项, so that 我可以避免传入无效的参数值。

#### Acceptance Criteria

1. WHEN 定义排序方式时 THE SDK SHALL 提供 SortType 枚举包含各种排序选项
2. WHEN 定义京粉频道时 THE SDK SHALL 提供 JdEliteId 枚举包含所有频道 ID
3. WHEN 定义店铺类型时 THE SDK SHALL 提供 ShopType 枚举区分淘宝店和天猫店
4. WHEN 定义转链类型时 THE SDK SHALL 提供 ChainType 枚举区分长链和短链
5. WHEN 定义订单查询类型时 THE SDK SHALL 提供 OrderQueryType 枚举区分查询方式
6. WHEN 枚举需要序列化时 THE SDK SHALL 实现 Serialize 和 Deserialize trait
7. WHEN 定义各平台特有枚举时 THE SDK SHALL 将枚举放在对应平台的子模块中
