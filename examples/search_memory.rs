//! 多平台搜索内存条示例
//!
//! 在淘宝和京东平台搜索"内存条"商品
//!
//! 运行方式:
//! ```bash
//! cargo run --example search_memory --features full
//! ```
//!
//! 注意:
//! - 淘宝全网搜索需要 sid 和 pid 参数（需要授权）
//! - 京东搜索使用 hot_goods API

use ztk_rust_sdk::{JdHotGoodsRequest, SearchGoodsRequest, ZtkClient, ZtkResult};

/// 折淘客 AppKey
const APPKEY: &str = "";

/// 淘客授权 SID
const SID: &str = "60211";

/// 淘客 PID
/// 格式: mm_xxx_xxx_xxx
const PID: &str = "";

#[tokio::main]
async fn main() -> ZtkResult<()> {
    println!("=== 多平台搜索内存条示例 ===\n");

    // 创建客户端
    let client = ZtkClient::new(APPKEY).build()?;
    println!("✓ 客户端创建成功\n");

    // 1. 淘宝全网搜索
    println!("【淘宝平台】全网搜索内存条...");
    search_taobao(&client).await?;

    println!("\n{}\n", "=".repeat(60));

    // 2. 京东平台搜索
    println!("【京东平台】搜索内存条...");
    search_jd(&client).await?;

    println!("\n=== 搜索完成 ===");
    Ok(())
}

/// 淘宝全网搜索内存条
async fn search_taobao(client: &ZtkClient) -> ZtkResult<()> {
    let request = SearchGoodsRequest::new(SID, PID)
        .keyword("内存条")
        .page(1)
        .page_size(5)
        .sort("sale_num_desc") // 按销量排序
        .filter(2); // 中度过滤

    match client.taobao().search_goods(request).await {
        Ok(response) => {
            println!("状态码: {}", response.status);

            if let Some(goods_list) = response.content {
                if goods_list.is_empty() {
                    println!("未找到商品");
                } else {
                    println!("找到 {} 个商品:\n", goods_list.len());

                    for (i, goods) in goods_list.iter().enumerate() {
                        println!(
                            "{}. {}",
                            i + 1,
                            goods.title.as_deref().unwrap_or("未知商品")
                        );
                        println!("   原价: ¥{}", goods.size.as_deref().unwrap_or("--"));
                        println!(
                            "   券后价: ¥{}",
                            goods.quanhou_jiage.as_deref().unwrap_or("--")
                        );
                        println!("   佣金比例: {}%", goods.tkrate3.as_deref().unwrap_or("--"));
                        println!("   店铺: {}", goods.nick.as_deref().unwrap_or("--"));
                        println!("   销量: {}", goods.volume.as_deref().unwrap_or("--"));
                        println!();
                    }
                }
            } else {
                println!("未找到商品");
            }
        }
        Err(e) => {
            println!("搜索失败: {}", e);
            println!("\n提示: 请确保 SID 和 PID 参数正确");
            println!("- SID: 淘客账号授权 ID，在折淘客平台获取");
            println!("- PID: 淘客推广位 ID，格式为 mm_xxx_xxx_xxx");
        }
    }

    Ok(())
}

/// 京东平台搜索内存条
async fn search_jd(client: &ZtkClient) -> ZtkResult<()> {
    let request = JdHotGoodsRequest::new()
        .keyword("内存条")
        .page_index(1)
        .page_size(5)
        .goods_type(7); // 7-数码家电

    match client.jd().hot_goods(request).await {
        Ok(response) => {
            println!("状态码: {}", response.status);

            if let Some(goods_list) = response.content {
                if goods_list.is_empty() {
                    println!("未找到商品");
                } else {
                    println!("找到 {} 个商品:\n", goods_list.len());

                    for (i, goods) in goods_list.iter().enumerate() {
                        println!(
                            "{}. {}",
                            i + 1,
                            goods.short_name.as_deref().unwrap_or("未知商品")
                        );
                        if let Some(price) = goods.goods_price {
                            println!("   原价: ¥{:.2}", price);
                        }
                        if let Some(discount_price) = goods.discount_price_after {
                            println!("   券后价: ¥{:.2}", discount_price);
                        }
                        if let Some(commission) = goods.commissionshare {
                            println!("   佣金比例: {:.2}%", commission);
                        }
                        if let Some(url) = &goods.goods_link {
                            println!("   链接: {}", url);
                        }
                        println!();
                    }
                }
            } else {
                println!("未找到商品");
            }
        }
        Err(e) => {
            println!("搜索失败: {}", e);
        }
    }

    Ok(())
}
