# 折淘客 Tab 接口文档

> 来源: https://www.zhetaoke.com/one/api3.aspx
> 导出时间: 2025/12/6 14:50:29

[TOC]

## 京东转链API-新

**接口名称：**京东转链API-新（自动匹配官方优惠券）

**接口地址：**https://api.zhetaoke.com:10001/api/open\_jing\_union\_open\_promotion\_byunionid\_get.ashx

**返回数据：**json

**请求方式：**get/post,请尽量使用post。

**请求示例：**https://api.zhetaoke.com:10001/api/open\_jing\_union\_open\_promotion\_byunionid\_get.ashx?appkey=&materialId=&unionId=&positionId=&chainType=2

**请求参数说明：**

|  | 名称 | 类型 | 必填 | 说明 |
| --- | --- | --- | --- | --- |
|   | appkey | String | 是 | 折淘客对接秘钥appkey，折淘客通用。 |
|   | materialId | string | 是 | 推广物料url，例如活动链接、商品链接等；支持仅传入skuid  
重要的事情说三遍：该参数需要进行Urlencode编码！该参数需要进行Urlencode编码！该参数需要进行Urlencode编码！  
 |
|   | unionId | string | 是 | 京东联盟ID，为一串数字。 |
|   | positionId | string | 否 | 自定义推广位id  
重要的事情说三遍：该参数可以自定义数字！该参数可以自定义数字！该参数可以自定义数字！  
自定义的数字，自己在本地跟用户做好关联，订单中会透出自定义的数字。  
如果返利需要用到此字段，如果是导购，不需要此字段。  
 |
|   | pid | string | 否 | 联盟子推客身份标识（不能传入接口调用者自己的pid） |
|   | couponUrl | string | 否 | 优惠券领取链接，在使用优惠券、商品二合一功能时入参，且materialId须为商品详情页链接。  
该参数值不为空时，优先使用传入的优惠券，否则自动匹配官方优惠券。  
重要的事情说三遍：该参数需要进行Urlencode编码！该参数需要进行Urlencode编码！该参数需要进行Urlencode编码！  
 |
|   | subUnionId | string | 否 | 子渠道标识，您可自定义传入字母、数字或下划线，最多支持80个字符，该参数会在订单行查询接口中展示。 |
|   | chainType | string | 否 | 转链类型，1：长链， 2 ：短链 ，3： 长链+短链，默认短链，短链有效期60天。 |
|   | giftCouponKey | string | 否 | 礼金批次号。 |
|   | signurl | string | 否 | 默认0，signurl=0，返回官方转链结果，signurl=5，返回结果整合京东商品详情API和京东转链API，调用一次接口，获取商品详情、商品优惠券信息和商品转链结果。 |

**相关说明：**

|  | 名称 | 类型 | 说明 |
| --- | --- | --- | --- |
|   | 使用说明 |  | 支持导购，支持返利  
1、如果需要返利，positionId自定义数字，订单中会透出自定义的数字。  
1、如果是导购，positionId为空即可。  
 |

**signurl=0，返回：**

```json
{
                            "jd\_union\_open\_promotion\_byunionid\_get\_response": {
                            "result": "{\\"code\\":200,\\"data\\":{\\"shortURL\\":\\"https://u.jd.com/JtIyKXd\\"},\\"message\\":\\"success\\",\\"requestId\\":\\"o\_0b11e4d1\_l3gpo6z1\_26591923\\"}",
                            "code": "0"
                            }
                            }
                           /\*shortURL为自己的推广短链接\*/
                           /\*clickURL为自己的推广长链接\*/
```

**signurl=5，返回：**

```json
{
	                            "status": 200,                      /\*状态\*/
	                            "content": \[{
		                            "code": "6646",                 /\*折淘客编号\*/
		                            "type\_one\_id": "1",             /\*分类ID，可参考折淘客分类\*/
		                            "tao\_id": "554832820990",       /\*商品ID\*/
		                            "title": "储物箱整理抽屉式收纳柜",                   /\*商品短标题\*/
		                            "jianjie": "【爆款返场，出口日本】进口PP材质，可叠加透明可视抽屉收纳盒，分格抽屉物品分类齐全，区域分化一目了然，自由组合，大容量，让你居家生活更舒心，收纳你的生活！【赠运费险】", /\*商品简介\*/
		                            "pict\_url": "https://img14.360buyimg.com/bao/......jpg",                   /\*商品主图\*/
		                            "user\_type": "0",                   /\*是否京东自营，0是非京东自营，1是京东自营\*/
		                            "seller\_id": "953997",                   /\*卖家ID\*/ /\*店铺地址可自行拼接，https://mall.jd.com/index-953997.html \*/
		                            "shop\_dsr": "4.80",                   /\*商品描述分\*/
		                            "volume": "169",                   /\*年销量\*/
		                            "size": "26.90",                   /\*折扣价\*/
		                            "quanhou\_jiage": "16.90",                   /\*券后价\*/
		                            "date\_time\_yongjin": "2018/11/8 14:45:22",                   /\*数据更新时间\*/
		                            "tkrate3": "30.00",                   /\*佣金比率\*/
		                            "yongjin\_type": "MKT",                   /\*佣金类型\*/
		                            "coupon\_id": "fe31d6c8132c462ba59bd72e03d2eae0",                   /\*优惠券ID\*/
		                            "coupon\_start\_time": "2018-11-05",                   /\*优惠券开始时间\*/
		                            "coupon\_end\_time": "2018-11-10",                   /\*优惠券结束时间\*/
		                            "coupon\_info\_money": "10",                   /\*优惠券金额\*/
		                            "coupon\_total\_count": "100000",                   /\*优惠券总数量，目前官方为0，无用\*/
		                            "coupon\_remain\_count": "99965",                   /\*优惠券剩余数量，目前官方为0，无用\*/
		                            "coupon\_info": "满80.00元减10元",                   /\*优惠券信息\*/
		                            "juhuasuan": "0",                       /\*是否京东配送，1是\*/
		                            "taoqianggou": "0",                     /\*是否京东拼购，1是\*/
		                            "haitao": "0",                          /\*是否海淘（京东国际），1是\*/
		                            "jiyoujia": "0",                        /\*是否京喜商品，1是\*/
		                            "jinpaimaijia": "0",                    /\*是否京东好店，1是\*/
		                            "pinpai": "0",                          /\*是否精选品牌，1是\*/
		                            "pinpai\_name": "",                      /\*品牌名称\*/
		                            "yunfeixian": "0",                      /\*是否有运费险，1有\*/
		                            "nick": "\*\*\*旗舰店",                    /\*卖家昵称\*/
		                            "small\_images": "https://img14.360buyimg.com/bao/.......jpg|https://img14.360buyimg.com/bao/.......jpg",                   /\*商品小图列表\*/
		                            "white\_image": "https://img14.360buyimg.com/bao/......jpg",                   /\*商品白底图\*/
		                            "tao\_title": "抽屉式收纳箱塑料储物箱整理箱衣服物家用宿舍衣柜内衣内裤收纳盒",                   /\*商品长标题\*/
		                            "provcity": "浙江 杭州",                   /\*宝贝所在地\*/
		                            "shop\_title": "\*\*\*旗舰店",                   /\*店铺名称\*/
		                            "zhibo\_url": "https://cloud.video.taobao.com/play/u/849090736/p/2/e/6/t/1/219853683676.mp4",                   /\*视频地址\*/
		                            "sellCount": "860000",                   /\*网页实时总销量-废弃\*/
		                            "commentCount": "250000",                   /\*评论数量\*/
		                            "favcount": "180000",                   /\*收藏数量\*/
		                            "score1": "4.9",                   /\*宝贝描述分\*/
		                            "score2": "4.9",                   /\*卖家服务分\*/
		                            "score3": "4.9",                   /\*物流服务分\*/
		                            "creditLevel": "15",                   /\*店铺评分\*/
		                            "shopIcon": "//img14.360buyimg.com/imgextra/de/a6/TB10dC8SX.jpg",       /\*店铺logo\*/
		                            "pcDescContent": "1.jpg|2.jpg|3.jpg|4.jpg|5.jpg",  /\*图文详情图片地址\*/
		                            "item\_url": "https://item.jd.com/10025768652616.html",  /\*商品url\*/
		                            "category\_id": "50006126",                   /\*叶子类目id\*/
		                            "category\_name": "收纳盒",                   /\*叶子类目name\*/
		                            "level\_one\_category\_id": "122928002",                   /\*一级类目id\*/
		                            "level\_one\_category\_name": "收纳整理",                   /\*一级类目name\*/
		                            "tkfee3": "5.07",                   /\*返佣金额\*/
		                            "biaoqian": ""满300元,省30元"",                   /\*店铺活动\*/
		                            "tag": "赶紧拯救你厨房的油腻吧😂植护🌿加厚厨房抽纸高密度纤维纸张，吸油💧很不错5包装|￥14.8元💰✔厨房小帮手，用处多多",                   /\*朋友圈文案，需要自己进行urldecode\*/
		                            "date\_time": "2018/11/8 14:45:22",                   /\*数据添加时间\*/
		                            "coupon\_click\_url": "https://union-click.jd.com/jdc?e=1002294009&p=",                   /\*推广长链接\*/
		                            "shorturl": "https://u.jd.com/JdIOWzt"                   /\*推广短链接\*/
	                            }\]
                            }

                            /\*shorturl为自己的推广短链接\*/
                            /\*coupon\_click\_url为自己的推广长链接\*/
```

---

## 京粉精选商品API

**接口名称：**京粉精选商品API

**接口地址：**https://api.zhetaoke.com:10001/api/open\_jing\_union\_open\_goods\_jingfen\_query.ashx

**返回数据：**json

**请求方式：**get

**请求示例：**https://api.zhetaoke.com:10001/api/open\_jing\_union\_open\_goods\_jingfen\_query.ashx?appkey=&eliteId=

**请求参数说明：**

|  | 名称 | 类型 | 必填 | 说明 |
| --- | --- | --- | --- | --- |
|   | appkey | String | 是 | 折淘客对接秘钥appkey，折淘客通用。 |
|   | eliteId | String | 是 | 频道ID:1-好券商品,2-精选卖场,10-9.9包邮,15-京东配送,22-实时热销榜,23-为你推荐,24-数码家电,25-超市,26-母婴玩具,27-家具日用,28-美妆穿搭,30-图书文具,31-今日必推,32-京东好物,33-京东秒杀,34-拼购商品,40-高收益榜,41-自营热卖榜,108-秒杀进行中,109-新品首发,110-自营,112-京东爆品,125-首购商品,129-高佣榜单,130-视频商品,153-历史最低价商品榜，210-极速版商品，238-新人价商品，247-京喜9.9，249-京喜秒杀 |
|   | pageIndex | String | 否 | 页码。 |
|   | pageSize | String | 否 | 每页数量，单页数最大20，默认20 |
|   | sortName | String | 否 | 排序字段(price：单价, commissionShare：佣金比例, commission：佣金， inOrderCount30DaysSku：sku维度30天引单量，comments：评论数，goodComments：好评数) |
|   | sort | String | 否 | asc,desc升降序,默认降序。 |
|   | pid | String | 否 | 联盟id\_应用id\_推广位id，三段式 |
|   | fields | String | 否 | 支持出参数据筛选，逗号','分隔，目前可用：videoInfo(视频信息),hotWords(热词),similar(相似推荐商品),documentInfo(段子信息)，skuLabelInfo（商品标签），promotionLabelInfo（商品促销标签） |
|   | forbidTypes | String | 否 | 10微信京东购物小程序禁售，11微信京喜小程序禁售 |
|   |  |  |  | [点击这里查看官方接口说明！点击这里查看官方接口说明！点击这里查看官方接口说明！](https://union.jd.com/openplatform/api/10417)  
[点击这里查看官方接口说明！点击这里查看官方接口说明！点击这里查看官方接口说明！](https://union.jd.com/openplatform/api/10417)  
[点击这里查看官方接口说明！点击这里查看官方接口说明！点击这里查看官方接口说明！](https://union.jd.com/openplatform/api/10417)  
 |

**相关说明：**

|  | 名称 | 类型 | 说明 |
| --- | --- | --- | --- |
|   |

**JSON返回示例：[点击这里查看官方接口说明](https://union.jd.com/openplatform/api/10420)**

```json
{
    "jd\_union\_open\_goods\_jingfen\_query\_response":{
        "queryResult":{
            "code":"200",
            "data":{
                "jfGoodsResp":{
                    "bookInfo":{
                        "isbn":"9787515515564"
                    },
                    "materialUrl":"item.jd.com/26898778009.html",
                    "documentInfo":{
                        "document":"温和亲肤的配方 洁净面部污垢 为肌肤添加补水保湿养分 泡沫绵密丰富 温润不易紧绷 弱酸性氨基酸系 含有自然成分 容易冲洗",
                        "discount":"29.9碧素堂氨基酸洗面奶"
                    },
                    "imageInfo":{
                        "whiteImage":"https://img14.360buyimg.com/pop/jfs/t1/74611/40/9199/226994/5d6f1c60E211d7a9e/e69c31469897a95a.png",
                        "imageList":{
                            "urlInfo":{
                                "url":"http://img14.360buyimg.com/ads/jfs/t22495/56/628456568/380476/9befc935/5b39fb01N7d1af390.jpg"
                            }
                        }
                    },
                    "pinGouInfo":{
                        "pingouEndTime":"2133999048000",
                        "pingouPrice":"39.9",
                        "pingouTmCount":"2",
                        "pingouUrl":"https://wq.jd.com/pingou\_api/GetAutoTuan?sku\_id=35097232463 from=cps",
                        "pingouStartTime":"1569224455000"
                    },
                    "forbidTypes":"\[0,10,11\]",
                    "resourceInfo":{
                        "eliteId":"11",
                        "eliteName":"品牌好货-潮流范儿"
                    },
                    "skuLabelInfo":{
                        "is7ToReturn":"1",
                        "fxg":"1",
                        "fxgServiceList":{
                            "characteristicServiceInfo":{
                                "serviceName":"破损包退换"
                            }
                        }
                    },
                    "skuName":"便携式女士香水持久淡香小样 初见系列香水 遇见时光",
                    "priceInfo":{
                        "lowestPrice":"14.9",
                        "lowestCouponPrice":"10.9",
                        "price":"39.9",
                        "historyPriceDay":"180",
                        "lowestPriceType":"2"
                    },
                    "spuid":"3491692",
                    "commissionInfo":{
                        "isLock":"1",
                        "commissionShare":"50",
                        "plusCommissionShare":"50",
                        "commission":"22.68",
                        "startTime":"1601364491000",
                        "couponCommission":"12.68",
                        "endTime":"1601364491062"
                    },
                    "skuId":"26898778009",
                    "brandCode":"7998",
                    "owner":"g",
                    "shopInfo":{
                        "logisticsLvyueScore":"9.69",
                        "shopLevel":"3.5",
                        "userEvaluateScore":"9.46",
                        "scoreRankRate":"94.36",
                        "afterServiceScore":"8.98",
                        "shopName":"XXXX旗舰店",
                        "shopLabel":"1",
                        "afsFactorScoreRankGrade":"中",
                        "shopId":"45619",
                        "logisticsFactorScoreRankGrade":"高",
                        "commentFactorScoreRankGrade":"高"
                    },
                    "brandName":"悍途（Humtto）",
                    "comments":"999",
                    "seckillInfo":{
                        "seckillOriPrice":"36.9",
                        "seckillPrice":"26.8",
                        "seckillStartTime":"1574474399000",
                        "seckillEndTime":"1574388000000"
                    },
                    "couponInfo":{
                        "couponList":{
                            "coupon":{
                                "useEndTime":"1532921782000",
                                "getEndTime":"1532921782000",
                                "useStartTime":"1532921782000",
                                "quota":"39",
                                "bindType":"3",
                                "link":"http://coupon.jd.com/ilink/couponActiveFront/front\_index.action?XXXXXXX",
                                "platformType":"0",
                                "discount":"30",
                                "getStartTime":"1532921782000",
                                "hotValue":"5",
                                "isBest":"1"
                            }
                        }
                    },
                    "videoInfo":{
                        "videoList":{
                            "video":{
                                "duration":"10",
                                "high":"300",
                                "playType":"high",
                                "videoType":"1",
                                "imageUrl":"https://img.300hu.com/4c1f7a6atransbjngwcloud1oss/44128edd173016898433773569/imageSampleSnapshot/1555986468\_406717890.100\_2756.jpg",
                                "width":"400",
                                "playUrl":"https://vod.https://vod.300hu.com/4c1f7a6atransbjngwcloud1oss/44128edd173016898433773569/v.f20.mp4?dockingId=2bc88c56-a44d-45c4-99b4-d9b68557e4e9storageSource=3.com/4c1f7a6atransbjngwcloud1oss/44128edd173016898433773569/v.f20.mp4?dockingId=2bc88c56-a44d-45c4-99b4-d9b68557e4e9storageSource=3"
                            }
                        }
                    },
                    "secondPriceInfoList":{
                        "secondPriceInfo":{
                            "secondPriceType":"2",
                            "secondPrice":"8.8"
                        }
                    },
                    "deliveryType":"1",
                    "goodCommentsShare":"99",
                    "categoryInfo":{
                        "cid1Name":"珠宝首饰",
                        "cid2Name":"木手串/把件",
                        "cid2":"12041",
                        "cid3Name":"其他",
                        "cid3":"12052",
                        "cid1":"6144"
                    },
                    "inOrderCount30DaysSku":"100",
                    "inOrderCount30Days":"6018",
                    "promotionLabelInfoList":{
                        "promotionLabelInfo":{
                            "promotionLabel":"满2件，总价打8折",
                            "lableName":"满折",
                            "promotionLableId":"5000125161",
                            "startTime":"1608998400000",
                            "endTime":"1608998400000"
                        }
                    },
                    "isHot":"1",
                    "jxFlags":"\[1,2,3\]"
                }
            },
            "message":"success",
            "totalCount":"100"
        }
    }
}
```

---

## 京东朋友圈火爆商品API

**接口名称：**京东朋友圈火爆商品API

**接口地址：**https://api.zhetaoke.com:10001/api/open\_jing\_goods\_list.ashx

**返回数据：**json

**请求方式：**get

**请求示例：**https://api.zhetaoke.com:10001/api/open\_jing\_goods\_list.ashx?appkey=&pageIndex=1&pageSize=20&sort=new

重要说明：本接口含精编社群推广文案、朋友圈推广文案，适合做社群和朋友圈推广！  
重要说明：本接口含精编社群推广文案、朋友圈推广文案，适合做社群和朋友圈推广！  
重要说明：本接口含精编社群推广文案、朋友圈推广文案，适合做社群和朋友圈推广！  
  
转链方法：1、获取字段goods\_link（商品链接）和discount\_link（优惠券链接），2、使用接口“京东转链API-新”进行转链。  
转链方法：1、获取字段goods\_link（商品链接）和discount\_link（优惠券链接），2、使用接口“京东转链API-新”进行转链。  
转链方法：1、获取字段goods\_link（商品链接）和discount\_link（优惠券链接），2、使用接口“京东转链API-新”进行转链。  

**请求参数说明：**

|  | 名称 | 类型 | 必填 | 说明 |
| --- | --- | --- | --- | --- |
|   | appkey | String | 是 | 折淘客对接秘钥appkey，折淘客通用。 |
|   | pageIndex | String | 否 | 页码。 |
|   | pageSize | String | 否 | 每页数量，单页数最大20，默认20 |
|   | goods\_id | String | 否 | 商品ID。 |
|   | keyword | String | 否 | 关键词，字数同京东商品名称一致，目前未限制。 |
|   | goods\_type | String | 否 | 一级类目： 0全部；1居家日用；2食品；3生鲜；4图书；5美妆个护；6母婴；7数码家电；8内衣；9配饰；10女装；11男装；12鞋品；13家装家纺；14文娱车品；15箱包；16户外运动。 |
|   | commission\_rate\_start | String | 否 | 佣金比例≥，值为空：全部商品，=20：佣金比例≥20的商品，=50：佣金比例≥50的商品。 |
|   | sale\_num\_start | String | 否 | 年销量≥ 值为空：全部商品，=500：年销量≥500的商品，=10000：年销量≥10000的商品。 |
|   | price | String | 否 | 价格区间，值为空：全部商品，0.0-9.9：9.9元商品，0.0-19.9：19.9元商品。 |
|   | coupon\_amount\_start | String | 否 | 券面额≥ 值为空：全部商品，=5：券面额≥5的商品，=20：券面额≥20的商品，=100：券面额≥100的商品。 |
|   | sort | String | 否 | 商品排序方式，new：按照综合排序，  
sale\_num\_asc：按照年销量从小到大排序，sale\_num\_desc：按照年销量从大到小排序，  
commission\_rate\_asc：按照佣金比例从小到大排序，commission\_rate\_desc：按照佣金比例从大到小排序，  
price\_asc：按照价格从小到大排序，price\_desc：按照价格从大到小排序，  
coupon\_info\_money\_asc：按照优惠券金额从小到大排序，coupon\_info\_money\_desc：按照优惠券金额从大到小排序，  
code：按照code值从大到小排序，date\_time：按照更新时间排序，random：按照随机排序 |
|   | total\_count | String | 否 | 返回商品总数，1：返回总数，0或者其他值：返回商品列表。 |

**相关说明：**

|  | 名称 | 类型 | 说明 |
| --- | --- | --- | --- |
|   |

**JSON返回示例：**

```json
{
    "status":200,
    "content":\[
        {
            "state":1,  /\*状态（1=正常；2=下架）\*/
            "J\_state":2,  /\*商品状态(注:2正常，其他状态为非正常状态)\*/
            "short\_name":"【京东商城】洁柔手帕纸 3层6片\*24",  /\*商品精编标题\*/
            "goods\_type":1,  /\*商品一级类目\*/
            "goods\_img":"http://img14.360buyimg.com/def69202103121440359578.jpg",  /\*商品主图\*/
            "goods\_price":19.9,  /\*商品原价\*/
            "goods\_content":"洁柔原浆手帕纸，柔软舒适，强韧、湿水不易破，质优价廉~好货囤起来~",  /\*商品文案\*/
            "goods\_recommend":"",  /\*推荐理由\*/
            "commissionshare":20,  /\*商品佣金比例\*/
            "goods\_link":"https://u.jd.com/uNX2Wsh",  /\*商品落地页\*/
            "discount\_price":10,  /\*优惠券面额\*/
            "discount\_link":"http://coupon.m.jd.com/coupons/show.action?key=6e04be7fc93c45baaccafa1b4e885fee&roleId=47659163",  /\*领券链接\*/
            "today\_num":0,  /\*今日领券数（商品今日领券总量，券类型为商品券）\*/
            "quota":15,  /\*券消费限额\*/
            "get\_start\_time":"2021/03/11 16:08:56",  /\*优惠券领券开始时间\*/
            "get\_end\_time":"2021/03/18 23:59:58",  /\*优惠券领取到期时间\*/
            "final\_price":9.9,  /\*商品最终价格（使用优惠券后的商品最终价格）\*/
            "historyPriceDay":0,  /\*历史最低价天数\*/
            "circle\_content":"%e2%80%94%e2%80%94%e2%80%94%e6%b4%81%e6%9f%94",  /\*朋友圈推广文案\*/
            "link\_content":"%e2%80%94%e2%80%94%e2%80%94%e6%b4%81%e6%9f%94%e",  /\*精编社群推广文案\*/
            "comments":265,  /\*评论数\*/
            "goodCommentsShare":96,  /\*商品好评率\*/
            "shop\_id":10387290,  /\*店铺id\*/
            "shop\_name":"汕科家庭清洁拼购专营店",  /\*店铺名称\*/
            "inOrderCount30Days":331,  /\*近30天引入单量（京东商品近30天销量）\*/
            "imageList":"\[{"url":"https:\\/\\/img14.360buyimg.com\\/jfs.jpg"}，{"url":"https:\\/\\/img14.360buyimg.com\\/jfs.jpg"}\]",  /\*京东图片合集（第一张图为京东商品主图） \*/
            "img\_info":"\["http://img14.360buyimg.com/imgzone/jfs/t1/17.jpg"\]",  /\*商品细节图，组图\*/
            "goods\_id":10026665449261,  /\*商品SKU\*/
            "subsidy\_type":2,  /\*类型\*/
            "subsidy\_start\_time":"2021/03/12 17:17:18",  /\*补贴开始时间\*/
            "subsidy\_end\_time":"2021/03/18 23:59:59",  /\*补贴结束时间\*/
            "add\_time":"2021/03/12 17:17:49",  /\*商品添加时间\*/
            "subsidy\_sku":"10026665449261，10026665449262，10026665449263，10026665449264",  /\*补贴商品SKU\*/
            "update\_time":"2021/03/12 11:37:47",  /\*商品更新时间\*/
            "brand\_id":"216",  /\*品牌ID\*/
            "brandCode":"27382",  /\*京东\_品牌code\*/
            "brandName":"洁柔（C&S）",  /\*京东\_品牌名称\*/
            "owner":"p",  /\*店铺性质:（g=自营；p=pop）\*/
            "cid1":15901,  /\*京东一级类目ID\*/
            "cid1Name":"家庭清洁/纸品",  /\*京东一级类目名称\*/
            "cid2":15902,  /\*京东二级类目ID\*/
            "cid2Name":"清洁纸品",  /\*京东二级类目名称\*/
            "cid3":15910,  /\*京东三级类目ID\*/
            "cid3Name":"手帕纸"  /\*京东三级类目名称\*/
        }
    \]
}
```

---

## 京东订单查询API

**接口名称：**京东订单查询API

**接口地址：**https://api.zhetaoke.com:10001/api/open\_jing\_union\_openz\_order\_row\_query.ashx

**返回数据：**json

**请求方式：**get

**请求示例：**https://api.zhetaoke.com:10001/api/open\_jing\_union\_openz\_order\_row\_query.ashx?appkey=&jd\_app\_key=&jd\_app\_secret=&key=

查询推广订单及佣金信息，可查询最近90天内下单的订单，会随着订单状态变化同步更新数据。  
支持按下单时间、完成时间或更新时间查询。建议按更新时间每分钟调用一次，查询最近一分钟的订单更新数据。  
支持查询subunionid、推广位、PID参数，支持普通推客查询。  
注意：此接口使用的是自己的京东应用的流量。  
  

**请求参数说明：**

|  | 名称 | 类型 | 必填 | 说明 |
| --- | --- | --- | --- | --- |
|   | appkey | String | 是 | 折淘客对接秘钥appkey，折淘客通用。 |
|   | jd\_app\_key | String | 是 | 京东开放平台应用的appkey，注意此参数值如果为空，下面的key参数必须传入。 |
|   | jd\_app\_secret | String | 是 | 京东开放平台应用的appsecret，注意此参数值如果为空，下面的key参数必须传入。 |
|   | pageIndex | String | 是 | 页码。 |
|   | pageSize | String | 是 | 每页数量，单页数最大500，默认20 |
|   | type | String | 是 | 订单时间查询类型(1：下单时间，2：完成时间（购买用户确认收货时间），3：更新时间 |
|   | startTime | String | 是 | 开始时间 格式yyyy-MM-dd HH:mm:ss，与endTime间隔不超过1小时 |
|   | endTime | String | 是 | 结束时间 格式yyyy-MM-dd HH:mm:ss，与startTime间隔不超过1小时 |
|   | childUnionId | String | 否 | 子推客unionID，传入该值可查询子推客的订单，注意不可和key同时传入。 |
|   | key | String | 否 | 工具商传入推客的授权key，可帮助该推客查询订单，注意不可和childUnionid同时传入。注意此参数值如果为空，jd\_app\_key和jd\_app\_secret参数必须传入。 |
|   | fields | String | 否 | 支持出参数据筛选，逗号','分隔，目前可用：goodsInfo（商品信息）,categoryInfo(类目信息） |
|   |  |  |  | [点击这里查看官方接口说明！点击这里查看官方接口说明！点击这里查看官方接口说明！](https://union.jd.com/openplatform/api/v2?apiName=jd.union.open.order.row.query)  
[点击这里查看官方接口说明！点击这里查看官方接口说明！点击这里查看官方接口说明！](https://union.jd.com/openplatform/api/v2?apiName=jd.union.open.order.row.query)  
[点击这里查看官方接口说明！点击这里查看官方接口说明！点击这里查看官方接口说明！](https://union.jd.com/openplatform/api/v2?apiName=jd.union.open.order.row.query)  
 |

**相关说明：**

|  | 名称 | 类型 | 说明 |
| --- | --- | --- | --- |
|   |

**JSON返回示例：**

```json
{
    "jd\_union\_open\_order\_row\_query\_response":{
        "queryResult":{
            "code":"200",
            "data":{
                "orderRowResp":{
                    "commissionRate":"5.0",
                    "estimateCosPrice":"54.0",
                    "orderId":"108618000005",
                    "validCode":"16",
                    "sign":"B44C0FC3F104167FEC8A53DFD2B26E40",
                    "estimateFee":"2.43",
                    "pid":"618\_618\_618",
                    "cpActId":"0",
                    "rid":"37843",
                    "orderEmt":"2",
                    "skuName":"朴坊 WAZZUPbaby 变色龙盲盒第三弹 第三代龙出没注意盲盒 单个盲盒，款式随机",
                    "popId":"709982",
                    "orderTime":"2020-01-02 15:50:16",
                    "modifyTime":"2020-01-02 16:01:03",
                    "payMonth":"0",
                    "giftCouponOcsAmount":"0",
                    "price":"69.0",
                    "unionTag":"00000000000000000000000000000001",
                    "cid2":"11158",
                    "id":"415900297816660001",
                    "cid3":"11969",
                    "finalRate":"90.0",
                    "actualFee":"0.0",
                    "cid1":"1620",
                    "skuId":"44303679033",
                    "skuFrozenNum":"0",
                    "ext1":"hello\_world",
                    "finishTime":"2020-01-03 15:59:16",
                    "subsidyRate":"0.0",
                    "unionAlias":"\*\*平台",
                    "unionId":"1000618618",
                    "giftCouponKey":"xxx\_coupon\_key",
                    "parentId":"0",
                    "plus":"0",
                    "categoryInfo":{
                        "cid1Name":"珠宝首饰",
                        "cid2Name":"木手串/把件",
                        "cid2":"2",
                        "cid3Name":"其他",
                        "cid3":"3",
                        "cid1":"1"
                    },
                    "subUnionId":"331967",
                    "unionRole":"1",
                    "skuReturnNum":"0",
                    "traceType":"2",
                    "positionId":"0",
                    "actualCosPrice":"0.0",
                    "balanceExt":"{20191020:10,20191120:-2}",
                    "siteId":"61866",
                    "proPriceAmount":"6.18",
                    "expressStatus":"10",
                    "skuNum":"1",
                    "subSideRate":"90.0",
                    "goodsInfo":{
                        "owner":"g",
                        "mainSkuId":"6161111",
                        "productId":"1236547",
                        "imageUrl":"http://img14.360buyimg.com/ads/jfs/t22495/56/628456568/380476/9befc935/5b39fb01N7d1af390.jpg",
                        "shopName":"XXXX旗舰店",
                        "shopId":"45619"
                    }
                }
            },
            "hasMore":"true",
            "message":"success"
        }
    }
}
```

---

## 京享礼金商品API

**接口名称：**京享礼金商品API

**接口地址：**https://api.zhetaoke.com:10001/api/open\_jing\_goods\_lijin\_list.ashx

**返回数据：**json

**请求方式：**get

**请求示例：**https://api.zhetaoke.com:10001/api/open\_jing\_goods\_lijin\_list.ashx?appkey=&pageIndex=1&pageSize=20&sort=new

重要说明：本接口为京享礼金商品列表，适合做社群和朋友圈推广！  
重要说明：本接口为京享礼金商品列表，适合做社群和朋友圈推广！  
重要说明：本接口为京享礼金商品列表，适合做社群和朋友圈推广！  
  
转链方法：1、获取字段goods\_link（商品链接）和lijin\_id（礼金ID），2、使用接口“京东转链API-新”进行转链。  
转链方法：1、获取字段goods\_link（商品链接）和lijin\_id（礼金ID），2、使用接口“京东转链API-新”进行转链。  
转链方法：1、获取字段goods\_link（商品链接）和lijin\_id（礼金ID），2、使用接口“京东转链API-新”进行转链。  

**请求参数说明：**

|  | 名称 | 类型 | 必填 | 说明 |
| --- | --- | --- | --- | --- |
|   | appkey | String | 是 | 折淘客对接秘钥appkey，折淘客通用。 |
|   | pageIndex | String | 否 | 页码。 |
|   | pageSize | String | 否 | 每页数量，单页数最大20，默认20 |
|   | keyword | String | 否 | 关键词，字数同京东商品名称一致，目前未限制。 |
|   | commission\_rate\_start | String | 否 | 佣金比例≥，值为空：全部商品，=20：佣金比例≥20的商品，=50：佣金比例≥50的商品。 |
|   | sale\_num\_start | String | 否 | 年销量≥ 值为空：全部商品，=500：年销量≥500的商品，=10000：年销量≥10000的商品。 |
|   | price | String | 否 | 价格区间，值为空：全部商品，0.0-9.9：9.9元商品，0.0-19.9：19.9元商品。 |
|   | coupon\_amount\_start | String | 否 | 券面额≥ 值为空：全部商品，=5：券面额≥5的商品，=20：券面额≥20的商品，=100：券面额≥100的商品。 |
|   | sort | String | 否 | 商品排序方式，new：按照综合排序，  
sale\_num\_asc：按照年销量从小到大排序，sale\_num\_desc：按照年销量从大到小排序，  
commission\_rate\_asc：按照佣金比例从小到大排序，commission\_rate\_desc：按照佣金比例从大到小排序，  
price\_asc：按照价格从小到大排序，price\_desc：按照价格从大到小排序，  
coupon\_info\_money\_asc：按照优惠券金额从小到大排序，coupon\_info\_money\_desc：按照优惠券金额从大到小排序，  
code：按照code值从大到小排序，date\_time：按照更新时间排序，random：按照随机排序 |
|   | total\_count | String | 否 | 返回商品总数，1：返回总数，0或者其他值：返回商品列表。 |

**相关说明：**

|  | 名称 | 类型 | 说明 |
| --- | --- | --- | --- |
|   |

**JSON返回示例：**

```json
{
    "status":200,
    "content":\[
        {
            "state":1,  /\*状态（1=正常；2=下架）\*/
            "J\_state":2,  /\*商品状态(注:2正常，其他状态为非正常状态)\*/
            "short\_name":"【京东商城】洁柔手帕纸 3层6片\*24",  /\*商品精编标题\*/
            "goods\_type":1,  /\*商品一级类目\*/
            "goods\_img":"http://img14.360buyimg.com/def69202103121440359578.jpg",  /\*商品主图\*/
            "goods\_price":19.9,  /\*商品原价\*/
            "goods\_content":"洁柔原浆手帕纸，柔软舒适，强韧、湿水不易破，质优价廉~好货囤起来~",  /\*商品文案\*/
            "goods\_recommend":"",  /\*推荐理由\*/
            "commissionshare":20,  /\*商品佣金比例\*/
            "goods\_link":"https://u.jd.com/uNX2Wsh",  /\*商品落地页\*/
            "discount\_price":10,  /\*优惠券面额\*/
            "discount\_link":"http://coupon.m.jd.com/coupons/show.action?key=6e04be7fc93c45baaccafa1b4e885fee&roleId=47659163",  /\*领券链接\*/
            "today\_num":0,  /\*今日领券数（商品今日领券总量，券类型为商品券）\*/
            "quota":15,  /\*券消费限额\*/
            "get\_start\_time":"2021/03/11 16:08:56",  /\*优惠券领券开始时间\*/
            "get\_end\_time":"2021/03/18 23:59:58",  /\*优惠券领取到期时间\*/
            "final\_price":9.9,  /\*商品最终价格（使用优惠券后的商品最终价格）\*/
            "historyPriceDay":0,  /\*历史最低价天数\*/
            "circle\_content":"%e2%80%94%e2%80%94%e2%80%94%e6%b4%81%e6%9f%94",  /\*朋友圈推广文案\*/
            "link\_content":"%e2%80%94%e2%80%94%e2%80%94%e6%b4%81%e6%9f%94%e",  /\*精编社群推广文案\*/
            "comments":265,  /\*评论数\*/
            "goodCommentsShare":96,  /\*商品好评率\*/
            "shop\_id":10387290,  /\*店铺id\*/
            "shop\_name":"汕科家庭清洁拼购专营店",  /\*店铺名称\*/
            "inOrderCount30Days":331,  /\*近30天引入单量（京东商品近30天销量）\*/
            "imageList":"\[{"url":"https:\\/\\/img14.360buyimg.com\\/jfs.jpg"}，{"url":"https:\\/\\/img14.360buyimg.com\\/jfs.jpg"}\]",  /\*京东图片合集（第一张图为京东商品主图） \*/
            "img\_info":"\["http://img14.360buyimg.com/imgzone/jfs/t1/17.jpg"\]",  /\*商品细节图，组图\*/
            "goods\_id":10026665449261,  /\*商品SKU\*/
            "subsidy\_type":2,  /\*类型\*/
            "subsidy\_start\_time":"2021/03/12 17:17:18",  /\*补贴开始时间\*/
            "subsidy\_end\_time":"2021/03/18 23:59:59",  /\*补贴结束时间\*/
            "add\_time":"2021/03/12 17:17:49",  /\*商品添加时间\*/
            "subsidy\_sku":"10026665449261，10026665449262，10026665449263，10026665449264",  /\*补贴商品SKU\*/
            "update\_time":"2021/03/12 11:37:47",  /\*商品更新时间\*/
            "brand\_id":"216",  /\*品牌ID\*/
            "brandCode":"27382",  /\*京东\_品牌code\*/
            "brandName":"洁柔（C&S）",  /\*京东\_品牌名称\*/
            "owner":"p",  /\*店铺性质:（g=自营；p=pop）\*/
            "cid1":15901,  /\*京东一级类目ID\*/
            "cid1Name":"家庭清洁/纸品",  /\*京东一级类目名称\*/
            "cid2":15902,  /\*京东二级类目ID\*/
            "cid2Name":"清洁纸品",  /\*京东二级类目名称\*/
            "cid3":15910,  /\*京东三级类目ID\*/
            "cid3Name":"手帕纸",  /\*京东三级类目名称\*/
            "lijin\_name":"家装节京享礼金（10元）",  /\*礼金名称\*/
            "lijin\_id":"1d308968b00ff73f",  /\*礼金ID\*/
            "lijin\_money":"10",  /\*礼金面额\*/
            "lijin\_count":"40000",  /\*礼金数量\*/
            "lijin\_lingqu\_start\_time":"2021-09-18 00:00",  /\*礼金领取有效期-开始时间\*/
            "lijin\_lingqu\_end\_time":"2021-09-30 23:59",  /\*礼金领取有效期-结束时间\*/
            "lijin\_use\_start\_time":"2021-09-18 00:00",  /\*礼金使用有效期-开始时间\*/
            "lijin\_use\_end\_time":"2021-09-18 00:00",  /\*礼金使用有效期-结束时间\*/
            "lijin\_create\_time":"2021-09-18 00:00"  /\*礼金创建时间\*/
        }
    \]
}
```

---

## 京东商品详情API[详情图]

**接口名称：**京东商品详情API\[详情图\]--此接口可获取商品详情查询接口,大字段信息，包含商品详情图等。

**接口地址：**https://api.zhetaoke.com:10001/api/open\_jd\_union\_open\_goods\_bigfield\_query.ashx

**返回数据：**json

**请求方式：**get

**请求示例：**https://api.zhetaoke.com:10001/api/open\_jd\_union\_open\_goods\_bigfield\_query.ashx?appkey=&content=

**请求参数说明：**

|  | 名称 | 类型 | 必填 | 说明 |
| --- | --- | --- | --- | --- |
|   | appkey | String | 是 | 折淘客对接秘钥appkey，折淘客通用。 |
|   | content | string | 是 | 京东商品字符串id或者京东url链接  
重要的事情说三遍：该参数需要进行Urlencode编码！该参数需要进行Urlencode编码！该参数需要进行Urlencode编码！  
 |
|   | fields | string | 否 | 查询域集合，不填写则查询全部。  
目目前支持：categoryInfo（类目信息）,imageInfo（图片信息）,baseBigFieldInfo（基础大字段信息）,bookBigFieldInfo（图书大字段信息）,videoBigFieldInfo（影音大字段信息）,detailImages（详情图）  
 |

**相关说明：**

|  | 名称 | 类型 | 说明 |
| --- | --- | --- | --- |
|   |

**JSON返回示例：**

```json
\[
    {
        "baseBigFieldInfo":{
            "wareQD":"主机\*1 鼻套\*1 激光器\*1 电源适配器\*1 说明书\*1",
            "wdis":""
        },
        "categoryInfo":{
            "cid1":9192,
            "cid1Name":"医疗保健",
            "cid2":9197,
            "cid2Name":"保健器械",
            "cid3":12591,
            "cid3Name":"理疗仪"
        },
        "detailImages":"http://img30.360buyimg.com/sku/jfs/t1/203789/6/10911/151650/616555f3E7874d88d/dde9ecab1754d04c.jpg,
                                http://img30.360buyimg.com/sku/jfs/t1/217979/8/2072/196239/6179182fE2ea42b71/1b7d36406d4a7582.jpg,
                                http://img30.360buyimg.com/sku/jfs/t1/199956/16/14524/114573/6179370fE30403586/884bbfc44f3713d2.jpg,
                                http://img30.360buyimg.com/sku/jfs/t1/214571/21/2041/128607/61791832E2a9a6cbd/5ebbfb2eb9ab186b.jpg,
                                http://img30.360buyimg.com/sku/jfs/t1/202348/1/4860/715881/61334461E3f9f4403/c66b8fd8bb6d5755.jpg,
                                http://img30.360buyimg.com/sku/jfs/t1/204893/21/4716/249002/61334460E3892a17e/213370adad468227.jpg,
                                http://img30.360buyimg.com/sku/jfs/t1/191002/39/21360/176492/61334460E3ece2178/21f32f5d3161aed1.png,
                                http://img30.360buyimg.com/sku/jfs/t1/184513/6/16295/128174/6101012fE288a57e1/4c4cd65f43e1b13b.jpg,
                                http://img30.360buyimg.com/sku/jfs/t1/184050/25/16336/74217/6101012bE630e0a42/7acaeb51f1553b58.jpg,
                                http://img30.360buyimg.com/sku/jfs/t1/194007/24/15495/53073/61010128E9a9d7dec/7ec51b786a5ee897.jpg,
                                http://img30.360buyimg.com/sku/jfs/t1/178927/30/16303/93314/6101012dEf9877b68/32a3dfa82e3ccd46.jpg,
                                http://img30.360buyimg.com/sku/jfs/t1/190444/29/15456/60217/61010129E90d31320/6f1bb3cbc18d947b.jpg,
                                http://img30.360buyimg.com/sku/jfs/t1/196282/28/15424/105455/6101012eE2e55ab47/9943b911186580da.jpg,
                                http://img30.360buyimg.com/sku/jfs/t1/196424/35/15278/78559/6101012cE4c23f23c/e89b345990558cf3.jpg,
                                http://img30.360buyimg.com/sku/jfs/t1/179276/22/16510/124169/6101012fEa21f68f0/5538f6c0b9a17d56.jpg,
                                http://img30.360buyimg.com/sku/jfs/t1/187613/40/15358/96899/6101012eE5f657341/6dae28cd2dc8e1b2.jpg,
                                http://img30.360buyimg.com/sku/jfs/t1/191820/34/15649/113362/6101012fE0a7ec3ef/8f39405e52609aa0.jpg,
                                http://img30.360buyimg.com/sku/jfs/t1/178615/19/16481/68945/6101012cE64180826/2f29e7b299db340c.jpg,
                                http://img30.360buyimg.com/sku/jfs/t1/182905/22/14069/561514/60ee8d0cE16caf09e/2d4241bb9545407a.gif,
                                http://img30.360buyimg.com/sku/jfs/t1/196584/22/15241/112683/61010171E5a68c0a3/1091379f451ac246.jpg,
                                http://img30.360buyimg.com/sku/jfs/t1/184433/22/16351/92279/61010170E879c3ec0/1d8b0093c4c67471.jpg,
                                http://img30.360buyimg.com/sku/jfs/t1/193155/15/15267/91345/61010170Ea77f7358/e94cc9d404e66c30.jpg,
                                http://img30.360buyimg.com/sku/jfs/t1/191214/21/15437/42852/61010162Eb6da85c9/79385a707cc91f48.jpg,
                                http://img30.360buyimg.com/sku/jfs/t1/179519/18/16121/80871/61010170E0e3feffc/13a6723763f0a79a.jpg,
                                http://img30.360buyimg.com/sku/jfs/t1/179892/27/16288/80297/61010170E05953042/6aa8e87ead7e3fc4.jpg,
                                http://img30.360buyimg.com/sku/jfs/t1/182194/9/16243/197919/61010172E4c88ee61/7262d7310cee70dd.jpg,
                                http://img30.360buyimg.com/sku/jfs/t1/200205/14/132/113296/61010171E748da3da/cc1ac9ba0911794d.jpg,
                                http://img30.360buyimg.com/sku/jfs/t1/185144/14/16250/129216/61010172E7244ee85/9dc2f70b23f61bc2.jpg,
                                http://img30.360buyimg.com/sku/jfs/t1/186368/2/15591/51770/61010165Eb460991a/d0a447dcc10b21e4.jpg,
                                http://img30.360buyimg.com/sku/jfs/t1/179434/23/16285/106370/61010171E45d05074/63a63994c62a12f5.jpg,
                                http://img30.360buyimg.com/sku/jfs/t1/185189/30/16292/130886/61010172Eb1a46698/7ed40c93fea081cd.jpg,
                                http://img30.360buyimg.com/sku/jfs/t1/176871/9/16303/101391/61010171E3ce79b0b/65053c621d552ae6.jpg,
                                http://img30.360buyimg.com/sku/jfs/t1/177372/40/16503/272054/61010172E7c2e9f5b/899f852ba3231398.jpg,
                                http://img30.360buyimg.com/sku/jfs/t1/200305/30/130/114566/61010171E24fac675/0a241b3df2b6a26b.jpg,
                                http://img30.360buyimg.com/sku/jfs/t1/183801/1/16404/50458/61010196E2ffba563/3468239adfdf4a99.jpg,
                                http://img30.360buyimg.com/sku/jfs/t1/184493/25/16258/49178/610101c1E7bb5d664/33af6ba78fa95d56.jpg",
                                  /\*商品详情图\*/
        "imageInfo":{
            "imageList":\[
                {
                    "url":"https://img14.360buyimg.com/pop/jfs/t1/147442/19/23613/110601/61a9ccd6Ed3b2b054/43594ad3a68f0f07.jpg"
                },
                {
                    "url":"https://img14.360buyimg.com/pop/jfs/t1/211240/36/4520/99303/61628e1dE124844eb/d194d12c9871262f.jpg"
                },
                {
                    "url":"https://img14.360buyimg.com/pop/jfs/t1/171948/8/19583/93514/60ee8d57E9ba7fa44/112bb3ffdde3f736.jpg"
                },
                {
                    "url":"https://img14.360buyimg.com/pop/jfs/t1/97677/1/18964/43710/60ee8d56Ec181a76e/33805c5dfd221d08.jpg"
                },
                {
                    "url":"https://img14.360buyimg.com/pop/jfs/t1/39376/15/11664/43958/60ee98bfE5f48e1ef/c884e221c2a58520.jpg"
                },
                {
                    "url":"https://img14.360buyimg.com/pop/jfs/t1/203928/30/9081/96118/61557992E17178464/3b8c4268a49d229c.jpg"
                },
                {
                    "url":"https://img14.360buyimg.com/pop/jfs/t1/169696/7/24866/72019/61976626Ef7b0926f/5726c5a5da92c99a.jpg"
                },
                {
                    "url":"https://img14.360buyimg.com/pop/jfs/t1/209491/4/7574/96493/617f83b8E3e6c3942/a5997b6abc5f482f.jpg"
                },
                {
                    "url":"https://img14.360buyimg.com/pop/jfs/t1/186627/21/13237/95112/60ee8dabE28f06f8a/d2464b883ddd225c.jpg"
                },
                {
                    "url":"https://img14.360buyimg.com/pop/jfs/t1/205675/28/10294/116794/61628e6fEf8d15619/03692df206d07b9a.jpg"
                }
            \]
        },
        "mainSkuId":100022751812,
        "owner":"g",
        "productId":100022751812,
        "skuId":100022751812,
        "skuName":"光大夫 鼻炎治疗仪鼻炎仪急性慢性过敏性鼻炎理疗仪家用成人儿童半导体激光理疗仪器",
        "skuStatus":1
    }
\]
```

---

## 美团转链API

**接口名称：**美团转链API

**接口地址：**https://api.zhetaoke.com:10001/api/open\_meituan\_generateLink.ashx

**返回数据：**json

**请求方式：**get/post,请尽量使用post。

**请求示例：**https://api.zhetaoke.com:10001/api/open\_meituan\_generateLink.ashx?appkey=&sid=&actId=33&linkType=1&miniCode=1

**请求参数说明：**

|  | 名称 | 类型 | 必填 | 说明 |
| --- | --- | --- | --- | --- |
|   | appkey | String | 是 | 折淘客对接秘钥appkey，折淘客通用。 |
|   | sid | string | 是 | 对应的淘客账号授权SID |
|   | actId | string | 是 | 支持美团活动id，支持美团活动链接。  
活动ID：7，美团外卖节，帮你吃更好，佣金比例3%起。  
活动ID：272，【商超果蔬】超值爆款鲜花全场3折起！鲜花新客满58-12 部分活动商家满98-10，佣金15%起！  
活动ID：273，【美团闪购】红包天天领。  
活动ID：284，【玩乐变美】天天可领红包！佣金1.1%-4.1%。  
活动ID：309，【美食团购】红包天天领，红包订单佣金2%。  
活动ID：442，【高佣专场】吃喝玩乐全品类好货，商品佣金高达15%+。  
活动ID：529，【特惠券包】4.9元购180元神券包！  
活动ID：689，【美团拼好饭】爆品一口价会场！大牌6.9起 0元起送 免配送 免打包。  
更多美团联盟活动，请参考接口：http://api.zhetaoke.com:10000/api/api\_activity.ashx?appkey=&activityId=&type=10  
更多美团联盟活动，请参考接口：http://api.zhetaoke.com:10000/api/api\_activity.ashx?appkey=&activityId=&type=10  
更多美团联盟活动，请参考接口：http://api.zhetaoke.com:10000/api/api\_activity.ashx?appkey=&activityId=&type=10  
  
![](http://v.zhetaoke.com:10000/images/a1.png) |
|   | linkType | string | 是 | 链接类型，枚举值：1 H5长链接；2 H5短链接；3 deeplink(唤起)链接；4 微信小程序唤起路径；5 团口令 |
|   | miniCode | string | 否 | 是否生成小程序二维码，1生成，如果不需要此图，请忽略该参数，会降低转链性能。  
![](http://v.zhetaoke.com:10000/uploads_qrcode/meituan.jpg) |
|   | miniCode2 | string | 否 | 是否生成长链接二维码推广大图，1生成，如果不需要此图，请忽略该参数，会降低转链性能。  
如果需要此图，minicode参数必须等于1。  
![](http://v.zhetaoke.com:10000/uploads_qrcode/meituan2.jpg) |
|   | miniCode3 | string | 否 | 是否生成小程序二维码推广大图，1生成，如果不需要此图，请忽略该参数，会降低转链性能。  
如果需要此图，minicode参数必须等于1。  
![](http://v.zhetaoke.com:10000/uploads_qrcode/meituan3.jpg) |
|   | platform | string | 否 | 自定义平台名称  
此参数需要联系折淘客官方进行备案审核。否则不要使用。  
此参数需要联系折淘客官方进行备案审核。否则不要使用。  
此参数需要联系折淘客官方进行备案审核。否则不要使用。  
如果你有多个折淘客账号都需要推广，需要此参数，否则不需要。  
 |
|   | customer\_id | string | 否 | 自定义参数  
只允许数字，最大长度11位。  
此字段可以用来区分用户，进行下级返利。  
 |

**相关说明：**

|  | 名称 | 类型 | 说明 |
| --- | --- | --- | --- |

**JSON返回示例：**

```json
{
                                "status":0,
                                "des":"推广链接生成成功",
                                "data":"https://click.meituan.com/t?t=1&c=1&p=OWMpZ-uzIFOVe6JyOONs3dXuqV0qcAf-r-KCvH",  /\*自己的美团推广链接\*/
                                "wx\_mini\_pic":"http://p0.meituan.net/poiqrcode/b407b0c4346e854c4468a704ab42f156103581.jpg"  /\*自己的小程序二维码推广小图\*/
                                "qrcode\_chang\_pic":"http://www.zhetaoke.com/uploads\_qrcode/20210429/20210429449433.jpg", /\*自己的长链接二维码推广大图\*/
                                "qrcode\_wx\_pic":"http://www.zhetaoke.com/uploads\_qrcode/20210429/2021047444942.jpg"  /\*自己的小程序二维码推广大图\*/
                            }
```

---

## 美团订单查询API

**接口名称：**美团订单查询API

**接口地址：**https://api.zhetaoke.com:10001/api/open\_meituan\_orderList2.ashx

**返回数据：**json

**请求方式：**get

**请求示例：**https://api.zhetaoke.com:10001/api/open\_meituan\_orderList2.ashx?appkey=&type=&startTime=&endTime=

根据订单支付时间或者订单更新时间获取最近90天内的美团订单数据  
支持按支付时间、更新时间查询。  
建议按照订单支付时间，每分钟查询一次，按照订单更新时间，每分钟查询一次即可。  
个别长时间不结算的订单可以单独用订单编号来查询。  
  
[点击这里在线查询美团订单信息！点击这里在线查询美团订单信息！点击这里在线查询美团订单信息！](http://www.zhetaoke.com/user/extend/extend_app_dingdan_meituan.aspx)  
  

**请求参数说明：**

|  | 名称 | 类型 | 必填 | 说明 |
| --- | --- | --- | --- | --- |
|   | appkey | String | 是 | 折淘客对接秘钥appkey，折淘客通用。 |
|   | type | String | 是 | 时间类型，1表示按照订单支付时间获取，2表示按照订单更新时间获取。 |
|   | page | String | 是 | 页码 |
|   | page\_size | String | 是 | 每页数量，单页数最大50，默认50 |
|   | startTime | String | 否 | 订单开始时间,例子：2021-04-30 08:00:00 |
|   | endTime | String | 否 | 订单结束时间,例子：2021-04-30 09:00:00 |
|   | orderid | String | 否 | 订单编号，必须输入完整的订单编号。 |
|   | sid | String | 否 | 折淘客授权sid，无值，表示获取appkey下所有美团订单。 |

**相关说明：**

|  | 名称 | 类型 | 说明 |
| --- | --- | --- | --- |
|   |

**JSON返回示例：**

```json
{
    "status":200,
    "content":\[
        {
            "code":"15",      /\*折淘客自动增长列\*/
            "account":"83076790@qq.com",      /\*折淘客账号\*/
            "appkey\_ztk":"a2a33cacfb1f4005b818ffd0ab7c951d",      /\*折淘客appkey\*/
            "sid":"16ztkzheta",   /\*美团原始推广位sid\*/
            "is\_jiesuan":"0",     /\*折淘客是否结算，0未结算，1已结算\*/
            "jiesuan\_time":"",      /\*折淘客结算时间\*/
            "jiesuan\_profit":"1.16",      /\*折淘客结算金额\*/
            "orderid":"105518514197820013",   /\*订单编号\*/
            "paytime":"2021/04/27 12:22:09",      /\*订单支付时间\*/
            "payprice":"21.50",   /\*订单实际支付金额\*/
            "profit":1.29,    /\*预估佣金\*/
            "smstitle":"霸碗盖码饭（凤凰大厦店）",    /\*订单标题\*/
            "refundprice":0,      /\*退款金额\*/
            "refundtime":"",      /\*退款时间\*/
            "status":"8",     /\*订单状态，1已付款，8已完成，9已退款或风控\*/
            "update\_time":"",   /\*数据最后更新时间\*/
            "type":"4",   /\*订单类型，0表示团购订单，2表示酒店订单，4表示外卖订单，5表示话费&团好货订单，6表示闪购订单，8表示优选订单。\*/
            "sid\_ztk":"16",   /\*折淘客授权sid\*/
            "customer\_id\_ztk":"100000",   /\*推广者自定义编号\*/
            "platform\_ztk":"zhetaoke",     /\*推广者平台类型，默认zhetaoke\*/
            "actId":"33",   /\*活动id\*/
            "businessLine":"2",   /\*业务线类型,1：平台，2：外卖/闪购，3：酒店，4：优选，5：团APP拉新\*/
            "subBusinessLine":"1"   /\*子业务线，当是外卖订单时，subBusinessLine=1，是闪购订单时，subBusinessLine=2\*/
        },
        {
            "code":"14",
            "account":"83076790@qq.com",
            "appkey\_ztk":"a2a33cacfb1f4005b818ffd0ab7c951d",
            "sid":"16ztkzheta",
            "is\_jiesuan":"0",
            "jiesuan\_time":null,
            "jiesuan\_profit":"1.16",
            "orderid":"105518514197820012",
            "paytime":"2021/04/27 12:22:09",
            "payprice":"21.50",
            "profit":1.29,
            "smstitle":"霸碗盖码饭（凤凰大厦店）",
            "refundprice":0,
            "refundtime":"",
            "status":"8",
            "update\_time":null,
            "type":"4",
            "sid\_ztk":"16",
            "customer\_id\_ztk":"100000",
            "platform\_ztk":"zhetaoke"，
            "actId":"33",
            "businessLine":"2",
            "subBusinessLine":"1"
        }
    \]
}
```

---

## 唯品会转链API

**接口名称：**唯品会转链API

**接口地址：**https://api.zhetaoke.com:10001/api/open\_vip\_genByVIPUrlWithOauth.ashx

**返回数据：**json

**请求方式：**get

**请求示例：**https://api.zhetaoke.com:10001/api/open\_vip\_genByVIPUrlWithOauth.ashx?appkey=&sid=&url=&chanTag=

根据唯品会商品id或者唯品会链接生成联盟链接。  
  
[点击这里在线进行唯品会授权！点击这里在线进行唯品会授权！点击这里在线进行唯品会授权！](http://www.zhetaoke.com/user/shouquan_vip.aspx)  
  

**请求参数说明：**

|  | 名称 | 类型 | 必填 | 说明 |
| --- | --- | --- | --- | --- |
|   | appkey | String | 是 | 折淘客对接秘钥appkey，折淘客通用。 |
|   | sid | String | 是 | 对应的唯品会账号授权SID，[点击这里查看SID！](http://www.zhetaoke.com/user/shouquan_vip.aspx) |
|   | url | String | 是 | 唯品会商品id或者唯品会链接;  
商品id例子：6919173790682972930;  
唯品会链接例子：https://t.vip.com/URKiMYwbrW6 |
|   | chanTag | String | 否 | 渠道标识，即推广位PID。 |
|   | statParam | String | 否 | 自定义渠道统计参数(最大长度为256字符) |

**相关说明：**

|  | 名称 | 类型 | 说明 |
| --- | --- | --- | --- |
|   |

**JSON返回示例：**

```json
{
    "returnCode":"0",
    "result":{
        "urlInfoList":\[
            {
                "noEvokeUrl":"https://t.vip.com/mlCYWuKuVNA?wq=1",  /\*CPS短链接：不唤起快应用\*/
                "vipQuickAppUrl":"hap://app/com.VIP.VIPQuickAPP/pages/product/detail",  /\*唯品会快应用链接\*/
                "vipWxUrl":"pages/productDetail/productDetail?goodsId=6919197068968114384",  /\*唯品会小程序链接：仅在根据商品id获取时返回\*/
                "deeplinkUrl":"vipshop://showGoodsDetail?pid=6919197068968114384&goodsType=1",  /\*CPS Deeplink链接\*/
                "longUrl":"https://t.vip.com/redirect.php?url=eyJzY2hlbWVjb2RlIjoibWOi2NiJ9",  /\*CPS长连接\*/
                "source":"https://m.vip.com/product-1710616528-6919197068968114384.html",  /\*如果根据商品id生成链接，该值商品id， 如果根据链接生成链接，该值为唯品会链接\*/
                "ulUrl":"https://click.union.vip.com/deeplink/showGoodsDetail",  /\*CPS通用连接\*/
                "url":"https://t.vip.com/mlCYWuKuVNA",  /\*CPS短链接\*/
                "traFrom":"adp%3AC01V4m8y6lrhjl1o%3A%3Amig\_code",  /\*小程序CPS参数：通用小程序跟单参数\*/
                "noEvokeLongUrl":"https://t.vip.com/redirect.php?url="  /\*CPS长链接：不唤起快应用\*/
            }
        \]
    }
}
```

---

## 唯品会账号授权API

**接口名称：**唯品会账号授权API

**接口地址：**https://api.zhetaoke.com:10001/api/vip\_shouquan.ashx

**返回数据：**json

**请求方式：**url访问

**请求示例：**https://api.zhetaoke.com:10001/api/vip\_shouquan.ashx?appkey=&backurl=

唯品会账号授权API接口。  
  
[点击这里在线进行唯品会授权！点击这里在线进行唯品会授权！点击这里在线进行唯品会授权！](http://www.zhetaoke.com/user/shouquan_vip.aspx)  
  

**请求参数说明：**

|  | 名称 | 类型 | 必填 | 说明 |
| --- | --- | --- | --- | --- |
|   | appkey | String | 是 | 折淘客对接秘钥appkey，折淘客通用。 |
|   | backurl | String | 是 | 唯品会账号授权后，登记到你的授权列表，并将授权信息返回至你的回调地址  
回调地址说明：http://www.xxx.com/authback.html （回调地址一定要http://或者https://开头）  
一个折淘客服务平台账号理论上可以授权10000个以上甚至更多唯品会账号，如有疑问，请联系我们。  
此接口主要是方便大家开发，开发者通过些接口生成链接后，发给推广者，他们授权后即可回调授权信息到开发者的服务器，这样，也就不用麻烦推广者再去单独注册一个折淘客账户。 |

**相关说明：**

|  | 名称 | 类型 | 说明 |
| --- | --- | --- | --- |
|   |

**授权成功后回调例子：**

```json
https://www.xxx.com/?account=111111@qq.com&sid=59471&user\_id=5055D865B66D688C1A8929AA55B93A8A&access\_token=&auth\_time=1624365583&expire\_time=1655949117
```

---

## 唯品会获取授权列表API

**接口名称：**唯品会获取授权列表API

**接口地址：**https://api.zhetaoke.com:10001/api/vip\_shouquaninfo.ashx

**返回数据：**json

**请求方式：**get

**请求示例：**https://api.zhetaoke.com:10001/api/vip\_shouquaninfo.ashx?appkey=&page=&expire\_day=&sid

获取唯品会账户授权列表API接口。  
  

**请求参数说明：**

|  | 名称 | 类型 | 必填 | 说明 |
| --- | --- | --- | --- | --- |
|   | appkey | String | 是 | 折淘客对接秘钥appkey，折淘客通用。 |
|   | page | String | 否 | 第几页，每页最多返回100个 |
|   | expire\_day | String | 否 | 还剩下几天授权过期，取值范围0-7之间的整数，比如值为3，表示获取还剩下3天授权过期淘客账号信息 |
|   | sid | String | 否 | 对应的唯品会账号授权ID[点击查看](/user/shouquan_vip.aspx) |

**相关说明：**

|  | 名称 | 类型 | 说明 |
| --- | --- | --- | --- |
|   |

**JSON返回示例：**

```json
{
    "status": 200,
    "data": \[
        {
            "sid": "2\*\*",  /\*唯品会账号授权后，在折淘客生成的ID\*/
            "account": "8307\*\*\*",  /\*折淘客账号\*/
            "user\_id": "2586\*\*\*",  /\*唯品会账号，编号形式\*/
            "user\_nick": "",  /\*暂时无用\*/
            "auth\_time": "2018/11/12 15:55:49",  /\*授权时间\*/
            "expire\_time": "2018/12/12 15:55:49",  /\*授权过期时间\*/
            "access\_token": "248DC2AC610C96C7B2BA71C5CEE739C67EFDD019"  /\*access\_token\*/
        }
    \]
}
```

---

## 唯品会获取订单列表API

**接口名称：**唯品会获取订单列表API

**接口地址：**https://api.zhetaoke.com:10001/api/open\_vip\_orderListWithOauth.ashx

**返回数据：**json

**请求方式：**get

**请求示例：**https://api.zhetaoke.com:10001/api/open\_vip\_orderListWithOauth.ashx?appkey=&sid=&page=1

获取订单信息列表，最大查询时间区间默认不超过10天。  
  
[点击这里在线进行唯品会授权！点击这里在线进行唯品会授权！点击这里在线进行唯品会授权！](http://www.zhetaoke.com/user/shouquan_vip.aspx)  
  

**请求参数说明：**

|  | 名称 | 类型 | 必填 | 说明 |
| --- | --- | --- | --- | --- |
|   | appkey | String | 是 | 折淘客对接秘钥appkey，折淘客通用。 |
|   | sid | String | 是 | 对应的唯品会账号授权SID，[点击这里查看SID！](http://www.zhetaoke.com/user/shouquan_vip.aspx) |
|   | status | String | 否 | 订单状态:0-不合格，1-待定，2-已完结，该参数不设置默认代表全部状态 |
|   | orderTimeStart | long | 否 | 订单时间起始 时间戳 单位毫秒 必须13位数字 |
|   | orderTimeEnd | long | 否 | 订单时间结束 时间戳 单位毫秒 必须13位数字 |
|   | page | int | 否 | 页码：从1开始 |
|   | pageSize | int | 否 | 页面大小：默认20 |
|   | updateTimeStart | long | 否 | 更新时间-起始 时间戳 单位毫秒 必须13位数字 |
|   | updateTimeEnd | long | 否 | 更新时间-结束 时间戳 单位毫秒 必须13位数字 |
|   | orderSnList | string | 否 | 订单号列表：当传入订单号列表时，订单时间和更新时间区间可不传入，可传入多个订单号，中间用英文逗号分隔。 |
|   | vendorCode | string | 否 | vendorCode,工具商方式下会传入 |
|   | chanTag | string | 否 | 渠道标识，即推广位PID |
|   |  |  |  | [点击查看详细返回字段说明！](https://vop.vip.com/home#/api/method/detail/com.vip.adp.api.open.service.UnionOrderV2Service-2.0.0/orderListWithOauth)  
[点击查看详细返回字段说明！](https://vop.vip.com/home#/api/method/detail/com.vip.adp.api.open.service.UnionOrderV2Service-2.0.0/orderListWithOauth)  
[点击查看详细返回字段说明！](https://vop.vip.com/home#/api/method/detail/com.vip.adp.api.open.service.UnionOrderV2Service-2.0.0/orderListWithOauth) |

**相关说明：**

|  | 名称 | 类型 | 说明 |
| --- | --- | --- | --- |
|   |

**JSON返回示例：**

```json
{
   
}
```

---

## 唯品会线报接口API

**接口名称：**唯品会线报接口API

**接口地址：**https://api.zhetaoke.com:10001/api/api\_xianbao.ashx

**返回数据：**json

**请求方式：**get

**请求示例：**https://api.zhetaoke.com:10001/api/api\_xianbao.ashx?appkey=&id=340319271&type=0&page=1&page\_size=10&msg=1

返回指定群编号的线报商品信息，主要是唯品会线报信息。  
  
[点击这里在线进行唯品会授权！点击这里在线进行唯品会授权！点击这里在线进行唯品会授权！](http://www.zhetaoke.com/user/shouquan_vip.aspx)  
  

**请求参数说明：**

|  | 名称 | 类型 | 必填 | 说明 |
| --- | --- | --- | --- | --- |
|   | appkey | String | 是 | 折淘客对接秘钥appkey，折淘客通用。 |
|   | page | String | 否 | 分页获取数据,第几页 |
|   | page\_size | String | 否 | 每页数据条数（默认每页20条），可自定义1-50之间 |
|   | id | String | 否 | 群号：340319271，折淘客线报群10（主唯品会）  
返回指定群最近24小时内的线报数据。 可以每隔5秒-10秒循环调用该接口，发现新数据后，保存到本地即可。 本接口，严禁开多线程进行调用。本接口有5秒缓存，开多线程也没用。 状态返回301表示，最近24小时内无新数据！ |
|   | type | String | 否 | type=0，表示qq群，type=1，表示wx群 |
|   | msg | String | 否 | msg=1，表示返回详细线报数据内容。 |

**相关说明：**

|  | 名称 | 类型 | 说明 |
| --- | --- | --- | --- |
|   |

**JSON返回示例：**

```json
{
    "status":200,
    "msg":\[
        {
            "add\_time":"2020/08/04 16:19:53",   /\*添加时间\*/
            "type":"0",   /\*类型，0表示qq群\*/
            "id":"43104737",   /\*群号\*/
            "content":"大宝SOD密】面霜润肤乳100mL",   /\*线报文案\*/
            "code":"264"   /\*自动增长列，不会重复\*/
        },
        {
            "add\_time":"2020/08/04 16:19:15",
            "type":"0",
            "id":"43104737",
            "content":"【晴雨两用】防紫外线折叠伞",
            "code":"262"
        }
    \]
}
```

---

## 唯品会精编商品文案API

**接口名称：**唯品会精编商品文案API

**接口地址：**https://api.zhetaoke.com:10001/api/open\_vip\_wenan\_list.ashx

**返回数据：**json

**请求方式：**get

**请求示例：**https://api.zhetaoke.com:10001/api/open\_vip\_wenan\_list.ashx?appkey=&page=&page\_size=&sort=new

获取唯品会精编商品文案，主要是球鞋、衣服类精编社群文案。  
  
[点击这里在线进行唯品会授权！点击这里在线进行唯品会授权！点击这里在线进行唯品会授权！](http://www.zhetaoke.com/user/shouquan_vip.aspx)  
  

**请求参数说明：**

|  | 名称 | 类型 | 必填 | 说明 |
| --- | --- | --- | --- | --- |
|   | appkey | String | 是 | 折淘客对接秘钥appkey，折淘客通用。 |
|   | page | String | 否 | 分页获取数据,第几页 |
|   | page\_size | String | 否 | 每页数据条数（默认每页20条），可自定义1-50之间 |
|   | sort | String | 否 | 商品排序方式，  
new：按照综合排序，  
code：按照code值从大到小排序，  
date\_time：按照更新时间排序，  
random：按照随机排序 |
|   | total\_count | String | 否 | 是否获取商品总数，  
1：返回商品总数，  
不返回商品总数，请不要拼接该参数。 |

**相关说明：**

|  | 名称 | 类型 | 说明 |
| --- | --- | --- | --- |
|   |

**JSON返回示例：**

```json
{
    "status":200,
    "content":\[
        {
            "code":"28",   /\*自动增长列，不会重复\*/
            "add\_time":"2021/6/24 12:04:50",   /\*添加时间\*/
            "content":"\\u0037\\u0039\\u5143\\u0020\\u0055\\u002e\\u0053",   /\*精编文案\*/
            "id":"6919325925298524997",   /\*商品id\*/
            "pic":"https://a.vpimg4.com/5/0\_50.jpg",   /\*商品图片地址\*/
            "shop\_name":"U.S. POLO ASSN.",   /\*品牌名称\*/
            "quanhou\_price":"59"   /\*券后价格\*/
        },
        {
            "code":"27",
            "add\_time":"2021/6/24 11:47:02",
            "content":"\\u0036\\u0038\\u5143\\u0020\\u62d3\\u8def\\u8005",
            "id":"6918929865129640277",
            "pic":"https://a.vpimg2.com/upload/merchandise/pdcvis/612262/2020/082.jpg",
            "shop\_name":"拓路者",
            "quanhou\_price":"59"
        },
        {
            "code":"26",
            "add\_time":"2021/6/24 11:44:14",
            "content":"\\u0038\\u0039\\u5143\\u0020\\u610f\\u5c14\\u5eb7",
            "id":"6919220045206075782",
            "pic":"https://a.vpimg3.com/upload/merchandise/pdcvis/2021/04/27/.jpg",
            "shop\_name":"意尔康",
            "quanhou\_price":"59"
        }
    \]
}
```

---

## 唯品会关键词查询商品API

**接口名称：**唯品会关键词查询商品API

**接口地址：**https://api.zhetaoke.com:10001/api/open\_vip\_queryWithOauth.ashx

**返回数据：**json

**请求方式：**get

**请求示例：**https://api.zhetaoke.com:10001/api/open\_vip\_queryWithOauth.ashx?appkey=&sid=&page=&pageSize=&keyword=球鞋

根据关键字查询唯品会商品列表，支持指定排序字段。  
  
[点击这里在线进行唯品会授权！点击这里在线进行唯品会授权！点击这里在线进行唯品会授权！](http://www.zhetaoke.com/user/shouquan_vip.aspx)  
  

**请求参数说明：**

|  | 名称 | 类型 | 必填 | 说明 |
| --- | --- | --- | --- | --- |
|   | appkey | String | 是 | 折淘客对接秘钥appkey，折淘客通用。 |
|   | sid | String | 是 | 对应的唯品会账号授权SID，[点击这里查看SID！](http://www.zhetaoke.com/user/shouquan_vip.aspx) |
|   | keyword | String | 是 | 搜索关键词 |
|   | fieldName | String | 否 | 价格排序：PRICE，DISCOUNT：折扣排序，销量排序：SALES |
|   | order | String | 否 | 排序顺序：0-正序，1-逆序，默认正序 |
|   | page | String | 否 | 分页获取数据,第几页 |
|   | pageSize | String | 否 | 每页数据条数（默认每页20条），可自定义1-50之间 |
|   | priceStart | String | 否 | 价格区间---start |
|   | priceEnd | String | 否 | 价格区间---end |

**相关说明：**

|  | 名称 | 类型 | 说明 |
| --- | --- | --- | --- |
|   |

**JSON返回示例：**

```json
{
    "returnCode":"0",
    "result":{
        "total":3250,   /\*商品总数\*/
        "goodsInfoList":\[
            {
                "marketPrice":"339.00",   /\*市场价（元）\*/
                "commissionRate":"3",   /\*佣金比例（%）\*/
                "goodsId":"6918901004295698011",   /\*商品id\*/
                "discount":"0.68",   /\*折扣:唯品价/市场价 保留两位小数字符串\*/
                "categoryName":"篮球鞋",   /\*商品三级分类\*/
                "haiTao":0,   /\*是否海淘商品标识：1是 0不是\*/
                "cat2ndName":"运动鞋",   /\*商品二级分类名称\*/
                "cat1stName":"运动户外",   /\*商品一级分类名称\*/
                "vipPrice":"179.00",   /\*唯品价（元）\*/
                "commission":"5.37",   /\*佣金金额（元）\*/
                "cat1stId":1056,   /\*商品一级分类id\*/
                "goodsName":"【爆款】夏季新款男鞋防滑耐磨篮球鞋",   /\*商品名称\*/
                "brandName":"乔丹",   /\*商品品牌名称\*/
                "brandLogoFull":"http://a.vpimg3.com/upload/brandcool/y.png",   /\*商品品牌logo全路径地址\*/
                "brandStoreSn":"10004065",   /\*商品品牌sn\*/
                "sellTimeFrom":1537864378000,   /\*商品售卖开始时间,时间戳，单位毫秒\*/
                "schemeStartTime":1622518497000,   /\*商品所属推广方案开始时间：时间戳，单位：毫秒\*/
                "schemeEndTime":2145888000000,   /\*商品所属推广方案结束时间：时间戳，单位：毫秒\*/
                "sourceType":0,   /\*商品类型：0-自营，1-MP\*/
                "sellTimeTo":1735660800000,   /\*商品售卖结束时间,时间戳, 单位毫秒\*/
                "brandId":1710614043,   /\*商品所属档期（专场）id\*/
                "goodsThumbUrl":"https://a.vpimg3.com/upload/merchandise/pdcvis/20.jpg",   /\*商品缩略图\*/
                "cat2ndId":384271,   /\*商品二级分类id\*/
                "spuId":"SPU-09BCC22080080507",   /\*商品spuId\*/
                "storeInfo":{
                    "storeName":"唯品自营",   /\*店铺名称\*/
                    "storeId":"ST00000"   /\*店铺id\*/
                },
                "goodsMainPicture":"https://a.vpimg2.com/upload/merchand8.jpg",   /\*商品主图\*/
                "destUrl":"https://m.vip.com/product-1710614043-6918901004295698011.html",   /\*商品落地页\*/
                "categoryId":384341,   /\*商品三级分类id\*/
                "status":1   /\*商品状态：0-下架，1-上架\*/
            },
            {
                "marketPrice":"299.00",
                "commissionRate":"3",
                "goodsId":"6918712848504084507",
                "discount":"0.46",
                "categoryName":"篮球鞋",
                "haiTao":0,
                "cat2ndName":"运动鞋",
                "cat1stName":"运动户外",
                "vipPrice":"137.00",
                "commission":"4.11",
                "cat1stId":1056,
                "goodsName":"【爆款】高帮减震耐磨防滑男款篮球鞋 实战篮球战靴 运动鞋男",
                "brandName":"乔丹",
                "brandLogoFull":"http://a.vpimg3.com/upload/brandcool/0/0fad8037af2243689b627c59792a7909/10004065/primary.png",
                "brandStoreSn":"10004065",
                "sellTimeFrom":1537864378000,
                "schemeStartTime":1622518497000,
                "schemeEndTime":2145888000000,
                "sourceType":0,
                "sellTimeTo":1735660800000,
                "brandId":1710614043,
                "goodsThumbUrl":"https://a.vpimg2.com/upload/merchandise/pdcvis/146645/2020/0326/193/940b9b42-1ba8-458f-b2aa-1649ec56f7c7\_750x750\_50.jpg",
                "cat2ndId":384271,
                "spuId":"SPU-06D0FBC080100287",
                "storeInfo":{
                    "storeName":"唯品自营",
                    "storeId":"ST00000"
                },
                "goodsMainPicture":"https://a.vpimg4.com/upload/merchandise/pdcvis/146645/2020/0326/193/940b9b42-1ba8-458f-b2aa-1649ec56f7c7.jpg",
                "destUrl":"https://m.vip.com/product-1710614043-6918712848504084507.html",
                "categoryId":384341,
                "status":1
            }
        \],
        "pageSize":2,
        "sortFields":\[
            {
                "fieldName":"PRICE",
                "fieldDesc":"价格排序"
            },
            {
                "fieldName":"DISCOUNT",
                "fieldDesc":"折扣排序"
            },
            {
                "fieldName":"SALES",
                "fieldDesc":"销量排序"
            }
        \],
        "page":1
    }
}
```

---

## 唯品会商品详情V2新API

**接口名称：**唯品会商品详情V2新API，可以查询详情信息、SKU信息、详情图信息、库存信息、评价信息、活动信息、优惠券红包信息等。

**接口地址：**https://api.zhetaoke.com:10001/api/open\_vip\_getByGoodsIdsV2WithOauth.ashx

**返回数据：**json

**请求方式：**get

**请求示例：**https://api.zhetaoke.com:10001/api/open\_vip\_getByGoodsIdsV2WithOauth.ashx?appkey=&sid=&id=&queryDetail=true&extendSku=true

根据唯品会商品id或者唯品会链接获取唯品会商品详情信息,V2版本接口，可获得sku等更多详情。  
  
[点击这里在线进行唯品会授权！点击这里在线进行唯品会授权！点击这里在线进行唯品会授权！](http://www.zhetaoke.com/user/shouquan_vip.aspx)  
  

**请求参数说明：**

|  | 名称 | 类型 | 必填 | 说明 |
| --- | --- | --- | --- | --- |
|   | appkey | String | 是 | 折淘客对接秘钥appkey，折淘客通用。 |
|   | sid | String | 是 | 对应的唯品会账号授权SID，[点击这里查看SID！](http://www.zhetaoke.com/user/shouquan_vip.aspx) |
|   | id | String | 是 | 唯品会商品id或者唯品会链接;  
商品id例子：6919173790682972930;  
唯品会链接例子：https://t.vip.com/URKiMYwbrW6 |
|   | queryDetail | String | 否 | 是否查询详情信息：此参数设为true，将查询出商品轮播图和详情图，默认为false |
|   | queryStock | String | 否 | 是否查询商品库存状态：默认为false |
|   | queryReputation | String | 否 | 是否查询商品评价信息：默认为false |
|   | queryStoreServiceCapability | String | 否 | 是否查询商品所属店铺服务能力信息：默认为false |
|   | queryPMSAct | String | 否 | 是否查询商品活动信息:默认为false |
|   | extendBySpu | String | 否 | 是否按照同spu扩展:此参数为true时，将根据传入的商品id扩展查询出同spu下的所有商品id，默认为false |
|   | queryExclusiveCoupon | String | 否 | 是否查询渠道专属红包信息： 默认不查询 |
|   | extendSku | String | 否 | 是否扩展查询商品sku信息: 此参数为true时，将根据传入的商品id扩展查询出对应的sku信息，默认为false。指定此参数查询较耗时，建议单个查询，并且业务上避免重复查询场景 |
|   |  |  |  | [点击这里查看官方接口说明！点击这里查看官方接口说明！点击这里查看官方接口说明！](https://vop.vip.com/home#/api/method/detail/com.vip.adp.api.open.service.UnionGoodsV2Service-2.0.0/getByGoodsIdsV2WithOauth)  
[点击这里查看官方接口说明！点击这里查看官方接口说明！点击这里查看官方接口说明！](https://vop.vip.com/home#/api/method/detail/com.vip.adp.api.open.service.UnionGoodsV2Service-2.0.0/getByGoodsIdsV2WithOauth)  
[点击这里查看官方接口说明！点击这里查看官方接口说明！点击这里查看官方接口说明！](https://vop.vip.com/home#/api/method/detail/com.vip.adp.api.open.service.UnionGoodsV2Service-2.0.0/getByGoodsIdsV2WithOauth)  
 |

**相关说明：**

|  | 名称 | 类型 | 说明 |
| --- | --- | --- | --- |
|   |

**JSON返回示例：**

```json
{
    "returnCode":"0",
    "result":\[
        {
            "marketPrice":"539.00",   /\*市场价（元）\*/
            "commissionRate":"5",   /\*佣金比例（%）\*/
            "goodsId":"6919173790682972930",   /\*商品id\*/
            "discount":"0.33",   /\*折扣:唯品价/市场价 保留两位小数字符串\*/
            "goodsCarouselPictures":\[   /\*商品轮播图：根据商品id查询时返回，商品列表不返回\*/
                "https://a.vpimg3.com/upload/merchandise/pdcvis/2021/05/26/128/0a3ccc7d-0fbf-47e1-83cd-e36942f05db7\_750x750\_75.jpg",
                "https://a.vpimg2.com/upload/merchandise/pdcvis/2021/05/26/24/edbf721a-a1a1-4e0e-91f6-1f5c1ea4dea5\_750x750\_75.jpg",
                "https://a.vpimg2.com/upload/merchandise/pdcvis/2021/05/26/126/48531ce7-5066-43af-bc04-d95dceb6af25\_750x750\_75.jpg",
                "https://a.vpimg2.com/upload/merchandise/pdcvis/2021/05/26/81/009d86a1-16af-4a30-bc85-ea57ed72c405\_750x750\_75.jpg",
                "https://a.vpimg4.com/upload/merchandise/pdcvis/2021/05/26/68/f3f05e76-d1ab-45dc-a8ee-cbc60d493dab\_750x750\_75.jpg",
                "https://a.vpimg3.com/upload/merchandise/pdcvis/2021/05/26/146/36bab0a3-b225-46e2-88c9-e7d85c0ef1f7\_750x750\_75.jpg",
                "https://a.vpimg2.com/upload/merchandise/pdcvis/2021/05/26/51/8082c3ff-8ed4-4bb9-9f60-8fea0ab3975d\_750x750\_75.jpg",
                "https://a.vpimg2.com/upload/merchandise/pdcvis/2021/05/26/3/1b7f3a46-e8cf-41b9-9672-d3e874d00785\_750x750\_75.jpg",
                "https://a.vpimg3.com/upload/merchandise/pdcvis/2021/05/26/112/ada1964a-3e4b-4aa9-876c-16c1c725a965\_750x750\_75.jpg",
                "https://a.vpimg3.com/upload/merchandise/pdcvis/2021/05/26/53/871a16a5-0fb3-47a2-a2af-f5a79e369970\_750x750\_75.jpg",
                "https://a.vpimg3.com/upload/merchandise/pdcvis/2021/05/26/95/484fb9be-2113-4019-bdc5-de08651bc33d\_750x750\_75.jpg",
                "https://a.vpimg3.com/upload/merchandise/pdcvis/2021/05/26/155/26a4d750-cbba-42ba-9696-e7ea446fd50a\_750x750\_75.jpg"
            \],
            "goodsDetailPictures":\[   /\*商品详情图片：根据商品id查询商品信息时返回，商品列表不返回\*/
                "https://a.vpimg2.com/upload/merchandise/pdcvis/2021/05/26/15/5e14907f-ea65-4d2a-9240-5ec8fd64a9f3.jpg",
                "https://a.vpimg2.com/upload/merchandise/pdcvis/2021/05/26/40/0bc22eff-767d-4441-b4f2-a9098173b4fa.jpg",
                "https://a.vpimg3.com/upload/merchandise/pdcvis/2021/05/26/147/170eb239-c82d-498a-bbac-35393279abde.jpg",
                "https://a.vpimg3.com/upload/merchandise/pdcvis/2021/05/26/86/db546029-c366-4a99-8d62-b9d54f53143b.jpg",
                "https://a.vpimg4.com/upload/merchandise/pdcvis/2021/05/26/37/6fd79f5a-3a30-42a2-9bc3-90c0bae8e664.jpg",
                "https://a.vpimg4.com/upload/merchandise/pdcvis/2021/05/26/190/6401eced-0d25-4472-b061-84b3854efd66.jpg",
                "https://a.vpimg3.com/upload/merchandise/pdcvis/2021/05/26/4/c845cade-361e-4c45-b2ef-ec2a521340c7.jpg",
                "https://a.vpimg4.com/upload/merchandise/pdcvis/2021/05/26/134/56599130-43f2-4975-b20f-16b63ab2fca9.jpg",
                "https://a.vpimg4.com/upload/merchandise/pdcvis/2021/05/26/94/3609256c-f40b-43ee-83d9-58579664d4be.jpg",
                "https://a.vpimg3.com/upload/merchandise/pdcvis/2021/05/26/2/7a38b543-13cb-4451-b7c0-b3a7eab8d82b.jpg",
                "https://a.vpimg4.com/upload/merchandise/pdcvis/2021/05/26/37/d672f3f0-82b1-4c98-9989-02e09b71d075.jpg",
                "https://a.vpimg2.com/upload/merchandise/pdcvis/2021/05/26/100/88a7045a-0618-483f-987d-9f37aff36112.jpg",
                "https://a.vpimg3.com/upload/merchandise/pdcvis/2021/05/26/164/fc2e7a78-b77d-4b93-9d19-206767aa4e83.jpg",
                "https://a.vpimg3.com/upload/merchandise/pdcvis/2021/05/26/71/1eb02600-b605-47ee-a59d-0e66e611f1dd.jpg"
            \],
            "categoryName":"男式T恤",   /\*商品三级分类\*/
            "haiTao":0,   /\*是否海淘商品标识：1是 0不是\*/
            "cat2ndName":"男上装",   /\*商品二级分类名称\*/
            "cat1stName":"男装",   /\*商品一级分类名称\*/
            "vipPrice":"81.00",   /\*唯品价（元）\*/
            "commission":"4.05",   /\*佣金金额（元）\*/
            "cat1stId":337,   /\*商品一级分类id\*/
            "goodsName":"【色织拼接】夏季新款商务休闲纯棉透气百搭条纹经典圆领T恤男",   /\*商品名称\*/
            "brandName":"杉杉",   /\*商品品牌名称\*/
            "brandLogoFull":"http://a.vpimg3.com/upload/brandcool/0/20.png",   /\*商品品牌logo全路径地址\*/
            "brandStoreSn":"10004760",   /\*商品品牌sn\*/
            "skuInfos":\[
                {
                    "sizeId":"6919173790682977026",   /\*商品尺码id(唯品会体系下和skuId等同)\*/
                    "marketPrice":"539",   /\*市场价:单位元\*/
                    "vipPrice":"99",   /\*唯品价:单位元\*/
                    "saleProps":{   /\*售卖属性信息（134 - 颜色,453 - 尺码）\*/
                        "453":"L",
                        "134":"藏青"
                    }
                },
                {
                    "sizeId":"6919173790699758338",
                    "marketPrice":"539",
                    "vipPrice":"99",
                    "saleProps":{
                        "453":"M",
                        "134":"藏青"
                    }
                },
                {
                    "sizeId":"6919173791473799938",
                    "marketPrice":"539",
                    "vipPrice":"99",
                    "saleProps":{
                        "453":"XL",
                        "134":"藏青"
                    }
                },
                {
                    "sizeId":"6919173792383464194",
                    "marketPrice":"539",
                    "vipPrice":"99",
                    "saleProps":{
                        "453":"XXL",
                        "134":"藏青"
                    }
                },
                {
                    "sizeId":"6919173792619131650",
                    "marketPrice":"539",
                    "vipPrice":"99",
                    "saleProps":{
                        "453":"XXXL",
                        "134":"藏青"
                    }
                },
                {
                    "sizeId":"6919173792770675458",
                    "marketPrice":"539",
                    "vipPrice":"99",
                    "saleProps":{
                        "453":"S",
                        "134":"藏青"
                    }
                }
            \],
            "sellTimeFrom":1537854724000,   /\*商品售卖开始时间,时间戳，单位毫秒\*/
            "schemeStartTime":1622518497000,   /\*商品所属推广方案开始时间：时间戳，单位：毫秒\*/
            "schemeEndTime":2145888000000,   /\*商品推广计划有效期预估截止时间：仅为预估时间，仅做参考；时间戳，单位：毫秒\*/
            "sourceType":0,   /\*商品类型：0-自营，1-MP\*/
            "sellTimeTo":1735660800000,   /\*商品售卖开始时间,时间戳，单位毫秒\*/
            "brandId":1710613538,   /\*商品所属档期（专场）id\*/
            "goodsThumbUrl":"https://a.vpimg2.com/upload/merchan50.jpg",   /\*商品缩略图\*/
            "cat2ndId":338,   /\*商品二级分类id\*/
            "spuId":"SPU-0BA84D10803804EF",   /\*商品spuId\*/
            "storeInfo":{
                "storeName":"唯品自营",   /\*店铺名称\*/
                "storeId":"ST00000"   /\*店铺id\*/
            },
            "goodsMainPicture":"https://a.vpimg4.com/uploa.jpg",   /\*商品主图\*/
            "destUrl":"https://m.vip.com/product-1710613538-6919173790682972930.html",   /\*商品落地页\*/
            "categoryId":340,   /\*商品三级分类id\*/
            "status":1   /\*商品状态：0-下架，1-上架\*/
        }
    \]
}
```

---

## 考拉转链API

**接口名称：**考拉转链API

**接口地址：**https://api.zhetaoke.com:10001/api/open\_kaola\_zhuanke\_api\_zhuanlian.ashx

**返回数据：**json

**请求方式：**get、post

**请求示例：**https://api.zhetaoke.com:10001/api/open\_kaola\_zhuanke\_api\_zhuanlian.ashx?appkey=&sid=&targetUrl=&trackingCode2=

根据考拉商品id或者考拉商品链接生成推广链接。  
  

**请求参数说明：**

|  | 名称 | 类型 | 必填 | 说明 |
| --- | --- | --- | --- | --- |
|   | appkey | String | 是 | 折淘客对接秘钥appkey，折淘客通用。 |
|   | sid | String | 是 | 对应的淘客账号授权SID |
|   | targetUrl | String | 是 | 考拉商品id或者考拉链接;  
商品id例子：6919173790682972930;  
考拉链接例子：http://lu.kaola.com/5DiMFU |
|   | trackingCode2 | String | 否 | 自定义参数  
只允许数字，最大长度11位  
此字段可以用来区分用户，进行下级返利。  
 |

**相关说明：**

|  | 名称 | 类型 | 说明 |
| --- | --- | --- | --- |
|   |

**JSON返回示例：**

```json
{
"code": 200,
"msg": "SUCCESS",
"data": \[
{
"originalLink": "http://lu.kaola.com/50uT0o",   /\*原始链接\*/
"shareLink": "https://cps.kaola.com/cps/zhuankeLogin?unionId=zo%3F\_\_da\_dad3e203\_5c4e7c4025b92c00",   /\*推广长链接\*/
"shortLink": "http://lu.kaola.com/1Sv5Fq"   /\*推广短链接\*/
}
\]
}
```

---

## 考拉精选商品列表API

**接口名称：**考拉精选商品列表API

**接口地址：**https://api.zhetaoke.com:10001/api/open\_kaola\_zhuanke\_api\_querySelectedGoods.ashx

**返回数据：**json

**请求方式：**get

**请求示例：**https://api.zhetaoke.com:10001/api/open\_kaola\_zhuanke\_api\_querySelectedGoods.ashx?appkey=&poolName=&pageNo=&pageSize=

获取考拉精选商品列表，只返回商品ID列表。  
  

**请求参数说明：**

|  | 名称 | 类型 | 必填 | 说明 |
| --- | --- | --- | --- | --- |
|   | appkey | String | 是 | 折淘客对接秘钥appkey，折淘客通用。 |
|   | poolName | String | 是 | 精选商品池名称，1-每日平价商品2-高佣必推商品3-新人专享商品4-会员专属商品5-低价包邮商品6-考拉自营爆款7-考拉商家爆款8-黑卡用户最爱买商品9-美妆个护热销品10-食品保健热销品11-母婴热销品12-时尚热销品13-家居宠物热销品14-每日秒杀商品15-黑卡好价商品16-高转化好物商品17-开卡一单省回商品18-会员专属促销选品池19-好券商品 |
|   | pageNo | String | 否 | 分页获取数据,第几页 |
|   | pageSize | String | 否 | 每页数据条数（默认每页20条），可自定义1-200之间 |

**相关说明：**

|  | 名称 | 类型 | 说明 |
| --- | --- | --- | --- |
|   |

**JSON返回示例：**

```json
{
"code": 200,
"msg": "SUCCESS",
"data": {
"goodsIdList": \[   /\*商品ID\*/
2480622,
5245545,
1631853,
9912153,
297474,
2904488,
1359520,
1651086,
38545,
9124380,
1577567,
28011,
9124376,
297078,
158563,
28619,
9067272,
1330333,
1577617,
6235455
\],
"pageNo": 1,
"pageSize": 20,
"totalRecord": 693
}
}
```

---

## 考拉关键词查询商品API

**接口名称：**考拉关键词查询商品API

**接口地址：**https://api.zhetaoke.com:10001/api/open\_kaola\_zhuanke\_api\_searchGoods.ashx

**返回数据：**json

**请求方式：**get

**请求示例：**https://api.zhetaoke.com:10001/api/open\_kaola\_zhuanke\_api\_searchGoods.ashx?appkey=&type=&desc=&pageNo=&pageSize=&keyWord=球鞋

根据关键字查询考拉商品列表，支持指定排序字段。  
  

**请求参数说明：**

|  | 名称 | 类型 | 必填 | 说明 |
| --- | --- | --- | --- | --- |
|   | appkey | String | 是 | 折淘客对接秘钥appkey，折淘客通用。 |
|   | keyWord | String | 是 | 搜索关键词 |
|   | type | String | 否 | 排序方式 0 综合排序 1 价格排序 2 销量排序 10 佣金比例排序 默认0排序方式 |
|   | desc | String | 否 | 是否降序，只对价格排序方式有效，默认降序 true 降序 flase 升序 |
|   | pageNo | String | 否 | 分页获取数据,第几页 |
|   | pageSize | String | 否 | 每页数据条数（默认每页20条），可自定义1-50之间 |

**相关说明：**

|  | 名称 | 类型 | 说明 |
| --- | --- | --- | --- |
|   |

**JSON返回示例：**

```json
{
"code": 200,
"msg": "SUCCESS",
"data": {
"dataList": \[
{
"activityInfo": {
"activityLable": "特价",
"noPostage": 0
},
"baseInfo": {
"brandCountryName": "美国",
"brandName": "NIKE 耐克",
"detailImgList": \[
"https://kaola-haitao.oss.kaolacdn.com/ea7645e88c24422a94ddc6ae5de690f91622726754081721AA9F9D98CDE2F2ADAE53CCC698617.jpg",
"https://kaola-haitao.oss.kaolacdn.com/3a089dc064bb4ed2b5c0272347f16cd11622726754105759892A471CF08AB205F714C501FA70B.jpg",
"https://kaola-haitao.oss.kaolacdn.com/3ea3dc13-b8ac-4913-abc1-439741bcbcd8.jpg?x-oss-process=image/crop,x\_0,y\_0,w\_890,h\_500",
"https://kaola-haitao.oss.kaolacdn.com/3ea3dc13-b8ac-4913-abc1-439741bcbcd8.jpg?x-oss-process=image/crop,x\_0,y\_500,w\_890,h\_500",
"https://kaola-haitao.oss.kaolacdn.com/3ea3dc13-b8ac-4913-abc1-439741bcbcd8.jpg?x-oss-process=image/crop,x\_0,y\_1000,w\_890,h\_500",
"https://kaola-haitao.oss.kaolacdn.com/3ea3dc13-b8ac-4913-abc1-439741bcbcd8.jpg?x-oss-process=image/crop,x\_0,y\_1500,w\_890,h\_500",
"https://kaola-haitao.oss.kaolacdn.com/3ea3dc13-b8ac-4913-abc1-439741bcbcd8.jpg?x-oss-process=image/crop,x\_0,y\_2000,w\_890,h\_500",
"https://kaola-haitao.oss.kaolacdn.com/3ea3dc13-b8ac-4913-abc1-439741bcbcd8.jpg?x-oss-process=image/crop,x\_0,y\_2500,w\_890,h\_500",
"https://kaola-haitao.oss.kaolacdn.com/3ea3dc13-b8ac-4913-abc1-439741bcbcd8.jpg?x-oss-process=image/crop,x\_0,y\_3000,w\_890,h\_500",
"https://kaola-haitao.oss.kaolacdn.com/3ea3dc13-b8ac-4913-abc1-439741bcbcd8.jpg?x-oss-process=image/crop,x\_0,y\_3500,w\_890,h\_500",
"https://kaola-haitao.oss.kaolacdn.com/3ea3dc13-b8ac-4913-abc1-439741bcbcd8.jpg?x-oss-process=image/crop,x\_0,y\_4000,w\_890,h\_500",
"https://kaola-haitao.oss.kaolacdn.com/3ea3dc13-b8ac-4913-abc1-439741bcbcd8.jpg?x-oss-process=image/crop,x\_0,y\_4500,w\_890,h\_500",
"https://kaola-haitao.oss.kaolacdn.com/3ea3dc13-b8ac-4913-abc1-439741bcbcd8.jpg?x-oss-process=image/crop,x\_0,y\_5000,w\_890,h\_500"
\],
"goodsSubTitle": "耐克女子休闲鞋。该款鞋采用了经典的白色鞋身+红色LOGO的撞色设计。干净又夺人眼球。鞋型经典轻便透气设计，女性出街好伙伴。 ",
"goodsTitle": "【考拉自营】NIKE 耐克小空一WMNS NIKE COURT VISION LO 女子休闲鞋CD5434",
"groupBuyGoods": 0,
"imageList": \[
"https://kaola-haitao.oss.kaolacdn.com/88533cfc83a7a7834b2895842392c632.jpg",
"https://kaola-haitao.oss.kaolacdn.com/3a48ad7fce2081ab5f110185db5ce9d6.jpg",
"https://kaola-haitao.oss.kaolacdn.com/e968a1e0-b68b-4ffa-9798-3d7d10edba5e.jpg",
"https://kaola-haitao.oss.kaolacdn.com/1cb366542bee5177b4d86c8372a6622a.jpg",
"https://kaola-haitao.oss.kaolacdn.com/8510f1523c3a33122932c5ed6fec05b3.jpg",
"https://kaola-haitao.oss.kaolacdn.com/f79f45d261b28243cf7c42f89e59e9f8.jpg"
\],
"importType": 3,
"interPurch": 0,
"onlineStatus": 1,
"self": 1,
"store": 1
},
"categoryInfo": \[
{
"categoryId": 7578,
"categoryName": "运动户外",
"level": 2
},
{
"categoryId": 12305,
"categoryName": "运动用品",
"level": 3
},
{
"categoryId": 7584,
"categoryName": "运动鞋",
"level": 4
},
{
"categoryId": 7991,
"categoryName": "休闲鞋",
"level": 5
}
\],
"commissionInfo": {
"commissionRate": 0.032,
"expireType": 1
},
"depositInfo": {
"deposit": false,
"depositEndTime": 0,
"depositStartTime": 0,
"payEndTime": 0,
"payStartTime": 0
},
"goodsId": 8434647,
"linkInfo": {
"goodsDetailUrl": "https://m-goods.kaola.com/product/8434647.html",
"goodsPCUrl": "https://goods.kaola.com/product/8434647.html",
"miniShareUrl": "https://cps.kaola.com/cps/mini/zhuankeLogin?unionId=zhuanke\_701339384&showWapBanner=0&rd=259200000&targetUrl=https%3A%2F%2Fm-goods.kaola.com%2Fproduct%2F8434647.html%3F\_\_da\_dad3e203\_5c4e7c4025b92c00",
"shareUrl": "https://cps.kaola.com/cps/zhuankeLogin?unionId=zhuanke\_701339384&showWapBanner=0&targetUrl=https%3A%2F%2Fm-goods.kaola.com%2Fproduct%2F8434647.html%3F\_\_da\_dad3e203\_5c4e7c4025b92c00"
},
"priceInfo": {
"currentPrice": 399,
"marketPrice": 549,
"memberCurrentPrice": 331.55,
"memberPriceSpread": 67.45
},
"promotionInfo": \[
{
"activityGoodsSellCount": 0,
"activityGoodsStore": 0,
"applyUserType": 0,
"endTime": "2022-01-15 23:59:59",
"goodsId": 8434647,
"promotionSalePrice": 399,
"skuId": "20594332480",
"startTime": "2022-01-09 20:00:00",
"storeLimit": false
}
\]
}
\],
"pageNo": 1,
"pageSize": 1,
"total": 2195
}
}
```

---

## 考拉商品详情API

**接口名称：**考拉商品详情API，可以查询详情信息等。

**接口地址：**https://api.zhetaoke.com:10001/api/open\_kaola\_zhuanke\_api\_queryGoodsInfo.ashx

**返回数据：**json

**请求方式：**get

**请求示例：**https://api.zhetaoke.com:10001/api/open\_kaola\_zhuanke\_api\_queryGoodsInfo.ashx?appkey=&goodsIds=

根据考拉商品id获取考拉商品详情信息。  
  

**请求参数说明：**

|  | 名称 | 类型 | 必填 | 说明 |
| --- | --- | --- | --- | --- |
|   | appkey | String | 是 | 折淘客对接秘钥appkey，折淘客通用。 |
|   | goodsIds | String | 是 | 考拉商品id;  
商品id例子：6919173790682972930; |

**相关说明：**

|  | 名称 | 类型 | 说明 |
| --- | --- | --- | --- |
|   |

**JSON返回示例：**

```json
{
"code": 200,
"msg": "SUCCESS",
"data": \[
{
"activityInfo": {
"activityLable": "每满400减50,1件9折,包税,特价",
"noPostage": 0
},
"baseInfo": {
"brandCountryName": "美国",
"brandName": "vitafusion",
"detailImgList": \[
"https://kaola-haitao.oss.kaolacdn.com/c3341a74-32ae-4ae5-b16b-2fbdb3d52a7b.png?x-oss-process=image/crop,x\_0,y\_0,w\_960,h\_500",
"https://kaola-haitao.oss.kaolacdn.com/c3341a74-32ae-4ae5-b16b-2fbdb3d52a7b.png?x-oss-process=image/crop,x\_0,y\_500,w\_960,h\_500",
"https://kaola-haitao.oss.kaolacdn.com/393adaa7-e9d6-4095-ae81-0aadb13bd86b.jpg?x-oss-process=image/crop,x\_0,y\_0,w\_750,h\_500",
"https://kaola-haitao.oss.kaolacdn.com/393adaa7-e9d6-4095-ae81-0aadb13bd86b.jpg?x-oss-process=image/crop,x\_0,y\_500,w\_750,h\_500",
"https://kaola-haitao.oss.kaolacdn.com/393adaa7-e9d6-4095-ae81-0aadb13bd86b.jpg?x-oss-process=image/crop,x\_0,y\_1000,w\_750,h\_500",
"https://kaola-haitao.oss.kaolacdn.com/393adaa7-e9d6-4095-ae81-0aadb13bd86b.jpg?x-oss-process=image/crop,x\_0,y\_1500,w\_750,h\_500",
"https://kaola-haitao.oss.kaolacdn.com/393adaa7-e9d6-4095-ae81-0aadb13bd86b.jpg?x-oss-process=image/crop,x\_0,y\_2000,w\_750,h\_500",
"https://kaola-haitao.oss.kaolacdn.com/393adaa7-e9d6-4095-ae81-0aadb13bd86b.jpg?x-oss-process=image/crop,x\_0,y\_2500,w\_750,h\_500",
"https://kaola-haitao.oss.kaolacdn.com/393adaa7-e9d6-4095-ae81-0aadb13bd86b.jpg?x-oss-process=image/crop,x\_0,y\_3000,w\_750,h\_500",
"https://kaola-haitao.oss.kaolacdn.com/393adaa7-e9d6-4095-ae81-0aadb13bd86b.jpg?x-oss-process=image/crop,x\_0,y\_3500,w\_750,h\_500",
"https://kaola-haitao.oss.kaolacdn.com/393adaa7-e9d6-4095-ae81-0aadb13bd86b.jpg?x-oss-process=image/crop,x\_0,y\_4000,w\_750,h\_500",
"https://kaola-haitao.oss.kaolacdn.com/393adaa7-e9d6-4095-ae81-0aadb13bd86b.jpg?x-oss-process=image/crop,x\_0,y\_4500,w\_750,h\_500",
"https://kaola-haitao.oss.kaolacdn.com/393adaa7-e9d6-4095-ae81-0aadb13bd86b.jpg?x-oss-process=image/crop,x\_0,y\_5000,w\_750,h\_500",
"https://kaola-haitao.oss.kaolacdn.com/393adaa7-e9d6-4095-ae81-0aadb13bd86b.jpg?x-oss-process=image/crop,x\_0,y\_5500,w\_750,h\_500",
"https://kaola-haitao.oss.kaolacdn.com/393adaa7-e9d6-4095-ae81-0aadb13bd86b.jpg?x-oss-process=image/crop,x\_0,y\_6000,w\_750,h\_500",
"https://kaola-haitao.oss.kaolacdn.com/393adaa7-e9d6-4095-ae81-0aadb13bd86b.jpg?x-oss-process=image/crop,x\_0,y\_6500,w\_750,h\_500",
"https://kaola-haitao.oss.kaolacdn.com/393adaa7-e9d6-4095-ae81-0aadb13bd86b.jpg?x-oss-process=image/crop,x\_0,y\_7000,w\_750,h\_500",
"https://kaola-haitao.oss.kaolacdn.com/393adaa7-e9d6-4095-ae81-0aadb13bd86b.jpg?x-oss-process=image/crop,x\_0,y\_7500,w\_750,h\_500",
"https://kaola-haitao.oss.kaolacdn.com/393adaa7-e9d6-4095-ae81-0aadb13bd86b.jpg?x-oss-process=image/crop,x\_0,y\_8000,w\_750,h\_500",
"https://kaola-haitao.oss.kaolacdn.com/393adaa7-e9d6-4095-ae81-0aadb13bd86b.jpg?x-oss-process=image/crop,x\_0,y\_8500,w\_750,h\_500",
"https://kaola-haitao.oss.kaolacdn.com/393adaa7-e9d6-4095-ae81-0aadb13bd86b.jpg?x-oss-process=image/crop,x\_0,y\_9000,w\_750,h\_500",
"https://kaola-haitao.oss.kaolacdn.com/393adaa7-e9d6-4095-ae81-0aadb13bd86b.jpg?x-oss-process=image/crop,x\_0,y\_9500,w\_750,h\_500",
"https://kaola-haitao.oss.kaolacdn.com/393adaa7-e9d6-4095-ae81-0aadb13bd86b.jpg?x-oss-process=image/crop,x\_0,y\_10000,w\_750,h\_500",
"https://kaola-haitao.oss.kaolacdn.com/393adaa7-e9d6-4095-ae81-0aadb13bd86b.jpg?x-oss-process=image/crop,x\_0,y\_10500,w\_750,h\_500",
"https://kaola-haitao.oss.kaolacdn.com/393adaa7-e9d6-4095-ae81-0aadb13bd86b.jpg?x-oss-process=image/crop,x\_0,y\_11000,w\_750,h\_500",
"https://kaola-haitao.oss.kaolacdn.com/393adaa7-e9d6-4095-ae81-0aadb13bd86b.jpg?x-oss-process=image/crop,x\_0,y\_11500,w\_750,h\_500",
"https://kaola-haitao.oss.kaolacdn.com/393adaa7-e9d6-4095-ae81-0aadb13bd86b.jpg?x-oss-process=image/crop,x\_0,y\_12000,w\_750,h\_500",
"https://kaola-haitao.oss.kaolacdn.com/393adaa7-e9d6-4095-ae81-0aadb13bd86b.jpg?x-oss-process=image/crop,x\_0,y\_12500,w\_750,h\_500",
"https://kaola-haitao.oss.kaolacdn.com/393adaa7-e9d6-4095-ae81-0aadb13bd86b.jpg?x-oss-process=image/crop,x\_0,y\_13000,w\_750,h\_500",
"https://kaola-haitao.oss.kaolacdn.com/393adaa7-e9d6-4095-ae81-0aadb13bd86b.jpg?x-oss-process=image/crop,x\_0,y\_13500,w\_750,h\_500",
"https://kaola-haitao.oss.kaolacdn.com/393adaa7-e9d6-4095-ae81-0aadb13bd86b.jpg?x-oss-process=image/crop,x\_0,y\_14000,w\_750,h\_500",
"https://kaola-haitao.oss.kaolacdn.com/393adaa7-e9d6-4095-ae81-0aadb13bd86b.jpg?x-oss-process=image/crop,x\_0,y\_14500,w\_750,h\_500",
"https://kaola-haitao.oss.kaolacdn.com/393adaa7-e9d6-4095-ae81-0aadb13bd86b.jpg?x-oss-process=image/crop,x\_0,y\_15000,w\_750,h\_500",
"https://kaola-haitao.oss.kaolacdn.com/393adaa7-e9d6-4095-ae81-0aadb13bd86b.jpg?x-oss-process=image/crop,x\_0,y\_15500,w\_750,h\_500"
\],
"goodsSubTitle": " ",
"goodsTitle": "vitafusion 褪黑素软糖 60粒/瓶\*3瓶",
"groupBuyGoods": 0,
"imageList": \[
"https://kaola-haitao.oss.kaolacdn.com/3e2fb0b6-e829-4f58-b4af-e4c274e0a231.jpg",
"https://kaola-haitao.oss.kaolacdn.com/244b63b9-2913-46ef-8bb1-febf2de53f6b.jpg",
"https://kaola-haitao.oss.kaolacdn.com/c4a10d19-c56e-4a86-b0d6-74d86cd43ae6.jpg",
"https://kaola-haitao.oss.kaolacdn.com/398bf4a6-301c-4ba4-8ed7-6ddadb9207b6.jpg"
\],
"importType": 1,
"interPurch": 0,
"onlineStatus": 1,
"self": 1,
"store": 1
},
"categoryInfo": \[
{
"categoryId": 837,
"categoryName": "医药健康",
"level": 2
},
{
"categoryId": 12296,
"categoryName": "营养成分",
"level": 3
},
{
"categoryId": 1574,
"categoryName": "蛋白质/氨基酸",
"level": 4
},
{
"categoryId": 12997,
"categoryName": "褪黑素",
"level": 5
}
\],
"commissionInfo": {
"commissionRate": 0.172,
"endTime": "2022-01-15 23:59:59",
"expireType": 0,
"startTime": "2022-01-07 00:00:00"
},
"depositInfo": {
"deposit": false,
"depositEndTime": 0,
"depositStartTime": 0,
"payEndTime": 0,
"payStartTime": 0
},
"goodsId": 2480622,
"linkInfo": {
"goodsDetailUrl": "https://m-goods.kaola.com/product/2480622.html",
"goodsPCUrl": "https://goods.kaola.com/product/2480622.html",
"miniShareUrl": "https://cps.kaola.com/cps/mini/zhuankeLogin?unionId=zhuanke\_701339384&showWapBanner=0&rd=259200000&targetUrl=https%3A%2F%2Fm-goods.kaola.com%2Fproduct%2F2480622.html%3F\_\_da\_dad3e203\_5c4e7c4025b92c00",
"shareUrl": "https://cps.kaola.com/cps/zhuankeLogin?unionId=zhuanke\_701339384&showWapBanner=0&targetUrl=https%3A%2F%2Fm-goods.kaola.com%2Fproduct%2F2480622.html%3F\_\_da\_dad3e203\_5c4e7c4025b92c00",
"shortShareUrl": "http://lu.kaola.com/3tmWKg"
},
"priceInfo": {
"currentPrice": 192,
"marketPrice": 429,
"memberCurrentPrice": 164.16,
"memberPriceSpread": 27.84
},
"promotionInfo": \[
{
"activityGoodsSellCount": 0,
"activityGoodsStore": 0,
"applyUserType": 0,
"endTime": "2022-01-15 23:59:59",
"goodsId": 2480622,
"promotionSalePrice": 192,
"skuId": "20024469581",
"startTime": "2022-01-09 20:00:00",
"storeLimit": false
}
\]
}
\]
}
```

---

## 考拉订单查询API

**接口名称：**考拉订单查询API（折淘客联盟结算佣金的所有订单，都可以通过此接口查询。）

**接口地址：**https://api.zhetaoke.com:10001/api/open\_lianmeng\_orderList.ashx

**返回数据：**json

**请求方式：**get

**请求示例：**https://api.zhetaoke.com:10001/api/open\_lianmeng\_orderList.ashx?appkey=&type=&startTime=&endTime=

根据订单支付时间或者订单更新时间获取最近90天内的折淘客联盟订单数据  
支持按支付时间、更新时间查询。  
建议按照订单支付时间，每分钟查询一次，按照订单更新时间，每分钟查询一次即可。  
个别长时间不结算的订单可以单独用订单编号来查询。  
  
[点击这里在线查询折淘客联盟订单信息！点击这里在线查询折淘客联盟订单信息！点击这里在线查询折淘客联盟订单信息！](/user/extend/extend_app_dingdan_lianmeng.aspx)  
  

**请求参数说明：**

|  | 名称 | 类型 | 必填 | 说明 |
| --- | --- | --- | --- | --- |
|   | appkey | String | 是 | 折淘客对接秘钥appkey，折淘客通用。 |
|   | type | String | 是 | 时间类型，1表示按照订单支付时间获取，2表示按照订单更新时间获取，3表示按照订单编号获取。 |
|   | page | String | 是 | 页码 |
|   | page\_size | String | 是 | 每页数量，单页数最大50，默认50 |
|   | startTime | String | 否 | 订单开始时间,例子：2021-04-30 08:00:00 |
|   | endTime | String | 否 | 订单结束时间,例子：2021-04-30 09:00:00 |
|   | orderid | String | 否 | 订单编号，必须输入完整的订单编号。 |
|   | sid | String | 否 | 折淘客授权sid，无值，表示获取appkey下所有折淘客联盟订单。 |
|   | san\_pingtai\_id | String | 否 | 订单所属平台id，1美团、2考拉、3苏宁、4淘宝、5京东、6拼多多、7唯品会、8饿了么、9抖音等 |

**相关说明：**

|  | 名称 | 类型 | 说明 |
| --- | --- | --- | --- |
|   |

**JSON返回示例：**

```json
{
    "status":200,
    "content":\[
        {
            "code":"15",      /\*折淘客自动增长列\*/
            "account":"83076790@qq.com",      /\*折淘客账号\*/
            "appkey\_ztk":"a2a33cacfb1f4005b818ffd0ab7c951d",      /\*折淘客appkey\*/
            "sid":"16ztk",   /\*原始推广位sid\*/
            "is\_jiesuan":"0",     /\*折淘客是否结算，0未结算，1已结算\*/
            "jiesuan\_time":"",      /\*折淘客结算时间\*/
            "jiesuan\_profit":"1.16",      /\*折淘客结算金额\*/
            "orderid":"105518514197820013",   /\*订单编号\*/
            "paytime":"2021/04/27 12:22:09",      /\*订单支付时间\*/
            "payprice":"21.50",   /\*订单实际支付金额\*/
            "profit":1.29,    /\*预估佣金\*/
            "smstitle":"霸碗盖码饭（凤凰大厦店）",    /\*订单标题\*/
            "refundprice":0,      /\*退款金额\*/
            "refundtime":"",      /\*退款时间\*/
            "status":"8",     /\*订单状态，1已付款，8已完成，9已退款或风控\*/
            "update\_time":"",   /\*数据最后更新时间\*/
            "type":"4",   /\*订单类型\*/
            "sid\_ztk":"16",   /\*折淘客授权sid\*/
            "customer\_id\_ztk":"100000",   /\*推广者自定义编号\*/
            "platform\_ztk":"zhetaoke",     /\*推广者平台类型，默认zhetaoke\*/
            "actId":"33",   /\*活动id\*/
            "san\_pingtai":"美团",   /\*订单所属平台,考拉、美团、苏宁等\*/
            "san\_pingtai\_id":"1"   /\*订单所属平台id，1美团、2考拉、3苏宁、4淘宝、5京东、6拼多多、7唯品会、8饿了么、9抖音等\*/
        },
        {
            "code":"14",
            "account":"83076790@qq.com",
            "appkey\_ztk":"a2a33cacfb1f4005b818ffd0ab7c951d",
            "sid":"16ztkzheta",
            "is\_jiesuan":"0",
            "jiesuan\_time":null,
            "jiesuan\_profit":"1.16",
            "orderid":"105518514197820012",
            "paytime":"2021/04/27 12:22:09",
            "payprice":"21.50",
            "profit":1.29,
            "smstitle":"霸碗盖码饭（凤凰大厦店）",
            "refundprice":0,
            "refundtime":"",
            "status":"8",
            "update\_time":null,
            "type":"4",
            "sid\_ztk":"16",
            "customer\_id\_ztk":"100000",
            "platform\_ztk":"zhetaoke"，
            "actId":"33",
            "san\_pingtai":"美团",
            "san\_pingtai\_id":"1"
        }
    \]
}
```

---

## 饿了么转链API

**接口名称：**饿了么转链API

**接口地址：**https://api.zhetaoke.com:10001/api/open\_eleme\_generateLink.ashx

**返回数据：**json

**请求方式：**get/post,请尽量使用post。

**请求示例：**https://api.zhetaoke.com:10001/api/open\_eleme\_generateLink.ashx?appkey=&sid=&activity\_id=1&customer\_id=

**请求参数说明：**

|  | 名称 | 类型 | 必填 | 说明 |
| --- | --- | --- | --- | --- |
|   | appkey | String | 是 | 折淘客对接秘钥appkey，折淘客通用。 |
|   | sid | string | 是 | 对应的淘客账号授权SID |
|   | activity\_id | string | 是 | 饿了么相关活动id  
活动ID：10144，饿了么天天领红包，最高抢66元大额红包，预估佣金收益6%。（重点推广！重点推广！重点推广！）  
活动ID：11862，限时快闪，最高领18元红包。（重点推广！重点推广！重点推广！）  
活动ID：12819，闪购天天领红包,预估佣金收益6%。（重点推广！重点推广！重点推广！）  
活动ID：12628，淘宝闪购，领最高15元红包，预估佣金收益6%。  
活动ID：12688，来闪购一下 可赢免单福利，闪购大额满减红包，预估佣金收益6%。  
活动ID：12918，淘宝闪购惊喜福利，1分钱喝奶茶，预估佣金收益6%。  
更多饿了么活动，请参考接口：http://api.zhetaoke.com:10000/api/api\_activity.ashx?appkey=&activityId=&type=11  
更多饿了么活动，请参考接口：http://api.zhetaoke.com:10000/api/api\_activity.ashx?appkey=&activityId=&type=11  
更多饿了么活动，请参考接口：http://api.zhetaoke.com:10000/api/api\_activity.ashx?appkey=&activityId=&type=11  
 |
|   | customer\_id | string | 否 | 自定义参数  
只允许数字，最大长度11位。  
此字段可以用来区分用户，进行下级返利。  
 |

**相关说明：**

|  | 名称 | 类型 | 说明 |
| --- | --- | --- | --- |

**JSON返回示例：**

```json
{
                                  "alibaba\_alsc\_union\_eleme\_promotion\_officialactivity\_get\_response": {
                                    "data": {
                                      "description": "最高抢66元大额红包",      /\*描述\*/
                                      "end\_time": 1714492799,       /\*结束时间（秒）\*/
                                      "id": "10144",       /\*活动ID\*/
                                      "link": {
                                        "alipay\_mini\_url": "alipays://platformapi/startapp?appId=2021001110676437&page=plugin36a5c6c77",       /\*支付宝小程序推广链接\*/
                                        "ele\_scheme\_url": "eleme://web?action=ali.open.nav&module=h5&fastmode=1",       /\*饿了么唤端链接\*/
                                        "h5\_short\_link": "https://u.ele.me/Rel9JsB9",       /\*h5推广地址短链\*/
                                        "h5\_url": "https://fc.ele.me/a/MWJhZDM1OTY5ZTg5MTFlYzllYWEwMDE2M2UxM2FkMTE=?scene=a9344de7debe4a779e8474336a5c6c77",       /\*h5推广地址\*/
                                        "mini\_qrcode": "https://img.alicdn.com/imgextra/i2/6000000000582/O1CN01VV9Kyu1GAbKKcfXay\_!!6000000000582-0-o2oad.jpg",       /\*微信独立二维码\*/
                                        "picture": "https://img.alicdn.com/imgextra/i3/6000000001663/O1CN01sh4YHs1O9hTZPG2qd\_!!6000000001663-0-o2oad.jpg",       /\*推广图片地址\*/
                                        "tb\_mini\_qrcode": "https://img.alicdn.com/imgextra/i2/6000000006790/O1CN01mt8XTZ201s6b2q5Qv\_!!6000000006790-0-o2oad.jpg",       /\*淘宝独立二维码\*/
                                        "tb\_qr\_code": "https://img.alicdn.com/imgextra/i4/6000000001701/O1CN01gTvVPk1OR6WaAxeXH\_!!6000000001701-0-o2oad.jpg",       /\*淘宝二维码图片地址\*/
                                        "wx\_appid": "wxece3a9a4c82f58c9",       /\*小程序appId\*/
                                        "wx\_path": "commercialize/pages/taoke-guide/index?scene=a9344de7debe4a779e8474336a5c6c77"       /\*微信小程序path链接\*/
                                      },
                                      "picture": "https://img.alicdn.com/imgextra/i3/6000000001594/O1CN0187p2Km1Ne68biBOa5\_!!6000000001594-2-o2oad.png",       /\*活动创意图片\*/
                                      "start\_time": 1649865600,       /\*起始时间（秒）\*/
                                      "title": "饿了么天天领红包"       /\*标题\*/
                                    },
                                    "message": "success",
                                    "request\_id": "15s3jznf0in4t",
                                    "result\_code": 0
                                  }
                                }
```

---

## 饿了么订单查询API

**接口名称：**饿了么订单查询API（折淘客联盟结算佣金的所有订单，都可以通过此接口查询。）

**接口地址：**https://api.zhetaoke.com:10001/api/open\_lianmeng\_orderList.ashx

**返回数据：**json

**请求方式：**get

**请求示例：**https://api.zhetaoke.com:10001/api/open\_lianmeng\_orderList.ashx?appkey=&type=&startTime=&endTime=

根据订单支付时间或者订单更新时间获取最近90天内的折淘客联盟订单数据  
支持按支付时间、更新时间查询。  
建议按照订单支付时间，每分钟查询一次，按照订单更新时间，每分钟查询一次即可。  
个别长时间不结算的订单可以单独用订单编号来查询。  
  
[点击这里在线查询折淘客联盟订单信息！点击这里在线查询折淘客联盟订单信息！点击这里在线查询折淘客联盟订单信息！](/user/extend/extend_app_dingdan_lianmeng.aspx)  
  

**请求参数说明：**

|  | 名称 | 类型 | 必填 | 说明 |
| --- | --- | --- | --- | --- |
|   | appkey | String | 是 | 折淘客对接秘钥appkey，折淘客通用。 |
|   | type | String | 是 | 时间类型，1表示按照订单支付时间获取，2表示按照订单更新时间获取，3表示按照订单编号获取。 |
|   | page | String | 是 | 页码 |
|   | page\_size | String | 是 | 每页数量，单页数最大50，默认50 |
|   | startTime | String | 否 | 订单开始时间,例子：2021-04-30 08:00:00 |
|   | endTime | String | 否 | 订单结束时间,例子：2021-04-30 09:00:00 |
|   | orderid | String | 否 | 订单编号，必须输入完整的订单编号。 |
|   | sid | String | 否 | 折淘客授权sid，无值，表示获取appkey下所有折淘客联盟订单。 |
|   | san\_pingtai\_id | String | 否 | 订单所属平台id，1美团、2考拉、3苏宁、4淘宝、5京东、6拼多多、7唯品会、8饿了么、9抖音等 |

**相关说明：**

|  | 名称 | 类型 | 说明 |
| --- | --- | --- | --- |
|   |

**JSON返回示例：**

```json
{
    "status":200,
    "content":\[
        {
            "code":"15",      /\*折淘客自动增长列\*/
            "account":"83076790@qq.com",      /\*折淘客账号\*/
            "appkey\_ztk":"a2a33cacfb1f4005b818ffd0ab7c951d",      /\*折淘客appkey\*/
            "sid":"16ztk",   /\*原始推广位sid\*/
            "is\_jiesuan":"0",     /\*折淘客是否结算，0未结算，1已结算\*/
            "jiesuan\_time":"",      /\*折淘客结算时间\*/
            "jiesuan\_profit":"1.16",      /\*折淘客结算金额\*/
            "orderid":"105518514197820013",   /\*订单编号\*/
            "paytime":"2021/04/27 12:22:09",      /\*订单支付时间\*/
            "payprice":"21.50",   /\*订单实际支付金额\*/
            "profit":1.29,    /\*预估佣金\*/
            "smstitle":"霸碗盖码饭（凤凰大厦店）",    /\*订单标题\*/
            "refundprice":0,      /\*退款金额\*/
            "refundtime":"",      /\*退款时间\*/
            "status":"8",     /\*订单状态，1已付款，8已完成，9已退款或风控\*/
            "update\_time":"",   /\*数据最后更新时间\*/
            "type":"4",   /\*订单类型\*/
            "sid\_ztk":"16",   /\*折淘客授权sid\*/
            "customer\_id\_ztk":"100000",   /\*推广者自定义编号\*/
            "platform\_ztk":"zhetaoke",     /\*推广者平台类型，默认zhetaoke\*/
            "actId":"33",   /\*活动id\*/
            "san\_pingtai":"美团",   /\*订单所属平台id，1美团、2考拉、3苏宁、4淘宝、5京东、6拼多多、7唯品会、8饿了么、9抖音等\*/
            "san\_pingtai\_id":"1"   /\*订单所属平台id，1美团、2考拉、3苏宁、4淘宝、5京东、6拼多多、7唯品会、8饿了么、9抖音等\*/
        },
        {
            "code":"14",
            "account":"83076790@qq.com",
            "appkey\_ztk":"a2a33cacfb1f4005b818ffd0ab7c951d",
            "sid":"16ztkzheta",
            "is\_jiesuan":"0",
            "jiesuan\_time":null,
            "jiesuan\_profit":"1.16",
            "orderid":"105518514197820012",
            "paytime":"2021/04/27 12:22:09",
            "payprice":"21.50",
            "profit":1.29,
            "smstitle":"霸碗盖码饭（凤凰大厦店）",
            "refundprice":0,
            "refundtime":"",
            "status":"8",
            "update\_time":null,
            "type":"4",
            "sid\_ztk":"16",
            "customer\_id\_ztk":"100000",
            "platform\_ztk":"zhetaoke"，
            "actId":"33",
            "san\_pingtai":"美团",
            "san\_pingtai\_id":"1"
        }
    \]
}
```

---

## 拼多多转链API

**接口名称：**拼多多高效转链API

**接口地址：**https://api.zhetaoke.com:10001/api/open\_pdd\_zhuanlian\_new.ashx

**返回数据：**json

**请求方式：**get

**请求示例：**https://api.zhetaoke.com:10001/api/open\_pdd\_zhuanlian\_new.ashx?appkey=&pdd\_app\_key=&pdd\_app\_secret=&pid=&content=

拼多多高效转链API。  
  

**请求参数说明：**

|  | 名称 | 类型 | 必填 | 说明 |
| --- | --- | --- | --- | --- |
|   | appkey | String | 是 | 折淘客对接秘钥appkey，折淘客通用。 |
|   | pdd\_app\_key | String | 是 | 拼多多开放平台应用的appkey，必填。 |
|   | pdd\_app\_secret | String | 是 | 拼多多开放平台应用的appsecret，必填。 |
|   | pid | String | 是 | 拼多多平台的推广位，必填。 |
|   | content | String | 是 | 支持纯数字id，或者拼多多推广短链。 |
|   | custom\_parameters | String | 否 | 自定义参数，为链接打上自定义标签；自定义参数最长限制64个字节；格式为： {"uid":"15611448080","sid":""} ，其中 uid 用户唯一标识，可自行加密后传入，每个用户仅且对应一个标识，必填；  
重要的事情说三遍：该参数需要进行Urlencode编码！该参数需要进行Urlencode编码！该参数需要进行Urlencode编码！  
重要的事情说三遍：返利才需要此参数，导购不需要！返利才需要此参数，导购不需要！返利才需要此参数，导购不需要！ |
|   |  |  |  | pdd\_app\_key、pdd\_app\_secret、pid请到拼多多开发平台和多多进宝平台去获取。  
  
1、pdd\_app\_key和pdd\_app\_secret获取网址：  
https://open.pinduoduo.com/#/application/index  
需要创建应用，拼多多官方审核后，即可使用。  
  
2、pid获取网址：  
https://jinbao.pinduoduo.com/manage/pid-manage  
可直接新建推广位，创建1个即可。  
  
3、关联应用ID---此步骤必须做  
在多多进宝平台中，关联创建的应用id，即可使用。此步骤必须做，不关联，不能转链。  
https://jinbao.pinduoduo.com/third-party/rank  
  
4、授权备案pid---此步骤必须做  
通过左边的备案按钮进行pid授权备案，此步骤必须做，不授权备案，不能转链。  
两个功能，一个是授权备案，一个是查询备案是否成功。  
  
以上四步必须全部完成才能正常转链。  
以上四步必须全部完成才能正常转链。  
以上四步必须全部完成才能正常转链。  
  
注意：拼多多联盟规定，一个pid备案后，只能用在一台机器上，多个机器上使用会被限制。  
注意：拼多多联盟规定，一个pid备案后，只能用在一台机器上，多个机器上使用会被限制。  
注意：拼多多联盟规定，一个pid备案后，只能用在一台机器上，多个机器上使用会被限制。  
 |

**相关说明：**

|  | 名称 | 类型 | 说明 |
| --- | --- | --- | --- |
|   |

**JSON返回示例：**

```json
{
"goods\_zs\_unit\_generate\_response": {
"multi\_group\_mobile\_short\_url": "https://p.pinduoduo.com/2sB0eGpK",     /\*推广短链接（唤起拼多多app）\*/
"multi\_group\_url": "https://mobile.yangkeduo.com/duo\_coupon\_landing.html?goods\_id=453581732819&pid=9747210\_128134481",     /\*双人团推广长链接\*/
"mobile\_url": "https://mobile.yangkeduo.com/duo\_coupon\_landing.html?goods\_id=453581732819&pid=9747210\_128134481",     /\*普通长链，微信环境下进入领券页点领券拉起小程序，浏览器环境下直接拉起APP，未安装拼多多APP时落地页点领券拉起登录页\*/
"multi\_group\_short\_url": "https://p.pinduoduo.com/HrQ0enzA",     /\*双人团推广短链接\*/
"mobile\_short\_url": "https://p.pinduoduo.com/QpD0PmVF",     /\*对应出参mobile\_url的短链接，与mobile\_url功能一致\*/
"multi\_group\_mobile\_url": "https://mobile.yangkeduo.com/duo\_coupon\_landing.html?goods\_id=453581732819&pid=9747210\_128134481",     /\*推广长链接（可唤起拼多多app）\*/
"request\_id": "16942627387113959",
"url": "https://mobile.yangkeduo.com/duo\_coupon\_landing.html?goods\_id=453581732819&pid=9747210\_128134481",     /\*普通长链。微信环境下进入领券页点领券拉起小程序，浏览器环境下优先拉起微信小程序\*/
"short\_url": "https://p.pinduoduo.com/bOL0pBPH"     /\*对应出参url的短链接，与url功能一致\*/
}
}
```

---

## 拼多多商品详情API（简版）

**接口名称：**拼多多商品详情API（简版）

**接口地址：**https://api.zhetaoke.com:10001/api/open\_pdd\_goods\_detail\_get\_new.ashx

**返回数据：**json

**请求方式：**get

**请求示例：**https://api.zhetaoke.com:10001/api/open\_pdd\_goods\_detail\_get\_new.ashx?appkey=&pdd\_app\_key=&pdd\_app\_secret=&pid=&content=

拼多多商品详情API（简版）。  
  

**请求参数说明：**

|  | 名称 | 类型 | 必填 | 说明 |
| --- | --- | --- | --- | --- |
|   | appkey | String | 是 | 折淘客对接秘钥appkey，折淘客通用。 |
|   | pdd\_app\_key | String | 是 | 拼多多开放平台应用的appkey，必填。 |
|   | pdd\_app\_secret | String | 是 | 拼多多开放平台应用的appsecret，必填。 |
|   | pid | String | 是 | 拼多多平台的推广位，必填。 |
|   | content | String | 是 | 支持纯数字id。 |

**相关说明：**

|  | 名称 | 类型 | 说明 |
| --- | --- | --- | --- |
|   |

**JSON返回示例：**

```json
{
"goods\_basic\_detail\_response": {
"goods\_list": \[
{
"goods\_name": "吃欢荞麦面皮麻酱味酸辣味冲泡免煮方便面拌面袋装擀面皮速食整箱",
"goods\_pic": "https://img.pddpic.com/gaudit-image/2023-06-09/9431858c23c01ce9ef578144be6bc47b.jpeg",
"min\_normal\_price": 1900,
"goods\_id": 453581732819,
"min\_group\_price": 990
}
\],
"request\_id": "16942480364163726"
}
}
```

---

## 拼多多授权备案API

**接口名称：**拼多多授权备案API

**接口地址：**https://api.zhetaoke.com:10001/api/open\_pdd\_shouquan\_new.ashx

**返回数据：**json

**请求方式：**get和url访问

**请求示例：**https://api.zhetaoke.com:10001/api/open\_pdd\_shouquan\_new.ashx?appkey=&pdd\_app\_key=&pdd\_app\_secret=&pid=

拼多多授权备案API。  
  

**请求参数说明：**

|  | 名称 | 类型 | 必填 | 说明 |
| --- | --- | --- | --- | --- |
|   | appkey | String | 是 | 折淘客对接秘钥appkey，折淘客通用。 |
|   | pdd\_app\_key | String | 是 | 拼多多开放平台应用的appkey，必填。 |
|   | pdd\_app\_secret | String | 是 | 拼多多开放平台应用的appsecret，必填。 |
|   | pid | String | 是 | 拼多多平台的推广位，必填。  
获取到json结果后，拿到url链接，到需要转链的服务器浏览器上打开，然后根据拼多多要求授权备案即可。 |
|   | custom\_parameters | String | 否 | 自定义参数，为链接打上自定义标签；自定义参数最长限制64个字节；格式为： {"uid":"15611448080","sid":""} ，其中 uid 用户唯一标识，可自行加密后传入，每个用户仅且对应一个标识，必填；  
重要的事情说三遍：该参数需要进行Urlencode编码！该参数需要进行Urlencode编码！该参数需要进行Urlencode编码！  
重要的事情说三遍：返利才需要此参数，导购不需要！返利才需要此参数，导购不需要！返利才需要此参数，导购不需要！ |
|   | generate\_qq\_app | String | 否 | 是否生成qq小程序，默认false |
|   | generate\_we\_app | String | 否 | 是否生成拼多多福利券微信小程序推广信息，默认false |

**相关说明：**

|  | 名称 | 类型 | 说明 |
| --- | --- | --- | --- |
|   |

**JSON返回示例：**

```json
{
"rp\_promotion\_url\_generate\_response": {
"url\_list": \[
{
"mobile\_url": "https://mobile.yangkeduo.com/duo\_coupon\_landing.html\_",     /\*移动版授权备案地址，将该url拷贝到需要转链的服务器的浏览器中，按照要求授权备案即可。\*/
"url": "https://mobile.yangkeduo.com/duo\_coupon\_landing.html"      /\*电脑版授权备案地址，将该url拷贝到需要转链的服务器的浏览器中，按照要求授权备案即可。\*/
}
\],
"request\_id": "16942480613190789"
}
}
```

---

## 拼多多授权备案查询API

**接口名称：**拼多多授权备案查询API

**接口地址：**https://api.zhetaoke.com:10001/api/open\_pdd\_shouquan\_query\_new.ashx

**返回数据：**json

**请求方式：**get

**请求示例：**https://api.zhetaoke.com:10001/api/open\_pdd\_shouquan\_query\_new.ashx?appkey=&pdd\_app\_key=&pdd\_app\_secret=&pid=

拼多多授权备案查询API。  
  

**请求参数说明：**

|  | 名称 | 类型 | 必填 | 说明 |
| --- | --- | --- | --- | --- |
|   | appkey | String | 是 | 折淘客对接秘钥appkey，折淘客通用。 |
|   | pdd\_app\_key | String | 是 | 拼多多开放平台应用的appkey，必填。 |
|   | pdd\_app\_secret | String | 是 | 拼多多开放平台应用的appsecret，必填。 |
|   | pid | String | 是 | 拼多多平台的推广位，必填。 |
|   | custom\_parameters | String | 否 | 自定义参数，为链接打上自定义标签；自定义参数最长限制64个字节；格式为： {"uid":"15611448080","sid":""} ，其中 uid 用户唯一标识，可自行加密后传入，每个用户仅且对应一个标识，必填；  
重要的事情说三遍：该参数需要进行Urlencode编码！该参数需要进行Urlencode编码！该参数需要进行Urlencode编码！  
重要的事情说三遍：返利才需要此参数，导购不需要！返利才需要此参数，导购不需要！返利才需要此参数，导购不需要！ |

**相关说明：**

|  | 名称 | 类型 | 说明 |
| --- | --- | --- | --- |
|  |

**JSON返回示例：**

```json
{
"authority\_query\_response": {
"bind": 1,     /\*1表示授权备案成功，其他值表示失败\*/
"request\_id": "16942467231374196"
}
}
```

---

## 拼多多订单查询API

**接口名称：**拼多多订单查询API

**接口地址：**https://api.zhetaoke.com:10001/api/open\_pdd\_dingdan\_new.ashx

**返回数据：**json

**请求方式：**get

**请求示例：**https://api.zhetaoke.com:10001/api/open\_pdd\_dingdan\_new.ashx?appkey=&pdd\_app\_key=&pdd\_app\_secret=&start\_update\_time=&end\_update\_time=

拼多多订单查询API。  
  

**请求参数说明：**

|  | 名称 | 类型 | 必填 | 说明 |
| --- | --- | --- | --- | --- |
|   | appkey | String | 是 | 折淘客对接秘钥appkey，折淘客通用。 |
|   | pdd\_app\_key | String | 是 | 拼多多开放平台应用的appkey，必填。 |
|   | pdd\_app\_secret | String | 是 | 拼多多开放平台应用的appsecret，必填。 |
|   | start\_update\_time | String | 是 | 最近90天内多多进宝商品订单更新时间--查询时间开始。此时间为时间戳，10位秒级时间戳。 |
|   | end\_update\_time | String | 是 | 最近90天内多多进宝商品订单更新时间--查询时间结束，和开始时间相差不能超过24小时。此时间为时间戳，10位秒级时间戳。 |
|   | query\_order\_type | String | 否 | 订单类型：1-推广订单；2-直播间订单 |
|   | page | String | 否 | 第几页，从1到10000，默认1，注：使用最后更新时间范围增量同步时，必须采用倒序的分页方式（从最后一页往回取）才能避免漏单问题。 |
|   | page\_size | String | 否 | 返回的每页结果订单数，默认为100，范围为10到100，建议使用40~50。 |
|   | cash\_gift\_order | String | 否 | 是否为礼金订单，查询礼金订单时，订单类型不填（默认推广订单）。 |
|   |  |  |  | [点击这里查看官方接口说明！点击这里查看官方接口说明！点击这里查看官方接口说明！](https://open.pinduoduo.com/application/document/api?id=pdd.ddk.order.list.increment.get&permissionId=2)  
[点击这里查看官方接口说明！点击这里查看官方接口说明！点击这里查看官方接口说明！](https://open.pinduoduo.com/application/document/api?id=pdd.ddk.order.list.increment.get&permissionId=2)  
[点击这里查看官方接口说明！点击这里查看官方接口说明！点击这里查看官方接口说明！](https://open.pinduoduo.com/application/document/api?id=pdd.ddk.order.list.increment.get&permissionId=2)  
 |

**相关说明：**

|  | 名称 | 类型 | 说明 |
| --- | --- | --- | --- |
|  |

**JSON返回示例：**

```json
{
"order\_list\_get\_response": {
"total\_count": 0,
"order\_list": \[\],
"request\_id": "16942626445817516"
}
}
```

---

## 拼多多商品详情API（详版）

**接口名称：**拼多多商品详情查询API（详版）

**接口地址：**https://api.zhetaoke.com:10001/api/open\_pdd\_goods\_detail\_search\_new.ashx

**返回数据：**json

**请求方式：**get

**请求示例：**https://api.zhetaoke.com:10001/api/open\_pdd\_goods\_detail\_search\_new.ashx?appkey=&pdd\_app\_key=&pdd\_app\_secret=&pid=&custom\_parameters=&keyword=&cat\_id=

拼多多商品详情查询API（详版）。  
  

**请求参数说明：**

|  | 名称 | 类型 | 必填 | 说明 |
| --- | --- | --- | --- | --- |
|   | appkey | String | 是 | 折淘客对接秘钥appkey，折淘客通用。 |
|   | pdd\_app\_key | String | 是 | 拼多多开放平台应用的appkey，必填。 |
|   | pdd\_app\_secret | String | 是 | 拼多多开放平台应用的appsecret，必填。 |
|   | pid | String | 是 | 拼多多平台的推广位，必填。 |
|   | custom\_parameters | String | 否 | 自定义参数，为链接打上自定义标签；自定义参数最长限制64个字节；格式为： {"uid":"15611448080","sid":""} ，其中 uid 用户唯一标识，可自行加密后传入，每个用户仅且对应一个标识，必填；  
重要的事情说三遍：该参数需要进行Urlencode编码！该参数需要进行Urlencode编码！该参数需要进行Urlencode编码！  
重要的事情说三遍：返利才需要此参数，导购不需要！返利才需要此参数，导购不需要！返利才需要此参数，导购不需要！ |
|   | keyword | String | 是 | 商品关键词。可支持goods\_id、拼多多商品链接、推广长链和推广短链。 |
|   | cat\_id | String | 否 | 商品类目ID，使用pdd.goods.cats.get接口获取。 |
|   | activity\_tags | String | 否 | 活动商品标记数组，例：\[4,7\]，4-秒杀，7-百亿补贴，10851-千万补贴，11879-千万神券，10913-招商礼金商品，  
31-品牌黑标，10564-精选爆品-官方直推爆款，10584-精选爆品-团长推荐，24-品牌高佣，其他的值请忽略 |
|   | block\_cat\_packages | String | 否 | 屏蔽商品类目包：1-拼多多小程序屏蔽的类目&关键词;2-虚拟类目;3-医疗器械;4-处方药;5-非处方药;6-冬奥元素相关商品。 |
|   | block\_cats | String | 否 | 自定义屏蔽一级/二级/三级类目ID，自定义数量不超过20个;使用pdd.goods.cats.get接口获取cat\_id。 |
|   | is\_brand\_goods | String | 否 | 是否为品牌商品。 |
|   | merchant\_type | String | 否 | 店铺类型，1-个人，2-企业，3-旗舰店，4-专卖店，5-专营店，6-普通店（未传为全部）。 |
|   | sort\_type | String | 否 | 排序方式:0-综合排序;1-按佣金比率升序;2-按佣金比例降序;3-按价格升序;4-按价格降序;5-按销量升序;6-按销量降序;  
7-优惠券金额排序升序;8-优惠券金额排序降序;9-券后价升序排序;10-券后价降序排序;11-按照加入多多进宝时间升序;  
12-按照加入多多进宝时间降序;13-按佣金金额升序排序;14-按佣金金额降序排序;15-店铺描述评分升序;16-店铺描述评分降序;  
17-店铺物流评分升序;18-店铺物流评分降序;19-店铺服务评分升序;20-店铺服务评分降序;27-描述评分击败同类店铺百分比升序，  
28-描述评分击败同类店铺百分比降序，29-物流评分击败同类店铺百分比升序，30-物流评分击败同类店铺百分比降序，  
31-服务评分击败同类店铺百分比升序，32-服务评分击败同类店铺百分比降序。 |
|   | use\_customized | String | 否 | 是否使用个性化推荐，true表示使用，false表示不使用，默认true。 |
|   | with\_coupon | String | 否 | 是否只返回优惠券的商品，false返回所有商品，true只返回有优惠券的商品。 |
|   |  |  |  | [点击这里查看官方接口说明！点击这里查看官方接口说明！点击这里查看官方接口说明！](https://open.pinduoduo.com/application/document/api?id=pdd.ddk.goods.detail&permissionId=2)  
[点击这里查看官方接口说明！点击这里查看官方接口说明！点击这里查看官方接口说明！](https://open.pinduoduo.com/application/document/api?id=pdd.ddk.goods.detail&permissionId=2)  
[点击这里查看官方接口说明！点击这里查看官方接口说明！点击这里查看官方接口说明！](https://open.pinduoduo.com/application/document/api?id=pdd.ddk.goods.detail&permissionId=2)  
 |

**相关说明：**

|  | 名称 | 类型 | 说明 |
| --- | --- | --- | --- |
|  |

**JSON返回示例：**

```json
{
}
```

---

## 拼多多商品类目接口

**接口名称：**拼多多商品标准类目接口

**接口地址：**https://api.zhetaoke.com:10001/api/open\_pdd\_goods\_cats\_new.ashx

**返回数据：**json

**请求方式：**get

**请求示例：**https://api.zhetaoke.com:10001/api/open\_pdd\_goods\_cats\_new.ashx?appkey=&pdd\_app\_key=&pdd\_app\_secret=&parent\_cat\_id=0

拼多多商品标准类目接口。  
  

**请求参数说明：**

|  | 名称 | 类型 | 必填 | 说明 |
| --- | --- | --- | --- | --- |
|   | appkey | String | 是 | 折淘客对接秘钥appkey，折淘客通用。 |
|   | pdd\_app\_key | String | 是 | 拼多多开放平台应用的appkey，必填。 |
|   | pdd\_app\_secret | String | 是 | 拼多多开放平台应用的appsecret，必填。 |
|   | parent\_cat\_id | String | 是 | 值=0时为顶点cat\_id,通过树顶级节点获取cat树 |

**相关说明：**

|  | 名称 | 类型 | 说明 |
| --- | --- | --- | --- |
|  |

**JSON返回示例：**

```json
{
"goods\_cats\_get\_response": {
"goods\_cats\_list": \[
{
"cat\_name": "男装",
"level": 1,
"cat\_id": 239,
"parent\_cat\_id": 0
}
\],
"request\_id": "16942687923932889"
}
}
```

---

## 抖音商品转链API

**接口名称：**抖音商品转链API

**接口地址：**https://api.zhetaoke.com:10001/api/open\_douyin\_zhuanlian.ashx

**返回数据：**json

**请求方式：**get或者post

**请求示例：**https://api.zhetaoke.com:10001/api/open\_douyin\_zhuanlian.ashx?appkey=&sid=&product\_url=&external\_info=

抖音商品转链API。  
  

**请求参数说明：**

|  | 名称 | 类型 | 必填 | 说明 |
| --- | --- | --- | --- | --- |
|   | appkey | String | 是 | 折淘客对接秘钥appkey，折淘客通用。 |
|   | sid | String | 是 | 对应的淘客账号授权SID。 |
|   | product\_url | String | 是 | 商品URL/口令/短链接。  
重要的事情说三遍：该参数需要进行Urlencode编码！该参数需要进行Urlencode编码！该参数需要进行Urlencode编码！ |
|   | external\_info | String | 否 | 自定义字段，只允许数字，限制长度为11位,返利专用字段，不返利不要填写。 |
|   | need\_qr\_code | String | 否 | 是否需要二维码，需要会导致响应耗时增加，true或者false，默认false。 |
|   | use\_coupon | String | 否 | 是否返回商品惠后价领券链接(如果商品有优惠则返回，否则不返回)，true或者false，默认false。 |
|   | need\_share\_link | String | 否 | 是否返回站外H5链接，true或者false，默认false。 |
|   |  |  |  | [点击这里查看官方接口说明！点击这里查看官方接口说明！点击这里查看官方接口说明！](https://op.jinritemai.com/docs/api-docs/61/1464)  
[点击这里查看官方接口说明！点击这里查看官方接口说明！点击这里查看官方接口说明！](https://op.jinritemai.com/docs/api-docs/61/1464)  
[点击这里查看官方接口说明！点击这里查看官方接口说明！点击这里查看官方接口说明！](https://op.jinritemai.com/docs/api-docs/61/1464)  
 |

**相关说明：**

|  | 名称 | 类型 | 说明 |
| --- | --- | --- | --- |
|  |

**JSON返回示例：**

```json
{
  "data": {
    "data": {
      "coupon\_link": {
        "coupon\_status": "0",
        "deeplink": "snssdk1128://ec\_goods\_detail/?promotion\_id=3471556980941674",
        "qrcode": {
          "height": "360",
          "url": "https://p3.douyinpic.com/img/aweme-qrcode/PqFhya700510763792421~c5\_360x360.webp?from=1723184",
          "width": "360"
        },
        "share\_command": "长按覆\[呲牙\]zhi此条消息，咑\[玫瑰\]搜\[烟花\]，查看商品详情Ig9FOHrrl8ηη",
        "share\_link": "https://alliance.jinritemai.com"
      },
      "dy\_deeplink": "nssdk1128://ec\_goods\_detail/?promotion\_id=3471556980941674&kol\_id=99575927&so",
      "dy\_password": "长按覆\[呲牙\]zhi此条消息，咑\[玫瑰\]搜\[烟花\]，查看商品详情Ig9FOHrrl8ηη",
      "dy\_zlink": "https://z.douyin.com/xxxxxxx",
      "qr\_code": {
        "height": "360",
        "url": "https://p3.douyinpic.com/img/aweme-qrcode/PqFhya700510763792421~c5\_360x360.webp?from=1723184",
        "width": "360"
      },
      "share\_link": "https://haohuo.jinritemai.com/ecommerce/trade/detail/index.html?id=",
      "use\_ins\_activity": "0"
    }
  },
  "code": 10000,
  "msg": "success",
  "sub\_code": "",
  "sub\_msg": ""
}
```

---

## 抖音商品详情API

**接口名称：**抖音商品详情API

**接口地址：**https://api.zhetaoke.com:10001/api/open\_douyin\_product\_detail.ashx

**返回数据：**json

**请求方式：**get或者post

**请求示例：**https://api.zhetaoke.com:10001/api/open\_douyin\_product\_detail.ashx?appkey=&sid=&product\_ids=

抖音商品详情API。  
  

**请求参数说明：**

|  | 名称 | 类型 | 必填 | 说明 |
| --- | --- | --- | --- | --- |
|   | appkey | String | 是 | 折淘客对接秘钥appkey，折淘客通用。 |
|   | sid | String | 是 | 对应的淘客账号授权SID。 |
|   | product\_ids | String | 是 | 商品 ID 列表(最多20个),用英文逗号分割。 |
|   | fields | String | 否 | 值为：base\_info,promotion\_info （需要返回的字段，多个用英文逗号隔开。不传只返回商品基础信息，强烈建议按需获取数据） |
|   |  |  |  | [点击这里查看官方接口说明！点击这里查看官方接口说明！点击这里查看官方接口说明！](https://op.jinritemai.com/docs/api-docs/61/2867)  
[点击这里查看官方接口说明！点击这里查看官方接口说明！点击这里查看官方接口说明！](https://op.jinritemai.com/docs/api-docs/61/2867)  
[点击这里查看官方接口说明！点击这里查看官方接口说明！点击这里查看官方接口说明！](https://op.jinritemai.com/docs/api-docs/61/2867)  
 |

**相关说明：**

|  | 名称 | 类型 | 说明 |
| --- | --- | --- | --- |
|  |

**JSON返回示例：**

```json
{
  "data": {
    "products": \[
      {
        "activity\_info": {
          "activity\_type": "1",
          "success": "true"
        },
        "base\_info": {
          "category\_id": "2",
          "category\_name": "玩具乐器",
          "cover": "https://sf1-ttcdn-tos.pstatp.com/img/temai/Fpt8famR3twTGYz09e25nz5PbTRuwww800-800~1125x1125\_q50.webp",
          "detail\_url": "https://haohuo.jinritemai.com/views/product/item2?id=3400312511185213726",
          "first\_cid": "11",
          "imgs": "\[轮播图\]",
          "in\_stock": "true",
          "price": "100",
          "product\_id": "3461984765147123",
          "sales": "100",
          "second\_cid": "2011",
          "success": "true",
          "third\_cid": "3040",
          "title": "商品名称"
        },
        "brand\_info": {
          "brand\_id": "237461",
          "brand\_name\_cn": "品牌名",
          "brand\_name\_en": "brand\_name",
          "success": "true"
        },
        "comment\_info": {
          "comment\_num": "2321",
          "comment\_score": "2",
          "success": "true"
        },
        "coupon\_info": {
          "available\_coupons": \[
            {
              "apply\_end\_time": "2016-01-01 12:00:00",
              "apply\_start\_time": "2016-01-01 12:00:00",
              "coupon\_type": "1",
              "discount\_desc": "满20减1",
              "type\_desc": "平台券",
              "use\_end\_time": "2016-01-01 12:00:00",
              "use\_start\_time": "2016-01-01 12:00:00",
              "valid\_period": "3600",
              "validity\_type": "1"
            }
          \],
          "coupon\_price": "100",
          "success": "true"
        },
        "detail\_brief": {
          "component\_info": \[
            {
              "image": {
                "height": "400",
                "pic\_url": "https://p6-item.ecombdimg.com/img/ecom-shop-material/EMNEddDN\_m\_ffdc461d338f14893f98a41a76173528\_sx\_113416\_www750-1000~tpmsx3fupr-resize:750:1000.jpeg",
                "width": "400"
              }
            }
          \],
          "success": "true"
        },
        "logistics\_info": {
          "post\_free": "true",
          "success": "true",
          "text": "48小时内从浙江省发货 运费3元起"
        },
        "month\_sale\_data": {
          "daily\_statistics": \[
            {
              "date": "20210101",
              "kol\_num": "100",
              "order\_num": "1000",
              "view\_num": "100"
            }
          \],
          "kol\_num": "1000",
          "order\_num": "1000",
          "success": "true",
          "view\_num": "1000"
        },
        "presell\_info": {
          "presell\_type": "0",
          "success": "true"
        },
        "promotion\_info": {
          "cos\_fee": "100",
          "cos\_ratio": "10",
          "success": "true"
        },
        "qualification\_info": {
          "has\_sxt": "true",
          "success": "true"
        },
        "rights\_info": {
          "is\_assured": "true",
          "success": "true"
        },
        "share\_info": {
          "sharable": "true",
          "success": "true"
        },
        "shop\_info": {
          "shop\_id": "25316",
          "shop\_name": "店铺名称",
          "shop\_total\_score": {
            "logistics\_score": {
              "level": "1",
              "score": "4.95",
              "text": "物流体验分"
            },
            "product\_score": {
              "level": "1",
              "score": "4.95",
              "text": "商品体验分"
            },
            "service\_score": {
              "level": "1",
              "score": "4.95",
              "text": "商家服务分"
            },
            "shop\_score": {
              "level": "1",
              "score": "4.95",
              "text": "商家体验分"
            }
          },
          "success": "true"
        },
        "tags": {
          "has\_douin\_goods\_tag": "true",
          "has\_shop\_brand\_tag": "true",
          "has\_subsidy\_tag": "true",
          "has\_supermarket\_tag": "true",
          "success": "true"
        },
        "trade\_info": {
          "limit\_min\_sale": "false",
          "success": "true"
        }
      }
    \]
  },
  "code": 10000,
  "msg": "success",
  "sub\_code": "",
  "sub\_msg": ""
}
```

---

## 抖音商品搜索API

**接口名称：**抖音商品搜索API

**接口地址：**https://api.zhetaoke.com:10001/api/open\_douyin\_product\_search.ashx

**返回数据：**json

**请求方式：**get或者post

**请求示例：**https://api.zhetaoke.com:10001/api/open\_douyin\_product\_search.ashx?appkey=&sid=&title=

抖音商品搜索API。  
  

**请求参数说明：**

|  | 名称 | 类型 | 必填 | 说明 |
| --- | --- | --- | --- | --- |
|   | appkey | String | 是 | 折淘客对接秘钥appkey，折淘客通用。 |
|   | sid | String | 是 | 对应的淘客账号授权SID。 |
|   | title | String | 否 | 商品标题，返回标题中包含某个关键词的商品。  
重要的事情说三遍：该参数需要进行Urlencode编码！该参数需要进行Urlencode编码！该参数需要进行Urlencode编码！ |
|   | first\_cids | String | 否 | 筛选商品一级类目 |
|   | second\_cids | String | 否 | 筛选商品二级类目 |
|   | third\_cids | String | 否 | 筛选商品三级类目 |
|   | price\_min | String | 否 | 筛选价格区间-最小值（单位为分） |
|   | price\_max | String | 否 | 筛选价格区间-最大值（单位为分） |
|   | sell\_num\_min | String | 否 | 筛选历史销量区间-最小值 |
|   | sell\_num\_max | String | 否 | 筛选历史销量区间-最大值 |
|   | search\_type | String | 否 | 召回结果排序条件，0默认排序1历史销量排序2价格排序3佣金金额排序4佣金比例排序； |
|   | sort\_type | String | 否 | 排序顺序（0升序1降序） |
|   | cos\_fee\_min | String | 否 | 筛选普通佣金区间-最小值（单位为分） |
|   | cos\_fee\_max | String | 否 | 筛选普通佣金区间-最大值（单位为分） |
|   | cos\_ratio\_min | String | 否 | 筛选普通佣金率区间-最小值 （乘100，例如1.1%为110） |
|   | cos\_ratio\_max | String | 否 | 筛选普通佣金率区间-最大值（乘100，例如1.1%为110） |
|   | page | String | 否 | 分页（从1开始） |
|   | page\_size | String | 否 | 每页的数量（小于等于20） |
|   | share\_status | String | 否 | 获取商品分销状态。1: 仅返回可分销商品；0:返回全量商品 |
|   | tag | String | 否 | 商品标签。 0:返回全量商品; 1:返回超值购商品; 2:返回抖音超市(次日达)商品 |
|   |  |  |  | [点击这里查看官方接口说明！点击这里查看官方接口说明！点击这里查看官方接口说明！](https://op.jinritemai.com/docs/api-docs/61/924)  
[点击这里查看官方接口说明！点击这里查看官方接口说明！点击这里查看官方接口说明！](https://op.jinritemai.com/docs/api-docs/61/924)  
[点击这里查看官方接口说明！点击这里查看官方接口说明！点击这里查看官方接口说明！](https://op.jinritemai.com/docs/api-docs/61/924)  
 |

**相关说明：**

|  | 名称 | 类型 | 说明 |
| --- | --- | --- | --- |
|  |

**JSON返回示例：**

```json
{
  "data": {
    "products": \[
      {
        "activity\_id": "1",
        "available\_coupons": \[
          {
            "apply\_end\_time": "2016-01-01 13:00:00",
            "apply\_start\_time": "2016-01-01 12:00:00",
            "coupon\_type": "1",
            "discount\_desc": "满20减1",
            "type\_desc": "平台券",
            "use\_end\_time": "2016-01-01 13:00:00",
            "use\_start\_time": "2016-01-01 12:00:00",
            "valid\_period": "3600",
            "validity\_type": "1"
          }
        \],
        "cos\_fee": "1",
        "cos\_ratio": "10",
        "coupon\_price": "92",
        "cover": "https://sf1-ttcdn-tos.pstatp.com/img/temai/Fpt8famR3twTGYz09e25nz5PbTRuwww800-800~1125x1125\_q50.webp",
        "detail\_url": "https://haohuo.jinritemai.com/views/product/item2?id=3400312511185213726",
        "first\_cid": "1206",
        "in\_stock": "1",
        "limit\_min\_sale": "false",
        "post\_free": "true",
        "presell\_type": "0",
        "price": "100",
        "product\_id": "3400312511185213726",
        "sales": "1234",
        "second\_cid": "1402",
        "sharable": "true",
        "shop\_id": "2232",
        "shop\_name": "maggie测试小铺",
        "third\_cid": "2634",
        "title": "测试商品"
      }
    \],
    "total": "1000"
  },
  "code": 10000,
  "msg": "success",
  "sub\_code": "",
  "sub\_msg": ""
}
```

---

## 抖音口令解析API

**接口名称：**抖音口令解析API

**接口地址：**https://api.zhetaoke.com:10001/api/open\_douyin\_kouling\_jiexi.ashx

**返回数据：**json

**请求方式：**get或者post

**请求示例：**https://api.zhetaoke.com:10001/api/open\_douyin\_kouling\_jiexi.ashx?appkey=&sid=&command=

抖音口令解析API。  
  

**请求参数说明：**

|  | 名称 | 类型 | 必填 | 说明 |
| --- | --- | --- | --- | --- |
|   | appkey | String | 是 | 折淘客对接秘钥appkey，折淘客通用。 |
|   | sid | String | 是 | 对应的淘客账号授权SID。 |
|   | command | String | 是 | 口令/短链接。  
重要的事情说三遍：该参数需要进行Urlencode编码！该参数需要进行Urlencode编码！该参数需要进行Urlencode编码！ |
|   |  |  |  | [点击这里查看官方接口说明！点击这里查看官方接口说明！点击这里查看官方接口说明！](https://op.jinritemai.com/docs/api-docs/61/4010)  
[点击这里查看官方接口说明！点击这里查看官方接口说明！点击这里查看官方接口说明！](https://op.jinritemai.com/docs/api-docs/61/4010)  
[点击这里查看官方接口说明！点击这里查看官方接口说明！点击这里查看官方接口说明！](https://op.jinritemai.com/docs/api-docs/61/4010)  
 |

**相关说明：**

|  | 名称 | 类型 | 说明 |
| --- | --- | --- | --- |
|  |

**JSON返回示例：**

```json
{
  "data": {
    "command\_info": {
      "activity\_info": {
        "extra\_params": "{\\"product\_id\\":\\"12345678\\"}\\"",
        "material\_id": "123456",
        "share\_info": {
          "deeplink": "snssdk1128://ec\_goods\_detail/?promotion\_id=3471556980941674&kol\_id=99575927",
          "qrcode": {
            "height": "360",
            "url": "https://p3.douyinpic.com/img/aweme-qrcode/PqFhya700510763792421~c5\_360x360.webp?from=1723184",
            "width": "360"
          },
          "share\_command": "长按覆\[呲牙\]zhi此条消息，咑\[玫瑰\]搜\[烟花\]，查看商品详情Ig9FOHrrl8ηη",
          "zlink": "https://z.douyin.com/xxxxxxx"
        }
      },
      "command\_type": "1",
      "live\_appoint\_info": {
        "author\_buyin\_id": "112233",
        "live\_appointment\_id": "123456",
        "share\_info": {
          "deeplink": "snssdk1128://ec\_goods\_detail/?promotion\_id=3471556980941674&kol\_id=99575927",
          "qrcode": {
            "height": "360",
            "url": "https://p3.douyinpic.com/img/aweme-qrcode/PqFhya700510763792421~c5\_360x360.webp?from=1723184",
            "width": "360"
          },
          "share\_command": "长按覆\[呲牙\]zhi此条消息，咑\[玫瑰\]搜\[烟花\]，查看商品详情Ig9FOHrrl8ηη"
        }
      },
      "live\_info": {
        "author\_buyin\_id": "112233",
        "product\_id": "123456",
        "share\_info": {
          "deeplink": "snssdk1128://ec\_goods\_detail/?promotion\_id=3471556980941674&kol\_id=99575927",
          "qrcode": {
            "height": "360",
            "url": "https://p3.douyinpic.com/img/aweme-qrcode/PqFhya700510763792421~c5\_360x360.webp?from=1723184",
            "width": "360"
          },
          "share\_command": "长按覆\[呲牙\]zhi此条消息，咑\[玫瑰\]搜\[烟花\]，查看商品详情Ig9FOHrrl8ηη",
          "zlink": "https://z.douyin.com/xxxxxxx"
        }
      },
      "product\_info": {
        "ins\_activity\_param": "abc",
        "product\_id": "123456",
        "share\_info": {
          "deeplink": "snssdk1128://ec\_goods\_detail/?promotion\_id=3471556980941674&kol\_id=99575927",
          "qrcode": {
            "height": "360",
            "url": "https://p3.douyinpic.com/img/aweme-qrcode/PqFhya700510763792421~c5\_360x360.webp?from=1723184",
            "width": "360"
          },
          "share\_command": "长按覆\[呲牙\]zhi此条消息，咑\[玫瑰\]搜\[烟花\]，查看商品详情Ig9FOHrrl8ηη",
          "share\_link": "https://haohuo.jinritemai.com/ecommerce/trade/detail/index.html?id=",
          "zlink": "https://z.douyin.com/xxxxxxx"
        }
      },
      "redpack\_info": {
        "redpack\_id": "123456",
        "share\_info": {
          "deeplink": "snssdk1128://ec\_goods\_detail/?promotion\_id=3471556980941674&kol\_id=99575927",
          "qrcode": {
            "height": "360",
            "url": "https://p3.douyinpic.com/img/aweme-qrcode/PqFhya700510763792421~c5\_360x360.webp?from=1723184",
            "width": "360"
          },
          "share\_command": "长按覆\[呲牙\]zhi此条消息，咑\[玫瑰\]搜\[烟花\]，查看商品详情Ig9FOHrrl8ηη"
        }
      }
    }
  },
  "code": 10000,
  "msg": "success",
  "sub\_code": "",
  "sub\_msg": ""
}
```

---

## 抖音订单查询API

**接口名称：**抖音订单查询API（折淘客联盟结算佣金的所有订单，都可以通过此接口查询。）

**接口地址：**https://api.zhetaoke.com:10001/api/open\_lianmeng\_orderList.ashx

**返回数据：**json

**请求方式：**get或者post

**请求示例：**https://api.zhetaoke.com:10001/api/open\_lianmeng\_orderList.ashx?appkey=&type=&startTime=&endTime=

根据订单支付时间或者订单更新时间获取最近90天内的折淘客联盟订单数据  
支持按支付时间、更新时间查询。  
建议按照订单支付时间，每分钟查询一次，按照订单更新时间，每分钟查询一次即可。  
个别长时间不结算的订单可以单独用订单编号来查询。  
  
[点击这里在线查询折淘客联盟订单信息！点击这里在线查询折淘客联盟订单信息！点击这里在线查询折淘客联盟订单信息！](/user/extend/extend_app_dingdan_lianmeng.aspx)  
  

**请求参数说明：**

|  | 名称 | 类型 | 必填 | 说明 |
| --- | --- | --- | --- | --- |
|   | appkey | String | 是 | 折淘客对接秘钥appkey，折淘客通用。 |
|   | type | String | 是 | 时间类型，1表示按照订单支付时间获取，2表示按照订单更新时间获取，3表示按照订单编号获取。 |
|   | page | String | 是 | 页码 |
|   | page\_size | String | 是 | 每页数量，单页数最大50，默认50 |
|   | startTime | String | 否 | 订单开始时间,例子：2021-04-30 08:00:00 |
|   | endTime | String | 否 | 订单结束时间,例子：2021-04-30 09:00:00 |
|   | orderid | String | 否 | 订单编号，必须输入完整的订单编号。 |
|   | sid | String | 否 | 折淘客授权sid，无值，表示获取appkey下所有折淘客联盟订单。 |
|   | san\_pingtai\_id | String | 否 | 订单所属平台id，1美团、2考拉、3苏宁、4淘宝、5京东、6拼多多、7唯品会、8饿了么、9抖音等 |

**相关说明：**

|  | 名称 | 类型 | 说明 |
| --- | --- | --- | --- |
|   |

**JSON返回示例：**

```json
{
    "status":200,
    "content":\[
        {
            "code":"15",      /\*折淘客自动增长列\*/
            "account":"83076790@qq.com",      /\*折淘客账号\*/
            "appkey\_ztk":"a2a33cacfb1f4005b818ffd0ab7c951d",      /\*折淘客appkey\*/
            "sid":"16ztk",   /\*原始推广位sid\*/
            "is\_jiesuan":"0",     /\*折淘客是否结算，0未结算，1已结算\*/
            "jiesuan\_time":"",      /\*折淘客结算时间\*/
            "jiesuan\_profit":"1.16",      /\*折淘客结算金额\*/
            "orderid":"105518514197820013",   /\*订单编号\*/
            "paytime":"2021/04/27 12:22:09",      /\*订单支付时间\*/
            "payprice":"21.50",   /\*订单实际支付金额\*/
            "profit":1.29,    /\*预估佣金\*/
            "smstitle":"霸碗盖码饭（凤凰大厦店）",    /\*店铺标题\*/
            "smstitle2":"",    /\*商品名称\*/
            "item\_id":"",    /\*商品id\*/
            "item\_pic":"",    /\*商品主图\*/
            "refundprice":0,      /\*退款金额\*/
            "refundtime":"",      /\*退款时间\*/
            "status":"8",     /\*订单状态，1已付款，8已完成，9已退款或风控\*/
            "update\_time":"",   /\*数据最后更新时间\*/
            "type":"4",   /\*订单类型\*/
            "sid\_ztk":"16",   /\*折淘客授权sid\*/
            "customer\_id\_ztk":"100000",   /\*推广者自定义编号\*/
            "platform\_ztk":"zhetaoke",     /\*推广者平台类型，默认zhetaoke\*/
            "actId":"33",   /\*活动id\*/
            "san\_pingtai":"美团",   /\*订单所属平台id，1美团、2考拉、3苏宁、4淘宝、5京东、6拼多多、7唯品会、8饿了么、9抖音等\*/
            "san\_pingtai\_id":"1"   /\*订单所属平台id，1美团、2考拉、3苏宁、4淘宝、5京东、6拼多多、7唯品会、8饿了么、9抖音等\*/
        },
        {
            "code":"14",
            "account":"83076790@qq.com",
            "appkey\_ztk":"a2a33cacfb1f4005b818ffd0ab7c951d",
            "sid":"16ztkzheta",
            "is\_jiesuan":"0",
            "jiesuan\_time":null,
            "jiesuan\_profit":"1.16",
            "orderid":"105518514197820012",
            "paytime":"2021/04/27 12:22:09",
            "payprice":"21.50",
            "profit":1.29,
            "smstitle":"霸碗盖码饭（凤凰大厦店）",
            "smstitle2":"",
            "item\_id":"",
            "item\_pic":"",
            "refundprice":0,
            "refundtime":"",
            "status":"8",
            "update\_time":null,
            "type":"4",
            "sid\_ztk":"16",
            "customer\_id\_ztk":"100000",
            "platform\_ztk":"zhetaoke"，
            "actId":"33",
            "san\_pingtai":"美团",
            "san\_pingtai\_id":"1"
        }
    \]
}
```

---

## 抖音直播间转链API

**接口名称：**抖音直播间转链API

**接口地址：**https://api.zhetaoke.com:10001/api/open\_douyin\_zhuanlian\_zhibojian.ashx

**返回数据：**json

**请求方式：**get或者post

**请求示例：**https://api.zhetaoke.com:10001/api/open\_douyin\_zhuanlian\_zhibojian.ashx?appkey=&sid=&buyin\_id=

抖音直播间转链API。  
  

**请求参数说明：**

|  | 名称 | 类型 | 必填 | 说明 |
| --- | --- | --- | --- | --- |
|   | appkey | String | 是 | 折淘客对接秘钥appkey，折淘客通用。 |
|   | sid | String | 是 | 对应的淘客账号授权SID。 |
|   | buyin\_id | String | 否 | 主播百应ID (buyinId和dy\_code最少填一项) |
|   | dy\_code | String | 否 | 直播间口令或者短链接（优先级 buyin\_id > dy\_code）） (buyinId和dy\_code最少填一项)  
重要的事情说三遍：该参数需要进行Urlencode编码！该参数需要进行Urlencode编码！该参数需要进行Urlencode编码！ |
|   | external\_info | String | 否 | 自定义字段，只允许数字，限制长度为11位,返利专用字段，不返利不要填写。 |
|   |  |  |  | [点击这里查看官方接口说明！点击这里查看官方接口说明！点击这里查看官方接口说明！](https://op.jinritemai.com/docs/api-docs/61/1297)  
[点击这里查看官方接口说明！点击这里查看官方接口说明！点击这里查看官方接口说明！](https://op.jinritemai.com/docs/api-docs/61/1297)  
[点击这里查看官方接口说明！点击这里查看官方接口说明！点击这里查看官方接口说明！](https://op.jinritemai.com/docs/api-docs/61/1297)  
 |

**相关说明：**

|  | 名称 | 类型 | 说明 |
| --- | --- | --- | --- |
|  |

**JSON返回示例：**

```json
{
  "data": {
    "data": {
      "dy\_deeplink": "snssdk://live..",
      "dy\_password": "6.923405940",
      "dy\_zlink": "https://z.douyin.com/xxxxxxx",
      "qr\_code": {
        "height": "200",
        "url": "https://ssddsd.jpg",
        "width": "200"
      }
    }
  },
  "code": 10000,
  "msg": "success",
  "sub\_code": "",
  "sub\_msg": ""
}
```

---

## 抖音活动转链API

**接口名称：**抖音活动转链API

**接口地址：**https://api.zhetaoke.com:10001/api/open\_douyin\_zhuanlian\_activity.ashx

**返回数据：**json

**请求方式：**get或者post

**请求示例：**https://api.zhetaoke.com:10001/api/open\_douyin\_zhuanlian\_activity.ashx?appkey=&sid=&material\_id=

抖音活动转链API。  
  

**请求参数说明：**

|  | 名称 | 类型 | 必填 | 说明 |
| --- | --- | --- | --- | --- |
|   | appkey | String | 是 | 折淘客对接秘钥appkey，折淘客通用。 |
|   | sid | String | 是 | 对应的淘客账号授权SID。 |
|   | material\_id | String | 是 | 活动页物料ID |
|   | external\_info | String | 否 | 自定义字段，只允许数字，限制长度为11位,返利专用字段，不返利不要填写。 |
|   | need\_qr\_code | String | 否 | 是否需要二维码，需要会导致响应耗时增加，true或者false，默认false。 |
|   |  |  |  | [点击这里查看官方接口说明！点击这里查看官方接口说明！点击这里查看官方接口说明！](https://op.jinritemai.com/docs/api-docs/61/3130)  
[点击这里查看官方接口说明！点击这里查看官方接口说明！点击这里查看官方接口说明！](https://op.jinritemai.com/docs/api-docs/61/3130)  
[点击这里查看官方接口说明！点击这里查看官方接口说明！点击这里查看官方接口说明！](https://op.jinritemai.com/docs/api-docs/61/3130)  
 |

**相关说明：**

|  | 名称 | 类型 | 说明 |
| --- | --- | --- | --- |
|  |

**JSON返回示例：**

```json
{
  "data": {
    "data": {
      "dy\_deeplink": "snssdk://live..",
      "dy\_password": "6.923405940",
      "dy\_zlink": "https://z.douyin.com/xxxxxxx",
      "qr\_code": {
        "height": "200",
        "url": "https://ssddsd.jpg",
        "width": "200"
      }
    }
  },
  "code": 10000,
  "msg": "success",
  "sub\_code": "",
  "sub\_msg": ""
}
```

---

## 抖音活动转链API

**接口名称：**抖音活动列表API

**接口地址：**https://api.zhetaoke.com:10001/api/open\_douyin\_activity\_list.ashx

**返回数据：**json

**请求方式：**get或者post

**请求示例：**https://api.zhetaoke.com:10001/api/open\_douyin\_activity\_list.ashx?appkey=&sid=&page=1&page\_size=20

抖音活动列表API。  
  

**请求参数说明：**

|  | 名称 | 类型 | 必填 | 说明 |
| --- | --- | --- | --- | --- |
|   | appkey | String | 是 | 折淘客对接秘钥appkey，折淘客通用。 |
|   | sid | String | 是 | 对应的淘客账号授权SID。 |
|   | page | String | 否 | 分页（从1开始） |
|   | page\_size | String | 否 | 每页的数量（小于等于20） |
|   | activity\_status | String | 否 | 活动状态（不填默认返回进行中活动，1:待开始，2:进行中，3: 已结束） |
|   |  |  |  | [点击这里查看官方接口说明！点击这里查看官方接口说明！点击这里查看官方接口说明！](https://op.jinritemai.com/docs/api-docs/61/3129)  
[点击这里查看官方接口说明！点击这里查看官方接口说明！点击这里查看官方接口说明！](https://op.jinritemai.com/docs/api-docs/61/3129)  
[点击这里查看官方接口说明！点击这里查看官方接口说明！点击这里查看官方接口说明！](https://op.jinritemai.com/docs/api-docs/61/3129)  
 |

**相关说明：**

|  | 名称 | 类型 | 说明 |
| --- | --- | --- | --- |
|  |

**JSON返回示例：**

```json
{
  "data": {
    "activity\_material\_list": \[
      {
        "activity\_id": "1000",
        "activity\_main\_cover": "https://xxxxxx/xxx",
        "activity\_name": "超值购",
        "activity\_preview\_cover": "https://xxxxxx/xxx",
        "activity\_promotion\_asset": "https://xxxxxx/xxx",
        "activity\_promotion\_cover": "https://xxxxxx/xxx",
        "activity\_rule": "活动推广规则",
        "activity\_rule\_link": "https://xxxxxx/xxx",
        "activity\_status": "2",
        "activity\_type": "0",
        "activity\_validity\_type": "0",
        "end\_time": "1234567890",
        "material\_id": "7208154716478685497",
        "start\_time": "1234567890"
      }
    \],
    "total": "100"
  },
  "code": 10000,
  "msg": "success",
  "sub\_code": "",
  "sub\_msg": ""
}
```

---

## 抖音直播间达人列表API

**接口名称：**抖音直播间达人列表API

**接口地址：**https://api.zhetaoke.com:10001/api/open\_douyin\_zhibojian\_dare\_list.ashx

**返回数据：**json

**请求方式：**get或者post

**请求示例：**https://api.zhetaoke.com:10001/api/open\_douyin\_zhibojian\_dare\_list.ashx?appkey=&sid=

抖音直播间达人列表API。  
  

**请求参数说明：**

|  | 名称 | 类型 | 必填 | 说明 |
| --- | --- | --- | --- | --- |
|   | appkey | String | 是 | 折淘客对接秘钥appkey，折淘客通用。 |
|   | sid | String | 是 | 对应的淘客账号授权SID。 |
|   | author\_type | String | 否 | 1 品牌 2 达人(默认值)  
 |
|   | author\_levels | String | 否 | 作者电商等级，0-7级 |
|   | frist\_cids | String | 否 | 商品行业类目，列表类型；使用【/alliance/activityProductCategoryList】接口获取 |
|   | author\_info | String | 否 | 达人昵称或者账号 |
|   | page | String | 否 | 分页，1开始 |
|   | page\_size | String | 否 | 分页大小 |
|   | sort\_by | String | 否 | 排序字段: 1-综合；2-销量；3-佣金率；4-粉丝数 |
|   | sort\_type | String | 否 | 排序方式：0-降序；1-升序 |
|   | live\_status | String | 否 | 开关播状态：0:所有，1:开播，2:关播 |
|   |  |  |  | [点击这里查看官方接口说明！点击这里查看官方接口说明！点击这里查看官方接口说明！](https://op.jinritemai.com/docs/api-docs/61/1396)  
[点击这里查看官方接口说明！点击这里查看官方接口说明！点击这里查看官方接口说明！](https://op.jinritemai.com/docs/api-docs/61/1396)  
[点击这里查看官方接口说明！点击这里查看官方接口说明！点击这里查看官方接口说明！](https://op.jinritemai.com/docs/api-docs/61/1396)  
 |

**相关说明：**

|  | 名称 | 类型 | 说明 |
| --- | --- | --- | --- |
|  |

**JSON返回示例：**

```json
{
  "data": {
    "data": {
      "coupon\_link": {
        "coupon\_status": "0",
        "deeplink": "snssdk1128://ec\_goods\_detail/?promotion\_id=3471556980941674",
        "qrcode": {
          "height": "360",
          "url": "https://p3.douyinpic.com/img/aweme-qrcode/PqFhya700510763792421~c5\_360x360.webp?from=1723184",
          "width": "360"
        },
        "share\_command": "长按覆\[呲牙\]zhi此条消息，咑\[玫瑰\]搜\[烟花\]，查看商品详情Ig9FOHrrl8ηη",
        "share\_link": "https://alliance.jinritemai.com"
      },
      "dy\_deeplink": "nssdk1128://ec\_goods\_detail/?promotion\_id=3471556980941674&kol\_id=99575927&so",
      "dy\_password": "长按覆\[呲牙\]zhi此条消息，咑\[玫瑰\]搜\[烟花\]，查看商品详情Ig9FOHrrl8ηη",
      "dy\_zlink": "https://z.douyin.com/xxxxxxx",
      "qr\_code": {
        "height": "360",
        "url": "https://p3.douyinpic.com/img/aweme-qrcode/PqFhya700510763792421~c5\_360x360.webp?from=1723184",
        "width": "360"
      },
      "share\_link": "https://haohuo.jinritemai.com/ecommerce/trade/detail/index.html?id=",
      "use\_ins\_activity": "0"
    }
  },
  "code": 10000,
  "msg": "success",
  "sub\_code": "",
  "sub\_msg": ""
}
```

---

## 抖音直播间商品列表API

**接口名称：**抖音直播间商品列表API

**接口地址：**https://api.zhetaoke.com:10001/api/open\_douyin\_distributionLive\_productlist.ashx

**返回数据：**json

**请求方式：**get或者post

**请求示例：**https://api.zhetaoke.com:10001/api/open\_douyin\_distributionLive\_productlist.ashx?appkey=&sid=&author\_buyin\_id=

抖音直播间商品列表API。  
  

**请求参数说明：**

|  | 名称 | 类型 | 必填 | 说明 |
| --- | --- | --- | --- | --- |
|   | appkey | String | 是 | 折淘客对接秘钥appkey，折淘客通用。 |
|   | sid | String | 是 | 对应的淘客账号授权SID。 |
|   | author\_buyin\_id | String | 否 | 主播的百应ID  
 |
|   |  |  |  | [点击这里查看官方接口说明！点击这里查看官方接口说明！点击这里查看官方接口说明！](https://op.jinritemai.com/docs/api-docs/61/1770)  
[点击这里查看官方接口说明！点击这里查看官方接口说明！点击这里查看官方接口说明！](https://op.jinritemai.com/docs/api-docs/61/1770)  
[点击这里查看官方接口说明！点击这里查看官方接口说明！点击这里查看官方接口说明！](https://op.jinritemai.com/docs/api-docs/61/1770)  
 |

**相关说明：**

|  | 名称 | 类型 | 说明 |
| --- | --- | --- | --- |
|  |

**JSON返回示例：**

```json
{
  "data": {
    "author\_buyin\_id": "75678931425",
    "author\_nickname": "测试用户",
    "create\_time": "1649841070",
    "flash\_products": \[
      {
        "ads\_promotion\_ratio": {
          "old\_fans\_ratio": "500",
          "share\_ratio": "1000"
        },
        "available\_coupons": \[
          {
            "coupon\_type": "1",
            "discount\_desc": "满50减10券",
            "type\_desc": "平台券"
          }
        \],
        "category\_id": "29187",
        "cover": "https://sf1-ttcdn-tos.pstatp.com/img/temai/Fpt8famR3twTGYz09e25nz5PbTRuwww800-800~1125x1125\_q50.webp",
        "first\_cid": "20018",
        "first\_cname": "零食/坚果/特产",
        "given\_products": \[
          {
            "cover": "https://sf1-ttcdn-tos.pstatp.com/img/temai/Fpt8famR3twTGYz09e25nz5PbTRuwww800-800~1125x1125\_q50.webp",
            "given\_skus": \[
              {
                "cover": "https://sf1-ttcdn-tos.pstatp.com/img/temai/Fpt8famR3twTGYz09e25nz5PbTRuwww800-800~1125x1125\_q50.webp",
                "product\_id": "2323109090032130190",
                "sku\_id": "2323109090032130191"
              }
            \],
            "product\_id": "2323109090032130190",
            "product\_name": "赠品-名称"
          }
        \],
        "is\_explaining": "false",
        "list\_num": "1",
        "live\_room\_price": "900",
        "price": "1000",
        "product\_id": "3094093204930502",
        "second\_cid": "20313",
        "second\_cname": "饼干/膨化",
        "third\_cid": "22977",
        "third\_cname": "饼干",
        "title": "测试商品岩烧芝士脆饼干"
      }
    \],
    "flash\_products\_count": "1",
    "has\_kol\_coupon": "false",
    "is\_flash\_live\_room": "false",
    "is\_rest\_assured": "false",
    "lottery\_products": \[
      {
        "lottery\_activity\_id": "111112222",
        "lottery\_product\_end\_time": "1674970993",
        "lottery\_product\_start\_time": "1674960993",
        "product\_info": {
          "ads\_promotion\_ratio": {
            "old\_fans\_ratio": "5000",
            "share\_ratio": "1000"
          },
          "available\_coupons": \[
            {
              "coupon\_type": "1",
              "discount\_desc": "满50减10",
              "type\_desc": "平台券"
            }
          \],
          "category\_id": "20098",
          "cover": "https://sf1-ttcdn-tos.pstatp.com/img/temai/Fpt8famR3twTGYz09e25nz5PbTRuwww800-800~1125x1125\_q50.webp",
          "first\_cid": "20088",
          "first\_cname": "零食/坚果/特产",
          "given\_products": \[
            {
              "cover": "https://sf1-ttcdn-tos.pstatp.com/img/temai/Fpt8famR3twTGYz09e25nz5PbTRuwww800-800~1125x1125\_q50.webp",
              "given\_skus": \[
                {
                  "cover": "https://sf1-ttcdn-tos.pstatp.com/img/temai/Fpt8famR3twTGYz09e25nz5PbTRuwww800-800~1125x1125\_q50.webp",
                  "product\_id": "88888",
                  "sku\_id": "8888"
                }
              \],
              "product\_id": "88888",
              "product\_name": "测试商品"
            }
          \],
          "is\_explaining": "true",
          "list\_num": "1",
          "live\_room\_price": "1000",
          "price": "1000",
          "product\_id": "3094093204930502",
          "second\_cid": "20089",
          "second\_cname": "饼干/膨化",
          "third\_cid": "20099",
          "third\_cname": "饼干",
          "title": "测试商品"
        }
      }
    \],
    "online\_num": "21231",
    "product\_count": "100",
    "products": \[
      {
        "ads\_promotion\_ratio": {
          "old\_fans\_ratio": "500",
          "share\_ratio": "1000"
        },
        "available\_coupons": \[
          {
            "coupon\_type": "1",
            "discount\_desc": "满50减10券",
            "type\_desc": "平台券"
          }
        \],
        "category\_id": "29187",
        "cover": "https://p6-aio.ecombdimg.com/obj/temai/6ef6ccb6699c9865be7e68c325bf22e7w",
        "first\_cid": "20018",
        "first\_cname": "零食/坚果/特产",
        "given\_products": \[
          {
            "cover": "https://sf1-ttcdn-tos.pstatp.com/img/temai/Fpt8famR3twTGYz09e25nz5PbTRuwww800-800~1125x1125\_q50.webp",
            "given\_skus": \[
              {
                "cover": "https://sf1-ttcdn-tos.pstatp.com/img/temai/Fpt8famR3twTGYz09e25nz5PbTRuwww800-800~1125x1125\_q50.webp",
                "product\_id": "2323109090032130190",
                "sku\_id": "2323109090032130191"
              }
            \],
            "product\_id": "2323109090032130190",
            "product\_name": "赠品-名称"
          }
        \],
        "is\_explaining": "true",
        "list\_num": "1",
        "live\_room\_price": "900",
        "price": "1000",
        "product\_id": "3094093204930502",
        "second\_cid": "20313",
        "second\_cname": "饼干/膨化",
        "third\_cid": "22977",
        "third\_cname": "饼干",
        "title": "测试商品岩烧芝士脆饼干"
      }
    \],
    "room\_id": "1292893292189385392859439"
  },
  "code": 10000,
  "msg": "success",
  "sub\_code": "",
  "sub\_msg": ""
}
```

---

