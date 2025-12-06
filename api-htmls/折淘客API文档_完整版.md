# 折淘客 API 接口文档汇总

> 生成时间: 2025/12/6 14:47:23

[TOC]

## 批量高佣转链API(淘口令)

**原始链接**: [批量高佣转链API(淘口令)](https://www.zhetaoke.com/user/open/open_gaoyongzhuanlian_tkl_piliang.aspx)

**批量高佣转链API（淘口令）---本接口采用12台16核心64G内存服务器，江苏和北京双机房，负载均衡技术，1G独享带宽，放心调用。**

手淘里拷贝出来的淘口令文案，必须输入代理类型sid、渠道pid和渠道rid，否则无法转链。  
代理类型的sid为返利模式，必须输入用户的渠道rid，否则无法转链。  
目前转链分代理类型、发单类型、导购类型等，如果转链有问题或者搞不清楚就全部授权即可，后台接口自动判断使用哪个授权类型。  

批量高佣转链API（淘口令）接口---将第三方淘口令文案转成自己的淘口令文案，只替换淘口令和链接，支持淘宝、京东、拼多多、苏宁、唯品会、美团等平台 ：[淘宝开放平台文档》》](https://open.taobao.com/api.htm?docId=65412&docType=2&scopeId=28320)

**接口地址：**https://api.zhetaoke.com:10001/api/open\_gaoyongzhuanlian\_tkl\_piliang.ashx

**请求实例：**https://api.zhetaoke.com:10001/api/open\_gaoyongzhuanlian\_tkl\_piliang.ashx?appkey=0a565628b2fb4712902712734cec2feb&sid=#sid#&pid=#pid#&tkl=￥TdJCbN68klT￥

**备用地址：**http://api.zhetaoke.cn:10000/api/open\_gaoyongzhuanlian\_tkl\_piliang.ashx

**特殊说明：**[主接口和备用接口详细说明（请大家优先使用com主域名接口地址，在com出现问题的时候，人工或者自动切换cn备用接口地址）](https://www.zhetaoke.com/help_detail_3_27.html)

**返回数据：**json

**请求方式：**get或者post

**请求参数说明：**

|  | 名称 | 类型 | 必填 | 说明 |
| --- | --- | --- | --- | --- |
|   | appkey | string | 是 | 折淘客的对接秘钥appkey |
|   | sid | string | 是 | 对应的淘客账号授权ID[点击查看](https://www.zhetaoke.com/user/shouquan.html) |
|   | pid | string | 是 | 淘客PID，mm\_xxx\_xxx\_xxx,三段格式，必须与授权的账户相同，否则出错 |
|   | unionId | string | 否 | 京东联盟ID，为一串数字。 |
|   | tkl | string | 是 | 淘口令文案,支持多个淘口令或者多个链接。请注意，该参数需要进行Urlencode编码后传入。  
0、支持淘宝、京东、拼多多、苏宁、唯品会、美团等平台；  
1、支持多个淘宝客商品淘口令和链接文案，免费；  
2、支持多个店铺淘口令和链接文案，免费；  
3、支持多个饿了么淘口令和链接文案，免费；  
4、支持多个优惠券淘口令和链接文案，免费；  
5、支持多个口碑淘口令和链接文案，免费；  
6、支持多个官方活动淘口令和链接文案，免费；  
7、支持其他类型和部分非淘宝客商品淘口令，需要用万能接口，需要金币；  
重要的事情说三遍：该参数需要进行Urlencode编码！该参数需要进行Urlencode编码！该参数需要进行Urlencode编码！ |
|   | relation\_id | string | 否 | 淘宝渠道关系ID，仅适用于淘宝渠道推广场景，只有文案为淘宝文案，该字段才有效。返利专用字段。  
 |
|   | positionId | string | 否 | 京东渠道关系ID，仅适用于京东渠道推广场景，只有文案为京东文案，该字段才有效。返利专用字段。  
自定义推广位id，重要的事情说三遍：该参数可以自定义数字！该参数可以自定义数字！该参数可以自定义数字！  
自定义的数字，自己在本地跟用户做好关联，订单中会透出自定义的数字。如果返利需要用到此字段，如果是导购，不需要此字段。 |
|   | custom\_parameters | string | 否 | 拼多多渠道关系ID，自定义参数，为链接打上自定义标签；  
自定义参数最长限制64个字节；格式为： {"uid":"15611448080","sid":""} ，其中 uid 用户唯一标识，可自行加密后传入，每个用户仅且对应一个标识，必填；  
重要的事情说三遍：该参数需要进行Urlencode编码！该参数需要进行Urlencode编码！该参数需要进行Urlencode编码！  
重要的事情说三遍：返利才需要此参数，导购不需要！返利才需要此参数，导购不需要！返利才需要此参数，导购不需要！  
 |
|   | chanTag | string | 否 | 唯品会渠道关系ID，仅适用于唯品会渠道推广场景，只有文案为唯品会文案，该字段才有效。返利专用字段。  
渠道标识，即推广位PID。 |
|   | customer\_id | string | 否 | 美团渠道关系ID，仅适用于美团渠道推广场景，只有文案为美团文案，该字段才有效。返利专用字段。  
自定义参数，只允许数字，最大长度11位。此字段可以用来区分用户，进行下级返利。 |

**成功返回示例：**

var tmp = { "status": 200, "content": "钻石牌五叶落地扇16英寸\\r\\n(WYKR31Vg0I5) AC66/", "tao\_id": "Yb30xKJsptXrZgegrkiMRcQU7-XZ8675f0AoG9yB4SwV", "plat": "1", "plat2": "2", "is\_youquan": "1", "pic": "https://img.alicdn.com/bao/uploaded/i4/2217887206483/O1CN01n2ZCly1xlGeaQKTSn\_!!0-item\_pic.jpg", "cid1": "50012100", "cid1\_name": "生活电器", "cid2": "50008557", "cid2\_name": "电风扇", "cid3": "", "cid3\_name": "", "volume": "40000", "commentCount": "13333", "title": "家用电风扇钻石牌遥控落地扇", "tao\_title": "钻石牌家用电风扇落地式风扇宿舍台式遥控办公室静音大风力摇头扇", "size": "69", "quanhou\_jiage": "49.1", "tkrate3": "15.01", "tkfee3": "7.37", "coupon\_info\_money": "13", "nick": "钻石牌桦键炜专卖店", "result\_tkl": "(WYKR31Vg0I5)", "result\_url": "https://uland.taobao.com/coupon/edetail?e=zEX8Bh33ICSlhe416e52", "chunwenzi": "0" } document.write("<pre>" + syntaxHighlight(tmp) + "</pre>")
```json
{
    "status": 200,         /\*状态，200表示成功，301表示失败\*/
    "content": "%e5%8d%b3%e5%8f%af%e9%a2%86%e5%88%b8%e8%b4%ad%e7%89%a9%e4%b8%8b%e5%8d%95+(lP5KcEYGJo9)",   /\*自己的文案\*/
    "tao\_id": "614876391300",         /\*文案中最后一个商品的ID\*/
    "plat": "1",         /\*1：淘宝，2：京东，3：拼多多，4：苏宁，5：唯品会，6：美团\*/
    "plat2": "1",         /\*该字段仅对plat=1有效，1：淘宝，2：天猫，3：天猫超市\*/
    "is\_youquan": "1",         /\*是否有优惠券\*/
    "pic": "https://img.alicdn.com/bao/uploaded/i3/O1CN01g9wxcL1ygVrNE7HTc\_!!0-item\_pic.jpg",        /\*文案中最后一个商品的商品主图\*/
    "cid1": "50012100",         /\*一级分类ID\*/
    "cid1\_name": "生活电器",         /\*一级分类名称\*/
    "cid2": "50008557",         /\*二级分类ID\*/
    "cid2\_name": "电风扇",         /\*二级分类名称\*/
    "cid3": "",         /\*三级分类ID\*/
    "cid3\_name": "",         /\*三级分类名称\*/
    "volume": "40000",         /\*总销量\*/
    "commentCount": "13333",         /\*评论数量\*/
    "title": "家用电风扇钻石牌遥控落地扇",         /\*商品短标题\*/
    "tao\_title": "钻石牌家用电风扇落地式风扇宿舍台式遥控办公室静音大风力摇头扇",         /\*商品长标题\*/
    "size": "69",         /\*原始价格\*/
    "quanhou\_jiage": "49.1",         /\*券后价格\*/
    "tkrate3": "15.01",         /\*佣金比例\*/
    "tkfee3": "7.37",         /\*佣金金额\*/
    "coupon\_info\_money": "13",         /\*优惠券金额\*/
    "nick": "钻石牌桦键炜专卖店",         /\*店铺昵称\*/
    "result\_tkl": (Y1vH2lT9emg),         /\*生成的推广淘口令\*/
    "result\_url": https://uland.taobao.com/coupon/edetail?e=MHOV6g5nQUWlhHvvyUNXZfh8CuWt5YH5OVuOuRD5gLJMm99\_2709@01,        /\*生成的推广链接\*/
    "chunwenzi": "0"         /\*文案是否纯文字，0否 1是\*/
}
```

接口调用说明：  
1、所有接口均免费提供，方便大家开发和接入自己的产品早日成为大牛淘客！  
2、使用高佣转链接口，生成您的淘客链接会自动申请最高佣金，包括鹊桥、定向计划、和营销计划（建议用户点击商品时触发式调用，降低服务器压力）！  
3、如需指定优惠券，在生成的二合一链接后，加入 &activityId=xxx券ID即可走指定券  
4、signurl=3，自动拼接s券规则：如果S券金额大于G券金额，用S券；如果有S券，无G券，用S券等。用signurl=3，不需要再调用解析商品编号API接口了。  

[接口测试](https://www.zhetaoke.com/user/open/test_open_gaoyongzhuanlian_tkl_piliang.aspx)

[高佣转链说明](http://www.zhetaoke.com/help_detail_3_25.html)

---

## 批量高佣转链API(商品ID)

**原始链接**: [批量高佣转链API(商品ID)](https://www.zhetaoke.com/user/open/open_gaoyongzhuanlian_piliang.aspx)

**批量高佣转链API(商品ID)---本接口采用12台16核心64G内存服务器，江苏和北京双机房，负载均衡技术，1G独享带宽，放心调用。**

批量高佣转链API(商品ID)---可同时获取20个渠道ID的高佣转链结果：[淘宝开放平台文档》》](https://open.taobao.com/api.htm?docId=65412&docType=2&scopeId=28320)

**接口地址：**https://api.zhetaoke.com:10001/api/open\_gaoyongzhuanlian\_piliang.ashx

**请求实例：**https://api.zhetaoke.com:10001/api/open\_gaoyongzhuanlian\_piliang.ashx?appkey=0a565628b2fb4712902712734cec2feb&sid=#sid#&pid=#pid#&num\_iid=#num\_iid#&relation\_id=11111,22222,33333&signurl=5

**备用地址：**http://api.zhetaoke.cn:10000/api/open\_gaoyongzhuanlian\_piliang.ashx

**特殊说明：**[主接口和备用接口详细说明（请大家优先使用com主域名接口地址，在com出现问题的时候，人工或者自动切换cn备用接口地址）](https://www.zhetaoke.com/help_detail_3_27.html)

**返回数据：**json

**请求方式：**get或者post

**请求参数说明：**

|  | 名称 | 类型 | 必填 | 说明  [接口在线测试](https://www.zhetaoke.com/user/open/test_open_gaoyongzhuanlian.aspx) |
| --- | --- | --- | --- | --- |
|   | appkey | string | 是 | 折淘客的对接秘钥appkey |
|   | sid | string | 是 | 对应的淘客账号授权ID[点击查看](https://www.zhetaoke.com/user/shouquan.html) |
|   | pid | string | 是 | 淘客PID，mm\_xxx\_xxx\_xxx,三段格式，必须与授权的账户相同，否则出错 |
|   | num\_iid | string | 是 | 商品ID,商品ID必须填 |
|   | relation\_id | string | 是 | 渠道关系ID，仅适用于渠道推广场景。  
多个渠道关系ID，用英文逗号,分割，最大20个。  
例子：111111,22222,33333,44444,55555  
该接口应用场景:可同时获取20个渠道ID的高佣转链结果，包括淘口令，适合同时发几百个群甚至几千个群的云发单场景。  
 |
|   | xid | string | 否 | 团长与下游渠道合作的特殊标识，用于统计渠道推广效果 |
|   | signurl | int | 否 | signurl=5，返回结果整合高佣转链API、解析商品编号API、全网商品详情API、淘口令创建API，已经自动判断和拼接使用全网G券还是全网S券。  
此接口目前只支持signurl=5，其它值暂时不支持。  
 |

**signurl=5，成功返回示例：/\*signurl=5，返回结果整合高佣转链API、解析商品编号API、全网商品详情API、淘口令创建API，已经自动判断和拼接使用全网G券还是全网S券\*/**

var tmp = { "status": 200, /\*状态\*/ "content": \[{ "code": "6646", /\*折淘客编号\*/ "type\_one\_id": "1", /\*分类ID，可参考折淘客分类\*/ "tao\_id": "554832820990", /\*商品ID\*/ "title": "储物箱整理抽屉式收纳柜", /\*商品短标题\*/ "jianjie": "【爆款返场，出口日本】进口PP材质，可叠加透明可视抽屉收纳盒，分格抽屉物品分类齐全，区域分化一目了然，自由组合，大容量，让你居家生活更舒心，收纳你的生活！【赠运费险】", /\*商品简介\*/ "pict\_url": "https://img.alicdn.com/bao/......jpg", /\*商品主图\*/ "user\_type": "0", /\*是否天猫，0是淘宝，1是天猫\*/ "seller\_id": "3073204721", /\*卖家ID\*/ /\*店铺地址可自行拼接，http://store.taobao.com/shop/view\_shop.htm?user\_number\_id=3073204721 \*/ "shop\_dsr": "4.80", /\*商品描述分\*/ "volume": "169", /\*年销量\*/ "size": "26.90", /\*折扣价\*/ "quanhou\_jiage": "16.90", /\*券后价\*/ "date\_time\_yongjin": "2018/11/8 14:45:22", /\*数据更新时间\*/ "tkrate3": "30.00", /\*佣金比率\*/ "yongjin\_type": "MKT", /\*佣金类型\*/ "coupon\_id": "fe31d6c8132c462ba59bd72e03d2eae0", /\*优惠券ID\*/ "coupon\_start\_time": "2018-11-05", /\*优惠券开始时间\*/ "coupon\_end\_time": "2018-11-10", /\*优惠券结束时间\*/ "coupon\_info\_money": "10", /\*优惠券金额\*/ "coupon\_total\_count": "100000", /\*优惠券总数量\*/ "coupon\_remain\_count": "99965", /\*优惠券剩余数量\*/ "coupon\_info": "满80.00元减10元", /\*优惠券信息\*/ "juhuasuan": "0", /\*是否聚划算，1是\*/ "taoqianggou": "0", /\*是否淘抢购，1是\*/ "haitao": "0", /\*是否海淘，1是\*/ "jiyoujia": "0", /\*是否极有家，1是\*/ "jinpaimaijia": "0", /\*是否金牌卖家，1是\*/ "pinpai": "0", /\*是否精选品牌，1是\*/ "pinpai\_name": "", /\*品牌名称\*/ "yunfeixian": "0", /\*是否有运费险，1有\*/ "nick": "\*\*\*旗舰店", /\*卖家昵称\*/ "small\_images": "https://img.alicdn.com/bao/.......jpg|https://img.alicdn.com/bao/.......jpg", /\*商品小图列表\*/ "white\_image": "https://img.alicdn.com/bao/......jpg", /\*商品白底图\*/ "tao\_title": "抽屉式收纳箱塑料储物箱整理箱衣服物家用宿舍衣柜内衣内裤收纳盒", /\*商品长标题\*/ "provcity": "浙江 杭州", /\*宝贝所在地\*/ "shop\_title": "\*\*\*旗舰店", /\*店铺名称\*/ "zhibo\_url": "https://cloud.video.taobao.com/play/u/849090736/p/2/e/6/t/1/219853683676.mp4", /\*视频地址\*/ "sellCount": "860000", /\*淘宝网页实时总销量\*/ "commentCount": "250000", /\*评论数量\*/ "favcount": "180000", /\*收藏数量\*/ "score1": "4.9", /\*宝贝描述分\*/ "score2": "4.9", /\*卖家服务分\*/ "score3": "4.9", /\*物流服务\*/ "creditLevel": "15", /\*店铺等级，一星 二星 三星 四星 五星 一钻 二钻 三钻 四钻 五钻 一皇冠 二皇冠 三皇冠 四皇冠 五皇冠 一金冠 二金冠 三金冠 四金冠 五金冠\*/ "shopIcon": "//img.alicdn.com/imgextra/de/a6/TB10dC8SX.jpg", /\*店铺logo\*/ "pcDescContent": "1.jpg|2.jpg|3.jpg|4.jpg|5.jpg", /\*图文详情图片地址\*/ "taobao\_url": "https://item.taobao.com/item.htm?id=581801364036", /\*商品url\*/ "category\_id": "50006126", /\*叶子类目id\*/ "category\_name": "收纳盒", /\*叶子类目name\*/ "level\_one\_category\_id": "122928002", /\*一级类目id\*/ "level\_one\_category\_name": "收纳整理", /\*一级类目name\*/ "tkfee3": "5.07", /\*返佣金额\*/ "min\_commission\_rate": "30.00", "coupon\_click\_url": "https://uland.taobao.com/coupon/edetail?e=xxxx&activityId=a64bdbb5bb8440be99d31344fa7e4500", "item\_url": "https://s.click.taobao.com/t?e=xxxx", "shorturl": "https://s.tb.cn/h.f77lBsX", "shorturl2": "https://s.click.taobao.com/YkpZ4zu", "relation\_id": "11111" }\] };
```json
{
	"status": 200,                      /\*状态\*/
	"content": \[{
		"code": "6646",                 /\*折淘客编号\*/
		"type\_one\_id": "1",             /\*分类ID，可参考折淘客分类\*/
		"tao\_id": "554832820990",       /\*商品ID\*/
		"title": "储物箱整理抽屉式收纳柜",                   /\*商品短标题\*/
		"jianjie": "【爆款返场，出口日本】进口PP材质，可叠加透明可视抽屉收纳盒，分格抽屉物品分类齐全，区域分化一目了然，自由组合，大容量，让你居家生活更舒心，收纳你的生活！【赠运费险】", /\*商品简介\*/
		"pict\_url": "https://img.alicdn.com/bao/......jpg",                   /\*商品主图\*/
		"user\_type": "0",                   /\*是否天猫，0是淘宝，1是天猫\*/
		"seller\_id": "3073204721",                   /\*卖家ID\*/ /\*店铺地址可自行拼接，http://store.taobao.com/shop/view\_shop.htm?user\_number\_id=3073204721 \*/
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
		"coupon\_total\_count": "100000",                   /\*优惠券总数量\*/
		"coupon\_remain\_count": "99965",                   /\*优惠券剩余数量\*/
		"coupon\_info": "满80.00元减10元",                   /\*优惠券信息\*/
		"juhuasuan": "0",                       /\*是否聚划算，1是\*/
		"taoqianggou": "0",                     /\*是否淘抢购，1是\*/
		"haitao": "0",                          /\*是否海淘，1是\*/
		"jiyoujia": "0",                        /\*是否极有家，1是\*/
		"jinpaimaijia": "0",                    /\*是否金牌卖家，1是\*/
		"pinpai": "0",                          /\*是否精选品牌，1是\*/
		"pinpai\_name": "",                      /\*品牌名称\*/
		"yunfeixian": "0",                      /\*是否有运费险，1有\*/
		"nick": "\*\*\*旗舰店",                    /\*卖家昵称\*/
		"small\_images": "https://img.alicdn.com/bao/.......jpg|https://img.alicdn.com/bao/.......jpg",                   /\*商品小图列表\*/
		"white\_image": "https://img.alicdn.com/bao/......jpg",                   /\*商品白底图\*/
		"tao\_title": "抽屉式收纳箱塑料储物箱整理箱衣服物家用宿舍衣柜内衣内裤收纳盒",                   /\*商品长标题\*/
		"provcity": "浙江 杭州",                   /\*宝贝所在地\*/
		"shop\_title": "\*\*\*旗舰店",                   /\*店铺名称\*/
		"zhibo\_url": "https://cloud.video.taobao.com/play/u/849090736/p/2/e/6/t/1/219853683676.mp4",                   /\*视频地址\*/
		"sellCount": "860000",                   /\*淘宝网页实时总销量\*/
		"commentCount": "250000",                   /\*评论数量\*/
		"favcount": "180000",                   /\*收藏数量\*/
		"score1": "4.9",                   /\*宝贝描述分\*/
		"score2": "4.9",                   /\*卖家服务分\*/
		"score3": "4.9",                   /\*物流服务分\*/
		"creditLevel": "15",                   /\*店铺等级（1-20），一星 二星 三星 四星 五星 一钻 二钻 三钻 四钻 五钻 一皇冠 二皇冠 三皇冠 四皇冠 五皇冠  一金冠 二金冠 三金冠 四金冠 五金冠\*/
		"shopIcon": "//img.alicdn.com/imgextra/de/a6/TB10dC8SX.jpg",       /\*店铺logo\*/
		"pcDescContent": "1.jpg|2.jpg|3.jpg|4.jpg|5.jpg",  /\*图文详情图片地址\*/
		"taobao\_url": "https://item.taobao.com/item.htm?id=581801364036",  /\*商品url\*/
		"category\_id": "50006126",                   /\*叶子类目id\*/
		"category\_name": "收纳盒",                   /\*叶子类目name\*/
		"level\_one\_category\_id": "122928002",                   /\*一级类目id\*/
		"level\_one\_category\_name": "收纳整理",                   /\*一级类目name\*/
		"tkfee3": "5.07",                   /\*返佣金额\*/
		"min\_commission\_rate": "30.00",          /\*当入参special\_id、relation\_id、external\_id时，该字段展示预估最低佣金率\*/
		"coupon\_click\_url": "https://uland.taobao.com/coupon/edetail?e=xxxx&activityId=a64bdbb5bb8440be99d31344fa7e4500",          /\*二合一推广链接，已经自动拼接S券\*/
		"item\_url": "https://s.click.taobao.com/t?e=xxxx",          /\*推广长链接，如果是渠道ID，请自行拼接上relationId的信息,否则订单信息中可能查不到渠道信息\*/
		"shorturl": "https://s.click.taobao.com/YkpZ4zu"          /\*淘宝短链接\*/
		"shorturl2": "https://s.tb.cn/h.f77lBsX"          /\*淘宝短链接2，此链接可以直接拉起手淘APP，只适用IOS。\*/
		"tkl": "￥Ry0tYxAT18H￥",          /\*淘口令\*/
		"relation\_id": "11111"          /\*渠道ID\*/
	}\]
}
```

**如果非淘客商品，返回示例：（此类商品无法获得佣金）**

var tmp = { "error\_response": { "code": 15, "msg": "Remote service error", "sub\_code": "isv.item-not-exist", "sub\_msg": "该item\_id对应宝贝已下架或非淘客宝贝", "request\_id": "10eoi3wvcot9l" } }; document.write("<pre>" + syntaxHighlight(tmp) + "</pre>")
```json
{
    "error\_response":{
        "code":15,
        "msg":"Remote service error",
        "sub\_code":"isv.item-not-exist",
        "sub\_msg":"该item\_id对应宝贝已下架或非淘客宝贝",
        "request\_id":"10eoi3wvcot9l"
    }
}
```

接口调用说明：  
1、所有接口均免费提供，方便大家开发和接入自己的产品早日成为大牛淘客！  
2、使用高佣转链接口，生成您的淘客链接会自动申请最高佣金，包括鹊桥、定向计划、和营销计划（建议用户点击商品时触发式调用，降低服务器压力）！  
3、如需指定优惠券，在生成的二合一链接后，加入 &activityId=xxx券ID即可走指定券  
4、signurl=3，自动拼接s券规则：如果S券金额大于G券金额，用S券；如果有S券，无G券，用S券等。用signurl=3，不需要再调用解析商品编号API接口了。  

[接口测试](https://www.zhetaoke.com/user/open/test_open_gaoyongzhuanlian.aspx)

[高佣转链说明](http://www.zhetaoke.com/help_detail_3_25.html)

---

## 高佣转链API(商品ID)

**原始链接**: [高佣转链API(商品ID)](https://www.zhetaoke.com/user/open/open_gaoyongzhuanlian.aspx)

**高佣转链API（商品ID）---本接口采用12台16核心64G内存服务器，江苏和北京双机房，负载均衡技术，1G独享带宽，放心调用。**

手淘里拷贝出来的淘口令文案，必须输入代理类型sid、渠道pid和渠道rid，否则无法转链。  
代理类型的sid为返利模式，必须输入用户的渠道rid，否则无法转链。  
目前转链分代理类型、发单类型、导购类型等，如果转链有问题或者搞不清楚就全部授权即可，后台接口自动判断使用哪个授权类型。  

高佣转链API接口：[淘宝开放平台文档》》](https://open.taobao.com/api.htm?docId=65412&docType=2&scopeId=28320)

**接口地址：**https://api.zhetaoke.com:10001/api/open\_gaoyongzhuanlian.ashx

**请求实例：**https://api.zhetaoke.com:10001/api/open\_gaoyongzhuanlian.ashx?appkey=0a565628b2fb4712902712734cec2feb&sid=#sid#&pid=#pid#&num\_iid=#num\_iid#&signurl=5

**备用地址：**http://api.zhetaoke.cn:10000/api/open\_gaoyongzhuanlian.ashx

**特殊说明：**[主接口和备用接口详细说明（请大家优先使用com主域名接口地址，在com出现问题的时候，人工或者自动切换cn备用接口地址）](https://www.zhetaoke.com/help_detail_3_27.html)

**返回数据：**json

**请求方式：**get或者post

**请求参数说明：**

|  | 名称 | 类型 | 必填 | 说明  [接口在线测试](https://www.zhetaoke.com/user/open/test_open_gaoyongzhuanlian.aspx) |
| --- | --- | --- | --- | --- |
|   | appkey | string | 是 | 折淘客的对接秘钥appkey |
|   | sid | string | 是 | 对应的淘客账号授权ID[点击查看](https://www.zhetaoke.com/user/shouquan.html) |
|   | pid | string | 是 | 淘客PID，mm\_xxx\_xxx\_xxx,三段格式，必须与授权的账户相同，否则出错 |
|   | num\_iid | string | 是 | 商品ID,商品ID必须填 |
|   | relation\_id | string | 否 | 渠道关系ID，仅适用于渠道推广场景。 |
|   | special\_id | string | 否 | 会员运营ID |
|   | signurl | int | 否 | signurl=5，返回结果整合高佣转链API、解析商品编号API、全网商品详情API、淘口令创建API，已经自动判断和拼接使用全网G券还是全网S券。  
signurl=4，返回结果整合高佣转链API、解析商品编号API、商品简版详情API、淘口令创建API，已经自动判断和拼接使用全网G券还是全网S券。  
signurl=3，返回结果整合高佣转链API、解析商品编号API，已经自动判断和拼接使用全网G券还是全网S券。  
signurl=0或1或2，返回官方高佣转链接口结果，需要自行判断和拼接使用全网G券或者全网S券。  
signurl=0，表示直接返回最终结果。  
signurl=1或2，表示直接返回最终结果。  
 |

**signurl=5，成功返回示例：/\*signurl=5，返回结果整合高佣转链API、解析商品编号API、全网商品详情API、淘口令创建API，已经自动判断和拼接使用全网G券还是全网S券\*/**

var tmp = { "status": 200, /\*状态\*/ "content": \[{ "code": "6646", /\*折淘客编号\*/ "type\_one\_id": "1", /\*分类ID，可参考折淘客分类\*/ "tao\_id": "554832820990", /\*商品ID\*/ "title": "储物箱整理抽屉式收纳柜", /\*商品短标题\*/ "jianjie": "【爆款返场，出口日本】进口PP材质，可叠加透明可视抽屉收纳盒，分格抽屉物品分类齐全，区域分化一目了然，自由组合，大容量，让你居家生活更舒心，收纳你的生活！【赠运费险】", /\*商品简介\*/ "pict\_url": "https://img.alicdn.com/bao/......jpg", /\*商品主图\*/ "user\_type": "0", /\*是否天猫，0是淘宝，1是天猫\*/ "seller\_id": "3073204721", /\*卖家ID\*/ /\*店铺地址可自行拼接，http://store.taobao.com/shop/view\_shop.htm?user\_number\_id=3073204721 \*/ "shop\_dsr": "4.80", /\*商品描述分\*/ "volume": "169", /\*年销量\*/ "size": "26.90", /\*折扣价\*/ "quanhou\_jiage": "16.90", /\*券后价\*/ "date\_time\_yongjin": "2018/11/8 14:45:22", /\*数据更新时间\*/ "tkrate3": "30.00", /\*佣金比率\*/ "yongjin\_type": "MKT", /\*佣金类型\*/ "coupon\_id": "fe31d6c8132c462ba59bd72e03d2eae0", /\*优惠券ID\*/ "coupon\_start\_time": "2018-11-05", /\*优惠券开始时间\*/ "coupon\_end\_time": "2018-11-10", /\*优惠券结束时间\*/ "coupon\_info\_money": "10", /\*优惠券金额\*/ "coupon\_total\_count": "100000", /\*优惠券总数量\*/ "coupon\_remain\_count": "99965", /\*优惠券剩余数量\*/ "coupon\_info": "满80.00元减10元", /\*优惠券信息\*/ "juhuasuan": "0", /\*是否聚划算，1是\*/ "taoqianggou": "0", /\*是否淘抢购，1是\*/ "haitao": "0", /\*是否海淘，1是\*/ "jiyoujia": "0", /\*是否极有家，1是\*/ "jinpaimaijia": "0", /\*是否金牌卖家，1是\*/ "pinpai": "0", /\*是否精选品牌，1是\*/ "pinpai\_name": "", /\*品牌名称\*/ "yunfeixian": "0", /\*是否有运费险，1有\*/ "nick": "\*\*\*旗舰店", /\*卖家昵称\*/ "small\_images": "https://img.alicdn.com/bao/.......jpg|https://img.alicdn.com/bao/.......jpg", /\*商品小图列表\*/ "white\_image": "https://img.alicdn.com/bao/......jpg", /\*商品白底图\*/ "tao\_title": "抽屉式收纳箱塑料储物箱整理箱衣服物家用宿舍衣柜内衣内裤收纳盒", /\*商品长标题\*/ "provcity": "浙江 杭州", /\*宝贝所在地\*/ "shop\_title": "\*\*\*旗舰店", /\*店铺名称\*/ "zhibo\_url": "https://cloud.video.taobao.com/play/u/849090736/p/2/e/6/t/1/219853683676.mp4", /\*视频地址\*/ "sellCount": "860000", /\*淘宝网页实时总销量\*/ "commentCount": "250000", /\*评论数量\*/ "favcount": "180000", /\*收藏数量\*/ "score1": "4.9", /\*宝贝描述分\*/ "score2": "4.9", /\*卖家服务分\*/ "score3": "4.9", /\*物流服务\*/ "creditLevel": "15", /\*店铺等级，一星 二星 三星 四星 五星 一钻 二钻 三钻 四钻 五钻 一皇冠 二皇冠 三皇冠 四皇冠 五皇冠 一金冠 二金冠 三金冠 四金冠 五金冠\*/ "shopIcon": "//img.alicdn.com/imgextra/de/a6/TB10dC8SX.jpg", /\*店铺logo\*/ "pcDescContent": "1.jpg|2.jpg|3.jpg|4.jpg|5.jpg", /\*图文详情图片地址\*/ "taobao\_url": "https://item.taobao.com/item.htm?id=581801364036", /\*商品url\*/ "category\_id": "50006126", /\*叶子类目id\*/ "category\_name": "收纳盒", /\*叶子类目name\*/ "level\_one\_category\_id": "122928002", /\*一级类目id\*/ "level\_one\_category\_name": "收纳整理", /\*一级类目name\*/ "tkfee3": "5.07", /\*返佣金额\*/ "min\_commission\_rate": "30.00", "coupon\_click\_url": "https://uland.taobao.com/coupon/edetail?e=xxxx&activityId=a64bdbb5bb8440be99d31344fa7e4500", "item\_url": "https://s.click.taobao.com/t?e=xxxx", "shorturl": "https://s.tb.cn/h.f77lBsX", "shorturl2": "https://s.click.taobao.com/YkpZ4zu", "tkl": "￥Ry0tYxAT18H￥" }\] };
```json
{
	"status": 200,                      /\*状态\*/
	"content": \[{
		"code": "6646",                 /\*折淘客编号\*/
		"type\_one\_id": "1",             /\*分类ID，可参考折淘客分类\*/
		"tao\_id": "554832820990",       /\*商品ID\*/
		"title": "储物箱整理抽屉式收纳柜",                   /\*商品短标题\*/
		"jianjie": "【爆款返场，出口日本】进口PP材质，可叠加透明可视抽屉收纳盒，分格抽屉物品分类齐全，区域分化一目了然，自由组合，大容量，让你居家生活更舒心，收纳你的生活！【赠运费险】", /\*商品简介\*/
		"pict\_url": "https://img.alicdn.com/bao/......jpg",                   /\*商品主图\*/
		"user\_type": "0",                   /\*是否天猫，0是淘宝，1是天猫\*/
		"seller\_id": "3073204721",                   /\*卖家ID\*/ /\*店铺地址可自行拼接，http://store.taobao.com/shop/view\_shop.htm?user\_number\_id=3073204721 \*/
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
		"coupon\_total\_count": "100000",                   /\*优惠券总数量\*/
		"coupon\_remain\_count": "99965",                   /\*优惠券剩余数量\*/
		"coupon\_info": "满80.00元减10元",                   /\*优惠券信息\*/
		"juhuasuan": "0",                       /\*是否聚划算，1是\*/
		"taoqianggou": "0",                     /\*是否淘抢购，1是\*/
		"haitao": "0",                          /\*是否海淘，1是\*/
		"jiyoujia": "0",                        /\*是否极有家，1是\*/
		"jinpaimaijia": "0",                    /\*是否金牌卖家，1是\*/
		"pinpai": "0",                          /\*是否精选品牌，1是\*/
		"pinpai\_name": "",                      /\*品牌名称\*/
		"yunfeixian": "0",                      /\*是否有运费险，1有\*/
		"nick": "\*\*\*旗舰店",                    /\*卖家昵称\*/
		"small\_images": "https://img.alicdn.com/bao/.......jpg|https://img.alicdn.com/bao/.......jpg",                   /\*商品小图列表\*/
		"white\_image": "https://img.alicdn.com/bao/......jpg",                   /\*商品白底图\*/
		"tao\_title": "抽屉式收纳箱塑料储物箱整理箱衣服物家用宿舍衣柜内衣内裤收纳盒",                   /\*商品长标题\*/
		"provcity": "浙江 杭州",                   /\*宝贝所在地\*/
		"shop\_title": "\*\*\*旗舰店",                   /\*店铺名称\*/
		"zhibo\_url": "https://cloud.video.taobao.com/play/u/849090736/p/2/e/6/t/1/219853683676.mp4",                   /\*视频地址\*/
		"sellCount": "860000",                   /\*淘宝网页实时总销量\*/
		"commentCount": "250000",                   /\*评论数量\*/
		"favcount": "180000",                   /\*收藏数量\*/
		"score1": "4.9",                   /\*宝贝描述分\*/
		"score2": "4.9",                   /\*卖家服务分\*/
		"score3": "4.9",                   /\*物流服务分\*/
		"creditLevel": "15",                   /\*店铺等级（1-20），一星 二星 三星 四星 五星 一钻 二钻 三钻 四钻 五钻 一皇冠 二皇冠 三皇冠 四皇冠 五皇冠  一金冠 二金冠 三金冠 四金冠 五金冠\*/
		"shopIcon": "//img.alicdn.com/imgextra/de/a6/TB10dC8SX.jpg",       /\*店铺logo\*/
		"pcDescContent": "1.jpg|2.jpg|3.jpg|4.jpg|5.jpg",  /\*图文详情图片地址\*/
		"taobao\_url": "https://item.taobao.com/item.htm?id=581801364036",  /\*商品url\*/
		"category\_id": "50006126",                   /\*叶子类目id\*/
		"category\_name": "收纳盒",                   /\*叶子类目name\*/
		"level\_one\_category\_id": "122928002",                   /\*一级类目id\*/
		"level\_one\_category\_name": "收纳整理",                   /\*一级类目name\*/
		"tkfee3": "5.07",                   /\*返佣金额\*/
		"min\_commission\_rate": "30.00",          /\*当入参special\_id、relation\_id、external\_id时，该字段展示预估最低佣金率\*/
		"coupon\_click\_url": "https://uland.taobao.com/coupon/edetail?e=xxxx&activityId=a64bdbb5bb8440be99d31344fa7e4500",          /\*二合一推广链接，已经自动拼接S券\*/
		"item\_url": "https://s.click.taobao.com/t?e=xxxx",          /\*推广长链接，如果是渠道ID，请自行拼接上relationId的信息,否则订单信息中可能查不到渠道信息\*/
		"shorturl": "https://s.click.taobao.com/YkpZ4zu"          /\*淘宝短链接\*/
		"shorturl2": "https://s.tb.cn/h.f77lBsX"          /\*淘宝短链接2，此链接可以直接拉起手淘APP，只适用IOS。\*/
		"tkl": "￥Ry0tYxAT18H￥",          /\*淘口令\*/
	}\]
}
```

**signurl=4，成功返回示例：/\*signurl=4，返回结果整合高佣转链API、解析商品编号API、全网商品简版详情API、淘口令创建API，已经自动判断和拼接使用全网G券还是全网S券\*/**

var tmp = { "category\_id": "1", "coupon\_click\_url": "https://uland.taobao.com/coupon/edetail?e=xxxx&activityId=a64bdbb5bb8440be99d31344fa7e4500", "coupon\_end\_time": "2018-09-26", "coupon\_info": "满40元减10元", "coupon\_amount": "10", "coupon\_startfee": "40", "coupon\_remain\_count": "6859", "coupon\_start\_time": "2018-09-25", "coupon\_total\_count": "8000", "coupon\_type": "3", "item\_id": "524136796550", "item\_url": "https://s.click.taobao.com/t?e=xxxx", "max\_commission\_rate": "20.3", "min\_commission\_rate": "20.3", "s\_coupon\_id": "a64bdbb5bb8440be99d31344fa7e4500", "s\_coupon\_amount": "20", "s\_coupon\_startfee": "40", "s\_coupon\_start\_time": "2018-09-25", "s\_coupon\_end\_time": "2018-09-26", "cat\_leaf\_name": "毛针织衫", "cat\_name": "女装/女士精品", "nick": "\*\*旗舰店", "pict\_url": "https://img.alicdn.com/bao/uploaded/pic.jpg", "provcity": "河北 石家庄", "seller\_id": "2687788888", "small\_images": "https://img.alicdn.com/3.jpg|https://img.alicdn.com/4.jpg", "title": "秋冬堆堆领羊绒衫女简约短款纯色高领毛衣套头修身针织羊毛打底衫", "user\_type": "1", "volume": "1000", "zk\_final\_price": "239", "shorturl": "https://s.click.taobao.com/YkpZ4zu", "shorturl2": "https://s.tb.cn/h.f77lBsX", "tkl": "￥Ry0tYxAT18H￥" }; document.write("<pre>" + syntaxHighlight(tmp) + "</pre>")
```json
{
    "category\_id": 1,          /\*后台一级类目\*/
    "coupon\_click\_url": "https://uland.taobao.com/coupon/edetail?e=xxxx&activityId=a64bdbb5bb8440be99d31344fa7e4500",          /\*二合一推广链接，已经自动拼接S券\*/
    "coupon\_end\_time": "2018-09-26",          /\*优惠券结束时间\*/
    "coupon\_info": "满40元减10元",          /\*优惠券面额\*/
    "coupon\_remain\_count": 6859          /\*优惠券剩余量\*/
    "coupon\_start\_time": "2018-09-25",          /\*优惠券开始时间\*/
    "coupon\_total\_count": 8000,          /\*优惠券总量\*/
    "coupon\_type": 3,           /\*优惠券(商品优惠券推广链接中的券)类型，1 公开券，2 私有券，3 妈妈券\*/
    "item\_id": "524136796550",          /\*商品id\*/
    "item\_url": "https://s.click.taobao.com/t?e=xxxx",          /\*推广长链接，如果是渠道ID，请自行拼接上relationId的信息,否则订单信息中可能查不到渠道信息\*/
    "max\_commission\_rate": "20.3",          /\*佣金比率(%)\*/
    "min\_commission\_rate": "20.3",          /\*当入参special\_id、relation\_id、external\_id时，该字段展示预估最低佣金率(%)\*/
    "s\_coupon\_id": "a64bdbb5bb8440be99d31344fa7e4500",          /\*特殊券ID，拼接规则参考下方文字\*/
    "s\_coupon\_amount": "10",          /\*特殊券面额\*/
    "s\_coupon\_startfee": "40",          /\*特殊券最低使用金额\*/
    "s\_coupon\_start\_time": "2018-09-25",          /\*特殊券开始时间\*/
    "s\_coupon\_end\_time": "2018-09-26",          /\*特殊券结束时间\*/
    "cat\_leaf\_name": "毛针织衫",          /\*叶子类目名称\*/
    "cat\_name": "女装/女士精品",          /\*一级类目名称\*/
    "nick": "\*\*旗舰店",          /\*卖家昵称\*/
    "pict\_url": "https://img.alicdn.com/bao/uploaded/pic.jpg",          /\*商品主图\*/
    "provcity": "河北 石家庄",          /\*宝贝所在地\*/
    "seller\_id": "2687788888",          /\*卖家ID\*/
    "small\_images": "https://img.alicdn.com/3.jpg|https://img.alicdn.com/4.jpg",          /\*商品小图列表\*/
    "title": "秋冬堆堆领羊绒衫女简约短款纯色高领毛衣套头修身针织羊毛打底衫",          /\*商品长标题\*/
    "user\_type": "1",          /\*是否天猫，0是淘宝，1是天猫\*/
    "volume": "1000",          /\*年销量\*/
    "zk\_final\_price": "239",          /\*折扣价\*/    
    "shorturl": "https://s.click.taobao.com/YkpZ4zu"          /\*淘宝短链接\*/
    "shorturl2": "https://s.tb.cn/h.f77lBsX"          /\*淘宝短链接2，此链接可以直接拉起手淘APP，只适用IOS。\*/
    "tkl": "￥Ry0tYxAT18H￥",          /\*淘口令\*/
}
```

**signurl=3，成功返回示例：/\*signurl=3，返回结果整合高佣转链API、解析商品编号API，已经自动判断和拼接使用全网G券还是全网S券\*/**

var tmp = { "category\_id": "1", "coupon\_click\_url": "https://uland.taobao.com/coupon/edetail?e=xxxx&activityId=a64bdbb5bb8440be99d31344fa7e4500", "coupon\_end\_time": "2018-09-26", "coupon\_info": "满40元减10元", "coupon\_amount": "10", "coupon\_startfee": "40", "coupon\_remain\_count": "6859", "coupon\_start\_time": "2018-09-25", "coupon\_total\_count": "8000", "coupon\_type": "3", "item\_id": "524136796550", "item\_url": "https://s.click.taobao.com/t?e=xxxx", "max\_commission\_rate": "20.3", "min\_commission\_rate": "20.3", "s\_coupon\_id": "a64bdbb5bb8440be99d31344fa7e4500", "s\_coupon\_amount": "20", "s\_coupon\_startfee": "40", "s\_coupon\_start\_time": "2018-09-25", "s\_coupon\_end\_time": "2018-09-26" }; document.write("<pre>" + syntaxHighlight(tmp) + "</pre>")
```json
{
    "category\_id": 1,          /\*后台一级类目\*/
    "coupon\_click\_url": "https://uland.taobao.com/coupon/edetail?e=xxxx&activityId=a64bdbb5bb8440be99d31344fa7e4500",          /\*二合一推广链接，已经自动拼接S券\*/
    "coupon\_end\_time": "2018-09-26",          /\*优惠券结束时间\*/
    "coupon\_info": "满40元减10元",          /\*优惠券面额\*/
    "coupon\_remain\_count": 6859          /\*优惠券剩余量\*/
    "coupon\_start\_time": "2018-09-25",          /\*优惠券开始时间\*/
    "coupon\_total\_count": 8000,          /\*优惠券总量\*/
    "coupon\_type": 3,           /\*优惠券(商品优惠券推广链接中的券)类型，1 公开券，2 私有券，3 妈妈券\*/
    "item\_id": "524136796550",          /\*商品id\*/
    "item\_url": "https://s.click.taobao.com/t?e=xxxx",          /\*推广长链接，如果是渠道ID，请自行拼接上relationId的信息,否则订单信息中可能查不到渠道信息\*/
    "max\_commission\_rate": "20.3",          /\*佣金比率(%)\*/
    "min\_commission\_rate": "20.3",          /\*当入参special\_id、relation\_id、external\_id时，该字段展示预估最低佣金率(%)\*/
    "s\_coupon\_id": "a64bdbb5bb8440be99d31344fa7e4500",          /\*特殊券ID，拼接规则参考下方文字\*/
    "s\_coupon\_amount": "10",          /\*特殊券面额\*/
    "s\_coupon\_startfee": "40",          /\*特殊券最低使用金额\*/
    "s\_coupon\_start\_time": "2018-09-25",          /\*特殊券开始时间\*/
    "s\_coupon\_end\_time": "2018-09-26"          /\*特殊券结束时间\*/
}
```

**signurl=0或1或2，如果有券，成功返回示例：/\*signurl=0或1或2，返回官方高佣转链接口结果，需要自行判断和拼接使用全网G券或者全网S券\*/**

var tmp = { "category\_id": 1, "coupon\_click\_url": "https://uland.taobao.com/coupon/edetail?e=xxxx", "coupon\_end\_time": "2018-09-26", "coupon\_info": "满40元减10元", "coupon\_remain\_count": 6859, "coupon\_start\_time": "2018-09-25", "coupon\_total\_count": 8000, "coupon\_type": 3, "item\_id": 524136796550, "item\_url": "https://s.click.taobao.com/t?e=xxxx", "max\_commission\_rate": "20.3", "min\_commission\_rate": "20.3" }; document.write("<pre>" + syntaxHighlight(tmp) + "</pre>")
```json
{
    "category\_id": 1,          /\*后台一级类目\*/
    "coupon\_click\_url": "https://uland.taobao.com/coupon/edetail?e=xxxx",          /\*二合一推广链接\*/
    "coupon\_end\_time": "2018-09-26",          /\*优惠券结束时间\*/
    "coupon\_info": "满40元减10元",          /\*优惠券面额\*/
    "coupon\_remain\_count": 6859          /\*优惠券剩余量\*/
    "coupon\_start\_time": "2018-09-25",          /\*优惠券开始时间\*/
    "coupon\_total\_count": 8000,          /\*优惠券总量\*/
    "coupon\_type": 3,           /\*优惠券(商品优惠券推广链接中的券)类型，1 公开券，2 私有券，3 妈妈券\*/
    "item\_id": "524136796550",          /\*商品id\*/
    "item\_url": "https://s.click.taobao.com/t?e=xxxx",          /\*推广长链接，如果是渠道ID，请自行拼接上relationId的信息,否则订单信息中可能查不到渠道信息\*/
    "max\_commission\_rate": "20.3",          /\*佣金比率(%)\*/
    "min\_commission\_rate": "20.3"          /\*当入参special\_id、relation\_id、external\_id时，该字段展示预估最低佣金率(%)\*/
}
```

**signurl=0或1或2，如果无券，成功返回示例：**

var tmp = { "category\_id": 1, "coupon\_click\_url": "https://uland.taobao.com/coupon/edetail?e=xxxx", "item\_id": 524136796550, "item\_url": "https://s.click.taobao.com/t?e=xxxx", "max\_commission\_rate": "20.3" }; document.write("<pre>" + syntaxHighlight(tmp) + "</pre>")
```json
{
    "category\_id": 1,
    "coupon\_click\_url": "https://uland.taobao.com/coupon/edetail?e=xxxx",
    "item\_id": "524136796550",
    "item\_url": "https://s.click.taobao.com/t?e=xxxx",
    "max\_commission\_rate": "20.3",
    "min\_commission\_rate": "20.3"
}
```

**如果非淘客商品，返回示例：（此类商品无法获得佣金）**

var tmp = { "error\_response": { "code": 15, "msg": "Remote service error", "sub\_code": "isv.item-not-exist", "sub\_msg": "该item\_id对应宝贝已下架或非淘客宝贝", "request\_id": "10eoi3wvcot9l" } }; document.write("<pre>" + syntaxHighlight(tmp) + "</pre>")
```json
{
    "error\_response":{
        "code":15,
        "msg":"Remote service error",
        "sub\_code":"isv.item-not-exist",
        "sub\_msg":"该item\_id对应宝贝已下架或非淘客宝贝",
        "request\_id":"10eoi3wvcot9l"
    }
}
```

接口调用说明：  
1、所有接口均免费提供，方便大家开发和接入自己的产品早日成为大牛淘客！  
2、使用高佣转链接口，生成您的淘客链接会自动申请最高佣金，包括鹊桥、定向计划、和营销计划（建议用户点击商品时触发式调用，降低服务器压力）！  
3、如需指定优惠券，在生成的二合一链接后，加入 &activityId=xxx券ID即可走指定券  
4、signurl=3，自动拼接s券规则：如果S券金额大于G券金额，用S券；如果有S券，无G券，用S券等。用signurl=3，不需要再调用解析商品编号API接口了。  

[接口测试](https://www.zhetaoke.com/user/open/test_open_gaoyongzhuanlian.aspx)

[高佣转链说明](http://www.zhetaoke.com/help_detail_3_25.html)

---

## 高佣转链API(淘口令)

**原始链接**: [高佣转链API(淘口令)](https://www.zhetaoke.com/user/open/open_gaoyongzhuanlian_tkl.aspx)

**高佣转链API（淘口令）---本接口采用12台16核心64G内存服务器，江苏和北京双机房，负载均衡技术，1G独享带宽，放心调用。**

手淘里拷贝出来的淘口令文案，必须输入代理类型sid、渠道pid和渠道rid，否则无法转链。  
代理类型的sid为返利模式，必须输入用户的渠道rid，否则无法转链。  
目前转链分代理类型、发单类型、导购类型等，如果转链有问题或者搞不清楚就全部授权即可，后台接口自动判断使用哪个授权类型。  

高佣转链API接口---支持淘口令文案、长链接、二合一链接、各种短链接、喵口令等 ：[淘宝开放平台文档》》](https://open.taobao.com/api.htm?docId=65412&docType=2&scopeId=28320)

**接口地址：**https://api.zhetaoke.com:10001/api/open\_gaoyongzhuanlian\_tkl.ashx

**请求实例：**https://api.zhetaoke.com:10001/api/open\_gaoyongzhuanlian\_tkl.ashx?appkey=0a565628b2fb4712902712734cec2feb&sid=#sid#&pid=#pid#&tkl=￥TdJCbN68klT￥&signurl=5

**备用地址：**http://api.zhetaoke.cn:10000/api/open\_gaoyongzhuanlian\_tkl.ashx

**特殊说明：**[主接口和备用接口详细说明（请大家优先使用com主域名接口地址，在com出现问题的时候，人工或者自动切换cn备用接口地址）](https://www.zhetaoke.com/help_detail_3_27.html)

**返回数据：**json

**请求方式：**get或者post

**请求参数说明：**

|  | 名称 | 类型 | 必填 | 说明  [接口在线测试](https://www.zhetaoke.com/user/open/test_open_gaoyongzhuanlian_tkl.aspx) |
| --- | --- | --- | --- | --- |
|   | appkey | string | 是 | 折淘客的对接秘钥appkey |
|   | sid | string | 是 | 对应的淘客账号授权ID[点击查看](https://www.zhetaoke.com/user/shouquan.html) |
|   | pid | string | 是 | 淘客PID，mm\_xxx\_xxx\_xxx,三段格式，必须与授权的账户相同，否则出错 |
|   | tkl | string | 是 | 淘口令文案或者二合一链接或者长链接或者短链接或者喵口令。请注意，该参数需要进行Urlencode编码后传入。  
淘口令文案：复制这条信息，￥ysuUbqEiTSt￥，打开【手机淘宝】即可。  
二合一链接：以https://uland.taobao.com/coupon/edetail?e=开头的链接  
长链接：以https://s.click.taobao.com/t?e=开头的链接  
短链接：类似https://s.click.taobao.com/R7rDLHw的链接  
喵口令：类似http://\*\*\*.com/s/3X51L?tm=cd5add的链接  
重要的事情说三遍：该参数需要进行Urlencode编码！该参数需要进行Urlencode编码！该参数需要进行Urlencode编码！ |
|   | relation\_id | string | 否 | 渠道关系ID，仅适用于渠道推广场景。 |
|   | special\_id | string | 否 | 会员运营ID |
|   | signurl | int | 否 | signurl=5，返回结果整合高佣转链API、解析商品编号API、全网商品详情API、淘口令创建API，已经自动判断和拼接使用全网G券还是全网S券。  
signurl=4，返回结果整合高佣转链API、解析商品编号API、商品简版详情API、淘口令创建API，已经自动判断和拼接使用全网G券还是全网S券。  
signurl=3，返回结果整合高佣转链API、解析商品编号API，已经自动判断和拼接使用全网G券还是全网S券。  
signurl=0或1或2，返回官方高佣转链接口结果，需要自行判断和拼接使用全网G券或者全网S券。  
signurl=0，表示直接返回最终结果。  
signurl=1或2，表示直接返回最终结果。  
 |

**signurl=5，成功返回示例：/\*signurl=5，返回结果整合高佣转链API、解析商品编号API、全网商品详情API、淘口令创建API，已经自动判断和拼接使用全网G券还是全网S券\*/**

var tmp = { "status": 200, /\*状态\*/ "content": \[{ "code": "6646", /\*折淘客编号\*/ "type\_one\_id": "1", /\*分类ID，可参考折淘客分类\*/ "tao\_id": "554832820990", /\*商品ID\*/ "title": "储物箱整理抽屉式收纳柜", /\*商品短标题\*/ "jianjie": "【爆款返场，出口日本】进口PP材质，可叠加透明可视抽屉收纳盒，分格抽屉物品分类齐全，区域分化一目了然，自由组合，大容量，让你居家生活更舒心，收纳你的生活！【赠运费险】", /\*商品简介\*/ "pict\_url": "https://img.alicdn.com/bao/......jpg", /\*商品主图\*/ "user\_type": "0", /\*是否天猫，0是淘宝，1是天猫\*/ "seller\_id": "3073204721", /\*卖家ID\*/ /\*店铺地址可自行拼接，http://store.taobao.com/shop/view\_shop.htm?user\_number\_id=3073204721 \*/ "shop\_dsr": "4.80", /\*商品描述分\*/ "volume": "169", /\*年销量\*/ "size": "26.90", /\*折扣价\*/ "quanhou\_jiage": "16.90", /\*券后价\*/ "date\_time\_yongjin": "2018/11/8 14:45:22", /\*数据更新时间\*/ "tkrate3": "30.00", /\*佣金比率\*/ "yongjin\_type": "MKT", /\*佣金类型\*/ "coupon\_id": "fe31d6c8132c462ba59bd72e03d2eae0", /\*优惠券ID\*/ "coupon\_start\_time": "2018-11-05", /\*优惠券开始时间\*/ "coupon\_end\_time": "2018-11-10", /\*优惠券结束时间\*/ "coupon\_info\_money": "10", /\*优惠券金额\*/ "coupon\_total\_count": "100000", /\*优惠券总数量\*/ "coupon\_remain\_count": "99965", /\*优惠券剩余数量\*/ "coupon\_info": "满80.00元减10元", /\*优惠券信息\*/ "juhuasuan": "0", /\*是否聚划算，1是\*/ "taoqianggou": "0", /\*是否淘抢购，1是\*/ "haitao": "0", /\*是否海淘，1是\*/ "jiyoujia": "0", /\*是否极有家，1是\*/ "jinpaimaijia": "0", /\*是否金牌卖家，1是\*/ "pinpai": "0", /\*是否精选品牌，1是\*/ "pinpai\_name": "", /\*品牌名称\*/ "yunfeixian": "0", /\*是否有运费险，1有\*/ "nick": "\*\*\*旗舰店", /\*卖家昵称\*/ "small\_images": "https://img.alicdn.com/bao/.......jpg|https://img.alicdn.com/bao/.......jpg", /\*商品小图列表\*/ "white\_image": "https://img.alicdn.com/bao/......jpg", /\*商品白底图\*/ "tao\_title": "抽屉式收纳箱塑料储物箱整理箱衣服物家用宿舍衣柜内衣内裤收纳盒", /\*商品长标题\*/ "provcity": "浙江 杭州", /\*宝贝所在地\*/ "shop\_title": "\*\*\*旗舰店", /\*店铺名称\*/ "zhibo\_url": "https://cloud.video.taobao.com/play/u/849090736/p/2/e/6/t/1/219853683676.mp4", /\*视频地址\*/ "sellCount": "860000", /\*淘宝网页实时总销量\*/ "commentCount": "250000", /\*评论数量\*/ "favcount": "180000", /\*收藏数量\*/ "score1": "4.9", /\*宝贝描述分\*/ "score2": "4.9", /\*卖家服务分\*/ "score3": "4.9", /\*物流服务\*/ "creditLevel": "15", /\*店铺等级，一星 二星 三星 四星 五星 一钻 二钻 三钻 四钻 五钻 一皇冠 二皇冠 三皇冠 四皇冠 五皇冠 一金冠 二金冠 三金冠 四金冠 五金冠\*/ "shopIcon": "//img.alicdn.com/imgextra/de/a6/TB10dC8SX.jpg", /\*店铺logo\*/ "pcDescContent": "1.jpg|2.jpg|3.jpg|4.jpg|5.jpg", /\*图文详情图片地址\*/ "taobao\_url": "https://item.taobao.com/item.htm?id=581801364036", /\*商品url\*/ "category\_id": "50006126", /\*叶子类目id\*/ "category\_name": "收纳盒", /\*叶子类目name\*/ "level\_one\_category\_id": "122928002", /\*一级类目id\*/ "level\_one\_category\_name": "收纳整理", /\*一级类目name\*/ "tkfee3": "5.07", /\*返佣金额\*/ "min\_commission\_rate": "30.00", "coupon\_click\_url": "https://uland.taobao.com/coupon/edetail?e=xxxx&activityId=a64bdbb5bb8440be99d31344fa7e4500", "item\_url": "https://s.click.taobao.com/t?e=xxxx", "shorturl": "https://s.tb.cn/h.f77lBsX", "shorturl2": "https://s.click.taobao.com/YkpZ4zu", "tkl": "￥Ry0tYxAT18H￥" }\] };
```json
{
	"status": 200,                      /\*状态\*/
	"content": \[{
		"code": "6646",                 /\*折淘客编号\*/
		"type\_one\_id": "1",             /\*分类ID，可参考折淘客分类\*/
		"tao\_id": "554832820990",       /\*商品ID\*/
		"title": "储物箱整理抽屉式收纳柜",                   /\*商品短标题\*/
		"jianjie": "【爆款返场，出口日本】进口PP材质，可叠加透明可视抽屉收纳盒，分格抽屉物品分类齐全，区域分化一目了然，自由组合，大容量，让你居家生活更舒心，收纳你的生活！【赠运费险】", /\*商品简介\*/
		"pict\_url": "https://img.alicdn.com/bao/......jpg",                   /\*商品主图\*/
		"user\_type": "0",                   /\*是否天猫，0是淘宝，1是天猫\*/
		"seller\_id": "3073204721",                   /\*卖家ID\*/ /\*店铺地址可自行拼接，http://store.taobao.com/shop/view\_shop.htm?user\_number\_id=3073204721 \*/
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
		"coupon\_total\_count": "100000",                   /\*优惠券总数量\*/
		"coupon\_remain\_count": "99965",                   /\*优惠券剩余数量\*/
		"coupon\_info": "满80.00元减10元",                   /\*优惠券信息\*/
		"juhuasuan": "0",                       /\*是否聚划算，1是\*/
		"taoqianggou": "0",                     /\*是否淘抢购，1是\*/
		"haitao": "0",                          /\*是否海淘，1是\*/
		"jiyoujia": "0",                        /\*是否极有家，1是\*/
		"jinpaimaijia": "0",                    /\*是否金牌卖家，1是\*/
		"pinpai": "0",                          /\*是否精选品牌，1是\*/
		"pinpai\_name": "",                      /\*品牌名称\*/
		"yunfeixian": "0",                      /\*是否有运费险，1有\*/
		"nick": "\*\*\*旗舰店",                    /\*卖家昵称\*/
		"small\_images": "https://img.alicdn.com/bao/.......jpg|https://img.alicdn.com/bao/.......jpg",                   /\*商品小图列表\*/
		"white\_image": "https://img.alicdn.com/bao/......jpg",                   /\*商品白底图\*/
		"tao\_title": "抽屉式收纳箱塑料储物箱整理箱衣服物家用宿舍衣柜内衣内裤收纳盒",                   /\*商品长标题\*/
		"provcity": "浙江 杭州",                   /\*宝贝所在地\*/
		"shop\_title": "\*\*\*旗舰店",                   /\*店铺名称\*/
		"zhibo\_url": "https://cloud.video.taobao.com/play/u/849090736/p/2/e/6/t/1/219853683676.mp4",                   /\*视频地址\*/
		"sellCount": "860000",                   /\*淘宝网页实时总销量\*/
		"commentCount": "250000",                   /\*评论数量\*/
		"favcount": "180000",                   /\*收藏数量\*/
		"score1": "4.9",                   /\*宝贝描述分\*/
		"score2": "4.9",                   /\*卖家服务分\*/
		"score3": "4.9",                   /\*物流服务分\*/
		"creditLevel": "15",                   /\*店铺等级（1-20），一星 二星 三星 四星 五星 一钻 二钻 三钻 四钻 五钻 一皇冠 二皇冠 三皇冠 四皇冠 五皇冠  一金冠 二金冠 三金冠 四金冠 五金冠\*/
		"shopIcon": "//img.alicdn.com/imgextra/de/a6/TB10dC8SX.jpg",       /\*店铺logo\*/
		"pcDescContent": "1.jpg|2.jpg|3.jpg|4.jpg|5.jpg",  /\*图文详情图片地址\*/
		"taobao\_url": "https://item.taobao.com/item.htm?id=581801364036",  /\*商品url\*/
		"category\_id": "50006126",                   /\*叶子类目id\*/
		"category\_name": "收纳盒",                   /\*叶子类目name\*/
		"level\_one\_category\_id": "122928002",                   /\*一级类目id\*/
		"level\_one\_category\_name": "收纳整理",                   /\*一级类目name\*/
		"tkfee3": "5.07",                   /\*返佣金额\*/
		"min\_commission\_rate": "30.00",          /\*当入参special\_id、relation\_id、external\_id时，该字段展示预估最低佣金率\*/
		"coupon\_click\_url": "https://uland.taobao.com/coupon/edetail?e=xxxx&activityId=a64bdbb5bb8440be99d31344fa7e4500",          /\*二合一推广链接，已经自动拼接S券\*/
		"item\_url": "https://s.click.taobao.com/t?e=xxxx",          /\*推广长链接，如果是渠道ID，请自行拼接上relationId的信息,否则订单信息中可能查不到渠道信息\*/
		"shorturl": "https://s.click.taobao.com/YkpZ4zu"          /\*淘宝短链接\*/
		"shorturl2": "https://s.tb.cn/h.f77lBsX"          /\*淘宝短链接2，此链接可以直接拉起手淘APP，只适用IOS。\*/
		"tkl": "￥Ry0tYxAT18H￥",          /\*淘口令\*/
	}\]
}
```

**signurl=4，成功返回示例：/\*signurl=4，返回结果整合高佣转链API、解析商品编号API、商品简版详情API、淘口令创建API，已经自动判断和拼接使用全网G券还是全网S券\*/**

var tmp = { "category\_id": "1", "coupon\_click\_url": "https://uland.taobao.com/coupon/edetail?e=xxxx&activityId=a64bdbb5bb8440be99d31344fa7e4500", "coupon\_end\_time": "2018-09-26", "coupon\_info": "满40元减10元", "coupon\_amount": "10", "coupon\_startfee": "40", "coupon\_remain\_count": "6859", "coupon\_start\_time": "2018-09-25", "coupon\_total\_count": "8000", "coupon\_type": "3", "item\_id": "524136796550", "item\_url": "https://s.click.taobao.com/t?e=xxxx", "max\_commission\_rate": "20.3", "min\_commission\_rate": "20.3", "s\_coupon\_id": "a64bdbb5bb8440be99d31344fa7e4500", "s\_coupon\_amount": "20", "s\_coupon\_startfee": "40", "s\_coupon\_start\_time": "2018-09-25", "s\_coupon\_end\_time": "2018-09-26", "cat\_leaf\_name": "毛针织衫", "cat\_name": "女装/女士精品", "nick": "\*\*旗舰店", "pict\_url": "https://img.alicdn.com/bao/uploaded/pic.jpg", "provcity": "河北 石家庄", "seller\_id": "2687788888", "small\_images": "https://img.alicdn.com/3.jpg|https://img.alicdn.com/4.jpg", "title": "秋冬堆堆领羊绒衫女简约短款纯色高领毛衣套头修身针织羊毛打底衫", "user\_type": "1", "volume": "1000", "zk\_final\_price": "239", "shorturl": "https://s.tb.cn/h.f77lBsX", "shorturl2": "https://s.click.taobao.com/YkpZ4zu", "tkl": "￥Ry0tYxAT18H￥" }; document.write("<pre>" + syntaxHighlight(tmp) + "</pre>")
```json
{
    "category\_id": 1,          /\*后台一级类目\*/
    "coupon\_click\_url": "https://uland.taobao.com/coupon/edetail?e=xxxx&activityId=a64bdbb5bb8440be99d31344fa7e4500",          /\*二合一推广链接，已经自动拼接S券\*/
    "coupon\_end\_time": "2018-09-26",          /\*优惠券结束时间\*/
    "coupon\_info": "满40元减10元",          /\*优惠券面额\*/
    "coupon\_remain\_count": 6859          /\*优惠券剩余量\*/
    "coupon\_start\_time": "2018-09-25",          /\*优惠券开始时间\*/
    "coupon\_total\_count": 8000,          /\*优惠券总量\*/
    "coupon\_type": 3,           /\*优惠券(商品优惠券推广链接中的券)类型，1 公开券，2 私有券，3 妈妈券\*/
    "item\_id": "524136796550",          /\*商品id\*/
    "item\_url": "https://s.click.taobao.com/t?e=xxxx",          /\*推广长链接，如果是渠道ID，请自行拼接上relationId的信息,否则订单信息中可能查不到渠道信息\*/
    "max\_commission\_rate": "20.3",          /\*佣金比率(%)\*/
    "min\_commission\_rate": "20.3",          /\*当入参special\_id、relation\_id、external\_id时，该字段展示预估最低佣金率(%)\*/
    "s\_coupon\_id": "a64bdbb5bb8440be99d31344fa7e4500",          /\*特殊券ID，拼接规则参考下方文字\*/
    "s\_coupon\_amount": "10",          /\*特殊券面额\*/
    "s\_coupon\_startfee": "40",          /\*特殊券最低使用金额\*/
    "s\_coupon\_start\_time": "2018-09-25",          /\*特殊券开始时间\*/
    "s\_coupon\_end\_time": "2018-09-26",          /\*特殊券结束时间\*/
    "cat\_leaf\_name": "毛针织衫",          /\*叶子类目名称\*/
    "cat\_name": "女装/女士精品",          /\*一级类目名称\*/
    "nick": "\*\*旗舰店",          /\*卖家昵称\*/
    "pict\_url": "https://img.alicdn.com/bao/uploaded/pic.jpg",          /\*商品主图\*/
    "provcity": "河北 石家庄",          /\*宝贝所在地\*/
    "seller\_id": "2687788888",          /\*卖家ID\*/
    "small\_images": "https://img.alicdn.com/3.jpg|https://img.alicdn.com/4.jpg",          /\*商品小图列表\*/
    "title": "秋冬堆堆领羊绒衫女简约短款纯色高领毛衣套头修身针织羊毛打底衫",          /\*商品长标题\*/
    "user\_type": "1",          /\*是否天猫，0是淘宝，1是天猫\*/
    "volume": "1000",          /\*年销量\*/
    "zk\_final\_price": "239",          /\*折扣价\*/
	"shorturl": "https://s.click.taobao.com/YkpZ4zu"          /\*淘宝短链接\*/
	"shorturl2": "https://s.tb.cn/h.f77lBsX"          /\*淘宝短链接2，此链接可以直接拉起手淘APP，只适用IOS。\*/
	"tkl": "￥Ry0tYxAT18H￥",          /\*淘口令\*/
}
```

**signurl=3，成功返回示例：/\*signurl=3，返回结果整合高佣转链API、解析商品编号API，已经自动判断和拼接使用全网G券还是全网S券\*/**

var tmp = { "category\_id": "1", "coupon\_click\_url": "https://uland.taobao.com/coupon/edetail?e=xxxx&activityId=a64bdbb5bb8440be99d31344fa7e4500", "coupon\_end\_time": "2018-09-26", "coupon\_info": "满40元减10元", "coupon\_amount": "10", "coupon\_startfee": "40", "coupon\_remain\_count": "6859", "coupon\_start\_time": "2018-09-25", "coupon\_total\_count": "8000", "coupon\_type": "3", "item\_id": "524136796550", "item\_url": "https://s.click.taobao.com/t?e=xxxx", "max\_commission\_rate": "20.3", "min\_commission\_rate": "20.3", "s\_coupon\_id": "a64bdbb5bb8440be99d31344fa7e4500", "s\_coupon\_amount": "20", "s\_coupon\_startfee": "40", "s\_coupon\_start\_time": "2018-09-25", "s\_coupon\_end\_time": "2018-09-26" }; document.write("<pre>" + syntaxHighlight(tmp) + "</pre>")
```json
{
    "category\_id": 1,          /\*后台一级类目\*/
    "coupon\_click\_url": "https://uland.taobao.com/coupon/edetail?e=xxxx&activityId=a64bdbb5bb8440be99d31344fa7e4500",          /\*二合一推广链接\*/
    "coupon\_end\_time": "2018-09-26",          /\*优惠券结束时间\*/
    "coupon\_info": "满40元减10元",          /\*优惠券面额\*/
    "coupon\_remain\_count": 6859          /\*优惠券剩余量\*/
    "coupon\_start\_time": "2018-09-25",          /\*优惠券开始时间\*/
    "coupon\_total\_count": 8000,          /\*优惠券总量\*/
    "coupon\_type": 3,           /\*优惠券(商品优惠券推广链接中的券)类型，1 公开券，2 私有券，3 妈妈券\*/
    "item\_id": "524136796550",          /\*商品id\*/
    "item\_url": "https://s.click.taobao.com/t?e=xxxx",          /\*推广长链接，如果是渠道ID，请自行拼接上relationId的信息,否则订单信息中可能查不到渠道信息\*/
    "max\_commission\_rate": "20.3",          /\*佣金比率(%)\*/
    "min\_commission\_rate": "20.3",          /\*当入参special\_id、relation\_id、external\_id时，该字段展示预估最低佣金率(%)\*/
    "s\_coupon\_id": "a64bdbb5bb8440be99d31344fa7e4500",          /\*特殊券ID，拼接规则参考下方文字\*/
    "s\_coupon\_amount": "10",          /\*特殊券面额\*/
    "s\_coupon\_startfee": "40",          /\*特殊券最低使用金额\*/
    "s\_coupon\_start\_time": "2018-09-25",          /\*特殊券开始时间\*/
    "s\_coupon\_end\_time": "2018-09-26",          /\*特殊券结束时间\*/
}
```

**signurl=0或1或2，如果有券，成功返回示例：/\*signurl=0或1或2，返回官方高佣转链接口结果，需要自行判断和拼接使用全网G券或者全网S券\*/**

var tmp = { "category\_id": 1, "coupon\_click\_url": "https://uland.taobao.com/coupon/edetail?e=xxxx", "coupon\_end\_time": "2018-09-26", "coupon\_info": "满40元减10元", "coupon\_remain\_count": 6859, "coupon\_start\_time": "2018-09-25", "coupon\_total\_count": 8000, "coupon\_type": 3, "item\_id": 524136796550, "item\_url": "https://s.click.taobao.com/t?e=xxxx", "max\_commission\_rate": "20.3", "min\_commission\_rate": "20.3" }; document.write("<pre>" + syntaxHighlight(tmp) + "</pre>")
```json
{
    "category\_id": 1,          /\*后台一级类目\*/
    "coupon\_click\_url": "https://uland.taobao.com/coupon/edetail?e=xxxx",          /\*二合一推广链接，已经自动拼接S券\*/
    "coupon\_end\_time": "2018-09-26",          /\*优惠券结束时间\*/
    "coupon\_info": "满40元减10元",          /\*优惠券面额\*/
    "coupon\_remain\_count": 6859          /\*优惠券剩余量\*/
    "coupon\_start\_time": "2018-09-25",          /\*优惠券开始时间\*/
    "coupon\_total\_count": 8000,          /\*优惠券总量\*/
    "coupon\_type": 3,           /\*优惠券(商品优惠券推广链接中的券)类型，1 公开券，2 私有券，3 妈妈券\*/
    "item\_id": "524136796550",          /\*商品id\*/
    "item\_url": "https://s.click.taobao.com/t?e=xxxx",          /\*推广长链接，如果是渠道ID，请自行拼接上relationId的信息,否则订单信息中可能查不到渠道信息\*/
    "max\_commission\_rate": "20.3",          /\*佣金比率(%)\*/
    "min\_commission\_rate": "20.3"          /\*当入参special\_id、relation\_id、external\_id时，该字段展示预估最低佣金率(%)\*/
}
```

**signurl=0或1或2，如果无券，成功返回示例：**

var tmp = { "category\_id": 1, "coupon\_click\_url": "https://uland.taobao.com/coupon/edetail?e=xxxx", "item\_id": 524136796550, "item\_url": "https://s.click.taobao.com/t?e=xxxx", "max\_commission\_rate": "20.3" }; document.write("<pre>" + syntaxHighlight(tmp) + "</pre>")
```json
{
    "category\_id": 1,
    "coupon\_click\_url": "https://uland.taobao.com/coupon/edetail?e=xxxx",
    "item\_id": "524136796550",
    "item\_url": "https://s.click.taobao.com/t?e=xxxx",
    "max\_commission\_rate": "20.3",
    "min\_commission\_rate": "20.3"
}
```

**如果非淘客商品，返回示例：（此类商品无法获得佣金）**

var tmp = { "error\_response": { "code": 15, "msg": "Remote service error", "sub\_code": "isv.item-not-exist", "sub\_msg": "该item\_id对应宝贝已下架或非淘客宝贝", "request\_id": "10eoi3wvcot9l" } }; document.write("<pre>" + syntaxHighlight(tmp) + "</pre>")
```json
{
    "error\_response":{
        "code":15,
        "msg":"Remote service error",
        "sub\_code":"isv.item-not-exist",
        "sub\_msg":"该item\_id对应宝贝已下架或非淘客宝贝",
        "request\_id":"10eoi3wvcot9l"
    }
}
```

接口调用说明：  
1、所有接口均免费提供，方便大家开发和接入自己的产品早日成为大牛淘客！  
2、使用高佣转链接口，生成您的淘客链接会自动申请最高佣金，包括鹊桥、定向计划、和营销计划（建议用户点击商品时触发式调用，降低服务器压力）！  
3、如需指定优惠券，在生成的二合一链接后，加入 &activityId=xxx券ID即可走指定券  
4、signurl=3，自动拼接s券规则：如果S券金额大于G券金额，用S券；如果有S券，无G券，用S券等。用signurl=3，不需要再调用解析商品编号API接口了。  

[接口测试](https://www.zhetaoke.com/user/open/test_open_gaoyongzhuanlian_tkl.aspx)

[高佣转链说明](http://www.zhetaoke.com/help_detail_3_25.html)

---

## 新订单查询API

**原始链接**: [新订单查询API](https://www.zhetaoke.com/user/open/open_dingdanchaxun2.aspx)

**新订单查询API---本接口采用12台16核心64G内存服务器，江苏和北京双机房，负载均衡技术，1G独享带宽，放心调用。**

新订单查询API接口：[淘宝开放平台文档》》](https://open.taobao.com/api.htm?docId=43755&docType=2&scopeId=16322)

**接口地址：**https://api.zhetaoke.com:10001/api/open\_dingdanchaxun2.ashx

**请求实例：**https://api.zhetaoke.com:10001/api/open\_dingdanchaxun2.ashx?appkey=0a565628b2fb4712902712734cec2feb&sid=#sid#&start\_time=&end\_time=&signurl=1

**备用地址：**http://api.zhetaoke.cn:10000/api/open\_dingdanchaxun2.ashx

**特殊说明：**[主接口和备用接口详细说明（请大家优先使用com主域名接口地址，在com出现问题的时候，人工或者自动切换cn备用接口地址）](https://www.zhetaoke.com/help_detail_3_27.html)

**返回数据：**json

**请求方式：**get或者post

**重要通知：**此接口禁止开多线程调用，发现后将被限制使用！！！正常调用即可！！！每2分钟调用1-3次即可！！！[更详细的接口使用说明请点击这里！更详细的接口使用说明请点击这里！](https://www.zhetaoke.com/help_detail_3_24.html)

**重要通知：**此接口禁止开多线程调用，发现后将被限制使用！！！正常调用即可！！！每2分钟调用1-3次即可！！！[更详细的接口使用说明请点击这里！更详细的接口使用说明请点击这里！](https://www.zhetaoke.com/help_detail_3_24.html)

**重要通知：**此接口禁止开多线程调用，发现后将被限制使用！！！正常调用即可！！！每2分钟调用1-3次即可！！！[更详细的接口使用说明请点击这里！更详细的接口使用说明请点击这里！](https://www.zhetaoke.com/help_detail_3_24.html)

**重要通知：**自2021年6月18日起，折淘客订单接口只返回使用折淘客转链接口产生的订单，包括淘宝订单和饿了么订单！如果疑问，请尽快联系：15611448080，QQ号：32894044

**重要通知：**自2021年6月18日起，折淘客订单接口只返回使用折淘客转链接口产生的订单，包括淘宝订单和饿了么订单！如果疑问，请尽快联系：15611448080，QQ号：32894044

**重要通知：**自2021年6月18日起，折淘客订单接口只返回使用折淘客转链接口产生的订单，包括淘宝订单和饿了么订单！如果疑问，请尽快联系：15611448080，QQ号：32894044

**重要通知：**订单接口使用前提：必须使用折淘客的高佣转链接口或者官方活动转链接口进行转链，否则不允许使用此订单接口！折淘客将定期清理不符合此要求的淘客授权账号！！

**重要通知：**订单接口使用前提：必须使用折淘客的高佣转链接口或者官方活动转链接口进行转链，否则不允许使用此订单接口！折淘客将定期清理不符合此要求的淘客授权账号！！

**重要通知：**订单接口使用前提：必须使用折淘客的高佣转链接口或者官方活动转链接口进行转链，否则不允许使用此订单接口！折淘客将定期清理不符合此要求的淘客授权账号！！

**订单维权接口：** https://api.zhetaoke.com:10001/Api/open\_dingdanchaxun2\_refund.ashx?appkey=d&sid=&page\_size=1&search\_type=1&refund\_type=1&start\_time=2022-02-08 00:00:00&page\_no=1&biz\_type=1&simplify=0

**订单维权接口官方说明：**[淘宝开放平台文档》》](https://open.taobao.com/api.htm?docId=43874&docType=2&scopeId=16322)

**请求参数说明：**

|  | 名称 | 类型 | 必填 | 说明  [接口在线测试](https://www.zhetaoke.com/user/open/test_open_dingdanchaxun2.aspx) |
| --- | --- | --- | --- | --- |
|   | appkey | string | 是 | 折淘客的对接秘钥appkey |
|   | sid | string | 是 | 对应的淘客账号授权ID[点击查看](https://www.zhetaoke.com/user/shouquan.html) |
|   | start\_time | string | 是 | 订单查询开始时间，2019-04-05 12:18:22 |
|   | end\_time | string | 是 | 订单查询结束时间，2019-04-25 15:18:22，目前最大可查3个小时内的数据  
但如618、双11、年货节等大促期间预估时间段不可超过20分钟，超过会提示错误。 |
|   | query\_type | string | 否 | 查询时间类型，1：按照订单淘客创建时间查询，2:按照订单淘客付款时间查询，3:按照订单淘客结算时间查询，4:按照订单更新时间查询； |
|   | position\_index | string | 否 | 位点，除第一页之外，都需要传递；前端原样返回。注意：从第二页开始，位点必须传递前一页返回的值，否则翻页无效。 |
|   | page\_size | string | 否 | 页大小，默认20，1~100 |
|   | member\_type | string | 否 | 推广者角色类型,2:二方，3:三方，不传，表示所有角色 |
|   | tk\_status | string | 否 | 淘客订单状态，12-付款，13-关闭，14-确认收货（暂时无法结算佣金），3-结算成功;不传，表示所有状态 |
|   | jump\_type | string | 否 | 跳转类型，当向前或者向后翻页必须提供,-1: 向前翻页,1：向后翻页 |
|   | page\_no | string | 否 | 第几页，默认1，1~100 |
|   | order\_scene | string | 否 | 场景订单场景类型，1:常规订单，2:渠道订单，3:会员运营订单，默认为1 |
|   | signurl | int | 否 | 值为1或者2，表示直接返回订单结果信息。 |

**成功返回示例：**[点击这里查看返回字段说明》》点击这里查看返回字段说明》》点击这里查看返回字段说明》》](https://open.taobao.com/api.htm?docId=43755&docType=2&scopeId=16322)

var tmp = { "tbk\_sc\_order\_details\_get\_response": { "data": { "results": { "publisher\_order\_dto": \[ { "tb\_paid\_time": "2019-04-22 15:15:05", "tk\_paid\_time": "2019-04-22 15:15:05", "pay\_price": "9.11", "pub\_share\_fee": "0", "trade\_id": "294159887445064307", "tk\_order\_role": 2, "tk\_earning\_time": "2019-04-22 15:15:05", "adzone\_id": 11, "pub\_share\_rate": "100", "refund\_tag": 0, "subsidy\_rate": "0", "tk\_total\_rate": "9.99", "item\_category\_name": "淘小铺", "seller\_nick": "--", "pub\_id": 98836808, "alimama\_rate": "0", "subsidy\_type": "--", "item\_img": "\\/\\/img.alicdn.com\\/bao\\/uploaded\\/i1\\/2200782262419\\/O1CN01b5qlop1TjwarUo8fo\_!!2200782262419.jpg", "pub\_share\_pre\_fee": "0", "alipay\_total\_price": "11.22", "item\_title": "tsh\_rj\_测试请不要拍\_阶佣11.1", "site\_name": "合伙人", "item\_num": 2, "subsidy\_fee": "0", "alimama\_share\_fee": "0", "trade\_parent\_id": "294159887445064307", "order\_type": "如意淘", "tk\_create\_time": "2019-04-22 15:15:05", "flow\_source": "--", "terminal\_type": "无线", "click\_time": "2019-04-22 15:14:55", "tk\_status": 13, "item\_price": "2.1", "item\_id": 590141576510, "adzone\_name": "", "total\_commission\_rate": "9.99", "item\_link": "", "site\_id": 45598009, "seller\_shop\_title": "--", "income\_rate": "9.99", "total\_commission\_fee": "0", "tk\_commission\_pre\_fee\_for\_media\_platform": "1.05", "tk\_commission\_fee\_for\_media\_platform": "1.05", "tk\_commission\_rate\_for\_media\_platform": "0.01", "special\_id": 2323, "relation\_id": 2323 } \] }, "has\_pre": false, "position\_index": "1555904214\_lGltNdNvSX2|1555917305\_lJfPMeFmdt2", "has\_next": true, "page\_no": 1, "page\_size": 11 } } }; document.write("<pre>" + syntaxHighlight(tmp) + "</pre>")
```json
{
    "tbk\_sc\_order\_details\_get\_response":{
        "data":{
            "results":{
                "publisher\_order\_dto":\[
                    {
                        "tb\_paid\_time":"2019-04-22 15:15:05",
                        "tk\_paid\_time":"2019-04-22 15:15:05",
                        "pay\_price":"9.11",
                        "pub\_share\_fee":"0",
                        "trade\_id":"294159887445064307",
                        "tk\_order\_role":2,
                        "tk\_earning\_time":"2019-04-22 15:15:05",
                        "adzone\_id":11,
                        "pub\_share\_rate":"100",
                        "refund\_tag":0,      /\*维权标签，0 含义为非维权 1 含义为维权订单，已经实现\*/
                        "subsidy\_rate":"0",
                        "tk\_total\_rate":"9.99",
                        "item\_category\_name":"淘小铺",
                        "seller\_nick":"--",
                        "pub\_id":98836808,
                        "alimama\_rate":"0",
                        "subsidy\_type":"--",
                        "item\_img":"\\/\\/img.alicdn.com\\/bao\\/uploaded\\/i1\\/2200782262419\\/O1CN01b5qlop1TjwarUo8fo\_!!2200782262419.jpg",
                        "pub\_share\_pre\_fee":"0",
                        "alipay\_total\_price":"11.22",
                        "item\_title":"tsh\_rj\_测试请不要拍\_阶佣11.1",
                        "site\_name":"合伙人",
                        "item\_num":2,
                        "subsidy\_fee":"0",
                        "alimama\_share\_fee":"0",
                        "trade\_parent\_id":"294159887445064307",
                        "order\_type":"如意淘",
                        "tk\_create\_time":"2019-04-22 15:15:05",
                        "flow\_source":"--",
                        "terminal\_type":"无线",
                        "click\_time":"2019-04-22 15:14:55",
                        "tk\_status":13,
                        "item\_price":"2.1",
                        "item\_id":590141576510,
                        "adzone\_name":"",
                        "total\_commission\_rate":"9.99",
                        "item\_link":"",
                        "site\_id":45598009,
                        "seller\_shop\_title":"--",
                        "income\_rate":"9.99",
                        "total\_commission\_fee":"0",
                        "tk\_commission\_pre\_fee\_for\_media\_platform":"1.05",
                        "tk\_commission\_fee\_for\_media\_platform":"1.05",
                        "tk\_commission\_rate\_for\_media\_platform":"0.01",
                        "special\_id":2323,
                        "relation\_id":2323
                    }
                \]
            },
            "has\_pre":false,
            "position\_index":"1555904214\_lGltNdNvSX2|1555917305\_lJfPMeFmdt2",
            "has\_next":true,
            "page\_no":1,
            "page\_size":11
        }
    }
}
```

接口调用说明：  
1、所有接口均免费提供，方便大家开发和接入自己的产品早日成为大牛淘客！  
2、请大家一定要合理使用此接口，每个授权账户调用频率在1分钟或2分钟以上,开发群中有接口具体使用文档。  
3、本接口为自行请求数据，请设置signurl为1，这样就只返回请求地址,获得请求地址后,继续get可获得详细数据  

  

[接口测试](https://www.zhetaoke.com/user/open/test_open_dingdanchaxun2.aspx)

[接口使用教程](http://www.zhetaoke.com/help_detail_3_24.html)

[接口使用教程](http://www.zhetaoke.com/help_detail_3_24.html)

[接口使用教程](http://www.zhetaoke.com/help_detail_3_24.html)

[接口使用教程](http://www.zhetaoke.com/help_detail_3_24.html)

[接口使用教程](http://www.zhetaoke.com/help_detail_3_24.html)

---

## 解析商品编号API

**原始链接**: [解析商品编号API](https://www.zhetaoke.com/user/open/open_shangpin_id.aspx)

**解析商品编号API**

[

*淘口令回流数据查询API*

](https://www.zhetaoke.com/user/open/open_tkl_report.aspx)

[

*淘口令创建API*

](https://www.zhetaoke.com/user/open/open_tkl_create.aspx)

[

*淘口令解析API*

](https://www.zhetaoke.com/user/open/open_shangpin_id.aspx)

重要通知！重要通知！重要通知！此页面上的三个接口，新增淘客pid参数，mm\_xxx\_xxx\_xxx,三段格式，必须与授权的账户相同，否则出错。必填字段！！！  
手淘里拷贝出来的淘口令文案，必须输入代理类型sid、渠道pid和渠道rid，否则无法解析。  
代理类型的sid为返利模式，必须输入用户的渠道rid，否则无法解析。  
目前转链分代理类型、发单类型、导购类型等，如果转链有问题或者搞不清楚就全部授权即可，后台接口自动判断使用哪个授权类型。  

1、解析商品编号API（返回商品ID和券ID）：支持淘口令文案、长链接、二合一链接、短链接、喵口令、新浪短链，可直接返回特殊优惠券

**接口地址：**https://api.zhetaoke.com:10001/api/open\_shangpin\_id.ashx?appkey=0a565628b2fb4712902712734cec2feb&sid=#sid#&content=￥TdJCbN68klT￥&type=1&jb=1&tlj=1&hjs=1&pid=&relation\_id=

<table class="api_table" border="0" cellspacing="0" cellpadding="0"><tbody><tr><td>&nbsp;</td><td width="90">接口说明：</td><td>此接口可以解析的淘口令，包括：淘宝客商品口令（二合一链接、长链接）。<br>其他非淘宝客商品和未知类型的活动或者会场链接，无法解析。<br><font color="red">纯数字ID和纯数字ID链接，需要收费，单个收费5金币，可点击上方菜单【赞助我们】进行充值。</font><br><font color="red">纯数字ID例子：673981878045</font><br><font color="red">纯数字ID链接例子：https://item.taobao.com/item.htm?id=673981878045</font><br><font color="red">使用方法：在接口上增加参数 &amp;jb=1 即可。</font><br></td></tr></tbody></table>

**接口说明：**该接口返回商品id和优惠券信息，详细参数和返回结果，请参考下方说明。

\-----------------------------------------------------------------------------------------------------------------------------

2、解析淘口令API（返回推广url）：支持淘口令文案，返回推广url链接

**免费接口地址：**https://api.zhetaoke.com:10001/api/open\_taokouling\_jiexi.ashx?appkey=0a565628b2fb4712902712734cec2feb&sid=#sid#&content=￥TdJCbN68klT￥&pid=&relation\_id=

<table class="api_table" border="0" cellspacing="0" cellpadding="0"><tbody><tr><td>&nbsp;</td><td width="90">接口说明：</td><td>此接口可以解析的淘口令，包括：淘宝客商品口令（二合一链接、长链接）<br>其他非淘宝客商品和未知类型的活动或者会场链接，无法解析。</td></tr></tbody></table>

**接口说明：**

\-----------------------------------------------------------------------------------------------------------------------------

3、获取跳转URL地址API：支持淘口令文案、淘宝长链接、淘宝短链接等各种跳转URL地址，返回原始推广url链接

**接口地址：**https://api.zhetaoke.com:10001/api/open\_get\_location.ashx?appkey=0a565628b2fb4712902712734cec2feb&sid=#sid#&content=￥TdJCbN68klT￥&type=1&pid=&relation\_id=

<table class="api_table" border="0" cellspacing="0" cellpadding="0"><tbody><tr><td>&nbsp;</td><td width="90">免费接口说明：</td><td>此接口可以解析的淘口令，包括：淘宝客商品口令（二合一链接、长链接）<br>通过原始URL可判断是否饿了么链接、口碑链接、店铺链接、官方活动链接、天猫超市链接、饿了么链接、商品链接、其他类型链接等。<br>url_type=1，表示商品链接，url_type=2，表示店铺链接，url_type=3，表示饿了么链接，url_type=4，表示优惠券链接，url_type=5，表示口碑链接，url_type=10，表示官方活动链接，url_type=20，表示其他类型链接。<br></td></tr></tbody></table>

  
  

\-----------------------------------------------------------------------------------------------------------------------------

**1、解析商品编号API（返回商品ID和券ID）：请求参数说明：**

|  | 名称 | 类型 | 必填 | 说明 |
| --- | --- | --- | --- | --- |
|   | appkey | string | 是 | 折淘客的对接秘钥appkey |
|   | sid | string | 是 | 对应的淘客账号授权ID[点击查看](https://www.zhetaoke.com/user/shouquan.html) |
|   | pid | string | 是 | 淘客PID，mm\_xxx\_xxx\_xxx,三段格式，必须与授权的账户相同，否则出错 |
|   | relation\_id | string | 否 | 渠道关系ID，仅适用于渠道推广场景。  
从手淘拷贝出来的带淘口令的文案，必须传入此参数，否则无法解析淘口令。  
传入此参数时，pid必须为渠道pid，不传入此参数，普通pid和渠道pid都可以。 |
|   | content | string | 是 | 淘口令文案或者二合一链接或者长链接或者短链接或者喵口令。请注意，该参数需要进行Urlencode编码后传入。  
淘口令文案：复制这条信息，￥ysuUbqEiTSt￥，打开【手机淘宝】即可。  
二合一链接：以https://uland.taobao.com/coupon/edetail?e=开头的链接  
长链接：以https://s.click.taobao.com/t?e=开头的链接  
短链接：类似https://s.click.taobao.com/R7rDLHw的链接  
喵口令：类似http://\*\*\*.com/s/3X51L?tm=cd5add的链接  
重要的事情说三遍：该参数需要进行Urlencode编码！该参数需要进行Urlencode编码！该参数需要进行Urlencode编码！ |
|   | type | string | 是 | 是否返回优惠券详细信息，type=1表示返回商品ID和优惠券详细信息，注意，优惠券信息可能为空 |
|   | tlj | string | 否 | 是否解析淘礼金淘口令，tlj=1表示解析，tlj=0表示不解析，默认tlj=0 |
|   | hjs | string | 否 | 是否解析火炬手淘口令，hjs=1表示解析，hjs=0表示不解析，默认hjs=1 |

**成功返回示例：（注：获得优惠券信息后，继续调用高佣转链接口，可获得官方优惠券信息，两个对比，可自行选择使用哪个优惠券。）**

var tmp = { "item\_id": "Xq5rvoOCGtXrawNgyQHpXiAUg-2BGDWkSOwP7ydM2tZo", "activity\_id": "c15af3c5bd7641419142344b214c1aa7", "amount": "20", "startFee": "50", "effectiveStartTime": "2019-03-21 00:00:00", "effectiveEndTime": "2019-03-22 23:59:59", "shopLogo": "//img.alicdn.com/bao/uploaded//3b/e5/TB1hP2enhWYBuNjy1zkSutGGpXa.jpg", "shopName": "天猫超市", "type": "", "material\_type": "1", "material\_id": "20150318020018107" }; document.write("<pre>" + syntaxHighlight(tmp) + "</pre>")
```json
{
    "item\_id":"524136796550",     /\*商品ID\*/
    "activity\_id":"c15af3c5bd7641419142344b214c1aa7",     /\*优惠券ID\*/
    "amount":"20",                   /\*优惠券面额\*/
    "startFee":"50",               /\*优惠券满减信息\*/
    "effectiveStartTime":"2019-03-21 00:00:00",             /\*优惠券开始时间\*/
    "effectiveEndTime":"2019-03-22 23:59:59",              /\*优惠券结束时间\*/
    "shopLogo":"//img.alicdn.com/bao/uploaded//3b/e5/TB1hP2enhWYBuNjy1zkSutGGpXa.jpg",             /\*店铺Logo\*/
    "shopName":"天猫超市"              /\*店铺名称\*/
    "type":""              /\*暂时没用\*/
    "material\_type":"1"              /\*1表示商品，2表示店铺，3表示官方活动\*/
    "material\_id":"20150318020018107"              /\*店铺id或者官方活动id，只有material\_type-2或者3的时候，才会返回对应值\*/
}
```

**失败返回示例：**

var tmp = { {"status":301,"content":"很抱歉！因某某原因而失败的错误提示！"} }; document.write("<pre>" + syntaxHighlight(tmp) + "</pre>")
```json
{"status":301,"content":"很抱歉！因某某原因而失败的错误提示！"}
```

接口调用说明：  
1、所有接口均免费提供，方便大家开发和接入自己的产品早日成为大牛淘客！  
2、使用高佣转链接口，生成您的淘客链接会自动申请最高佣金，包括鹊桥、定向计划、和营销计划（建议用户点击商品时触发式调用，降低服务器压力）！  
3、如需指定优惠券，在生成的二合一链接后，加入 &activityId=xxx券ID即可走指定券  

  
  

[接口1测试](https://www.zhetaoke.com/user/open/test_open_shangpin_id.aspx)

[接口2测试](https://www.zhetaoke.com/user/open/test_open_shangpin_id2.aspx)

[接口3测试](https://www.zhetaoke.com/user/open/test_open_shangpin_id3.aspx)

[解析接口说明](http://www.zhetaoke.com/help_detail_3_16.html)

---

## 淘客账号授权API

**原始链接**: [淘客账号授权API](https://www.zhetaoke.com/user/open/open_taokeshouquan.aspx)

**淘客账号授权API**

淘客账号授权API接口

**接口地址：**https://api.zhetaoke.com:10001/api/open\_taokeshouquan.ashx

**请求实例：**https://api.zhetaoke.com:10001/api/open\_taokeshouquan.ashx?appkey=0a565628b2fb4712902712734cec2feb&backurl=#backurl#&type=1&key=1

**返回数据：**链接跳转

**请求方式：**url访问

**流程介绍：**  
此接口主要是方便大家开发，开发者通过些接口生成链接后，发给淘客，他们授权后即可回调授权信息到开发者的服务器，这样，也就不用麻烦普通淘客再去单独注册一个折淘客账户。

**重要说明：**此授权地址如果放在自己的软件中，可能会泄露appkey，直接使用此授权地址转向后的淘宝地址，就不存在这种问题，淘宝地址中的appkey已经做了加密，每个appkey转向的这个淘宝地址是唯一的。地址如下：  
https://oauth.taobao.com/authorize?response\_type=code&client\_id=25635011&redirect\_uri=https://api.zhetaoke.com:10001/api/open\_taokeshouquan\_return.ashx?appkey=756399e3824db47d907dd06efc1c714bdac4b9ddf829bd670ac27fec0533b063517989af0cf1266fc5ab735757492cd57f6c354b69c900acabcfb17fa23db935

**请求参数说明：**

|  | 名称 | 类型 | 必填 | 说明 |
| --- | --- | --- | --- | --- |
|   | appkey | string | 是 | 折淘客的对接秘钥appkey |
|   | backurl | string | 是 | 淘客授权后，登记到你的授权列表，并将授权信息返回至你的回调地址，回调地址里可以带参数，用来区分不同的淘客！  
 |
|   | type | string | 否 | 淘客授权页面类型，1：电脑版授权页面，0：手机版授权页面，默认值1 |
|   | key | string | 否 | 淘客授权类型：  
1:代理（对应appkey：25635011），返利APP、代理APP等用渠道ID或者会员ID的模式，使用本类型授权。  
2:发单（对应appkey：33902843），社区发单模式，发QQ群，发微信群等模式，使用本类型授权。  
3:导购（对应appkey：33902043），CMS系统、单页等web网站推广模式，使用本类型授权。  
4:比价（对应appkey：33887878），消费者比价模式，从手淘分享出来的口令或者文案模式，使用本类型授权。 |

**回调地址说明：**

var tmp = "http://www.xxx.com/authback.html"; document.write("<pre>" + syntaxHighlight(tmp) + "</pre>")
```json
http://www.xxx.com/authback.html （回调地址一定要http://或者https://开头）
```

**授权成功后回调例子，token计算方法：md5(appkey + sid + tao\_id)，32位小写md5加密**

var tmp = "http://www.xxx.com/authback.html?account=8307\*\*\*&sid=12\*\*\*&tao\_id=2586\*\*\*&tao\_nick=淘客\*\*&auth\_time=\*\*\*&expire\_time=\*\*\*&key=1&lianmeng\_appkey=&token=0e0709a142f331b917031f26bc26990d"; document.write("<pre>" + syntaxHighlight(tmp) + "</pre>")
```json
http://www.xxx.com/authback.html?account=8307\*\*\*&sid=12\*\*\*&tao\_id=2586\*\*\*&tao\_nick=淘客\*\*&auth\_time=\*\*\*&expire\_time=\*\*\*&key=1&lianmeng\_appkey=&token=0e0709a142f331b917031f26bc26990d
```

接口调用说明：  
1、所有接口均免费提供，方便大家开发和接入自己的产品早日成为大牛淘客！  
2、此接口会生成一个折淘客的链接，你需要将此链接引导用户或你的员工去授权，授权成功后回调授权信息到你提供的回调地址上！

---

## 获取账户授权列表API

**原始链接**: [获取账户授权列表API](https://www.zhetaoke.com/user/open/open_taokeshouquaninfo.aspx)

**获取账户授权列表API**

获取账户授权列表API接口

**接口地址：**https://api.zhetaoke.com:10001/api/open\_taokeshouquaninfo.ashx

**请求实例：**https://api.zhetaoke.com:10001/api/open\_taokeshouquaninfo.ashx?appkey=0a565628b2fb4712902712734cec2feb&page=1

**返回数据：**json

**请求方式：**get或者post

**请求参数说明：**

|  | 名称 | 类型 | 必填 | 说明 |
| --- | --- | --- | --- | --- |
|   | appkey | string | 是 | 折淘客的对接秘钥appkey |
|   | page | string | 否 | 第几页，每页最多返回100个 |
|   | expire\_day | string | 否 | 还剩下几天授权过期，取值范围0-7之间的整数，比如值为3，表示获取还剩下3天授权过期淘客账号信息 |
|   | sid | string | 否 | 对应的淘客账号授权ID[点击查看](https://www.zhetaoke.com/user/shouquan.html) |

**成功返回示例：**

var tmp = { "status": 200, "data": \[ { "sid": "2\*\*", /\*折淘客sid编号，区分淘客，高佣转链等大部分接口需要\*/ "account": "8307\*\*\*", /\*折淘客账号\*/ "key": "1", /\*淘客授权类型，具体见【淘客账号授权API】接口\*/ "lianmeng\_appkey": "25635011", /\*折淘客工具商appkey\*/ "tao\_id": "2586\*\*\*", /\*授权的淘客账号id\*/ "tao\_nick": "淘客\*\*\*", /\*授权的淘客账号名称\*/ "auth\_time": "2018/11/12 15:55:49", /\*授权时间\*/ "expire\_time": "2018/12/12 15:55:49" /\*过期时间\*/ } \] }; document.write("<pre>" + syntaxHighlight(tmp) + "</pre>")
```json
{
    "status": 200,
    "data": \[
        {
            "sid": "2\*\*",                 /\*折淘客sid编号，区分淘客，高佣转链等大部分接口需要\*/
            "account": "8307\*\*\*",                 /\*折淘客账号\*/
            "key": "1",                 /\*淘客授权类型，具体见【淘客账号授权API】接口\*/
            "lianmeng\_appkey": "25635011",                 /\*折淘客工具商appkey\*/
            "tao\_id": "2586\*\*\*",                 /\*授权的淘客账号id\*/
            "tao\_nick": "淘客\*\*\*",                 /\*授权的淘客账号名称\*/
            "auth\_time": "2018/11/12 15:55:49",                 /\*授权时间\*/
            "expire\_time": "2018/12/12 15:55:49"                 /\*过期时间\*/
        }
    \]
}
```

接口调用说明：  
1、所有接口均免费提供，方便大家开发和接入自己的产品早日成为大牛淘客！  
2、使用高佣转链接口，生成您的淘客链接会自动申请最高佣金，包括鹊桥、定向计划、和营销计划（建议用户点击商品时触发式调用，降低服务器压力）！  
3、如需指定优惠券，在生成的二合一链接后，加入 &activityId=xxx券ID即可走指定券  
4、如需自行请求数据，请设置signurl为1，这样就只返回请求地址

---

## 淘宝短链接转换API

**原始链接**: [淘宝短链接转换API](https://www.zhetaoke.com/user/open/open_shorturl_taobao_get.aspx)

**淘宝短链转换API---本接口采用12台16核心64G内存服务器，江苏和北京双机房，负载均衡技术，1G独享带宽，放心调用。**

淘宝短链转换API：

**接口地址：**https://api.zhetaoke.com:10001/api/open\_shorturl\_taobao\_get.ashx

**请求实例：**https://api.zhetaoke.com:10001/api/open\_shorturl\_taobao\_get.ashx?appkey=0a565628b2fb4712902712734cec2feb&sid=#sid#&content=#content#

**返回数据：**json

**请求方式：**get或者post

**请求参数说明：**

|  | 名称 | 类型 | 必填 | 说明 |
| --- | --- | --- | --- | --- |
|   | appkey | string | 是 | 折淘客的对接秘钥appkey |
|   | sid | string | 是 | 对应的淘客账号授权ID[点击查看](https://www.zhetaoke.com/user/shouquan.html) |
|   | content | string | 是 | 输入原始url，转换得到淘宝短链接。要转短链接的网址，需要进行Urlencode编码，只支持uland.taobao.com，s.click.taobao.com， ai.taobao.com，temai.taobao.com的域名转换，否则判错。 |

**成功返回示例：**

var tmp = { "shorturl": "https://s.click.taobao.com/H4zazPx" }; document.write("<pre>" + syntaxHighlight(tmp) + "</pre>")
```json
{
    "shorturl":"https://s.click.taobao.com/H4zazPx"
}
```

**失败返回示例：**

var tmp = { {"status":301,"content":"很抱歉！因某某原因而失败的错误提示！"} }; document.write("<pre>" + syntaxHighlight(tmp) + "</pre>")
```json
{"status":301,"content":"很抱歉！因某某原因而失败的错误提示！"}
```

接口调用说明：  
1、所有接口均免费提供，方便大家开发和接入自己的产品早日成为大牛淘客！  
2、使用高佣转链接口，生成您的淘客链接会自动申请最高佣金，包括鹊桥、定向计划、和营销计划（建议用户点击商品时触发式调用，降低服务器压力）！  
3、如需指定优惠券，在生成的二合一链接后，加入 &activityId=xxx券ID即可走指定券  
4、如需自行请求数据，请设置signurl为1，这样就只返回请求地址

---

## 淘口令创建API

**原始链接**: [淘口令创建API](https://www.zhetaoke.com/user/open/open_tkl_create.aspx)

**淘口令创建API**

[

*淘口令回流数据查询API*

](https://www.zhetaoke.com/user/open/open_tkl_report.aspx)

[

*淘口令创建API*

](https://www.zhetaoke.com/user/open/open_tkl_create.aspx)

[

*淘口令解析API*

](https://www.zhetaoke.com/user/open/open_shangpin_id.aspx)

淘口令生成API：二合一链接、长链接、短链接等各种淘宝高佣链接，生成淘口令，如：￥D4A8bKYVD4h￥。[淘宝开放平台文档》》](http://open.taobao.com/api.htm?docId=31127&docType=2)

**接口地址：**https://api.zhetaoke.com:10001/api/open\_tkl\_create.ashx

**请求实例：**https://api.zhetaoke.com:10001/api/open\_tkl\_create.ashx?appkey=0a565628b2fb4712902712734cec2feb&sid=#sid#&text=#text#&url=#url#&logo=#logo#&signurl=0

**返回数据：**json

**请求方式：**get或者post

**请求参数说明：**

|  | 名称 | 类型 | 必填 | 说明 |
| --- | --- | --- | --- | --- |
|   | appkey | string | 是 | 折淘客的对接秘钥appkey |
|   | sid | string | 是 | 对应的淘客账号授权ID[点击查看](https://www.zhetaoke.com/user/shouquan.html) |
|   | text | string | 否 | 口令弹框内容，长度大于5个字符，如：美美小编精心推荐。尽量不要带特殊符号或特殊词，否则生成的淘口令手淘里可能打不开。  
 |
|   | url | string | 是 | 口令跳转目标页，如：https://uland.taobao.com/，必须以https开头，可以是二合一链接、长链接、短链接等各种淘宝高佣链接；  
请注意，该参数需要进行Urlencode编码后传入。。  
重要的事情说三遍：该参数需要进行Urlencode编码！该参数需要进行Urlencode编码！该参数需要进行Urlencode编码！ |
|   | logo | string | 否 | 口令弹框logoURL，如：https://img.alicdn.com/bao/uploaded/i2.jpg\_200x200.jpg |
|   | signurl | int | 否 | 值为0表示直接返回淘口令最终结果。 |
|   | type | string | 否 | 生成淘口令结果类型，type=0表示只返回淘口令，type=1表示返回官方原始淘口令文案，支持IOS14版本。  
此字段只对signurl=0有效，signurl=1或2时，返回联盟请求地址。  
 |

**成功返回示例：**

var tmp = { "model": "￥D4A8bKYVD4h￥" }; document.write("<pre>" + syntaxHighlight(tmp) + "</pre>")
```json
{
    "model":"￥D4A8bKYVD4h￥"
}
```

**失败返回示例：**

var tmp = { {"status":301,"content":"很抱歉！因某某原因而失败的错误提示！"} }; document.write("<pre>" + syntaxHighlight(tmp) + "</pre>")
```json
{"status":301,"content":"很抱歉！因某某原因而失败的错误提示！"}
```

接口调用说明：  
1、所有接口均免费提供，方便大家开发和接入自己的产品早日成为大牛淘客！  
2、使用高佣转链接口，生成您的淘客链接会自动申请最高佣金，包括鹊桥、定向计划、和营销计划（建议用户点击商品时触发式调用，降低服务器压力）！  
3、如需指定优惠券，在生成的二合一链接后，加入 &activityId=xxx券ID即可走指定券

---

## 店铺链接转换API

**原始链接**: [店铺链接转换API](https://www.zhetaoke.com/user/open/open_shop_convert.aspx)

**店铺链接转换API---本接口采用12台16核心64G内存服务器，江苏和北京双机房，负载均衡技术，1G独享带宽，放心调用。**

目前转链分代理类型、发单类型、导购类型等，如果转链有问题或者搞不清楚就全部授权即可，后台接口自动判断使用哪个授权类型。  

店铺链接转换API：生成一个或者多个指定的店铺推广长链接地址[淘宝开放平台文档》》](https://open.taobao.com/api.htm?docId=43878&docType=2)

**接口地址：**https://api.zhetaoke.com:10001/api/open\_shop\_convert.ashx

**请求实例：**https://api.zhetaoke.com:10001/api/open\_shop\_convert.ashx?appkey=0a565628b2fb4712902712734cec2feb&sid=#sid#&site\_id=&fields=user\_id,click\_url&user\_ids=&platform=1&adzone\_id=&unid=&signurl=0

**返回数据：**json

**请求方式：**get或者post

**请求参数说明：**

|  | 名称 | 类型 | 必填 | 说明 |
| --- | --- | --- | --- | --- |
|   | appkey | string | 是 | 折淘客的对接秘钥appkey |
|   | sid | string | 是 | 对应的淘客账号授权ID[点击查看](https://www.zhetaoke.com/user/shouquan.html) |
|   | site\_id | string | 是 | 备案的网站id, mm\_xx\_xx\_xx pid三段式中的第二段 |
|   | fields | string | 是 | 需返回的字段列表，如：user\_id,click\_url |
|   | user\_ids | string | 是 | 卖家ID编号，可通过全网商品详情API接口获得,seller\_id字段 |
|   | platform | string | 否 | 链接形式：1：PC，2：无线，默认：1 |
|   | adzone\_id | string | 是 | 广告位ID，区分效果位置 |
|   | relation\_id | string | 否 | 渠道关系ID，仅适用于渠道推广场景 |
|   | signurl | int | 否 | 值为0表示直接返回最终结果。 |

**成功返回示例：**

var tmp = { "tbk\_sc\_shop\_convert\_response":{ "results":{ "n\_tbk\_shop":\[ { "user\_id":123, "click\_url":"http:\\/\\/s.click.taobao.com\\/e=xxx" } \] } } } document.write("<pre>" + syntaxHighlight(tmp) + "</pre>")
```json
{
    "tbk\_sc\_shop\_convert\_response":{
        "results":{
            "n\_tbk\_shop":\[
                {
                    "user\_id":123,
                    "click\_url":"http:\\/\\/s.click.taobao.com\\/e=xxx"
                }
            \]
        }
    }
}
```

**失败返回示例：**

var tmp = { {"status":301,"content":"很抱歉！因某某原因而失败的错误提示！"} }; document.write("<pre>" + syntaxHighlight(tmp) + "</pre>")
```json
{"status":301,"content":"很抱歉！因某某原因而失败的错误提示！"}
```

接口调用说明：  
1、所有接口均免费提供，方便大家开发和接入自己的产品早日成为大牛淘客！  
2、使用高佣转链接口，生成您的淘客链接会自动申请最高佣金，包括鹊桥、定向计划、和营销计划（建议用户点击商品时触发式调用，降低服务器压力）！  
3、如需指定优惠券，在生成的二合一链接后，加入 &activityId=xxx券ID即可走指定券

---

## 淘礼金创建API

**原始链接**: [淘礼金创建API](https://www.zhetaoke.com/user/open/open_taolijin2_create.aspx)

**淘礼金创建API---本接口采用12台16核心64G内存服务器，江苏和北京双机房，负载均衡技术，1G独享带宽，放心调用。**

淘礼金创建API：此接口为媒体版。[淘宝开放平台文档》》](https://open.taobao.com/api.htm?docId=40173&docType=2&scopeId=15029)

**接口地址：**https://api.zhetaoke.com:10001/api/open\_taolijin2\_create.ashx

**请求实例：**https://api.zhetaoke.com:10001/api/open\_taolijin2\_create.ashx?appkey=自己应用appkey&secret=自己应用secret&adzone\_id=自己应用adzone\_id&item\_id=&total\_num=1&name=&user\_total\_win\_num\_limit=1&per\_face=1&send\_start\_time=&send\_end\_time=&use\_start\_time=&use\_end\_time=&campaign\_type=&signurl=0

**返回数据：**json

淘礼金发放及使用报表:此接口为媒体版。

**接口地址：**https://api.zhetaoke.com:10001/api/open\_taolijin2\_report.ashx?appkey=自己应用appkey&secret=自己应用secret&rights\_id=创建淘礼金时返回的rightsId&adzone\_id=推广位pid中的的第三段数字

**返回数据：**json

**请求方式：**get或者post

淘礼金暂停发放:此接口为媒体版。

**接口地址：**https://api.zhetaoke.com:10001/api/open\_taolijin2\_stop.ashx?appkey=自己应用appkey&secret=自己应用secret&rights\_id=创建淘礼金时返回的rightsId&adzone\_id=推广位pid中的的第三段数字

**返回数据：**json

**请求方式：**get或者post

**请求参数说明：**

|  | 名称 | 类型 | 必填 | 说明 |
| --- | --- | --- | --- | --- |
|   | appkey | string | 是 | 联盟后台自己应用的appkey |
|   | secret | string | 是 | 联盟后台自己应用的secret |
|   | adzone\_id | string | 是 | 自己应用的adzone\_id，pid推广位的第三段数字 |
|   | item\_id | string | 是 | 宝贝id |
|   | total\_num | string | 是 | 淘礼金总个数 |
|   | name | string | 是 | 淘礼金名称，最小5个字符，最大10个字符。例如：淘礼金来啦 |
|   | user\_total\_win\_num\_limit | string | 是 | 单用户累计中奖次数上限。例如：1 |
|   | per\_face | string | 是 | 单个淘礼金面额，支持两位小数，单位元。例如：10 |
|   | send\_start\_time | string | 是 | 发放开始时间，如：2019-09-01 00:00:00 |
|   | send\_end\_time | string | 是 | 发放截止时间，如：2019-09-02 00:00:00 |
|   | use\_start\_time | string | 是 | 使用开始日期，如：2019-09-01 |
|   | use\_end\_time | string | 是 | 使用截止时间，如：2019-09-02 |
|   | campaign\_type | string | 否 | CPS佣金计划类型，定向：DX；鹊桥：LINK\_EVENT；营销：MKT |
|   | signurl | int | 否 | 默认值为0，表示直接返回最终结果。 |

**成功返回示例：**

var tmp = { "tbk\_sc\_vegas\_tlj\_create\_response": { "result": { "model": { "rights\_id": "asfasdfasd", "send\_url": "https:\\/\\/www.taobao.com" }, "msg\_code": "1", "msg\_info": "1", "success": false } } } document.write("<pre>" + syntaxHighlight(tmp) + "</pre>")
```json
{
    "tbk\_sc\_vegas\_tlj\_create\_response": {
        "result": {
            "model": {
                "rights\_id": "asfasdfasd",
                "send\_url": "https:\\/\\/www.taobao.com"
            },
            "msg\_code": "1",
            "msg\_info": "1",
            "success": false
        }
    }
}
```

接口调用说明：  
1、所有接口均免费提供，方便大家开发和接入自己的产品早日成为大牛淘客！  
2、使用高佣转链接口，生成您的淘客链接会自动申请最高佣金，包括鹊桥、定向计划、和营销计划（建议用户点击商品时触发式调用，降低服务器压力）！  
3、如需指定优惠券，在生成的二合一链接后，加入 &activityId=xxx券ID即可走指定券  

  
  
  

[接口测试](https://www.zhetaoke.com/user/open/test_open_taolijin2_create.aspx)

---

## [渠道]淘宝客邀请码生成

**原始链接**: [[渠道]淘宝客邀请码生成](https://www.zhetaoke.com/user/open/open_sc_invitecode_get.aspx)

**淘宝客邀请码生成API---本接口采用12台16核心64G内存服务器，江苏和北京双机房，负载均衡技术，1G独享带宽，放心调用。**

淘宝客邀请码生成API：生成邀请码，然后让用户进行渠道备案，生成新的渠道ID。[淘宝开放平台文档》》](http://open.taobao.com/api.htm?docId=38046&docType=2&scopeId=14474)

**接口地址：**https://api.zhetaoke.com:10001/api/open\_sc\_invitecode\_get.ashx

**请求实例：**https://api.zhetaoke.com:10001/api/open\_sc\_invitecode\_get.ashx?appkey=0a565628b2fb4712902712734cec2feb&sid=#sid#&relation\_app=common&code\_type=1&relation\_id=

**返回数据：**json

**请求方式：**get或者post

**请求参数说明：**

|  | 名称 | 类型 | 必填 | 说明 |
| --- | --- | --- | --- | --- |
|   | appkey | string | 是 | 折淘客的对接秘钥appkey |
|   | sid | string | 是 | 对应的淘客账号授权ID[点击查看](https://www.zhetaoke.com/user/shouquan.html) |
|   | relation\_app | string | 是 | 渠道推广的物料类型，示例值：common |
|   | code\_type | int | 是 | 邀请码类型，1 - 渠道邀请，2 - 渠道裂变，3 -会员邀请 |
|   | relation\_id | int | 否 | 渠道关系ID |

**成功返回示例：**

var tmp = { "inviter\_code": "xxx" }; document.write("<pre>" + syntaxHighlight(tmp) + "</pre>")
```json
{
    "inviter\_code":"xxx"
}
```

接口调用说明：  
1、渠道推广专属功能  
  
  
  

  
  
  
  
  
  
  
  
  

[渠道ID说明](http://www.zhetaoke.com/help_detail_3_20.html)

---

## [渠道]淘宝客渠道备案

**原始链接**: [[渠道]淘宝客渠道备案](https://www.zhetaoke.com/user/open/open_sc_publisher_save.aspx)

**淘宝客渠道备案API---本接口采用12台16核心64G内存服务器，江苏和北京双机房，负载均衡技术，1G独享带宽，放心调用。**

淘宝客渠道备案API：用户进行渠道备案，生成新的渠道ID。支持渠道备案和会员备案。[淘宝开放平台文档》》](http://open.taobao.com/api.htm?docId=37988&docType=2&scopeId=14474)

**接口地址：**https://api.zhetaoke.com:10001/api/open\_sc\_publisher\_save.ashx

**单个邀请码请求实例：**https://api.zhetaoke.com:10001/api/open\_sc\_publisher\_save.ashx?inviter\_code=#inviter\_code#&inviter\_code\_s=#inviter\_code\_s#&backurl=#backurl#&s\_note=#s\_note#&type=1

**多个邀请码请求实例：**https://api.zhetaoke.com:10001/api/open\_sc\_publisher\_save2.ashx?inviter\_code=inviter\_code1|inviter\_code2|inviter\_code3&inviter\_code\_s=inviter\_code4|inviter\_code5|inviter\_code6&backurl=#backurl#&s\_note=#s\_note#&type=1

**返回数据：**json

**请求方式：**url访问

**请求参数说明：**

<table class="api_table" border="0" cellspacing="0" cellpadding="0"><tbody><tr><td>&nbsp;</td><td class="url">inviter_code</td><td class="url">string</td><td class="url">是</td><td>淘宝客邀请渠道的邀请码</td></tr><tr><td>&nbsp;</td><td class="url">inviter_code_s</td><td class="url">string</td><td class="url">否</td><td>淘宝客邀请会员的邀请码</td></tr><tr><td>&nbsp;</td><td class="url">backurl</td><td class="url">string</td><td class="url">是</td><td>代理授权，并登记备案后，返回渠道信息至回调地址。</td></tr><tr><td>&nbsp;</td><td class="url">s_note</td><td class="url">string</td><td class="url">否</td><td>渠道备注，此内容建议填写用户唯一标识（比如用户编号，账号等唯一字段），跟返回的relation_id字段做关联。</td></tr><tr><td>&nbsp;</td><td class="url">type</td><td class="url">string</td><td class="url red">否</td><td>淘客授权页面类型，1：电脑版授权页面，0：手机版授权页面，默认值1</td></tr></tbody></table>

**单个邀请码返回示例：desc=绑定成功，表示备案成功，其他值表示备案失败，具体参考desc返回内容**

var tmp = { "返回内容": "http://www.xxx.com?inviter\_code=EX\*\*\*&inviter\_code=EX\*\*\*&relation\_id=510\*\*&special\_id=&account\_name=w\*g&s\_note=999999&tao\_id=2586\*\*\*&tao\_nick=淘客\*\*&desc=绑定成功" }; document.write("<pre>" + syntaxHighlight(tmp) + "</pre>")
```json
{
    "返回内容": "http://www.xxx.com?inviter\_code=EX\*\*\*&inviter\_code\_s=EX\*\*\*  
&relation\_id=510\*\*&special\_id=&account\_name=w\*g&s\_note=999999&tao\_id=2586\*\*\*&tao\_nick=淘客\*\*&desc=绑定成功", 
}
```

**多个邀请码返回示例：desc=绑定成功，表示备案成功，其他值表示备案失败，具体参考desc返回内容**

var tmp = { "返回内容": "http://www.xxx.com?inviter\_code=EX\*\*\*|EX\*\*\*|EX\*\*\*&inviter\_code=EX\*\*\*|EX\*\*\*|EX\*\*\*&relation\_id=510\*\*|520\*\*|530\*\*&special\_id=540\*\*|550\*\*|560\*\*&account\_name=w\*g&s\_note=999999&tao\_id=2586\*\*\*&tao\_nick=淘客\*\*&desc=绑定成功" }; document.write("<pre>" + syntaxHighlight(tmp) + "</pre>")
```json
{
    "返回内容": "http://www.xxx.com?inviter\_code=EX\*\*\*|EX\*\*\*|EX\*\*\*&inviter\_code=EX\*\*\*|EX\*\*\*|EX\*\*\*  
&relation\_id=510\*\*|520\*\*|530\*\*&special\_id=540\*\*|550\*\*|560\*\*&account\_name=w\*g&s\_note=999999&tao\_id=2586\*\*\*&tao\_nick=淘客\*\*&desc=绑定成功", 
}
```

接口调用说明：  
1、淘宝客渠道备案API：用户进行渠道备案，生成新的渠道ID。支持渠道备案和会员备案。  

  
  
  

[渠道ID说明](http://www.zhetaoke.com/help_detail_3_20.html)

---

## [渠道]淘宝客渠道信息查询

**原始链接**: [[渠道]淘宝客渠道信息查询](https://www.zhetaoke.com/user/open/open_sc_publisher_get.aspx)

**淘宝客渠道信息查询API---本接口采用12台16核心64G内存服务器，江苏和北京双机房，负载均衡技术，1G独享带宽，放心调用。**

淘宝客渠道信息查询API：获取渠道ID信息列表。[淘宝开放平台文档》》](http://open.taobao.com/api.htm?docId=37989&docType=2&scopeId=14474)

**接口地址：**https://api.zhetaoke.com:10001/api/open\_sc\_publisher\_get.ashx

**请求实例：**https://api.zhetaoke.com:10001/api/open\_sc\_publisher\_get.ashx?appkey=0a565628b2fb4712902712734cec2feb&sid=#sid#&relation\_app=common&info\_type=1

**返回数据：**json

**请求方式：**get或者post

**请求参数说明：**

|  | 名称 | 类型 | 必填 | 说明 |
| --- | --- | --- | --- | --- |
|   | appkey | string | 是 | 折淘客的对接秘钥appkey |
|   | sid | string | 是 | 对应的淘客账号授权ID[点击查看](https://www.zhetaoke.com/user/shouquan.html) |
|   | relation\_app | string | 是 | 渠道推广的物料类型，示例值：common |
|   | info\_type | int | 是 | 类型，必选 1:渠道信息；2:会员信息 |
|   | relation\_id | int | 否 | 渠道备案 - 渠道关系ID |
|   | special\_id | string | 否 | 会员运营ID |
|   | external\_id | string | 否 | 淘宝客外部用户标记，如自身系统账户ID；微信ID等 |
|   | page\_no | int | 否 | 第几页，注意，页数从0开始 |
|   | page\_size | int | 否 | 每页大小 |

**成功返回示例：**

var tmp = { "relation\_app": "common", "create\_date": "2018-06-01 11:12:23", "account\_name": "xxx", "real\_name": "xxx", "relation\_id": 40232, "offline\_scene": "门店", "online\_scene": "微信群", "note": "小蜜蜂", "root\_pid": "mm\_1\_1\_1", "rtag": "小苹果" }; document.write("<pre>" + syntaxHighlight(tmp) + "</pre>")
```json
{
    "relation\_app":40232,                           //渠道备案 - 渠道推广的物料类型
    "create\_date":"2018-06-01 11:12:23",            //渠道备案 - 备案日期
    "account\_name":"xxx",                             //渠道备案 - 渠道昵称
    "real\_name":"xxx",                            //渠道备案 - 渠道姓名
    "relation\_id":40232,                            //渠道备案 - 渠道关系ID
    "offline\_scene":"其他",                            //渠道备案 - 线下场景信息，1 - 门店，2- 学校，3 - 工厂，4 - 其他
    "online\_scene":"其他",                            //渠道备案 - 线上场景信息，1 - 微信群，2- QQ群，3 - 其他
    "note":"小蜜蜂",                            //媒体侧渠道备注信息
    "root\_pid":"mm\_1\_1\_1",                            //渠道专属pid
    "rtag":"小苹果"                           //rtag信息，此字段不一定返回
}
```

接口调用说明：  
1、渠道推广专属功能  

[渠道ID说明](http://www.zhetaoke.com/help_detail_3_20.html)

---

## 批量全网商品详情(简版)

**原始链接**: [批量全网商品详情(简版)](https://www.zhetaoke.com/user/open/open_item_info.aspx)

**全网商品详情API（简版）---本接口采用12台16核心64G内存服务器，江苏和北京双机房，负载均衡技术，1G独享带宽，放心调用。**

重要通知！重要通知！重要通知！此接口新增淘客sid和pid两个参数，这两个字段为必填字段！！！  
重要通知！重要通知！重要通知！此接口新增淘客sid和pid两个参数，这两个字段为必填字段！！！  
重要通知！重要通知！重要通知！此接口新增淘客sid和pid两个参数，这两个字段为必填字段！！！  
重要通知！重要通知！重要通知！此接口新增淘客sid和pid两个参数，这两个字段为必填字段！！！  
2024年3月1日上午9点，sid和pid参数必须填写，不填将无法获取数据，该时间也可能根据官方要求提前，请大家尽快修改。  

全网商品详情API（简版）：输入1个或者最大20个商品ID串，返回对应商品详情（简版）。[淘宝开放平台文档》》](http://open.taobao.com/api.htm?docId=24518&docType=2)

**接口地址：**https://api.zhetaoke.com:10001/api/open\_item\_info.ashx

**请求实例：**https://api.zhetaoke.com:10001/api/open\_item\_info.ashx?appkey=0a565628b2fb4712902712734cec2feb&sid=&pid=&num\_iids=#num\_iids#

**返回数据：**json

**请求方式：**get或者post

**请求参数说明：**

|  | 名称 | 类型 | 必填 | 说明 |
| --- | --- | --- | --- | --- |
|   | appkey | string | 是 | 折淘客的对接秘钥appkey |
|   | sid | string | 是 | 对应的淘客账号授权ID[点击查看](https://www.zhetaoke.com/user/shouquan.html) |
|   | pid | string | 是 | 淘客PID，mm\_xxx\_xxx\_xxx,三段格式，必须与授权的账户相同，否则出错 |
|   | num\_iids | string | 是 | 商品ID串，用,分割，最大20个 |

**成功返回示例：**

var tmp = { "tbk\_item\_info\_get\_response": { "results": { "n\_tbk\_item": \[ { "cat\_leaf\_name": "杀虫剂（卫生农药）", "cat\_name": "洗护清洁剂/卫生巾/纸/香薰", "free\_shipment": true, "hot\_flag": "0", "input\_num\_iid": "npmPzqGtMtP9jjPUnBIYUP-ww3abJub5qmKd4QSN", "item\_url": "https://uland.taobao.com/item/edetail?id=bgm3zNoUgtK6NkJteqTQt6-ww3abJub5qmKd4QSN", "ju\_online\_end\_time": "0", "ju\_online\_start\_time": "0", "ju\_pre\_show\_end\_time": "0", "ju\_pre\_show\_start\_time": "0", "material\_lib\_type": "1,2", "nick": "BASF巴斯夫旗舰店", "num\_iid": "bgm3zNoUgtK6NkJteqTQt6-ww3abJub5qmKd4QSN", "pict\_url": "https://img.alicdn.com/bao/uploaded/i3/2200743383601/O1CN01p5HxRd1cTJ0PQK3DT\_!!0-item\_pic.jpg", "presale\_deposit": "0", "presale\_end\_time": 0, "presale\_start\_time": 0, "presale\_tail\_end\_time": 0, "presale\_tail\_start\_time": 0, "provcity": "浙江 衢州", "reserve\_price": "49.9", "seller\_id": 253469320469820929, "small\_images": { "string": \[ "https://img.alicdn.com/i3/2200743383601/O1CN01GvrroS1cTIy5fphDW\_!!0-item\_pic.jpg", "https://img.alicdn.com/i1/2200743383601/O1CN010wKJr71cTIlPWT3fR\_!!2200743383601.png", "https://img.alicdn.com/i1/2200743383601/O1CN01ScM4Rz1cTIkjbsmiK\_!!2200743383601.png" \] }, "superior\_brand": "0", "title": "德国巴斯夫家用蟑螂药灭杀除跳蚤喷雾全窝床上非无毒室内杀虫剂端", "tmall\_play\_activity\_end\_time": 0, "tmall\_play\_activity\_start\_time": 0, "user\_type": 1, "volume": 50000, "zk\_final\_price": "39.9" } \] }, "request\_id": "15rkm7pmkm3af" } }; document.write("<pre>" + syntaxHighlight(tmp) + "</pre>")
```json
{
        "tbk\_item\_info\_get\_response": {
        "results": {
        "n\_tbk\_item": \[
        {
        "cat\_leaf\_name": "杀虫剂（卫生农药）",
        "cat\_name": "洗护清洁剂/卫生巾/纸/香薰",
        "free\_shipment": true,
        "hot\_flag": "0",
        "input\_num\_iid": "npmPzqGtMtP9jjPUnBIYUP-ww3abJub5qmKd4QSN",
        "item\_url": "https://uland.taobao.com/item/edetail?id=bgm3zNoUgtK6NkJteqTQt6-ww3abJub5qmKd4QSN",
        "ju\_online\_end\_time": "0",
        "ju\_online\_start\_time": "0",
        "ju\_pre\_show\_end\_time": "0",
        "ju\_pre\_show\_start\_time": "0",
        "material\_lib\_type": "1,2",
        "nick": "BASF巴斯夫旗舰店",
        "num\_iid": "bgm3zNoUgtK6NkJteqTQt6-ww3abJub5qmKd4QSN",
        "pict\_url": "https://img.alicdn.com/bao/uploaded/i3/2200743383601/O1CN01p5HxRd1cTJ0PQK3DT\_!!0-item\_pic.jpg",
        "presale\_deposit": "0",
        "presale\_end\_time": 0,
        "presale\_start\_time": 0,
        "presale\_tail\_end\_time": 0,
        "presale\_tail\_start\_time": 0,
        "provcity": "浙江 衢州",
        "reserve\_price": "49.9",
        "seller\_id": 253469320469820929,
        "small\_images": {
        "string": \[
        "https://img.alicdn.com/i3/2200743383601/O1CN01GvrroS1cTIy5fphDW\_!!0-item\_pic.jpg",
        "https://img.alicdn.com/i1/2200743383601/O1CN010wKJr71cTIlPWT3fR\_!!2200743383601.png",
        "https://img.alicdn.com/i1/2200743383601/O1CN01ScM4Rz1cTIkjbsmiK\_!!2200743383601.png"
        \]
        },
        "superior\_brand": "0",
        "title": "德国巴斯夫家用蟑螂药灭杀除跳蚤喷雾全窝床上非无毒室内杀虫剂端",
        "tmall\_play\_activity\_end\_time": 0,
        "tmall\_play\_activity\_start\_time": 0,
        "user\_type": 1,
        "volume": 50000,
        "zk\_final\_price": "39.9"
        }
        \]
        },
        "request\_id": "15rkm7pmkm3af"
        }
}
```

接口调用说明：  
1、所有接口均免费提供，方便大家开发和接入自己的产品早日成为大牛淘客！  
2、使用高佣转链接口，生成您的淘客链接会自动申请最高佣金，包括鹊桥、定向计划、和营销计划（建议用户点击商品时触发式调用，降低服务器压力）！  
3、如需指定优惠券，在生成的二合一链接后，加入 &activityId=xxx券ID即可走指定券

---

## 淘宝联盟猜你喜欢商品API

**原始链接**: [淘宝联盟猜你喜欢商品API](https://www.zhetaoke.com/user/open/open_item_guess_like2.aspx)

**淘宝联盟猜你喜欢商品API---本接口采用12台16核心64G内存服务器，江苏和北京双机房，负载均衡技术，1G独享带宽，放心调用。**

重要通知！重要通知！重要通知！此接口新增淘客sid和pid两个参数，这两个字段为必填字段！！！  
重要通知！重要通知！重要通知！此接口新增淘客sid和pid两个参数，这两个字段为必填字段！！！  
重要通知！重要通知！重要通知！此接口新增淘客sid和pid两个参数，这两个字段为必填字段！！！  
重要通知！重要通知！重要通知！此接口新增淘客sid和pid两个参数，这两个字段为必填字段！！！  
2024年3月1日上午9点，sid和pid参数必须填写，不填将无法获取数据，该时间也可能根据官方要求提前，请大家尽快修改。  

淘宝联盟猜你喜欢商品API：传入用户设备信息，即可返回个性化的推荐结果，实现千人千面效果。。[淘宝开放平台文档》》](https://open.taobao.com/API.htm?docId=33947&docType=2)

**接口地址：**https://api.zhetaoke.com:10001/api/open\_item\_guess\_like2.ashx

**请求实例：**https://api.zhetaoke.com:10001/api/open\_item\_guess\_like2.ashx?appkey=0a565628b2fb4712902712734cec2feb&sid=&pid=&device\_value=&device\_encrypt=MD5&device\_type=IMEI

**返回数据：**json

**请求方式：**get或者post

**请求参数说明：**

|  | 名称 | 类型 | 必填 | 说明 |
| --- | --- | --- | --- | --- |
|   | appkey | string | 是 | 折淘客的对接秘钥appkey |
|   | sid | string | 是 | 对应的淘客账号授权ID[点击查看](https://www.zhetaoke.com/user/shouquan.html) |
|   | pid | string | 是 | 淘客PID，mm\_xxx\_xxx\_xxx,三段格式，必须与授权的账户相同，否则出错 |
|   | page | int | 否 | 分页获取数据,第几页 |
|   | page\_size | int | 否 | 每页数据条数（默认每页20条），可自定义1-50之间 |
|   | device\_value | string | 否 | 智能匹配-设备号加密后的值（MD5加密需32位小写），类型为OAID时传原始OAID值 |
|   | device\_encrypt | string | 否 | 智能匹配-设备号加密类型：MD5，类型为OAID时不传 |
|   | device\_type | string | 否 | 智能匹配-设备号类型：IMEI，或者IDFA，或者UTDID（UTDID不支持MD5加密），或者OAID |

**成功返回示例：**

var tmp = { "tbk\_dg\_optimus\_material\_response": { "is\_default": "false", "result\_list": { "map\_data": \[ { "category\_id": 50015380, "category\_name": "犬主粮", "click\_url": "//s.click.taobao.com/t?e=", "commission\_rate": "4.0", "coupon\_amount": 30, "coupon\_click\_url": "//uland.taobao.com/coupon/edetail?e=", "coupon\_end\_time": "1610207999000", "coupon\_remain\_count": 10000, "coupon\_share\_url": "//uland.taobao.com/coupon/edetail?e=", "coupon\_start\_fee": "275.0", "coupon\_start\_time": "1607788800000", "coupon\_total\_count": 10000, "item\_description": "", "item\_id": 521273701496, "level\_one\_category\_id": 29, "level\_one\_category\_name": "宠物/宠物食品及用品", "nick": "eastboyzhang", "pict\_url": "//img.alicdn.com/bao/uploaded/i4/190003103/O1CN01XEUi7a1YnDbCYjEbq\_!!190003103.jpg", "presale\_deposit": "0", "presale\_end\_time": 0, "presale\_start\_time": 0, "presale\_tail\_end\_time": 0, "presale\_tail\_start\_time": 0, "reserve\_price": "275", "seller\_id": 190003103, "shop\_title": "狗狗之家狗粮精品店", "short\_title": "力狼狗粮狼部落幼犬孕哺乳8kg狗粮", "small\_images": { "string": \[ "//img.alicdn.com/i1/190003103/O1CN01eRxYgg1YnDWz9J1fx\_!!190003103.jpg", "//img.alicdn.com/i1/190003103/O1CN01EbW1to1YnDX2Ud5Pw\_!!190003103.jpg", "//img.alicdn.com/i3/190003103/TB2M8S.gXXXXXaFXpXXXXXXXXXX\_!!190003103.jpg", "//img.alicdn.com/i2/190003103/O1CN01hEWv7A1YnDWzGg1X8\_!!190003103.jpg" \] }, "sub\_title": "", "title": "力狼狗粮狼部落幼犬孕犬哺乳犬狗粮8kg包邮送狗粮", "tmall\_play\_activity\_end\_time": 0, "tmall\_play\_activity\_start\_time": 0, "user\_type": 0, "volume": 10, "white\_image": "", "x\_id": "Rdsvqwt39XmBOHucrfEiQT7dTIRF47ERST8zZ8UvmQkYA9KK4FYlVqDkTlglRVrefEFjtzubEFbge6S8XC1Y7m4lRYj1ZWW7Y0BxGwaCvIk6STfEfBlBPp6PwtZoenzR4", "zk\_final\_price": "275" } \] }, "request\_id": "5bdqqdumgl4n" } }; document.write("<pre>" + syntaxHighlight(tmp) + "</pre>")
```json
{
    "tbk\_dg\_optimus\_material\_response":{
        "is\_default":"false",
        "result\_list":{
            "map\_data":\[
                {
                    "category\_id":50015380,
                    "category\_name":"犬主粮",
                    "click\_url":"//s.click.taobao.com/t?e=",
                    "commission\_rate":"4.0",
                    "coupon\_amount":30,
                    "coupon\_click\_url":"//uland.taobao.com/coupon/edetail?e=",
                    "coupon\_end\_time":"1610207999000",
                    "coupon\_remain\_count":10000,
                    "coupon\_share\_url":"//uland.taobao.com/coupon/edetail?e=",
                    "coupon\_start\_fee":"275.0",
                    "coupon\_start\_time":"1607788800000",
                    "coupon\_total\_count":10000,
                    "item\_description":"",
                    "item\_id":521273701496,
                    "level\_one\_category\_id":29,
                    "level\_one\_category\_name":"宠物/宠物食品及用品",
                    "nick":"eastboyzhang",
                    "pict\_url":"//img.alicdn.com/bao/uploaded/i4/190003103/O1CN01XEUi7a1YnDbCYjEbq\_!!190003103.jpg",
                    "presale\_deposit":"0",
                    "presale\_end\_time":0,
                    "presale\_start\_time":0,
                    "presale\_tail\_end\_time":0,
                    "presale\_tail\_start\_time":0,
                    "reserve\_price":"275",
                    "seller\_id":190003103,
                    "shop\_title":"狗狗之家狗粮精品店",
                    "short\_title":"力狼狗粮狼部落幼犬孕哺乳8kg狗粮",
                    "small\_images":{
                        "string":\[
                            "//img.alicdn.com/i1/190003103/O1CN01eRxYgg1YnDWz9J1fx\_!!190003103.jpg",
                            "//img.alicdn.com/i1/190003103/O1CN01EbW1to1YnDX2Ud5Pw\_!!190003103.jpg",
                            "//img.alicdn.com/i3/190003103/TB2M8S.gXXXXXaFXpXXXXXXXXXX\_!!190003103.jpg",
                            "//img.alicdn.com/i2/190003103/O1CN01hEWv7A1YnDWzGg1X8\_!!190003103.jpg"
                        \]
                    },
                    "sub\_title":"",
                    "title":"力狼狗粮狼部落幼犬孕犬哺乳犬狗粮8kg包邮送狗粮",
                    "tmall\_play\_activity\_end\_time":0,
                    "tmall\_play\_activity\_start\_time":0,
                    "user\_type":0,
                    "volume":10,
                    "white\_image":"",
                    "x\_id":"Rdsvqwt39XmBOHucrfEiQT7dTIRF47ERST8zZ8UvmQkYA9KK4FYlVqDkTlglRVrefEFjtzubEFbge6S8XC1Y7m4lRYj1ZWW7Y0BxGwaCvIk6STfEfBlBPp6PwtZoenzR4",
                    "zk\_final\_price":"275"
                }
            \]
        },
        "request\_id":"5bdqqdumgl4n"
    }
}
```

接口调用说明：  
1、所有接口均免费提供，方便大家开发和接入自己的产品早日成为大牛淘客！  
2、使用高佣转链接口，生成您的淘客链接会自动申请最高佣金，包括鹊桥、定向计划、和营销计划（建议用户点击商品时触发式调用，降低服务器压力）！  
3、如需指定优惠券，在生成的二合一链接后，加入 &activityId=xxx券ID即可走指定券

---

## 淘宝联盟官方活动转链API

**原始链接**: [淘宝联盟官方活动转链API](https://www.zhetaoke.com/user/open/open_activitylink_get.aspx)

**淘宝联盟官方活动转链API---本接口采用12台16核心64G内存服务器，江苏和北京双机房，负载均衡技术，1G独享带宽，放心调用。**

目前转链分代理类型、发单类型、导购类型等，如果转链有问题或者搞不清楚就全部授权即可，后台接口自动判断使用哪个授权类型。  

淘宝联盟官方活动转链API：根据淘宝联盟官方活动ID获取自己的官方活动推广URL。[淘宝开放平台文档》》](https://open.taobao.com/api.htm?spm=a219a.7386797.0.0.26f9669aHgy4YD&source=search&docId=48417&docType=2)

**接口地址：**https://api.zhetaoke.com:10001/api/open\_activitylink\_get.ashx

**请求实例：**https://api.zhetaoke.com:10001/api/open\_activitylink\_get.ashx?appkey=0a565628b2fb4712902712734cec2feb&sid=#sid#&adzone\_id=#adzone\_id#&site\_id=#site\_id#&promotion\_scene\_id=#promotion\_scene\_id#

**返回数据：**json

**请求方式：**get或者post

**请求参数说明：**

|  | 名称 | 类型 | 必填 | 说明 |
| --- | --- | --- | --- | --- |
|   | appkey | string | 是 | 折淘客的对接秘钥appkey |
|   | sid | string | 是 | 对应的淘客账号授权ID[点击查看](https://www.zhetaoke.com/user/shouquan.html) |
|   | adzone\_id | string | 是 | 推广位id，mm\_xx\_xx\_xx pid三段式中的第三段 |
|   | site\_id | int | 是 | 推广位id，mm\_xx\_xx\_xx pid三段式中的第二段 |
|   | promotion\_scene\_id | string | 是 | 官方活动ID，从官方活动页获取。[点击查看官方活动](https://pub.alimama.com/portal/v2/pages/activity/official/index.htm?pageNum=1&sortType=&sortOrder=)  
饿了么微信推广活动：20150318020002192，正式上线时间：2020年12月10日，小程序专用活动  
饿了么聚合页CPS推广活动ID：20150318020002597，正式上线时间：2020年10月10日  
饿了么果蔬商超活动ID：20150318020004284，正式上线时间：2020年6月1日  
饿了么早期活动ID：1571715733668，正式上线时间：2019年12月4日  
口碑主会场活动ID：1583739244161，正式上线时间：2020年3月17日  
生活服务分会场活动ID：1583739244162，正式上线时间：2020年3月17日  
飞猪旅行活动ID：20150318020002751，正式上线时间：2020年3月17日  
聚划算百亿补贴活动ID：20150318020000462，正式上线时间：2020年3月17日  
特价超级一分购活动ID：20150318020004151，正式上线时间：2020年3月17日  
限量一元购活动ID：20150318020003158，正式上线时间：2020年3月17日  
返回【本地化】微信推广二维码地址、推广长链接、推广短链接、物料素材下载地址、【本地化】微信小程序推广地址 |
|   | union\_id | string | 否 | 自定义输入串，英文和数字组成，长度不能大于12个字符，区分不同的推广渠道，目前该字段没用。 |
|   | relation\_id | string | 否 | 渠道关系ID，仅适用于渠道推广场景 |
|   | signurl | int | 否 | 值为0表示直接返回最终结果。 |

**成功返回示例：click\_url值为https://s.click.taobao.com开头的长链接，即表示返回正常，其余为失败。**

var tmp = { "tbk\_sc\_activity\_info\_get\_response": { "data": { "click\_url": "https://s.click.taobao.com/t?union\_lens=", "page\_end\_time": "2020-09-27", "page\_name": "超值-直营主会场", "page\_start\_time": "2020-09-23", "terminal\_type": "1" }, "request\_id": "v21g6p2jtt0o" } }; document.write("<pre>" + syntaxHighlight(tmp) + "</pre>")
```json
{
    "tbk\_sc\_activity\_info\_get\_response":{
        "data":{
            "click\_url":"https://s.click.taobao.com/t?union\_lens=",
            "page\_end\_time":"2020-09-27",
            "page\_name":"超值-直营主会场",
            "page\_start\_time":"2020-09-23",
            "terminal\_type":"1"
        },
        "request\_id":"v21g6p2jtt0o"
    }
}
```

接口调用说明：  
1、淘宝联盟官方活动转链API：根据淘宝联盟官方活动ID获取自己的官方活动推广URL；  
2、获取到长链接后，可通过其它API接口转成短链接或者生成淘口令。

  

[接口测试](https://www.zhetaoke.com/user/open/test_open_activitylink_get.aspx)

[活动列表](https://www.zhetaoke.com/list/activity_1_0_1.html)

---

## 淘宝联盟官方活动列表API

**原始链接**: [淘宝联盟官方活动列表API](https://www.zhetaoke.com/user/open/open_activity_list.aspx)

**淘宝联盟官方活动列表API---本接口采用12台16核心64G内存服务器，江苏和北京双机房，负载均衡技术，1G独享带宽，放心调用。**

淘宝联盟官方活动列表API：返回联盟官方活动列表，目前可返回淘宝联盟官方活动、京东联盟官方活动、美团联盟官方活动、饿了么官方活动。

**接口地址：** https://api.zhetaoke.com:10001/api/api\_activity.ashx?appkey=0a565628b2fb4712902712734cec2feb&activityId=&platformType=

**返回数据：**json

**请求方式：**get

**请求参数说明：**

|  | 名称 | 类型 | 必填 | 说明 |
| --- | --- | --- | --- | --- |
|   | appkey | string | 是 | 折淘客的对接秘钥appkey |
|   | activityId | string | 否 | 联盟官方活动ID |
|   | platformType | string | 否 | 投放平台，无线或PC，1为PC，2为无线 |
|   | type | string | 否 | 表示哪个平台的官方活动，type=0表示淘宝联盟官方活动，type=10表示美团联盟官方活动，type=11表示饿了么官方活动，type=12表示京东联盟官方活动，type=其他值表示淘宝联盟官方活动 |

**成功返回示例：**

var tmp = { "status": 200, "content": \[ { "activityId": "1574664919339", "pageUrl": "https://huodong.taobao.com/wow/a/act/tao/tmc/24519/wupr?wh\_pid=main-177305", "avgCommissionRate": "2.83", "pageName": "2019淘宝双12-主会场（不带超级红包）", "bannerUrl": "https://img.alicdn.com/tfs/TB1xz1Foy\_1gK0jSZFqXXcpaXXa-480-180.jpg", "platformName": "淘宝", "platformType": "2", "startTime": "2019/12/01", "endTime": "2019/12/12" }, { "activityId": "1574664919372", "pageUrl": "https://huodong.taobao.com/wow/a/act/tao/tmc/24519/wupr?wh\_pid=main-177305", "avgCommissionRate": "3.48", "pageName": "2019淘宝双12-主会场（不带超级红包）", "bannerUrl": "https://img.alicdn.com/tfs/TB1xz1Foy\_1gK0jSZFqXXcpaXXa-480-180.jpg", "platformName": "淘宝", "platformType": "2", "startTime": "2019/12/01", "endTime": "2019/12/12" }, { "activityId": "1574664919373", "pageUrl": "https://huodong.taobao.com/wow/a/act/tao/tmc/24519/wupr?wh\_pid=industry-177213", "avgCommissionRate": "2.96", "pageName": "2019淘宝双12-时髦女装", "bannerUrl": "https://img.alicdn.com/tfs/TB18ZiFouH2gK0jSZFEXXcqMpXa-440-180.jpg", "platformName": "淘宝", "platformType": "2", "startTime": "2019/12/01", "endTime": "2019/12/12" } \] }; document.write("<pre>" + syntaxHighlight(tmp) + "</pre>")
```json
{
    "status":200,
    "content":\[
        {
            "activityId":"1574664919339",       /\*官方活动ID\*/
            "pageUrl":"https://huodong.taobao.com/wow/a/act/tao/tmc/24519/wupr?wh\_pid=main-177305",     /\*官方活动原始链接，一定要通过官方活动推广API进行转链，才能推广。\*/
            "avgCommissionRate":"2.83",     /\*平均佣金比率\*/
            "pageName":"2019淘宝双12-主会场（不带超级红包）",     /\*官方活动标题\*/
            "bannerUrl":"https://img.alicdn.com/tfs/TB1xz1Foy\_1gK0jSZFqXXcpaXXa-480-180.jpg",       /\*官方活动图片\*/
            "platformName":"淘宝",        /\*类型，天猫或淘宝\*/
            "platformType":"2",     /\*投放平台，无线或PC，1为PC，2为无线\*/
            "startTime":"2019/12/01",       /\*开始时间\*/
            "endTime":"2019/12/12"      /\*结束时间\*/
        },
        {
            "activityId":"1574664919372",
            "pageUrl":"https://huodong.taobao.com/wow/a/act/tao/tmc/24519/wupr?wh\_pid=main-177305",
            "avgCommissionRate":"3.48",
            "pageName":"2019淘宝双12-主会场（不带超级红包）",
            "bannerUrl":"https://img.alicdn.com/tfs/TB1xz1Foy\_1gK0jSZFqXXcpaXXa-480-180.jpg",
            "platformName":"淘宝",
            "platformType":"2",
            "startTime":"2019/12/01",
            "endTime":"2019/12/12"
        },
        {
            "activityId":"1574664919373",
            "pageUrl":"https://huodong.taobao.com/wow/a/act/tao/tmc/24519/wupr?wh\_pid=industry-177213",
            "avgCommissionRate":"2.96",
            "pageName":"2019淘宝双12-时髦女装",
            "bannerUrl":"https://img.alicdn.com/tfs/TB18ZiFouH2gK0jSZFEXXcqMpXa-440-180.jpg",
            "platformName":"淘宝",
            "platformType":"2",
            "startTime":"2019/12/01",
            "endTime":"2019/12/12"
        }
    \]
}
```

  
  
  
  
  
  
  
  
  
  
  

[接口测试](https://www.zhetaoke.com/user/open/test_open_activitylink_get.aspx)

[活动列表](https://www.zhetaoke.com/list/activity_1_0_1.html)

---

## 创建推广位API

**原始链接**: [创建推广位API](https://www.zhetaoke.com/user/open/open_create_pid2.aspx)

**创建推广位API---本接口采用12台16核心64G内存服务器，江苏和北京双机房，负载均衡技术，1G独享带宽，放心调用。**

创建推广位API：为淘宝客提供创建推广位功能。

**接口地址：**https://api.zhetaoke.com:10001/api/open\_create\_pid2.ashx

**请求实例：**https://api.zhetaoke.com:10001/api/open\_create\_pid2.ashx?appkey=0a565628b2fb4712902712734cec2feb&sid=&site\_id=&adzone\_name=

**返回数据：**json

**请求方式：**get或者post

**请求参数说明：**

|  | 名称 | 类型 | 必填 | 说明 |
| --- | --- | --- | --- | --- |
|   | appkey | string | 是 | 折淘客的对接秘钥appkey |
|   | sid | string | 是 | 对应的淘客账号授权ID[点击查看](https://www.zhetaoke.com/user/shouquan.html) |
|   | site\_id | string | 是 | 网站ID或者媒体ID，淘宝联盟后台创建，推广管理-媒体备案管理-新增媒体备案，选择适合自己的备案类型。  
自己有网站或者app用自有平台类型，没有用他方平台类型。 |
|   | adzone\_name | string | 是 | 推广位名称，最大长度64字符，不要带空格变量等特殊符号。 |

**成功返回示例：**

var tmp = { "tbk\_sc\_adzone\_create\_response": { "data": { "model": "mm\_111111\_222222\_333333" } } } document.write("<pre>" + syntaxHighlight(tmp) + "</pre>")
```json
{
                                "tbk\_sc\_adzone\_create\_response": {
                                    "data": {
                                        "model": "mm\_111111\_222222\_333333"
                                    }
                                }
                            }
```

**失败返回示例：**

var tmp = { "status": 301, "content": "很抱歉！因某某原因而失败的错误提示！" }; document.write("<pre>" + syntaxHighlight(tmp) + "</pre>")
```json
{"status":301,"content":"很抱歉！因某某原因而失败的错误提示！"}
```

接口调用说明：  
1、所有接口均免费提供，方便大家开发和接入自己的产品早日成为大牛淘客！  
2、使用高佣转链接口，生成您的淘客链接会自动申请最高佣金，包括鹊桥、定向计划、和营销计划（建议用户点击商品时触发式调用，降低服务器压力）！  
3、如需指定优惠券，在生成的二合一链接后，加入 &activityId=xxx券ID即可走指定券  
4、如需自行请求数据，请设置signurl为1，这样就只返回请求地址

---

## 联盟订单查询API

**原始链接**: [联盟订单查询API](https://www.zhetaoke.com/user/open/open_dingdanchaxun_lianmeng.aspx)

**联盟订单查询API---本接口采用12台16核心64G内存服务器，江苏和北京双机房，负载均衡技术，1G独享带宽，放心调用。**

**接口名称：**联盟订单查询API（折淘客联盟结算佣金的所有订单，都可以通过此接口查询。）

**接口地址：**https://api.zhetaoke.com:10001/api/open\_lianmeng\_orderList.ashx

**返回数据：**json

**请求方式：**get

**请求示例：**https://api.zhetaoke.com:10001/api/open\_lianmeng\_orderList.ashx?appkey=&type=&startTime=&endTime=

根据订单支付时间或者订单更新时间获取最近90天内的折淘客联盟订单数据  
支持按支付时间、更新时间查询。  
建议按照订单支付时间，每分钟查询一次，按照订单更新时间，每分钟查询一次即可。  
个别长时间不结算的订单可以单独用订单编号来查询。  
  
[点击这里在线查询折淘客联盟订单信息！点击这里在线查询折淘客联盟订单信息！点击这里在线查询折淘客联盟订单信息！](https://www.zhetaoke.com/user/extend/extend_app_dingdan_lianmeng.aspx)  
  
请注意，此接口只返回折淘客联盟结算佣金的所有订单。  
请注意，此接口只返回折淘客联盟结算佣金的所有订单。  
请注意，此接口只返回折淘客联盟结算佣金的所有订单。  
  

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
|   | san\_pingtai\_id | String | 否 | 订单所属平台id，1美团、2考拉、3苏宁、4淘宝、5京东众筹、6拼多多、7唯品会、8饿了么、9抖音、12京东出单等 |

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
            "smstitle2":"霸碗盖码饭店",    /\*店铺名称\*/
            "item\_id":"111000000",    /\*商品编号，可能为空\*/
            "refundprice":0,      /\*退款金额\*/
            "refundtime":"",      /\*退款时间\*/
            "status":"8",     /\*订单状态，1已付款，8已完成，9已退款或风控\*/
            "update\_time":"",   /\*数据最后更新时间\*/
            "type":"4",   /\*订单类型\*/
            "sid\_ztk":"16",   /\*折淘客授权sid\*/
            "customer\_id\_ztk":"100000",   /\*推广者自定义编号\*/
            "platform\_ztk":"zhetaoke",     /\*推广者平台类型，默认zhetaoke\*/
            "actId":"33",   /\*活动id\*/
            "san\_pingtai":"美团",   /\*订单所属平台id，1美团、2考拉、3苏宁、4淘宝、5京东、6拼多多、7唯品会、8饿了么、9抖音、12京东出单等\*/
            "san\_pingtai\_id":"1"   /\*订单所属平台id，1美团、2考拉、3苏宁、4淘宝、5京东、6拼多多、7唯品会、8饿了么、9抖音、12京东出单等\*/
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
            "smstitle":"霸碗盖码饭店",
            "item\_id":"111000000",
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

接口调用说明：  
1、所有接口均免费提供，方便大家开发和接入自己的产品早日成为大牛淘客！  
2、请大家一定要合理使用此接口，每个授权账户调用频率在1分钟或2分钟以上,开发群中有接口具体使用文档。

---

## 全站领券商品API

**原始链接**: [全站领券商品API](https://www.zhetaoke.com/user/extend/extend_lingquan_site.aspx)

**全站领券商品API（可实现增量采集）---本接口采用12台16核心64G内存服务器，江苏和北京双机房，负载均衡技术，1G独享带宽，放心调用。**

重要通知！重要通知！重要通知！此接口新增淘客sid和pid两个参数，这两个字段为必填字段！！！  
重要通知！重要通知！重要通知！此接口新增淘客sid和pid两个参数，这两个字段为必填字段！！！  
重要通知！重要通知！重要通知！此接口新增淘客sid和pid两个参数，这两个字段为必填字段！！！  
重要通知！重要通知！重要通知！此接口新增淘客sid和pid两个参数，这两个字段为必填字段！！！  
2024年3月1日上午9点，sid和pid参数必须填写，不填将无法获取数据，该时间也可能根据官方要求提前，请大家尽快修改。  

全站领券商品API（可实现增量采集）：根据更新时间进行商品查询，返回佣金≥15%，动态描述分≥4.6的商品列表。

**接口地址：** https://api.zhetaoke.com:10001/api/api\_all.ashx?appkey=0a565628b2fb4712902712734cec2feb&page=1&page\_size=20&sort=new

全站领券商品API：返回指定查询条件的商品总数。

**接口地址：** https://api.zhetaoke.com:10001/api/api\_all.ashx?appkey=0a565628b2fb4712902712734cec2feb&page=1&page\_size=20&sort=new&total\_count=1

**备用地址：**http://api.zhetaoke.cn:10000/api/api\_all.ashx

**特殊说明：**[主接口和备用接口详细说明（请大家优先使用com主域名接口地址，在com出现问题的时候，人工或者自动切换cn备用接口地址）](https://www.zhetaoke.com/help_detail_3_27.html)

**返回数据：**json

**请求方式：**get

**重要说明：**可通过本接口进行全站商品采集，排序方式用sort=date\_time\_yongjin，可实现商品增量采集；配合失效商品API接口，可实现365天实时数据采集和更新。

**请求参数说明：**

|  | 名称 | 类型 | 必填 | 说明 |
| --- | --- | --- | --- | --- |
|   | appkey | string | 是 | 折淘客的对接秘钥appkey |
|   | sid | string | 是 | 对应的淘客账号授权ID[点击查看](https://www.zhetaoke.com/user/shouquan.html) |
|   | pid | string | 是 | 淘客PID，mm\_xxx\_xxx\_xxx,三段格式，必须与授权的账户相同，否则出错 |
|   | page | int | 否 | 分页获取数据,第几页 |
|   | page\_size | int | 否 | 每页数据条数（默认每页20条），可自定义1-50之间 |
|   | sort | string | 否 | 商品排序方式，new：按照综合排序，  
total\_sale\_num\_asc：按照总销量从小到大排序，total\_sale\_num\_desc：按照总销量从大到小排序，  
sale\_num\_asc：按照年销量从小到大排序，sale\_num\_desc：按照年销量从大到小排序，  
commission\_rate\_asc：按照佣金比例从小到大排序，commission\_rate\_desc：按照佣金比例从大到小排序，  
price\_asc：按照价格从小到大排序，price\_desc：按照价格从大到小排序，  
coupon\_info\_money\_asc：按照优惠券金额从小到大排序，coupon\_info\_money\_desc：按照优惠券金额从大到小排序，  
shop\_level\_asc：按照店铺等级从低到高排序，shop\_level\_desc：按照店铺等级从高到低排序，  
tkfee\_asc：按照返佣金额从低到高排序，tkfee\_desc：按照返佣金额从高到低排序，  
code：按照code值从大到小排序，date\_time：按照更新时间排序，random：按照随机排序 |
|   | cid | int | 否 | 一级商品分类，值为空：全部商品，1：女装，2：母婴，3：美妆，4：居家日用，5：鞋品，6：美食，7：文娱车品，8：数码家电，9：男装，10：内衣，11：箱包，12：配饰，13：户外运动，14：家装家纺 |
|   | tj | string | 否 | 是否天猫商品，值为空：全部商品，tmall：天猫商品，gold\_seller：金牌卖家 |
|   | jt | string | 否 | 淘抢购或者聚划算，值为空：全部商品，taoqianggou：淘抢购商品，juhuasuan：聚划算商品 |
|   | jh | string | 否 | 海淘或者极有家，值为空：全部商品，haitao：海淘商品，jiyoujia：极有家商品 |
|   | today | string | 否 | 今日商品，值为空：全部商品，1：今日商品 |
|   | yunfeixian | string | 否 | 运费险，值为空：全部商品，1：有运费险商品 |
|   | pinpai | string | 否 | 精选品牌，值为空：全部商品，1：精选品牌商品 |
|   | tianmaochaoshi | string | 否 | 天猫超市，值为空：全部商品，1：天猫超市商品 |
|   | price | string | 否 | 价格区间，值为空：全部商品，0.0-9.9：9.9元商品，0.0-19.9：19.9元商品 |
|   | commission\_rate\_start | string | 否 | 佣金比例≥，值为空：全部商品，=20：佣金比例≥20的商品，=50：佣金比例≥50的商品 |
|   | sale\_num\_start | string | 否 | 年销量≥ 值为空：全部商品，=500：年销量≥500的商品，=10000：年销量≥10000的商品 |
|   | dsr\_start | string | 否 | 动态评分≥ 值为空：全部商品，=4.6：动态评分≥4.6的商品，=4.9：动态评分≥4.9的商品 |
|   | coupon\_amount\_start | string | 否 | 券面额≥ 值为空：全部商品，=5：券面额≥5的商品，=20：券面额≥20的商品，=100：券面额≥100的商品 |
|   | q | string | 否 | 关键词 值为空：全部商品，：返回标题中带内衣的商品。支持关键字、支持店铺名称、支持商品明文链接。 |
|   | baodan | string | 否 | 极品爆单商品，值为空：全部商品，1：极品爆单商品（拍2件、拍3件、拍4件、拍5件、拍6件等） |
|   | py\_baoyou | string | 否 | 偏远地区包邮商品，值为空：全部商品，1：偏远地区包邮商品（新疆等地） |
|   | tag | string | 否 | 是否朋友圈火爆商品，值为空：全部商品，1：朋友圈火爆商品 |
|   | ifashion | string | 否 | 是否潮电街商品，值为空：全部商品，1：潮电街商品 |
|   | start\_date\_time\_yongjin | string | 否 | 数据更新开始时间，格式为：2019-08-22 17:00:00 |
|   | end\_date\_time\_yongjin | string | 否 | 数据更新结束时间，格式为：2019-08-22 17:10:00 |
|   | mission | string | 否 | 是否任务专属商品，值为2：任务专属商品 |

**成功返回示例：**/\*重要事情说三遍！重要事情说三遍！重要事情说三遍！api\_all/api\_dongdong/api\_quantian/api\_shishi/api\_videos/api\_xiaoshi/api\_yongjin接口所有参数都可以自由组合。\*/

var tmp = { "status": 200, /\*状态\*/ "content": \[{ "code": "6646", /\*折淘客编号\*/ "type\_one\_id": "1", /\*分类ID，可参考折淘客分类\*/ "tao\_id": "554832820990", /\*商品ID\*/ "title": "储物箱整理抽屉式收纳柜", /\*商品短标题\*/ "jianjie": "【爆款返场，出口日本】进口PP材质，可叠加透明可视抽屉收纳盒，分格抽屉物品分类齐全，区域分化一目了然，自由组合，大容量，让你居家生活更舒心，收纳你的生活！【赠运费险】", /\*商品简介\*/ "pict\_url": "https://img.alicdn.com/bao/......jpg", /\*商品主图\*/ "user\_type": "0", /\*是否天猫，0是淘宝，1是天猫\*/ "seller\_id": "3073204721", /\*卖家ID\*/ /\*店铺地址可自行拼接，http://store.taobao.com/shop/view\_shop.htm?user\_number\_id=3073204721 \*/ "shop\_dsr": "4.80", /\*商品描述分\*/ "volume": "169", /\*年销量\*/ "size": "26.90", /\*折扣价\*/ "quanhou\_jiage": "16.90", /\*券后价\*/ "date\_time\_yongjin": "2018/11/8 14:45:22", /\*数据更新时间\*/ "tkrate3": "30.00", /\*佣金比率\*/ "yongjin\_type": "MKT", /\*佣金类型\*/ "coupon\_id": "fe31d6c8132c462ba59bd72e03d2eae0", /\*优惠券ID 该商品如无优惠券，此字段为空\*/ "coupon\_start\_time": "2018-11-05 00:00:00", /\*优惠券开始时间 该商品如无优惠券，此字段为空\*/ "coupon\_end\_time": "2018-11-10 23:59:59", /\*优惠券结束时间 该商品如无优惠券，此字段为空\*/ "coupon\_info\_money": "10", /\*优惠券金额 该商品如无优惠券，此字段为0\*/ "coupon\_total\_count": "100000", /\*优惠券总数量 该商品如无优惠券，此字段为0\*/ "coupon\_remain\_count": "99965", /\*优惠券剩余数量 该商品如无优惠券，此字段为0\*/ "coupon\_info": "满80.00元减10元", /\*优惠券信息\*/ "juhuasuan": "0", /\*是否聚划算，1是\*/ "taoqianggou": "0", /\*是否淘抢购，1是\*/ "haitao": "0", /\*是否海淘，1是\*/ "jiyoujia": "0", /\*是否极有家，1是\*/ "jinpaimaijia": "0", /\*是否金牌卖家，1是\*/ "pinpai": "0", /\*是否精选品牌，1是\*/ "pinpai\_name": "", /\*品牌名称\*/ "yunfeixian": "0", /\*是否有运费险，1有\*/ "nick": "\*\*\*旗舰店", /\*卖家昵称\*/ "small\_images": "https://img.alicdn.com/bao/.......jpg|https://img.alicdn.com/bao/.......jpg", /\*商品小图列表\*/ "white\_image": "https://img.alicdn.com/bao/......jpg", /\*商品白底图\*/ "tao\_title": "抽屉式收纳箱塑料储物箱整理箱衣服物家用宿舍衣柜内衣内裤收纳盒", /\*商品长标题\*/ "provcity": "浙江 杭州", /\*宝贝所在地\*/ "shop\_title": "\*\*\*旗舰店", /\*店铺名称\*/ "zhibo\_url": "https://cloud.video.taobao.com/play/u/849090736/p/2/e/6/t/1/219853683676.mp4", /\*视频地址\*/ "sellCount": "860000", /\*淘宝网页实时总销量\*/ "commentCount": "250000", /\*评论数量\*/ "favcount": "180000", /\*收藏数量\*/ "score1": "4.9", /\*宝贝描述分\*/ "score2": "4.9", /\*卖家服务分\*/ "score3": "4.9", /\*物流服务\*/ "creditLevel": "15", /\*店铺等级，一星 二星 三星 四星 五星 一钻 二钻 三钻 四钻 五钻 一皇冠 二皇冠 三皇冠 四皇冠 五皇冠 一金冠 二金冠 三金冠 四金冠 五金冠\*/ "shopIcon": "//img.alicdn.com/imgextra/de/a6/TB10dC8SX.jpg", /\*店铺logo\*/ "pcDescContent": "1.jpg|2.jpg|3.jpg|4.jpg|5.jpg", /\*图文详情图片地址\*/ "item\_url": "https://item.taobao.com/item.htm?id=581801364036", /\*商品url\*/ "category\_id": "50006126", /\*叶子类目id\*/ "category\_name": "收纳盒", /\*叶子类目name\*/ "level\_one\_category\_id": "122928002", /\*一级类目id\*/ "level\_one\_category\_name": "收纳整理", /\*一级类目name\*/ "tkfee3": "5.07", /\*返佣金额\*/ "biaoqian": ""满300元,省30元"", /\*店铺活动\*/ "tag": "赶紧拯救你厨房的油腻吧😂<br>植护🌿加厚厨房抽纸<br>高密度纤维纸张，吸油💧很不错<br>5包装|￥14.8元💰<br>✔厨房小帮手，用处多多", /\*朋友圈文案，需要自己进行urldecode\*/ "date\_time": "2018/11/8 14:45:22" /\*数据添加时间\*/ }\] };
```json
{
	"status": 200,                      /\*状态\*/
	"content": \[{
		"code": "6646",                 /\*折淘客编号\*/
		"type\_one\_id": "1",             /\*分类ID，可参考折淘客分类\*/
		"tao\_id": "554832820990",       /\*商品ID\*/
		"title": "储物箱整理抽屉式收纳柜",                   /\*商品短标题\*/
		"jianjie": "【爆款返场，出口日本】进口PP材质，可叠加透明可视抽屉收纳盒，分格抽屉物品分类齐全，区域分化一目了然，自由组合，大容量，让你居家生活更舒心，收纳你的生活！【赠运费险】", /\*商品简介\*/
		"pict\_url": "https://img.alicdn.com/bao/......jpg",                   /\*商品主图\*/
		"user\_type": "0",                   /\*是否天猫，0是淘宝，1是天猫\*/
		"seller\_id": "3073204721",                   /\*卖家ID\*/ /\*店铺地址可自行拼接，http://store.taobao.com/shop/view\_shop.htm?user\_number\_id=3073204721 \*/
		"shop\_dsr": "4.80",                   /\*商品描述分\*/
		"volume": "169",                   /\*年销量\*/
		"size": "26.90",                   /\*折扣价\*/
		"quanhou\_jiage": "16.90",                   /\*券后价\*/
		"date\_time\_yongjin": "2018/11/8 14:45:22",                   /\*数据更新时间\*/
		"tkrate3": "30.00",                   /\*佣金比率\*/
		"yongjin\_type": "MKT",                   /\*佣金类型\*/
		"coupon\_id": "fe31d6c8132c462ba59bd72e03d2eae0",                   /\*优惠券ID 该商品如无优惠券，此字段为空\*/
		"coupon\_start\_time": "2018-11-05 00:00:00",                   /\*优惠券开始时间 该商品如无优惠券，此字段为空\*/
		"coupon\_end\_time": "2018-11-10 23:59:59",                   /\*优惠券结束时间 该商品如无优惠券，此字段为空\*/
		"coupon\_info\_money": "10",                   /\*优惠券金额 该商品如无优惠券，此字段为0\*/
		"coupon\_total\_count": "100000",                   /\*优惠券总数量 该商品如无优惠券，此字段为0\*/
		"coupon\_remain\_count": "99965",                   /\*优惠券剩余数量 该商品如无优惠券，此字段为0\*/
		"coupon\_info": "满80.00元减10元",                   /\*优惠券信息\*/
		"juhuasuan": "0",                       /\*是否聚划算，1是\*/
		"taoqianggou": "0",                     /\*是否淘抢购，1是\*/
		"haitao": "0",                          /\*是否海淘，1是\*/
		"jiyoujia": "0",                        /\*是否极有家，1是\*/
		"jinpaimaijia": "0",                    /\*是否金牌卖家，1是\*/
		"pinpai": "0",                          /\*是否精选品牌，1是\*/
		"pinpai\_name": "",                      /\*品牌名称\*/
		"yunfeixian": "0",                      /\*是否有运费险，1有\*/
		"nick": "\*\*\*旗舰店",                    /\*卖家昵称\*/
		"small\_images": "https://img.alicdn.com/bao/.......jpg|https://img.alicdn.com/bao/.......jpg",                   /\*商品小图列表\*/
		"white\_image": "https://img.alicdn.com/bao/......jpg",                   /\*商品白底图\*/
		"tao\_title": "抽屉式收纳箱塑料储物箱整理箱衣服物家用宿舍衣柜内衣内裤收纳盒",                   /\*商品长标题\*/
		"provcity": "浙江 杭州",                   /\*宝贝所在地\*/
		"shop\_title": "\*\*\*旗舰店",                   /\*店铺名称\*/
		"zhibo\_url": "https://cloud.video.taobao.com/play/u/849090736/p/2/e/6/t/1/219853683676.mp4",                   /\*视频地址\*/
		"sellCount": "860000",                   /\*淘宝网页实时总销量\*/
		"commentCount": "250000",                   /\*评论数量\*/
		"favcount": "180000",                   /\*收藏数量\*/
		"score1": "4.9",                   /\*宝贝描述分\*/
		"score2": "4.9",                   /\*卖家服务分\*/
		"score3": "4.9",                   /\*物流服务分\*/
		"creditLevel": "15",                   /\*店铺等级（1-20），一星 二星 三星 四星 五星 一钻 二钻 三钻 四钻 五钻 一皇冠 二皇冠 三皇冠 四皇冠 五皇冠  一金冠 二金冠 三金冠 四金冠 五金冠\*/
		"shopIcon": "//img.alicdn.com/imgextra/de/a6/TB10dC8SX.jpg",       /\*店铺logo\*/
		"pcDescContent": "1.jpg|2.jpg|3.jpg|4.jpg|5.jpg",  /\*图文详情图片地址\*/
		"item\_url": "https://item.taobao.com/item.htm?id=581801364036",  /\*商品url\*/
		"category\_id": "50006126",                   /\*叶子类目id\*/
		"category\_name": "收纳盒",                   /\*叶子类目name\*/
		"level\_one\_category\_id": "122928002",                   /\*一级类目id\*/
		"level\_one\_category\_name": "收纳整理",                   /\*一级类目name\*/
		"tkfee3": "5.07",                   /\*返佣金额\*/
		"biaoqian": ""满300元,省30元"",                   /\*店铺活动\*/
		"tag": "赶紧拯救你厨房的油腻吧😂植护🌿加厚厨房抽纸高密度纤维纸张，吸油💧很不错5包装|￥14.8元💰✔厨房小帮手，用处多多",                   /\*朋友圈文案，需要自己进行urldecode\*/
		"date\_time": "2018/11/8 14:45:22"                   /\*数据添加时间\*/
	}\]
}
```

**失败返回示例：**

var tmp = { "status": 301 };
```json
{
	"status": 301                      /\*状态\*/
}
```

[效果展示](http://cms.zhetaoke.com:10000/m/item/list.aspx?cate_id=0)

[商品采集说明](http://www.zhetaoke.com/help_detail_3_22.html)

---

## 线报商品API接口

**原始链接**: [线报商品API接口](https://www.zhetaoke.com/user/extend/extend_lingquan_xianbao.aspx)

**线报商品API接口---本接口采用12台16核心64G内存服务器，江苏和北京双机房，负载均衡技术，1G独享带宽，放心调用。**

线报商品API接口：返回指定群编号的线报商品信息。

**接口地址：** https://api.zhetaoke.com:10001/api/api\_xianbao.ashx?appkey=0a565628b2fb4712902712734cec2feb&id=&type=0&page=1&page\_size=10&msg=1&interval=10

线报群列表API接口：返回折淘客所有线报群列表。

**接口地址：** https://api.zhetaoke.com:10001/api/api\_xianbao\_qun.ashx?appkey=0a565628b2fb4712902712734cec2feb&page=1&page\_size=200

**返回数据：**json

**请求方式：**get

**请求参数说明：**

|  | 名称 | 类型 | 必填 | 说明 |
| --- | --- | --- | --- | --- |
|   | appkey | string | 是 | 折淘客的对接秘钥appkey |
|   | page | int | 否 | 分页获取数据,第几页 |
|   | page\_size | int | 否 | 每页数据条数（默认每页10条），可自定义1-10之间 |
|   | type | int | 否 | type=0，表示qq群，type=1，表示wx群 |
|   | id | string | 否 | qq群号或者wx群号  
id值为空表示获取所有群数据，id值可为单个或者多个，多个中间用英文逗号分割即可。  
  
id=43104737，折淘客精品采集群1（淘宝）  
id=1041362545，折淘客精品采集群8（淘宝）  
id=725556337，折淘客线报群5（淘宝京东拼多多）  
id=807313495，折淘客线报群6（淘宝京东拼多多）  
id=1038411329，折淘客线报群7（淘宝京东拼多多）  
id=1065747294，折淘客线报群8（淘宝京东拼多多）  
id=1067573294，折淘客线报群9（主京东）  
id=340319271，折淘客线报群10（主唯品会，球鞋衣服之类）  
  
id=1038261844，折淘客外卖福利优惠券群1  
id=10000012，折淘客外卖吃喝玩乐优惠券  
id=10000028，【折淘客】千寻京东线报群1  
id=10000029，【折淘客】新京东线报采集群  
id=10000030，【折淘客】新京东带主图采集群  
id=10000031，【折淘客】京东天猫优惠购物群  
id=10000032，【折淘客】京东联盟京粉采集8群  
id=10000033，【折淘客】京东线报采集  
id=10000034，【折淘客】嘿喵线报采集群  
id=10000035，【折淘客】MK京东采集群  
id=10000037，【折淘客】猫类用品采集群  
id=10000038，【折淘客】京东线报首发采集群  
id=10000039，MK拼多多采集群（水果零食蔬菜）  
id=10000040，【折淘客】老街购爆款十群  
id=10000041，【折淘客】葫芦娃采集群  
id=10000042，【折淘客】多啦C梦线报采集群  
id=10000043，【折淘客】老爱购线报采集  
id=10000044，【折淘客】秋名山捡漏基地B  
id=10000045，【折淘客】汤臣一品一号楼あ7  
id=10000046，【折淘客】MK无敌采集群  
id=10000047，【折淘客】开拓者3号  
id=10000048，【折淘客】萌萌的咖啡厅  
id=10000049，【折淘客】神车鸭8群  
id=10000050，【折淘客】京东首发家电线报群11  
id=10000051，【折淘客】京东首发家电线报群12  
id=10000053，【折淘客】瓜瓜采集群【精选】  
id=10000054，【折淘客】京东凑单秒杀线报群12  
id=10000055，【折淘客】京东性价屋  
id=10000056，【折淘客】淘宝性价屋  
id=10000057，【折淘客】Snoopy大白菜②  
id=10000058，【折淘客】憨憨采集群③  
id=10000059，【折淘客】京东捡漏群③  
id=10000060，【折淘客】小可爱捡漏1群  
id=10000061，【折淘客】夏天球鞋搬砖倒卖群  
id=10000062，【折淘客】海绵的优惠活动总群  
id=10000063，【折淘客】神车捡漏  
id=10000064，【折淘客】芬芬采集群  
id=10000065，【折淘客】京东淘宝羊毛线报群  
id=10000066，【折淘客】京豆线报  
id=10000067，【折淘客】京东活动群A③  
id=10000068，【折淘客】高佣拼多多采集群  
id=10000069，【折淘客】撸球鞋私家车群  
id=10000070，【折淘客】托托的吃土兽收容所  
id=10000071，【折淘客】甜的情报局  
id=10000072，【折淘客】优质猫超线报群  
id=10000073，【折淘客】网购捡漏免单采集群  
id=10000074，【折淘客】多啦A梦的人物罐  
id=10000075，【折淘客】淘九宠物采集群  
id=10000076，【折淘客】京东线报天猫VIP总群  
id=10000077，【折淘客】京东淘宝苏宁首发群  
id=10000078，【折淘客】妖娆线报③群  
id=10000079，【折淘客】猫猫的校园超市  
id=10000080，【折淘客】京东酒水爆款采集群  
id=10000081，【折淘客】小花猫京东爆款首发群  
id=10000082，【折淘客】京东线报群  
id=10000083，【折淘客】小鳄鱼俱乐部  
id=10000084，【折淘客】线报情报局  
id=10000085，【折淘客】许院二手书交流群  
id=10000086，【折淘客】三俗的中转站  
id=10000087，【折淘客】可爱猪の扇裁月魄  
id=10000088，【折淘客】拾榴的科技王国Ⅱ  
id=10000089，【折淘客】神忦研究所9  
id=10000090，【折淘客】三山的不吃土方舟  
id=10000091，【折淘客】Alan球鞋线报采集2  
id=10000092，【折淘客】壮壮线报采集群①  
id=10000093，【折淘客】丐帮贵宾团  
id=10000094，【折淘客】LG京东线报捡漏羊毛群  
id=10000095，【折淘客】伐木累免单秒杀2  
id=10000096，【折淘客】xiao线报采集群8  
id=10000097，【折淘客】手机茅台撸货线报①群  
id=10000098，【折淘客】手机茅台线报收货群  
id=10000099，【折淘客】唯品会特价商品群  
id=10000100，【折淘客】24必备文具资料群  
id=10000101，【折淘客】25版图书文具群  
id=10000102，【折淘客】2023茅台手机VIP线报  
id=10000103，【折淘客】捡漏大攻略  
id=10000104，【折淘客】小破站捡漏总部H1  
id=10000105，【折淘客】京淘白菜7群  
id=10000106，【折淘客】小蜜蜂狂欢QA5群  
id=10000107，【折淘客】拼多北大学采集群  
id=10000108，【折淘客】京东线报首发采集⑧群  
id=10000109，【折淘客】one京东vip线报采集群  
id=10000110，【折淘客】文案大师内容导购群⑤  
id=10000111，【折淘客】plus神价情报局-L8  
id=10000112，【折淘客】酷基一手球鞋搬砖0  
id=10000113，【折淘客】千寻京东线报群2  
id=10000115，【折淘客】时光京选购物交流1群  
id=10000116，【折淘客】范范京东采集群③  
id=10000117，【折淘客】舔狗捡垃圾①  
id=10000118，【折淘客】宠优汇同步采集群  
id=10000119，【折淘客】给猫咪蹲A12  
id=10000120，【折淘客】神价情报局  
id=10000121，【折淘客】NINE的寻宝分队  
id=10000122，【折淘客】小丸子优惠线报3群  
id=10000123，【折淘客】猫超和天降首发采集群  
id=10000124，【折淘客】京东猫超小苏群  
id=10000125，【折淘客】红豆种子1群  
id=10000126，【折淘客】小丸子淘宝京东白菜②群  
id=10000127，【折淘客】【宝藏】东东捡漏①  
id=10000128，【折淘客】富贵淘宝京东4群  
id=10000129，【折淘客】安逸优惠群  
id=10000130，【折淘客】羊毛博纯京东宝藏群  
id=10000131，【折淘客】小鹦鹉北京根据地  
id=10000132，【折淘客】纸捡漏市场  
id=10000133，【折淘客】老罗线报采集群  
id=10000134，【折淘客】老街的美妆采集群  
id=10000135，【折淘客】A20神车首发  
id=10000136，【折淘客】发车群(薅羊毛小分队)  
id=10000137，【折淘客】小可爱好物分享  
id=10000138，【折淘客】123好物采集  
id=10000139，【折淘客】神他M的⑨群  
id=10000140，【折淘客】MK-京东-精品采集群  
id=10000142，【折淘客】红豆公告群  
id=10000143，【折淘客】拾榴运动装备线报  
id=10000144，【折淘客】汤圆撸鞋精英一群  
id=10000145，【折淘客】可达星选5群  
id=10000146，【折淘客】26考研正版书籍群6  
id=10000147，【折淘客】静静的自习室  
id=10000148，【折淘客】丐帮6群  
id=10000149，【折淘客】桃子首发线报采集群  
id=10000150，【折淘客】柠檬采集群  
id=10000151，【折淘客】点点羊毛线报捡漏群  
id=10000152，【折淘客】守候优惠分享二群  
id=10000153，【折淘客】Yue撸鞋精英小战队  
id=10000154，【折淘客】轩哥球鞋搬砖线报③群  
id=10000155，【折淘客】帕瓦羊毛线报  
id=10000156，【折淘客】阿荣选品群  
id=99999999，【折淘客】测试群  
  
  
更多群请点击右边【效果展示】查看，更多群请点击右边【效果展示】查看，更多群请点击右边【效果展示】查看 |
|   | interval | int | 否 | interval=1，表示返回最近1分钟内的数据。  
interval=1440，表示返回最近1440分钟内的数据，最大值为1440，也就是最近1天内的数据。 |
|   |  |  |  | 返回指定群最近1-1440分钟内的线报数据。  
可以每隔5秒-10秒循环调用该接口，发现新数据后，保存到本地即可。  
本接口，严禁开多线程进行调用。本接口有5秒缓存，开多线程也没用。  
状态返回301表示，最近1-1440分钟内无新数据！  
 |
|   | msg | int | 否 | msg=1，表示返回详细线报数据内容。 |
|   | q | string | 否 | 比如搜索牛奶，天猫超市等关键词  
urlencode编码传入，urlencode编码传入，urlencode编码传入。 |
|   | plat | int | 否 | 文案所属平台  
1淘宝 2京东 3拼多多 4苏宁 5唯品会 6美团 |
|   | plat2 | int | 否 | 平台细分类型  
1淘宝 2天猫 3天猫超市 |
|   | type2 | int | 否 | 文案类型  
1凑单 2好价 3整点 4秒杀 5券优惠 6半价 |
|   | chunwenzi | string | 否 | 空值返回所有文案，1值返回纯文字文案，0值返回非纯文字文案。 |

**成功返回示例：**

var tmp = { }; document.write("<pre>" + syntaxHighlight(tmp) + "</pre>")
```json
{
    "status": 200,
    "msg": \[
        {
            "add\_time": "2023/7/7 10:22:34",            /\*添加时间\*/
            "type": "0",            /\*群类型\*/
            "id": "10000034",           /\*群号，虚拟\*/
            "content": "夏季冰凉t恤男女款((QsZLdHKHywZ)) CZ0001/",          /\*线报文案\*/
            "code": "2492951",          /\*自动增长列\*/
            "plat": "1",            /\*文案所属平台\*/
            "pic": "https://img.alicdn.com/bao/uploaded/i1/3864135846/O1CN01rS6\_pic.jpg",           /\*商品图片，可能为空\*/
            "num\_id": "oKayb0YurtXdgGjiGBtqUN-pOaVeosRVz0vWXMCjP",           /\*商品id，可能为空\*/
            "plat2": "0",           /\*如果是淘宝平台，此字段可以区分淘宝、天猫和天猫超市，可能为空，1表示淘宝，2表示天猫，3表示天猫超市\*/
            "type2": "1",           /\*文案类型 1凑单 2好价 3整点 4秒杀 5券优惠 6半价\*/
            "cid1": "",           /\*分类1\*/
            "cid1\_name": "",           /分类1名称\*/
            "cid2": "",           /\*分类2\*/
            "cid2\_name": "",           /\*分类2名称\*/
            "cid3": "",           /\*分类3\*/
            "cid3\_name": "",           /\*分类3名称，以上分类6个字段为淘宝或者京东返回，淘宝cid3和cid3\_name为空。\*/
            "chunwenzi": ""           /\*是否纯文字文案（没有口令和推广链接），0表示否，1表示是\*/
        },
        {
            "add\_time": "2023/7/7 10:22:26",
            "type": "0",
            "id": "10000031",
            "content": "乐力高活性益生菌颗粒调理冻干粉16元￥((cbJ9dHKHEiv))/ CZ1234 https://s.click.taobao.com/60fPYDu",
            "code": "2492950",
            "plat": "1",
            "pic": "https://img.alicdn.com/bao/uploaded/i2/2347973243/O1CN01Qu63e31ZpLIVO64TJ\_!!0-item\_pic.jpg",
            "num\_id": "NOVG0RBT5tWzAAPf3ytnUB-BZM7B6fgqwOoxY3i6p",
            "plat2": "0",
            "type2": "1",
            "cid1": "",
            "cid1\_name": "",
            "cid2": "",
            "cid2\_name": "",
            "cid3": "",
            "cid3\_name": "",
            "chunwenzi": ""
        },
        {
            "add\_time": "2023/7/7 10:22:09",
            "type": "0",
            "id": "340319271",
            "content": "299.0元 匹克官方旗舰店匹克态极4.0丨轻量化零感跑步鞋((jJ5LdHKtmjO))CZ0000/",
            "code": "2492949",
            "plat": "1",
            "pic": "https://img.alicdn.com/bao/uploaded/i4/94504595/O1CN01LNiISh1joYmLLVMJ8-94504595.jpg",
            "num\_id": "GKD7ZGdI8tGODDmSrxCgUJ-oVjBwouBBZM8wjeuJK",
            "plat2": "0",
            "type2": "1",
            "cid1": "",
            "cid1\_name": "",
            "cid2": "",
            "cid2\_name": "",
            "cid3": "",
            "cid3\_name": "",
            "chunwenzi": ""
        }
    \]
}
```

  
  
  
  
  
  
  
  
  
  
  
  
  
  
  
  
  
  
  
  
  

[效果展示](https://www.zhetaoke.com/xianbao.aspx?id=&page=1&q=)

[效果展示](https://www.zhetaoke.com/xianbao.aspx?id=&page=1&q=)

[效果展示](https://www.zhetaoke.com/xianbao.aspx?id=&page=1&q=)

---

## 咚咚抢商品API接口

**原始链接**: [咚咚抢商品API接口](https://www.zhetaoke.com/user/extend/extend_lingquan_dongdong.aspx)

**咚咚抢商品API接口---本接口采用12台16核心64G内存服务器，江苏和北京双机房，负载均衡技术，1G独享带宽，放心调用。**

咚咚抢商品API接口：返回咚咚抢商品列表，返回佣金≥15%，动态描述分≥4.6的商品列表。

**接口地址：** https://api.zhetaoke.com:10001/api/api\_dongdong.ashx?appkey=0a565628b2fb4712902712734cec2feb&page=1&page\_size=20&sort=new&t=0-23

咚咚抢商品API接口：返回指定查询条件的商品总数。

**接口地址：** https://api.zhetaoke.com:10001/api/api\_dongdong.ashx?appkey=0a565628b2fb4712902712734cec2feb&page=1&page\_size=20&sort=new&t=0-23&total\_count=1

**返回数据：**json

**请求方式：**get

**请求参数说明：**

|  | 名称 | 类型 | 必填 | 说明 |
| --- | --- | --- | --- | --- |
|   | appkey | string | 是 | 折淘客的对接秘钥appkey |
|   | page | int | 否 | 分页获取数据,第几页 |
|   | page\_size | int | 否 | 每页数据条数（默认每页20条），可自定义1-50之间 |
|   | sort | string | 否 | 商品排序方式，new：按照综合排序，  
total\_sale\_num\_asc：按照总销量从小到大排序，total\_sale\_num\_desc：按照总销量从大到小排序，  
sale\_num\_asc：按照年销量从小到大排序，sale\_num\_desc：按照年销量从大到小排序，  
commission\_rate\_asc：按照佣金比例从小到大排序，commission\_rate\_desc：按照佣金比例从大到小排序，  
price\_asc：按照价格从小到大排序，price\_desc：按照价格从大到小排序，  
coupon\_info\_money\_asc：按照优惠券金额从小到大排序，coupon\_info\_money\_desc：按照优惠券金额从大到小排序，  
shop\_level\_asc：按照店铺等级从低到高排序，shop\_level\_desc：按照店铺等级从高到低排序，  
tkfee\_asc：按照返佣金额从低到高排序，tkfee\_desc：按照返佣金额从高到低排序，  
code：按照code值从大到小排序，date\_time：按照更新时间排序，random：按照随机排序 |
|   | t | string | 否 | 按照时间段获取，值为空：全部咚咚抢商品，8-10：当日8点到10点之间咚咚抢商品，不包含10点及以后时间。  
0点，8点，10点，13点，15点，17点，19点，20点 |
|   | start\_dongdong\_time | string | 否 | 咚咚抢开始时间，格式为：2019-08-22 00:00:00，可用来获取昨日前日等数据，获取今日数据不需要输入。 |
|   | end\_dongdong\_time | string | 否 | 咚咚抢结束时间，格式为：2019-08-22 23:59:59，可用来获取昨日前日等数据，获取今日数据不需要输入。 |

**成功返回示例：**/\*重要事情说三遍！重要事情说三遍！重要事情说三遍！api\_all/api\_dongdong/api\_quantian/api\_shishi/api\_videos/api\_xiaoshi/api\_yongjin接口所有参数都可以自由组合。\*/

var tmp = { "status": 200, /\*状态\*/ "content": \[{ "code": "6646", /\*折淘客编号\*/ "type\_one\_id": "1", /\*分类ID，可参考折淘客分类\*/ "tao\_id": "554832820990", /\*商品ID\*/ "title": "储物箱整理抽屉式收纳柜", /\*商品短标题\*/ "jianjie": "【爆款返场，出口日本】进口PP材质，可叠加透明可视抽屉收纳盒，分格抽屉物品分类齐全，区域分化一目了然，自由组合，大容量，让你居家生活更舒心，收纳你的生活！【赠运费险】", /\*商品简介\*/ "pict\_url": "https://img.alicdn.com/bao/......jpg", /\*商品主图\*/ "user\_type": "0", /\*是否天猫，0是淘宝，1是天猫\*/ "seller\_id": "3073204721", /\*卖家ID\*/ /\*店铺地址可自行拼接，http://store.taobao.com/shop/view\_shop.htm?user\_number\_id=3073204721 \*/ "shop\_dsr": "4.80", /\*商品描述分\*/ "volume": "169", /\*年销量\*/ "size": "26.90", /\*折扣价\*/ "quanhou\_jiage": "16.90", /\*券后价\*/ "date\_time\_yongjin": "2018/11/8 14:45:22", /\*数据更新时间\*/ "tkrate3": "30.00", /\*佣金比率\*/ "yongjin\_type": "MKT", /\*佣金类型\*/ "coupon\_id": "fe31d6c8132c462ba59bd72e03d2eae0", /\*优惠券ID 该商品如无优惠券，此字段为空\*/ "coupon\_start\_time": "2018-11-05 00:00:00", /\*优惠券开始时间 该商品如无优惠券，此字段为空\*/ "coupon\_end\_time": "2018-11-10 23:59:59", /\*优惠券结束时间 该商品如无优惠券，此字段为空\*/ "coupon\_info\_money": "10", /\*优惠券金额 该商品如无优惠券，此字段为0\*/ "coupon\_total\_count": "100000", /\*优惠券总数量 该商品如无优惠券，此字段为0\*/ "coupon\_remain\_count": "99965", /\*优惠券剩余数量 该商品如无优惠券，此字段为0\*/ "coupon\_info": "满80.00元减10元", /\*优惠券信息\*/ "juhuasuan": "0", /\*是否聚划算，1是\*/ "taoqianggou": "0", /\*是否淘抢购，1是\*/ "haitao": "0", /\*是否海淘，1是\*/ "jiyoujia": "0", /\*是否极有家，1是\*/ "jinpaimaijia": "0", /\*是否金牌卖家，1是\*/ "pinpai": "0", /\*是否精选品牌，1是\*/ "pinpai\_name": "", /\*品牌名称\*/ "yunfeixian": "0", /\*是否有运费险，1有\*/ "nick": "\*\*\*旗舰店", /\*卖家昵称\*/ "small\_images": "https://img.alicdn.com/bao/.......jpg|https://img.alicdn.com/bao/.......jpg", /\*商品小图列表\*/ "white\_image": "https://img.alicdn.com/bao/......jpg", /\*商品白底图\*/ "tao\_title": "抽屉式收纳箱塑料储物箱整理箱衣服物家用宿舍衣柜内衣内裤收纳盒", /\*商品长标题\*/ "provcity": "浙江 杭州", /\*宝贝所在地\*/ "shop\_title": "\*\*\*旗舰店", /\*店铺名称\*/ "zhibo\_url": "https://cloud.video.taobao.com/play/u/849090736/p/2/e/6/t/1/219853683676.mp4", /\*视频地址\*/ "sellCount": "860000", /\*淘宝网页实时总销量\*/ "commentCount": "250000", /\*评论数量\*/ "favcount": "180000", /\*收藏数量\*/ "score1": "4.9", /\*宝贝描述分\*/ "score2": "4.9", /\*卖家服务分\*/ "score3": "4.9", /\*物流服务\*/ "creditLevel": "15", /\*店铺等级，一星 二星 三星 四星 五星 一钻 二钻 三钻 四钻 五钻 一皇冠 二皇冠 三皇冠 四皇冠 五皇冠 一金冠 二金冠 三金冠 四金冠 五金冠\*/ "shopIcon": "//img.alicdn.com/imgextra/de/a6/TB10dC8SX.jpg", /\*店铺logo\*/ "pcDescContent": "1.jpg|2.jpg|3.jpg|4.jpg|5.jpg", /\*图文详情图片地址\*/ "item\_url": "https://item.taobao.com/item.htm?id=581801364036", /\*商品url\*/ "category\_id": "50006126", /\*叶子类目id\*/ "category\_name": "收纳盒", /\*叶子类目name\*/ "level\_one\_category\_id": "122928002", /\*一级类目id\*/ "level\_one\_category\_name": "收纳整理", /\*一级类目name\*/ "tkfee3": "5.07", /\*返佣金额\*/ "biaoqian": ""满300元,省30元"", /\*店铺活动\*/ "tag": "赶紧拯救你厨房的油腻吧😂<br>植护🌿加厚厨房抽纸<br>高密度纤维纸张，吸油💧很不错<br>5包装|￥14.8元💰<br>✔厨房小帮手，用处多多", /\*朋友圈文案，需要自己进行urldecode\*/ "date\_time": "2018/11/8 14:45:22" /\*数据添加时间\*/ }\] };
```json
{
	"status": 200,                      /\*状态\*/
	"content": \[{
		"code": "6646",                 /\*折淘客编号\*/
		"type\_one\_id": "1",             /\*分类ID，可参考折淘客分类\*/
		"tao\_id": "554832820990",       /\*商品ID\*/
		"title": "储物箱整理抽屉式收纳柜",                   /\*商品短标题\*/
		"jianjie": "【爆款返场，出口日本】进口PP材质，可叠加透明可视抽屉收纳盒，分格抽屉物品分类齐全，区域分化一目了然，自由组合，大容量，让你居家生活更舒心，收纳你的生活！【赠运费险】", /\*商品简介\*/
		"pict\_url": "https://img.alicdn.com/bao/......jpg",                   /\*商品主图\*/
		"user\_type": "0",                   /\*是否天猫，0是淘宝，1是天猫\*/
		"seller\_id": "3073204721",                   /\*卖家ID\*/ /\*店铺地址可自行拼接，http://store.taobao.com/shop/view\_shop.htm?user\_number\_id=3073204721 \*/
		"shop\_dsr": "4.80",                   /\*商品描述分\*/
		"volume": "169",                   /\*年销量\*/
		"size": "26.90",                   /\*折扣价\*/
		"quanhou\_jiage": "16.90",                   /\*券后价\*/
		"date\_time\_yongjin": "2018/11/8 14:45:22",                   /\*数据更新时间\*/
		"tkrate3": "30.00",                   /\*佣金比率\*/
		"yongjin\_type": "MKT",                   /\*佣金类型\*/
		"coupon\_id": "fe31d6c8132c462ba59bd72e03d2eae0",                   /\*优惠券ID 该商品如无优惠券，此字段为空\*/
		"coupon\_start\_time": "2018-11-05 00:00:00",                   /\*优惠券开始时间 该商品如无优惠券，此字段为空\*/
		"coupon\_end\_time": "2018-11-10 23:59:59",                   /\*优惠券结束时间 该商品如无优惠券，此字段为空\*/
		"coupon\_info\_money": "10",                   /\*优惠券金额 该商品如无优惠券，此字段为0\*/
		"coupon\_total\_count": "100000",                   /\*优惠券总数量 该商品如无优惠券，此字段为0\*/
		"coupon\_remain\_count": "99965",                   /\*优惠券剩余数量 该商品如无优惠券，此字段为0\*/
		"coupon\_info": "满80.00元减10元",                   /\*优惠券信息\*/
		"juhuasuan": "0",                       /\*是否聚划算，1是\*/
		"taoqianggou": "0",                     /\*是否淘抢购，1是\*/
		"haitao": "0",                          /\*是否海淘，1是\*/
		"jiyoujia": "0",                        /\*是否极有家，1是\*/
		"jinpaimaijia": "0",                    /\*是否金牌卖家，1是\*/
		"pinpai": "0",                          /\*是否精选品牌，1是\*/
		"pinpai\_name": "",                      /\*品牌名称\*/
		"yunfeixian": "0",                      /\*是否有运费险，1有\*/
		"nick": "\*\*\*旗舰店",                    /\*卖家昵称\*/
		"small\_images": "https://img.alicdn.com/bao/.......jpg|https://img.alicdn.com/bao/.......jpg",                   /\*商品小图列表\*/
		"white\_image": "https://img.alicdn.com/bao/......jpg",                   /\*商品白底图\*/
		"tao\_title": "抽屉式收纳箱塑料储物箱整理箱衣服物家用宿舍衣柜内衣内裤收纳盒",                   /\*商品长标题\*/
		"provcity": "浙江 杭州",                   /\*宝贝所在地\*/
		"shop\_title": "\*\*\*旗舰店",                   /\*店铺名称\*/
		"zhibo\_url": "https://cloud.video.taobao.com/play/u/849090736/p/2/e/6/t/1/219853683676.mp4",                   /\*视频地址\*/
		"sellCount": "860000",                   /\*淘宝网页实时总销量\*/
		"commentCount": "250000",                   /\*评论数量\*/
		"favcount": "180000",                   /\*收藏数量\*/
		"score1": "4.9",                   /\*宝贝描述分\*/
		"score2": "4.9",                   /\*卖家服务分\*/
		"score3": "4.9",                   /\*物流服务分\*/
		"creditLevel": "15",                   /\*店铺等级（1-20），一星 二星 三星 四星 五星 一钻 二钻 三钻 四钻 五钻 一皇冠 二皇冠 三皇冠 四皇冠 五皇冠  一金冠 二金冠 三金冠 四金冠 五金冠\*/
		"shopIcon": "//img.alicdn.com/imgextra/de/a6/TB10dC8SX.jpg",       /\*店铺logo\*/
		"pcDescContent": "1.jpg|2.jpg|3.jpg|4.jpg|5.jpg",  /\*图文详情图片地址\*/
		"item\_url": "https://item.taobao.com/item.htm?id=581801364036",  /\*商品url\*/
		"category\_id": "50006126",                   /\*叶子类目id\*/
		"category\_name": "收纳盒",                   /\*叶子类目name\*/
		"level\_one\_category\_id": "122928002",                   /\*一级类目id\*/
		"level\_one\_category\_name": "收纳整理",                   /\*一级类目name\*/
		"tkfee3": "5.07",                   /\*返佣金额\*/
		"biaoqian": ""满300元,省30元"",                   /\*店铺活动\*/
		"tag": "赶紧拯救你厨房的油腻吧😂植护🌿加厚厨房抽纸高密度纤维纸张，吸油💧很不错5包装|￥14.8元💰✔厨房小帮手，用处多多",                   /\*朋友圈文案，需要自己进行urldecode\*/
		"date\_time": "2018/11/8 14:45:22"                   /\*数据添加时间\*/
	}\]
}
```

**失败返回示例：**

var tmp = { "status": 301 };
```json
{
	"status": 301                      /\*状态\*/
}
```

[效果展示](http://www.zhetaoke.com/list/dongdong.html)

---

## 朋友圈火爆商品API

**原始链接**: [朋友圈火爆商品API](https://www.zhetaoke.com/user/extend/extend_lingquan_pengyouquan.aspx)

**朋友圈火爆商品API---本接口采用12台16核心64G内存服务器，江苏和北京双机房，负载均衡技术，1G独享带宽，放心调用。**

朋友圈火爆商品API：返回朋友圈火爆商品列表，返回佣金≥15%，动态描述分≥4.6的商品列表。

**接口地址：** https://api.zhetaoke.com:10001/api/api\_all.ashx?appkey=0a565628b2fb4712902712734cec2feb&page=1&page\_size=20&sort=new&tag=1

朋友圈火爆商品API：返回指定查询条件的商品总数。

**接口地址：** https://api.zhetaoke.com:10001/api/api\_all.ashx?appkey=0a565628b2fb4712902712734cec2feb&page=1&page\_size=20&sort=new&tag=1&total\_count=1

**返回数据：**json

**请求方式：**get

**请求参数说明：**

|  | 名称 | 类型 | 必填 | 说明 |
| --- | --- | --- | --- | --- |
|   | appkey | string | 是 | 折淘客的对接秘钥appkey |
|   | page | int | 否 | 分页获取数据,第几页 |
|   | page\_size | int | 否 | 每页数据条数（默认每页20条），可自定义1-50之间 |
|   | sort | string | 否 | 商品排序方式，new：按照综合排序，  
total\_sale\_num\_asc：按照总销量从小到大排序，total\_sale\_num\_desc：按照总销量从大到小排序，  
sale\_num\_asc：按照年销量从小到大排序，sale\_num\_desc：按照年销量从大到小排序，  
commission\_rate\_asc：按照佣金比例从小到大排序，commission\_rate\_desc：按照佣金比例从大到小排序，  
price\_asc：按照价格从小到大排序，price\_desc：按照价格从大到小排序，  
coupon\_info\_money\_asc：按照优惠券金额从小到大排序，coupon\_info\_money\_desc：按照优惠券金额从大到小排序，  
shop\_level\_asc：按照店铺等级从低到高排序，shop\_level\_desc：按照店铺等级从高到低排序，  
tkfee\_asc：按照返佣金额从低到高排序，tkfee\_desc：按照返佣金额从高到低排序，  
code：按照code值从大到小排序，date\_time：按照更新时间排序，random：按照随机排序 |
|   | cid | int | 否 | 一级商品分类，值为空：全部商品，1：女装，2：母婴，3：美妆，4：居家日用，5：鞋品，6：美食，7：文娱车品，8：数码家电，9：男装，10：内衣，11：箱包，12：配饰，13：户外运动，14：家装家纺 |
|   | tag | string | 否 | 是否朋友圈火爆商品，值为空：全部商品，1：朋友圈火爆商品 |

**成功返回示例：**/\*重要事情说三遍！重要事情说三遍！重要事情说三遍！api\_all/api\_dongdong/api\_quantian/api\_shishi/api\_videos/api\_xiaoshi/api\_yongjin接口所有参数都可以自由组合。\*/

var tmp = { "status": 200, /\*状态\*/ "content": \[{ "code": "6646", /\*折淘客编号\*/ "type\_one\_id": "1", /\*分类ID，可参考折淘客分类\*/ "tao\_id": "554832820990", /\*商品ID\*/ "title": "储物箱整理抽屉式收纳柜", /\*商品短标题\*/ "jianjie": "【爆款返场，出口日本】进口PP材质，可叠加透明可视抽屉收纳盒，分格抽屉物品分类齐全，区域分化一目了然，自由组合，大容量，让你居家生活更舒心，收纳你的生活！【赠运费险】", /\*商品简介\*/ "pict\_url": "https://img.alicdn.com/bao/......jpg", /\*商品主图\*/ "user\_type": "0", /\*是否天猫，0是淘宝，1是天猫\*/ "seller\_id": "3073204721", /\*卖家ID\*/ /\*店铺地址可自行拼接，http://store.taobao.com/shop/view\_shop.htm?user\_number\_id=3073204721 \*/ "shop\_dsr": "4.80", /\*商品描述分\*/ "volume": "169", /\*年销量\*/ "size": "26.90", /\*折扣价\*/ "quanhou\_jiage": "16.90", /\*券后价\*/ "date\_time\_yongjin": "2018/11/8 14:45:22", /\*数据更新时间\*/ "tkrate3": "30.00", /\*佣金比率\*/ "yongjin\_type": "MKT", /\*佣金类型\*/ "coupon\_id": "fe31d6c8132c462ba59bd72e03d2eae0", /\*优惠券ID 该商品如无优惠券，此字段为空\*/ "coupon\_start\_time": "2018-11-05 00:00:00", /\*优惠券开始时间 该商品如无优惠券，此字段为空\*/ "coupon\_end\_time": "2018-11-10 23:59:59", /\*优惠券结束时间 该商品如无优惠券，此字段为空\*/ "coupon\_info\_money": "10", /\*优惠券金额 该商品如无优惠券，此字段为0\*/ "coupon\_total\_count": "100000", /\*优惠券总数量 该商品如无优惠券，此字段为0\*/ "coupon\_remain\_count": "99965", /\*优惠券剩余数量 该商品如无优惠券，此字段为0\*/ "coupon\_info": "满80.00元减10元", /\*优惠券信息\*/ "juhuasuan": "0", /\*是否聚划算，1是\*/ "taoqianggou": "0", /\*是否淘抢购，1是\*/ "haitao": "0", /\*是否海淘，1是\*/ "jiyoujia": "0", /\*是否极有家，1是\*/ "jinpaimaijia": "0", /\*是否金牌卖家，1是\*/ "pinpai": "0", /\*是否精选品牌，1是\*/ "pinpai\_name": "", /\*品牌名称\*/ "yunfeixian": "0", /\*是否有运费险，1有\*/ "nick": "\*\*\*旗舰店", /\*卖家昵称\*/ "small\_images": "https://img.alicdn.com/bao/.......jpg|https://img.alicdn.com/bao/.......jpg", /\*商品小图列表\*/ "white\_image": "https://img.alicdn.com/bao/......jpg", /\*商品白底图\*/ "tao\_title": "抽屉式收纳箱塑料储物箱整理箱衣服物家用宿舍衣柜内衣内裤收纳盒", /\*商品长标题\*/ "provcity": "浙江 杭州", /\*宝贝所在地\*/ "shop\_title": "\*\*\*旗舰店", /\*店铺名称\*/ "zhibo\_url": "https://cloud.video.taobao.com/play/u/849090736/p/2/e/6/t/1/219853683676.mp4", /\*视频地址\*/ "sellCount": "860000", /\*淘宝网页实时总销量\*/ "commentCount": "250000", /\*评论数量\*/ "favcount": "180000", /\*收藏数量\*/ "score1": "4.9", /\*宝贝描述分\*/ "score2": "4.9", /\*卖家服务分\*/ "score3": "4.9", /\*物流服务\*/ "creditLevel": "15", /\*店铺等级，一星 二星 三星 四星 五星 一钻 二钻 三钻 四钻 五钻 一皇冠 二皇冠 三皇冠 四皇冠 五皇冠 一金冠 二金冠 三金冠 四金冠 五金冠\*/ "shopIcon": "//img.alicdn.com/imgextra/de/a6/TB10dC8SX.jpg", /\*店铺logo\*/ "pcDescContent": "1.jpg|2.jpg|3.jpg|4.jpg|5.jpg", /\*图文详情图片地址\*/ "item\_url": "https://item.taobao.com/item.htm?id=581801364036", /\*商品url\*/ "category\_id": "50006126", /\*叶子类目id\*/ "category\_name": "收纳盒", /\*叶子类目name\*/ "level\_one\_category\_id": "122928002", /\*一级类目id\*/ "level\_one\_category\_name": "收纳整理", /\*一级类目name\*/ "tkfee3": "5.07", /\*返佣金额\*/ "biaoqian": ""满300元,省30元"", /\*店铺活动\*/ "tag": "赶紧拯救你厨房的油腻吧😂<br>植护🌿加厚厨房抽纸<br>高密度纤维纸张，吸油💧很不错<br>5包装|￥14.8元💰<br>✔厨房小帮手，用处多多", /\*朋友圈文案，需要自己进行urldecode\*/ "date\_time": "2018/11/8 14:45:22" /\*数据添加时间\*/ }\] };
```json
{
	"status": 200,                      /\*状态\*/
	"content": \[{
		"code": "6646",                 /\*折淘客编号\*/
		"type\_one\_id": "1",             /\*分类ID，可参考折淘客分类\*/
		"tao\_id": "554832820990",       /\*商品ID\*/
		"title": "储物箱整理抽屉式收纳柜",                   /\*商品短标题\*/
		"jianjie": "【爆款返场，出口日本】进口PP材质，可叠加透明可视抽屉收纳盒，分格抽屉物品分类齐全，区域分化一目了然，自由组合，大容量，让你居家生活更舒心，收纳你的生活！【赠运费险】", /\*商品简介\*/
		"pict\_url": "https://img.alicdn.com/bao/......jpg",                   /\*商品主图\*/
		"user\_type": "0",                   /\*是否天猫，0是淘宝，1是天猫\*/
		"seller\_id": "3073204721",                   /\*卖家ID\*/ /\*店铺地址可自行拼接，http://store.taobao.com/shop/view\_shop.htm?user\_number\_id=3073204721 \*/
		"shop\_dsr": "4.80",                   /\*商品描述分\*/
		"volume": "169",                   /\*年销量\*/
		"size": "26.90",                   /\*折扣价\*/
		"quanhou\_jiage": "16.90",                   /\*券后价\*/
		"date\_time\_yongjin": "2018/11/8 14:45:22",                   /\*数据更新时间\*/
		"tkrate3": "30.00",                   /\*佣金比率\*/
		"yongjin\_type": "MKT",                   /\*佣金类型\*/
		"coupon\_id": "fe31d6c8132c462ba59bd72e03d2eae0",                   /\*优惠券ID 该商品如无优惠券，此字段为空\*/
		"coupon\_start\_time": "2018-11-05 00:00:00",                   /\*优惠券开始时间 该商品如无优惠券，此字段为空\*/
		"coupon\_end\_time": "2018-11-10 23:59:59",                   /\*优惠券结束时间 该商品如无优惠券，此字段为空\*/
		"coupon\_info\_money": "10",                   /\*优惠券金额 该商品如无优惠券，此字段为0\*/
		"coupon\_total\_count": "100000",                   /\*优惠券总数量 该商品如无优惠券，此字段为0\*/
		"coupon\_remain\_count": "99965",                   /\*优惠券剩余数量 该商品如无优惠券，此字段为0\*/
		"coupon\_info": "满80.00元减10元",                   /\*优惠券信息\*/
		"juhuasuan": "0",                       /\*是否聚划算，1是\*/
		"taoqianggou": "0",                     /\*是否淘抢购，1是\*/
		"haitao": "0",                          /\*是否海淘，1是\*/
		"jiyoujia": "0",                        /\*是否极有家，1是\*/
		"jinpaimaijia": "0",                    /\*是否金牌卖家，1是\*/
		"pinpai": "0",                          /\*是否精选品牌，1是\*/
		"pinpai\_name": "",                      /\*品牌名称\*/
		"yunfeixian": "0",                      /\*是否有运费险，1有\*/
		"nick": "\*\*\*旗舰店",                    /\*卖家昵称\*/
		"small\_images": "https://img.alicdn.com/bao/.......jpg|https://img.alicdn.com/bao/.......jpg",                   /\*商品小图列表\*/
		"white\_image": "https://img.alicdn.com/bao/......jpg",                   /\*商品白底图\*/
		"tao\_title": "抽屉式收纳箱塑料储物箱整理箱衣服物家用宿舍衣柜内衣内裤收纳盒",                   /\*商品长标题\*/
		"provcity": "浙江 杭州",                   /\*宝贝所在地\*/
		"shop\_title": "\*\*\*旗舰店",                   /\*店铺名称\*/
		"zhibo\_url": "https://cloud.video.taobao.com/play/u/849090736/p/2/e/6/t/1/219853683676.mp4",                   /\*视频地址\*/
		"sellCount": "860000",                   /\*淘宝网页实时总销量\*/
		"commentCount": "250000",                   /\*评论数量\*/
		"favcount": "180000",                   /\*收藏数量\*/
		"score1": "4.9",                   /\*宝贝描述分\*/
		"score2": "4.9",                   /\*卖家服务分\*/
		"score3": "4.9",                   /\*物流服务分\*/
		"creditLevel": "15",                   /\*店铺等级（1-20），一星 二星 三星 四星 五星 一钻 二钻 三钻 四钻 五钻 一皇冠 二皇冠 三皇冠 四皇冠 五皇冠  一金冠 二金冠 三金冠 四金冠 五金冠\*/
		"shopIcon": "//img.alicdn.com/imgextra/de/a6/TB10dC8SX.jpg",       /\*店铺logo\*/
		"pcDescContent": "1.jpg|2.jpg|3.jpg|4.jpg|5.jpg",  /\*图文详情图片地址\*/
		"item\_url": "https://item.taobao.com/item.htm?id=581801364036",  /\*商品url\*/
		"category\_id": "50006126",                   /\*叶子类目id\*/
		"category\_name": "收纳盒",                   /\*叶子类目name\*/
		"level\_one\_category\_id": "122928002",                   /\*一级类目id\*/
		"level\_one\_category\_name": "收纳整理",                   /\*一级类目name\*/
		"tkfee3": "5.07",                   /\*返佣金额\*/
		"biaoqian": ""满300元,省30元"",                   /\*店铺活动\*/
		"tag": "赶紧拯救你厨房的油腻吧😂植护🌿加厚厨房抽纸高密度纤维纸张，吸油💧很不错5包装|￥14.8元💰✔厨房小帮手，用处多多",                   /\*朋友圈文案，需要自己进行urldecode\*/
		"date\_time": "2018/11/8 14:45:22"                   /\*数据添加时间\*/
	}\]
}
```

**失败返回示例：**

var tmp = { "status": 301 };
```json
{
	"status": 301                      /\*状态\*/
}
```

[效果展示](http://cms.zhetaoke.com:10000/m/item/list.aspx?tianmao=1)

---

## 批量全网商品详情API接口

**原始链接**: [批量全网商品详情API接口](https://www.zhetaoke.com/user/extend/extend_lingquan_detail_piliang.aspx)

**批量全网商品详情API接口---本接口采用12台16核心64G内存服务器，江苏和北京双机房，负载均衡技术，1G独享带宽，放心调用。**

重要通知！重要通知！重要通知！此接口新增淘客sid和pid两个参数，这两个字段为必填字段！！！  
重要通知！重要通知！重要通知！此接口新增淘客sid和pid两个参数，这两个字段为必填字段！！！  
重要通知！重要通知！重要通知！此接口新增淘客sid和pid两个参数，这两个字段为必填字段！！！  
重要通知！重要通知！重要通知！此接口新增淘客sid和pid两个参数，这两个字段为必填字段！！！  
2024年3月1日上午9点，sid和pid参数必须填写，不填将无法获取数据，该时间也可能根据官方要求提前，请大家尽快修改。  

批量全网商品详情API接口：可同时获取20个商品的全网商品详情信息，包含优惠券信息，可用来批量获取商品详情信息等超级功能。

**接口地址：** https://api.zhetaoke.com:10002/api/api\_detail\_piliang.ashx?appkey=0a565628b2fb4712902712734cec2feb&sid=&pid=&num\_iids=商品ID1,商品ID2,商品ID3

**备用地址：**http://api.zhetaoke.cn:10000/api/api\_detail\_piliang.ashx

**特殊说明：**[主接口和备用接口详细说明（请大家优先使用com主域名接口地址，在com出现问题的时候，人工或者自动切换cn备用接口地址）](https://www.zhetaoke.com/help_detail_3_27.html)

**返回数据：**json

**请求方式：**get

|  | 名称 | 类型 | 必填 | 说明 |
| --- | --- | --- | --- | --- |
|   | appkey | string | 是 | 折淘客的对接秘钥appkey |
|   | sid | string | 是 | 对应的淘客账号授权ID[点击查看](https://www.zhetaoke.com/user/shouquan.html) |
|   | pid | string | 是 | 淘客PID，mm\_xxx\_xxx\_xxx,三段格式，必须与授权的账户相同，否则出错 |
|   | num\_iids | string | 是 | 多个商品ID串，用英文逗号,分割，最大20个。  
该接口可以用来批量获取商品详情信息，包含优惠券信息、佣金信息等。  
该接口支持全网所有淘宝客商品。  
 |

**成功返回示例：**/\*重要事情说三遍！重要事情说三遍！重要事情说三遍！api\_all/api\_dongdong/api\_quantian/api\_shishi/api\_videos/api\_xiaoshi/api\_yongjin接口所有参数都可以自由组合。\*/

var tmp = { "status": 200, /\*状态\*/ "content": \[{ "code": "6646", /\*折淘客编号\*/ "type\_one\_id": "1", /\*分类ID，可参考折淘客分类\*/ "tao\_id": "554832820990", /\*商品ID\*/ "title": "储物箱整理抽屉式收纳柜", /\*商品短标题\*/ "jianjie": "【爆款返场，出口日本】进口PP材质，可叠加透明可视抽屉收纳盒，分格抽屉物品分类齐全，区域分化一目了然，自由组合，大容量，让你居家生活更舒心，收纳你的生活！【赠运费险】", /\*商品简介\*/ "pict\_url": "https://img.alicdn.com/bao/......jpg", /\*商品主图\*/ "user\_type": "0", /\*是否天猫，0是淘宝，1是天猫\*/ "seller\_id": "3073204721", /\*卖家ID\*/ /\*店铺地址可自行拼接，http://store.taobao.com/shop/view\_shop.htm?user\_number\_id=3073204721 \*/ "shop\_dsr": "4.80", /\*商品描述分\*/ "volume": "169", /\*年销量\*/ "size": "26.90", /\*折扣价\*/ "quanhou\_jiage": "16.90", /\*券后价\*/ "date\_time\_yongjin": "2018/11/8 14:45:22", /\*数据更新时间\*/ "tkrate3": "30.00", /\*佣金比率\*/ "yongjin\_type": "MKT", /\*佣金类型\*/ "coupon\_id": "fe31d6c8132c462ba59bd72e03d2eae0", /\*优惠券ID 该商品如无优惠券，此字段为空\*/ "coupon\_start\_time": "2018-11-05 00:00:00", /\*优惠券开始时间 该商品如无优惠券，此字段为空\*/ "coupon\_end\_time": "2018-11-10 23:59:59", /\*优惠券结束时间 该商品如无优惠券，此字段为空\*/ "coupon\_info\_money": "10", /\*优惠券金额 该商品如无优惠券，此字段为0\*/ "coupon\_total\_count": "100000", /\*优惠券总数量 该商品如无优惠券，此字段为0\*/ "coupon\_remain\_count": "99965", /\*优惠券剩余数量 该商品如无优惠券，此字段为0\*/ "coupon\_info": "满80.00元减10元", /\*优惠券信息\*/ "juhuasuan": "0", /\*是否聚划算，1是\*/ "taoqianggou": "0", /\*是否淘抢购，1是\*/ "haitao": "0", /\*是否海淘，1是\*/ "jiyoujia": "0", /\*是否极有家，1是\*/ "jinpaimaijia": "0", /\*是否金牌卖家，1是\*/ "pinpai": "0", /\*是否精选品牌，1是\*/ "pinpai\_name": "", /\*品牌名称\*/ "yunfeixian": "0", /\*是否有运费险，1有\*/ "nick": "\*\*\*旗舰店", /\*卖家昵称\*/ "small\_images": "https://img.alicdn.com/bao/.......jpg|https://img.alicdn.com/bao/.......jpg", /\*商品小图列表\*/ "white\_image": "https://img.alicdn.com/bao/......jpg", /\*商品白底图\*/ "tao\_title": "抽屉式收纳箱塑料储物箱整理箱衣服物家用宿舍衣柜内衣内裤收纳盒", /\*商品长标题\*/ "provcity": "浙江 杭州", /\*宝贝所在地\*/ "shop\_title": "\*\*\*旗舰店", /\*店铺名称\*/ "zhibo\_url": "https://cloud.video.taobao.com/play/u/849090736/p/2/e/6/t/1/219853683676.mp4", /\*视频地址\*/ "sellCount": "860000", /\*淘宝网页实时总销量\*/ "commentCount": "250000", /\*评论数量\*/ "favcount": "180000", /\*收藏数量\*/ "score1": "4.9", /\*宝贝描述分\*/ "score2": "4.9", /\*卖家服务分\*/ "score3": "4.9", /\*物流服务\*/ "creditLevel": "15", /\*店铺等级，一星 二星 三星 四星 五星 一钻 二钻 三钻 四钻 五钻 一皇冠 二皇冠 三皇冠 四皇冠 五皇冠 一金冠 二金冠 三金冠 四金冠 五金冠\*/ "shopIcon": "//img.alicdn.com/imgextra/de/a6/TB10dC8SX.jpg", /\*店铺logo\*/ "pcDescContent": "1.jpg|2.jpg|3.jpg|4.jpg|5.jpg", /\*图文详情图片地址\*/ "item\_url": "https://item.taobao.com/item.htm?id=581801364036", /\*商品url\*/ "category\_id": "50006126", /\*叶子类目id\*/ "category\_name": "收纳盒", /\*叶子类目name\*/ "level\_one\_category\_id": "122928002", /\*一级类目id\*/ "level\_one\_category\_name": "收纳整理", /\*一级类目name\*/ "tkfee3": "5.07", /\*返佣金额\*/ "biaoqian": ""满300元,省30元"", /\*店铺活动\*/ "tag": "赶紧拯救你厨房的油腻吧😂<br>植护🌿加厚厨房抽纸<br>高密度纤维纸张，吸油💧很不错<br>5包装|￥14.8元💰<br>✔厨房小帮手，用处多多", /\*朋友圈文案，需要自己进行urldecode\*/ "date\_time": "2018/11/8 14:45:22" /\*数据添加时间\*/ }\] };
```json
{
	"status": 200,                      /\*状态\*/
	"content": \[{
		"code": "6646",                 /\*折淘客编号\*/
		"type\_one\_id": "1",             /\*分类ID，可参考折淘客分类\*/
		"tao\_id": "554832820990",       /\*商品ID\*/
		"title": "储物箱整理抽屉式收纳柜",                   /\*商品短标题\*/
		"jianjie": "【爆款返场，出口日本】进口PP材质，可叠加透明可视抽屉收纳盒，分格抽屉物品分类齐全，区域分化一目了然，自由组合，大容量，让你居家生活更舒心，收纳你的生活！【赠运费险】", /\*商品简介\*/
		"pict\_url": "https://img.alicdn.com/bao/......jpg",                   /\*商品主图\*/
		"user\_type": "0",                   /\*是否天猫，0是淘宝，1是天猫\*/
		"seller\_id": "3073204721",                   /\*卖家ID\*/ /\*店铺地址可自行拼接，http://store.taobao.com/shop/view\_shop.htm?user\_number\_id=3073204721 \*/
		"shop\_dsr": "4.80",                   /\*商品描述分\*/
		"volume": "169",                   /\*年销量\*/
		"size": "26.90",                   /\*折扣价\*/
		"quanhou\_jiage": "16.90",                   /\*券后价\*/
		"date\_time\_yongjin": "2018/11/8 14:45:22",                   /\*数据更新时间\*/
		"tkrate3": "30.00",                   /\*佣金比率\*/
		"yongjin\_type": "MKT",                   /\*佣金类型\*/
		"coupon\_id": "fe31d6c8132c462ba59bd72e03d2eae0",                   /\*优惠券ID 该商品如无优惠券，此字段为空\*/
		"coupon\_start\_time": "2018-11-05 00:00:00",                   /\*优惠券开始时间 该商品如无优惠券，此字段为空\*/
		"coupon\_end\_time": "2018-11-10 23:59:59",                   /\*优惠券结束时间 该商品如无优惠券，此字段为空\*/
		"coupon\_info\_money": "10",                   /\*优惠券金额 该商品如无优惠券，此字段为0\*/
		"coupon\_total\_count": "100000",                   /\*优惠券总数量 该商品如无优惠券，此字段为0\*/
		"coupon\_remain\_count": "99965",                   /\*优惠券剩余数量 该商品如无优惠券，此字段为0\*/
		"coupon\_info": "满80.00元减10元",                   /\*优惠券信息\*/
		"juhuasuan": "0",                       /\*是否聚划算，1是\*/
		"taoqianggou": "0",                     /\*是否淘抢购，1是\*/
		"haitao": "0",                          /\*是否海淘，1是\*/
		"jiyoujia": "0",                        /\*是否极有家，1是\*/
		"jinpaimaijia": "0",                    /\*是否金牌卖家，1是\*/
		"pinpai": "0",                          /\*是否精选品牌，1是\*/
		"pinpai\_name": "",                      /\*品牌名称\*/
		"yunfeixian": "0",                      /\*是否有运费险，1有\*/
		"nick": "\*\*\*旗舰店",                    /\*卖家昵称\*/
		"small\_images": "https://img.alicdn.com/bao/.......jpg|https://img.alicdn.com/bao/.......jpg",                   /\*商品小图列表\*/
		"white\_image": "https://img.alicdn.com/bao/......jpg",                   /\*商品白底图\*/
		"tao\_title": "抽屉式收纳箱塑料储物箱整理箱衣服物家用宿舍衣柜内衣内裤收纳盒",                   /\*商品长标题\*/
		"provcity": "浙江 杭州",                   /\*宝贝所在地\*/
		"shop\_title": "\*\*\*旗舰店",                   /\*店铺名称\*/
		"zhibo\_url": "https://cloud.video.taobao.com/play/u/849090736/p/2/e/6/t/1/219853683676.mp4",                   /\*视频地址\*/
		"sellCount": "860000",                   /\*淘宝网页实时总销量\*/
		"commentCount": "250000",                   /\*评论数量\*/
		"favcount": "180000",                   /\*收藏数量\*/
		"score1": "4.9",                   /\*宝贝描述分\*/
		"score2": "4.9",                   /\*卖家服务分\*/
		"score3": "4.9",                   /\*物流服务分\*/
		"creditLevel": "15",                   /\*店铺等级（1-20），一星 二星 三星 四星 五星 一钻 二钻 三钻 四钻 五钻 一皇冠 二皇冠 三皇冠 四皇冠 五皇冠  一金冠 二金冠 三金冠 四金冠 五金冠\*/
		"shopIcon": "//img.alicdn.com/imgextra/de/a6/TB10dC8SX.jpg",       /\*店铺logo\*/
		"pcDescContent": "1.jpg|2.jpg|3.jpg|4.jpg|5.jpg",  /\*图文详情图片地址\*/
		"item\_url": "https://item.taobao.com/item.htm?id=581801364036",  /\*商品url\*/
		"category\_id": "50006126",                   /\*叶子类目id\*/
		"category\_name": "收纳盒",                   /\*叶子类目name\*/
		"level\_one\_category\_id": "122928002",                   /\*一级类目id\*/
		"level\_one\_category\_name": "收纳整理",                   /\*一级类目name\*/
		"tkfee3": "5.07",                   /\*返佣金额\*/
		"biaoqian": ""满300元,省30元"",                   /\*店铺活动\*/
		"tag": "赶紧拯救你厨房的油腻吧😂植护🌿加厚厨房抽纸高密度纤维纸张，吸油💧很不错5包装|￥14.8元💰✔厨房小帮手，用处多多",                   /\*朋友圈文案，需要自己进行urldecode\*/
		"date\_time": "2018/11/8 14:45:22"                   /\*数据添加时间\*/
	}\]
}
```

**失败返回示例：**

var tmp = { "status": 301 };
```json
{
	"status": 301                      /\*状态\*/
}
```

---

## 全网商品详情API接口

**原始链接**: [全网商品详情API接口](https://www.zhetaoke.com/user/extend/extend_lingquan_detail.aspx)

**全网商品详情API接口---本接口采用12台16核心64G内存服务器，江苏和北京双机房，负载均衡技术，1G独享带宽，放心调用。**

重要通知！重要通知！重要通知！此接口新增淘客sid和pid两个参数，这两个字段为必填字段！！！  
重要通知！重要通知！重要通知！此接口新增淘客sid和pid两个参数，这两个字段为必填字段！！！  
重要通知！重要通知！重要通知！此接口新增淘客sid和pid两个参数，这两个字段为必填字段！！！  
重要通知！重要通知！重要通知！此接口新增淘客sid和pid两个参数，这两个字段为必填字段！！！  
2024年3月1日上午9点，sid和pid参数必须填写，不填将无法获取数据，该时间也可能根据官方要求提前，请大家尽快修改。  

全网商品详情API接口：返回全网商品详情信息，包含优惠券信息，可用来开发全网搜索等超级功能。

**接口地址：** https://api.zhetaoke.com:10002/api/api\_detail.ashx?appkey=0a565628b2fb4712902712734cec2feb&sid=&pid=&tao\_id=商品ID

**备用地址：**http://api.zhetaoke.cn:10000/api/api\_detail.ashx

**特殊说明：**[主接口和备用接口详细说明（请大家优先使用com主域名接口地址，在com出现问题的时候，人工或者自动切换cn备用接口地址）](https://www.zhetaoke.com/help_detail_3_27.html)

**返回数据：**json

**请求方式：**get

**请求参数说明：（单个商品时，该接口可能返回两条记录，一条是S券商品信息，一条是G券商品信息，code>0表示S券商品，code=0表示G券商品）**  
  

|  | 名称 | 类型 | 必填 | 说明 |
| --- | --- | --- | --- | --- |
|   | appkey | string | 是 | 折淘客的对接秘钥appkey |
|   | sid | string | 是 | 对应的淘客账号授权ID[点击查看](https://www.zhetaoke.com/user/shouquan.html) |
|   | pid | string | 是 | 淘客PID，mm\_xxx\_xxx\_xxx,三段格式，必须与授权的账户相同，否则出错 |
|   | tao\_id | string | 是 | 单个商品ID  
该参数支持全网所有淘宝客商品。 |
|   | code | string | 否 | 折淘客编号，输入非折淘客商品编号，此参数无作用。code值可通过其它领券API接口结果获得。code值必须与tao\_id值对应。 |
|   | num\_iids | string | 否 | 多个商品ID串，用,分割，最大40个。  
如果该值非空，则只返回站内商品数据，否则返回全网商品。  
该参数的主要应用环境，可以循环更新站内商品数据。  
该参数非空时，目前限制一秒内访问一次。该参数为空时，无任何限制。  
该参数只支持站内商品。 |
|   | type | string | 否 | 是否返回S券和G券全部数据，type=0表示返回全部两条数据，type=1表示返回S券单条数据（如无S券数据返回G券单条数据），默认type=0。 |

**成功返回示例：**/\*重要事情说三遍！重要事情说三遍！重要事情说三遍！api\_all/api\_dongdong/api\_quantian/api\_shishi/api\_videos/api\_xiaoshi/api\_yongjin接口所有参数都可以自由组合。\*/

var tmp = { "status": 200, /\*状态\*/ "content": \[{ "code": "6646", /\*折淘客编号\*/ "type\_one\_id": "1", /\*分类ID，可参考折淘客分类\*/ "tao\_id": "554832820990", /\*商品ID\*/ "title": "储物箱整理抽屉式收纳柜", /\*商品短标题\*/ "jianjie": "【爆款返场，出口日本】进口PP材质，可叠加透明可视抽屉收纳盒，分格抽屉物品分类齐全，区域分化一目了然，自由组合，大容量，让你居家生活更舒心，收纳你的生活！【赠运费险】", /\*商品简介\*/ "pict\_url": "https://img.alicdn.com/bao/......jpg", /\*商品主图\*/ "user\_type": "0", /\*是否天猫，0是淘宝，1是天猫\*/ "seller\_id": "3073204721", /\*卖家ID\*/ /\*店铺地址可自行拼接，http://store.taobao.com/shop/view\_shop.htm?user\_number\_id=3073204721 \*/ "shop\_dsr": "4.80", /\*商品描述分\*/ "volume": "169", /\*年销量\*/ "size": "26.90", /\*折扣价\*/ "quanhou\_jiage": "16.90", /\*券后价\*/ "date\_time\_yongjin": "2018/11/8 14:45:22", /\*数据更新时间\*/ "tkrate3": "30.00", /\*佣金比率\*/ "yongjin\_type": "MKT", /\*佣金类型\*/ "coupon\_id": "fe31d6c8132c462ba59bd72e03d2eae0", /\*优惠券ID 该商品如无优惠券，此字段为空\*/ "coupon\_start\_time": "2018-11-05 00:00:00", /\*优惠券开始时间 该商品如无优惠券，此字段为空\*/ "coupon\_end\_time": "2018-11-10 23:59:59", /\*优惠券结束时间 该商品如无优惠券，此字段为空\*/ "coupon\_info\_money": "10", /\*优惠券金额 该商品如无优惠券，此字段为0\*/ "coupon\_total\_count": "100000", /\*优惠券总数量 该商品如无优惠券，此字段为0\*/ "coupon\_remain\_count": "99965", /\*优惠券剩余数量 该商品如无优惠券，此字段为0\*/ "coupon\_info": "满80.00元减10元", /\*优惠券信息\*/ "juhuasuan": "0", /\*是否聚划算，1是\*/ "taoqianggou": "0", /\*是否淘抢购，1是\*/ "haitao": "0", /\*是否海淘，1是\*/ "jiyoujia": "0", /\*是否极有家，1是\*/ "jinpaimaijia": "0", /\*是否金牌卖家，1是\*/ "pinpai": "0", /\*是否精选品牌，1是\*/ "pinpai\_name": "", /\*品牌名称\*/ "yunfeixian": "0", /\*是否有运费险，1有\*/ "nick": "\*\*\*旗舰店", /\*卖家昵称\*/ "small\_images": "https://img.alicdn.com/bao/.......jpg|https://img.alicdn.com/bao/.......jpg", /\*商品小图列表\*/ "white\_image": "https://img.alicdn.com/bao/......jpg", /\*商品白底图\*/ "tao\_title": "抽屉式收纳箱塑料储物箱整理箱衣服物家用宿舍衣柜内衣内裤收纳盒", /\*商品长标题\*/ "provcity": "浙江 杭州", /\*宝贝所在地\*/ "shop\_title": "\*\*\*旗舰店", /\*店铺名称\*/ "zhibo\_url": "https://cloud.video.taobao.com/play/u/849090736/p/2/e/6/t/1/219853683676.mp4", /\*视频地址\*/ "sellCount": "860000", /\*淘宝网页实时总销量\*/ "commentCount": "250000", /\*评论数量\*/ "favcount": "180000", /\*收藏数量\*/ "score1": "4.9", /\*宝贝描述分\*/ "score2": "4.9", /\*卖家服务分\*/ "score3": "4.9", /\*物流服务\*/ "creditLevel": "15", /\*店铺等级，一星 二星 三星 四星 五星 一钻 二钻 三钻 四钻 五钻 一皇冠 二皇冠 三皇冠 四皇冠 五皇冠 一金冠 二金冠 三金冠 四金冠 五金冠\*/ "shopIcon": "//img.alicdn.com/imgextra/de/a6/TB10dC8SX.jpg", /\*店铺logo\*/ "pcDescContent": "1.jpg|2.jpg|3.jpg|4.jpg|5.jpg", /\*图文详情图片地址\*/ "item\_url": "https://item.taobao.com/item.htm?id=581801364036", /\*商品url\*/ "category\_id": "50006126", /\*叶子类目id\*/ "category\_name": "收纳盒", /\*叶子类目name\*/ "level\_one\_category\_id": "122928002", /\*一级类目id\*/ "level\_one\_category\_name": "收纳整理", /\*一级类目name\*/ "tkfee3": "5.07", /\*返佣金额\*/ "biaoqian": ""满300元,省30元"", /\*店铺活动\*/ "tag": "赶紧拯救你厨房的油腻吧😂<br>植护🌿加厚厨房抽纸<br>高密度纤维纸张，吸油💧很不错<br>5包装|￥14.8元💰<br>✔厨房小帮手，用处多多", /\*朋友圈文案，需要自己进行urldecode\*/ "date\_time": "2018/11/8 14:45:22" /\*数据添加时间\*/ }\] };
```json
{
	"status": 200,                      /\*状态\*/
	"content": \[{
		"code": "6646",                 /\*折淘客编号\*/
		"type\_one\_id": "1",             /\*分类ID，可参考折淘客分类\*/
		"tao\_id": "554832820990",       /\*商品ID\*/
		"title": "储物箱整理抽屉式收纳柜",                   /\*商品短标题\*/
		"jianjie": "【爆款返场，出口日本】进口PP材质，可叠加透明可视抽屉收纳盒，分格抽屉物品分类齐全，区域分化一目了然，自由组合，大容量，让你居家生活更舒心，收纳你的生活！【赠运费险】", /\*商品简介\*/
		"pict\_url": "https://img.alicdn.com/bao/......jpg",                   /\*商品主图\*/
		"user\_type": "0",                   /\*是否天猫，0是淘宝，1是天猫\*/
		"seller\_id": "3073204721",                   /\*卖家ID\*/ /\*店铺地址可自行拼接，http://store.taobao.com/shop/view\_shop.htm?user\_number\_id=3073204721 \*/
		"shop\_dsr": "4.80",                   /\*商品描述分\*/
		"volume": "169",                   /\*年销量\*/
		"size": "26.90",                   /\*折扣价\*/
		"quanhou\_jiage": "16.90",                   /\*券后价\*/
		"date\_time\_yongjin": "2018/11/8 14:45:22",                   /\*数据更新时间\*/
		"tkrate3": "30.00",                   /\*佣金比率\*/
		"yongjin\_type": "MKT",                   /\*佣金类型\*/
		"coupon\_id": "fe31d6c8132c462ba59bd72e03d2eae0",                   /\*优惠券ID 该商品如无优惠券，此字段为空\*/
		"coupon\_start\_time": "2018-11-05 00:00:00",                   /\*优惠券开始时间 该商品如无优惠券，此字段为空\*/
		"coupon\_end\_time": "2018-11-10 23:59:59",                   /\*优惠券结束时间 该商品如无优惠券，此字段为空\*/
		"coupon\_info\_money": "10",                   /\*优惠券金额 该商品如无优惠券，此字段为0\*/
		"coupon\_total\_count": "100000",                   /\*优惠券总数量 该商品如无优惠券，此字段为0\*/
		"coupon\_remain\_count": "99965",                   /\*优惠券剩余数量 该商品如无优惠券，此字段为0\*/
		"coupon\_info": "满80.00元减10元",                   /\*优惠券信息\*/
		"juhuasuan": "0",                       /\*是否聚划算，1是\*/
		"taoqianggou": "0",                     /\*是否淘抢购，1是\*/
		"haitao": "0",                          /\*是否海淘，1是\*/
		"jiyoujia": "0",                        /\*是否极有家，1是\*/
		"jinpaimaijia": "0",                    /\*是否金牌卖家，1是\*/
		"pinpai": "0",                          /\*是否精选品牌，1是\*/
		"pinpai\_name": "",                      /\*品牌名称\*/
		"yunfeixian": "0",                      /\*是否有运费险，1有\*/
		"nick": "\*\*\*旗舰店",                    /\*卖家昵称\*/
		"small\_images": "https://img.alicdn.com/bao/.......jpg|https://img.alicdn.com/bao/.......jpg",                   /\*商品小图列表\*/
		"white\_image": "https://img.alicdn.com/bao/......jpg",                   /\*商品白底图\*/
		"tao\_title": "抽屉式收纳箱塑料储物箱整理箱衣服物家用宿舍衣柜内衣内裤收纳盒",                   /\*商品长标题\*/
		"provcity": "浙江 杭州",                   /\*宝贝所在地\*/
		"shop\_title": "\*\*\*旗舰店",                   /\*店铺名称\*/
		"zhibo\_url": "https://cloud.video.taobao.com/play/u/849090736/p/2/e/6/t/1/219853683676.mp4",                   /\*视频地址\*/
		"sellCount": "860000",                   /\*淘宝网页实时总销量\*/
		"commentCount": "250000",                   /\*评论数量\*/
		"favcount": "180000",                   /\*收藏数量\*/
		"score1": "4.9",                   /\*宝贝描述分\*/
		"score2": "4.9",                   /\*卖家服务分\*/
		"score3": "4.9",                   /\*物流服务分\*/
		"creditLevel": "15",                   /\*店铺等级（1-20），一星 二星 三星 四星 五星 一钻 二钻 三钻 四钻 五钻 一皇冠 二皇冠 三皇冠 四皇冠 五皇冠  一金冠 二金冠 三金冠 四金冠 五金冠\*/
		"shopIcon": "//img.alicdn.com/imgextra/de/a6/TB10dC8SX.jpg",       /\*店铺logo\*/
		"pcDescContent": "1.jpg|2.jpg|3.jpg|4.jpg|5.jpg",  /\*图文详情图片地址\*/
		"item\_url": "https://item.taobao.com/item.htm?id=581801364036",  /\*商品url\*/
		"category\_id": "50006126",                   /\*叶子类目id\*/
		"category\_name": "收纳盒",                   /\*叶子类目name\*/
		"level\_one\_category\_id": "122928002",                   /\*一级类目id\*/
		"level\_one\_category\_name": "收纳整理",                   /\*一级类目name\*/
		"tkfee3": "5.07",                   /\*返佣金额\*/
		"biaoqian": ""满300元,省30元"",                   /\*店铺活动\*/
		"tag": "赶紧拯救你厨房的油腻吧😂植护🌿加厚厨房抽纸高密度纤维纸张，吸油💧很不错5包装|￥14.8元💰✔厨房小帮手，用处多多",                   /\*朋友圈文案，需要自己进行urldecode\*/
		"date\_time": "2018/11/8 14:45:22"                   /\*数据添加时间\*/
	}\]
}
```

**失败返回示例：**

var tmp = { "status": 301 };
```json
{
	"status": 301                      /\*状态\*/
}
```

---

## 全网搜索商品API接口

**原始链接**: [全网搜索商品API接口](https://www.zhetaoke.com/user/extend/extend_lingquan_keywords.aspx)

**全网搜索商品API接口---本接口采用12台16核心64G内存服务器，江苏和北京双机房，负载均衡技术，1G独享带宽，放心调用。**

重要通知！重要通知！重要通知！此接口新增淘客sid和pid两个参数，这两个字段为必填字段！！！  
重要通知！重要通知！重要通知！此接口新增淘客sid和pid两个参数，这两个字段为必填字段！！！  
重要通知！重要通知！重要通知！此接口新增淘客sid和pid两个参数，这两个字段为必填字段！！！  
重要通知！重要通知！重要通知！此接口新增淘客sid和pid两个参数，这两个字段为必填字段！！！  
2024年3月1日上午9点，sid和pid参数必须填写，不填将无法获取数据，该时间也可能根据官方要求提前，请大家尽快修改。  

全网搜索商品API接口：根据关键字搜索商品，返回动态描述分≥4.6的商品列表，包含优惠券信息，可用来开发全网搜索等超级功能。

**接口特色：**通过大数据分析技术，有效过滤店淘商品、引流商品等垃圾商品。

**接口地址：** https://api.zhetaoke.com:10003/api/api\_quanwang.ashx?appkey=0a565628b2fb4712902712734cec2feb&sid=&pid=&page=1&page\_size=20&sort=new&q=内衣

**备用地址：**http://api.zhetaoke.cn:10000/api/api\_quanwang.ashx

**特殊说明：**[主接口和备用接口详细说明（请大家优先使用com主域名接口地址，在com出现问题的时候，人工或者自动切换cn备用接口地址）](https://www.zhetaoke.com/help_detail_3_27.html)

**返回数据：**json

**请求方式：**get

**重要说明：**此接口禁止开多线程采集，发现后将被限制使用！！！正常调用即可！！！

**重要说明：**此接口禁止开多线程采集，发现后将被限制使用！！！正常调用即可！！！

**重要说明：**此接口禁止开多线程采集，发现后将被限制使用！！！正常调用即可！！！

**重要说明：**利用淘宝分词接口，q参数可进行精确搜索和分词模糊搜索。比如：q=真学桌面看书架多功能书架儿童读书架桌上学生阅读架书立可伸缩书靠折叠书本支架简易夹书器，淘宝分词接口可返回“书架”。

**请求参数说明：**

|  | 名称 | 类型 | 必填 | 说明 |
| --- | --- | --- | --- | --- |
|   | appkey | string | 是 | 折淘客的对接秘钥appkey |
|   | sid | string | 是 | 对应的淘客账号授权ID[点击查看](https://www.zhetaoke.com/user/shouquan.html) |
|   | pid | string | 是 | 淘客PID，mm\_xxx\_xxx\_xxx,三段格式，必须与授权的账户相同，否则出错 |
|   | page | int | 否 | 分页获取数据,第几页 |
|   | page\_size | int | 否 | 每页数据条数（默认每页20条），可自定义1-50之间 |
|   | sort | string | 否 | 商品排序方式，new：按照综合排序，  
total\_sale\_num\_asc：按照总销量从小到大排序，total\_sale\_num\_desc：按照总销量从大到小排序，  
sale\_num\_asc：按照年销量从小到大排序，sale\_num\_desc：按照年销量从大到小排序，  
commission\_rate\_asc：按照佣金比例从小到大排序，commission\_rate\_desc：按照佣金比例从大到小排序，  
price\_asc：按照价格从小到大排序，price\_desc：按照价格从大到小排序。 |
|   | q | string | 否 | 输入“内衣”关键词，返回标题中带内衣的商品。支持关键字、支持店铺名称、支持商品明文链接。 |
|   | material\_id | string | 否 | 官方的物料Id，默认为空表示全网商品；  
 |
|   | youquan | string | 否 | 是否有券，1为有券，其它值为全部商品 |
|   | haiwai | string | 否 | 是否海外商品，1为海外，其它值为全部商品 |
|   | haoping | string | 否 | 是否好评商品，1为好评，其它值为全部商品 |
|   | tj | string | 否 | 是否天猫商品，值为空：全部商品，tmall：天猫商品 |
|   | itemloc | string | 否 | 商品所在地，值为空：全部商品，其它值：北京、上海、广州、深圳、重庆、杭州等。必须是城市名称，不能带省份。 |
|   | need\_prepay | string | 否 | 是否加入消费者保障，值为空：全部商品，1：加入消费者保障 |
|   | cat | string | 否 | 商品筛选-后台类目ID(category\_id)。用,分割，最大10个，该ID可以加入折淘客开放平台API群来获取。  
cat=50014023，表示：垂钓装备。  
cat=29，表示：宠物/宠物食品及用品。  
cat=50050359，表示：水产肉类/新鲜蔬果/熟食。  
cat=50454031，表示：景点门票/演艺演出/周边游。  
cat=122650005，表示：童鞋/婴儿鞋/亲子鞋。  
cat=122950001，表示：节庆用品/礼品。  
大家可以利用此参数，制作更多的特色专题栏目，如需要更多分类，请联系折淘客QQ群技术。  
 |
|   | start\_tk\_rate | string | 否 | 淘客佣金比率下限。如：输入20，表示大于等于20% |
|   | end\_tk\_rate | string | 否 | 淘客佣金比率上限。如：输入50，表示小于等于50% |
|   | start\_price | string | 否 | 折扣价格下限。如：输入100，表示大于等于100元 |
|   | end\_price | string | 否 | 折扣价格上限。如：输入200，表示小于等于200元 |
|   | type | string | 否 | 过滤值 值为0：不过滤，值为1：轻度过滤，值为2：中度过滤，强烈推荐值为2。 |

**成功返回示例：**/\*重要事情说三遍！重要事情说三遍！重要事情说三遍！api\_all/api\_dongdong/api\_quantian/api\_shishi/api\_videos/api\_xiaoshi/api\_yongjin接口所有参数都可以自由组合。\*/

var tmp = { "status": 200, /\*状态\*/ "content": \[{ "code": "6646", /\*折淘客编号\*/ "type\_one\_id": "1", /\*分类ID，可参考折淘客分类\*/ "tao\_id": "554832820990", /\*商品ID\*/ "title": "储物箱整理抽屉式收纳柜", /\*商品短标题\*/ "jianjie": "【爆款返场，出口日本】进口PP材质，可叠加透明可视抽屉收纳盒，分格抽屉物品分类齐全，区域分化一目了然，自由组合，大容量，让你居家生活更舒心，收纳你的生活！【赠运费险】", /\*商品简介\*/ "pict\_url": "https://img.alicdn.com/bao/......jpg", /\*商品主图\*/ "user\_type": "0", /\*是否天猫，0是淘宝，1是天猫\*/ "seller\_id": "3073204721", /\*卖家ID\*/ /\*店铺地址可自行拼接，http://store.taobao.com/shop/view\_shop.htm?user\_number\_id=3073204721 \*/ "shop\_dsr": "4.80", /\*商品描述分\*/ "volume": "169", /\*年销量\*/ "size": "26.90", /\*折扣价\*/ "quanhou\_jiage": "16.90", /\*券后价\*/ "date\_time\_yongjin": "2018/11/8 14:45:22", /\*数据更新时间\*/ "tkrate3": "30.00", /\*佣金比率\*/ "yongjin\_type": "MKT", /\*佣金类型\*/ "coupon\_id": "fe31d6c8132c462ba59bd72e03d2eae0", /\*优惠券ID 该商品如无优惠券，此字段为空\*/ "coupon\_start\_time": "2018-11-05 00:00:00", /\*优惠券开始时间 该商品如无优惠券，此字段为空\*/ "coupon\_end\_time": "2018-11-10 23:59:59", /\*优惠券结束时间 该商品如无优惠券，此字段为空\*/ "coupon\_info\_money": "10", /\*优惠券金额 该商品如无优惠券，此字段为0\*/ "coupon\_total\_count": "100000", /\*优惠券总数量 该商品如无优惠券，此字段为0\*/ "coupon\_remain\_count": "99965", /\*优惠券剩余数量 该商品如无优惠券，此字段为0\*/ "coupon\_info": "满80.00元减10元", /\*优惠券信息\*/ "juhuasuan": "0", /\*是否聚划算，1是\*/ "taoqianggou": "0", /\*是否淘抢购，1是\*/ "haitao": "0", /\*是否海淘，1是\*/ "jiyoujia": "0", /\*是否极有家，1是\*/ "jinpaimaijia": "0", /\*是否金牌卖家，1是\*/ "pinpai": "0", /\*是否精选品牌，1是\*/ "pinpai\_name": "", /\*品牌名称\*/ "yunfeixian": "0", /\*是否有运费险，1有\*/ "nick": "\*\*\*旗舰店", /\*卖家昵称\*/ "small\_images": "https://img.alicdn.com/bao/.......jpg|https://img.alicdn.com/bao/.......jpg", /\*商品小图列表\*/ "white\_image": "https://img.alicdn.com/bao/......jpg", /\*商品白底图\*/ "tao\_title": "抽屉式收纳箱塑料储物箱整理箱衣服物家用宿舍衣柜内衣内裤收纳盒", /\*商品长标题\*/ "provcity": "浙江 杭州", /\*宝贝所在地\*/ "shop\_title": "\*\*\*旗舰店", /\*店铺名称\*/ "zhibo\_url": "https://cloud.video.taobao.com/play/u/849090736/p/2/e/6/t/1/219853683676.mp4", /\*视频地址\*/ "sellCount": "860000", /\*淘宝网页实时总销量\*/ "commentCount": "250000", /\*评论数量\*/ "favcount": "180000", /\*收藏数量\*/ "score1": "4.9", /\*宝贝描述分\*/ "score2": "4.9", /\*卖家服务分\*/ "score3": "4.9", /\*物流服务\*/ "creditLevel": "15", /\*店铺等级，一星 二星 三星 四星 五星 一钻 二钻 三钻 四钻 五钻 一皇冠 二皇冠 三皇冠 四皇冠 五皇冠 一金冠 二金冠 三金冠 四金冠 五金冠\*/ "shopIcon": "//img.alicdn.com/imgextra/de/a6/TB10dC8SX.jpg", /\*店铺logo\*/ "pcDescContent": "1.jpg|2.jpg|3.jpg|4.jpg|5.jpg", /\*图文详情图片地址\*/ "item\_url": "https://item.taobao.com/item.htm?id=581801364036", /\*商品url\*/ "category\_id": "50006126", /\*叶子类目id\*/ "category\_name": "收纳盒", /\*叶子类目name\*/ "level\_one\_category\_id": "122928002", /\*一级类目id\*/ "level\_one\_category\_name": "收纳整理", /\*一级类目name\*/ "tkfee3": "5.07", /\*返佣金额\*/ "biaoqian": ""满300元,省30元"", /\*店铺活动\*/ "tag": "赶紧拯救你厨房的油腻吧😂<br>植护🌿加厚厨房抽纸<br>高密度纤维纸张，吸油💧很不错<br>5包装|￥14.8元💰<br>✔厨房小帮手，用处多多", /\*朋友圈文案，需要自己进行urldecode\*/ "date\_time": "2018/11/8 14:45:22" /\*数据添加时间\*/ }\] };
```json
{
	"status": 200,                      /\*状态\*/
	"content": \[{
		"code": "6646",                 /\*折淘客编号\*/
		"type\_one\_id": "1",             /\*分类ID，可参考折淘客分类\*/
		"tao\_id": "554832820990",       /\*商品ID\*/
		"title": "储物箱整理抽屉式收纳柜",                   /\*商品短标题\*/
		"jianjie": "【爆款返场，出口日本】进口PP材质，可叠加透明可视抽屉收纳盒，分格抽屉物品分类齐全，区域分化一目了然，自由组合，大容量，让你居家生活更舒心，收纳你的生活！【赠运费险】", /\*商品简介\*/
		"pict\_url": "https://img.alicdn.com/bao/......jpg",                   /\*商品主图\*/
		"user\_type": "0",                   /\*是否天猫，0是淘宝，1是天猫\*/
		"seller\_id": "3073204721",                   /\*卖家ID\*/ /\*店铺地址可自行拼接，http://store.taobao.com/shop/view\_shop.htm?user\_number\_id=3073204721 \*/
		"shop\_dsr": "4.80",                   /\*商品描述分\*/
		"volume": "169",                   /\*年销量\*/
		"size": "26.90",                   /\*折扣价\*/
		"quanhou\_jiage": "16.90",                   /\*券后价\*/
		"date\_time\_yongjin": "2018/11/8 14:45:22",                   /\*数据更新时间\*/
		"tkrate3": "30.00",                   /\*佣金比率\*/
		"yongjin\_type": "MKT",                   /\*佣金类型\*/
		"coupon\_id": "fe31d6c8132c462ba59bd72e03d2eae0",                   /\*优惠券ID 该商品如无优惠券，此字段为空\*/
		"coupon\_start\_time": "2018-11-05 00:00:00",                   /\*优惠券开始时间 该商品如无优惠券，此字段为空\*/
		"coupon\_end\_time": "2018-11-10 23:59:59",                   /\*优惠券结束时间 该商品如无优惠券，此字段为空\*/
		"coupon\_info\_money": "10",                   /\*优惠券金额 该商品如无优惠券，此字段为0\*/
		"coupon\_total\_count": "100000",                   /\*优惠券总数量 该商品如无优惠券，此字段为0\*/
		"coupon\_remain\_count": "99965",                   /\*优惠券剩余数量 该商品如无优惠券，此字段为0\*/
		"coupon\_info": "满80.00元减10元",                   /\*优惠券信息\*/
		"juhuasuan": "0",                       /\*是否聚划算，1是\*/
		"taoqianggou": "0",                     /\*是否淘抢购，1是\*/
		"haitao": "0",                          /\*是否海淘，1是\*/
		"jiyoujia": "0",                        /\*是否极有家，1是\*/
		"jinpaimaijia": "0",                    /\*是否金牌卖家，1是\*/
		"pinpai": "0",                          /\*是否精选品牌，1是\*/
		"pinpai\_name": "",                      /\*品牌名称\*/
		"yunfeixian": "0",                      /\*是否有运费险，1有\*/
		"nick": "\*\*\*旗舰店",                    /\*卖家昵称\*/
		"small\_images": "https://img.alicdn.com/bao/.......jpg|https://img.alicdn.com/bao/.......jpg",                   /\*商品小图列表\*/
		"white\_image": "https://img.alicdn.com/bao/......jpg",                   /\*商品白底图\*/
		"tao\_title": "抽屉式收纳箱塑料储物箱整理箱衣服物家用宿舍衣柜内衣内裤收纳盒",                   /\*商品长标题\*/
		"provcity": "浙江 杭州",                   /\*宝贝所在地\*/
		"shop\_title": "\*\*\*旗舰店",                   /\*店铺名称\*/
		"zhibo\_url": "https://cloud.video.taobao.com/play/u/849090736/p/2/e/6/t/1/219853683676.mp4",                   /\*视频地址\*/
		"sellCount": "860000",                   /\*淘宝网页实时总销量\*/
		"commentCount": "250000",                   /\*评论数量\*/
		"favcount": "180000",                   /\*收藏数量\*/
		"score1": "4.9",                   /\*宝贝描述分\*/
		"score2": "4.9",                   /\*卖家服务分\*/
		"score3": "4.9",                   /\*物流服务分\*/
		"creditLevel": "15",                   /\*店铺等级（1-20），一星 二星 三星 四星 五星 一钻 二钻 三钻 四钻 五钻 一皇冠 二皇冠 三皇冠 四皇冠 五皇冠  一金冠 二金冠 三金冠 四金冠 五金冠\*/
		"shopIcon": "//img.alicdn.com/imgextra/de/a6/TB10dC8SX.jpg",       /\*店铺logo\*/
		"pcDescContent": "1.jpg|2.jpg|3.jpg|4.jpg|5.jpg",  /\*图文详情图片地址\*/
		"item\_url": "https://item.taobao.com/item.htm?id=581801364036",  /\*商品url\*/
		"category\_id": "50006126",                   /\*叶子类目id\*/
		"category\_name": "收纳盒",                   /\*叶子类目name\*/
		"level\_one\_category\_id": "122928002",                   /\*一级类目id\*/
		"level\_one\_category\_name": "收纳整理",                   /\*一级类目name\*/
		"tkfee3": "5.07",                   /\*返佣金额\*/
		"biaoqian": ""满300元,省30元"",                   /\*店铺活动\*/
		"tag": "赶紧拯救你厨房的油腻吧😂植护🌿加厚厨房抽纸高密度纤维纸张，吸油💧很不错5包装|￥14.8元💰✔厨房小帮手，用处多多",                   /\*朋友圈文案，需要自己进行urldecode\*/
		"date\_time": "2018/11/8 14:45:22"                   /\*数据添加时间\*/
	}\]
}
```

**失败返回示例：**

var tmp = { "status": 301 };
```json
{
	"status": 301                      /\*状态\*/
}
```

[效果展示](http://cms.zhetaoke.com:10000/m/index_super.aspx)

[全网搜索说明](http://www.zhetaoke.com/help_detail_3_21.html)

---

## 两小时销量榜API接口

**原始链接**: [两小时销量榜API接口](https://www.zhetaoke.com/user/extend/extend_lingquan_xiaoshi.aspx)

**两小时销量榜API---本接口采用12台16核心64G内存服务器，江苏和北京双机房，负载均衡技术，1G独享带宽，放心调用。**

两小时销量榜API：返回两小时内销量榜单商品列表（前600个），返回佣金≥15%，动态描述分≥4.6的商品列表。

**接口地址：** https://api.zhetaoke.com:10001/api/api\_xiaoshi.ashx?appkey=0a565628b2fb4712902712734cec2feb&page=1&page\_size=20&sort=new&cid=

**返回数据：**json

**请求方式：**get

**请求参数说明：**

|  | 名称 | 类型 | 必填 | 说明 |
| --- | --- | --- | --- | --- |
|   | appkey | string | 是 | 折淘客的对接秘钥appkey |
|   | page | int | 否 | 分页获取数据,第几页 |
|   | page\_size | int | 否 | 每页数据条数（默认每页20条），可自定义1-50之间 |
|   | sort | string | 否 | 商品排序方式，new：按照两小时销量从高到低进行排序，  
total\_sale\_num\_asc：按照总销量从小到大排序，total\_sale\_num\_desc：按照总销量从大到小排序，  
sale\_num\_asc：按照年销量从小到大排序，sale\_num\_desc：按照年销量从大到小排序，  
commission\_rate\_asc：按照佣金比例从小到大排序，commission\_rate\_desc：按照佣金比例从大到小排序，  
price\_asc：按照价格从小到大排序，price\_desc：按照价格从大到小排序，  
coupon\_info\_money\_asc：按照优惠券金额从小到大排序，coupon\_info\_money\_desc：按照优惠券金额从大到小排序，  
shop\_level\_asc：按照店铺等级从低到高排序，shop\_level\_desc：按照店铺等级从高到低排序，  
tkfee\_asc：按照返佣金额从低到高排序，tkfee\_desc：按照返佣金额从高到低排序，  
code：按照code值从大到小排序，date\_time：按照更新时间排序，random：按照随机排序 |
|   | cid | int | 否 | 一级商品分类，值为空：全部商品，1：女装，2：母婴，3：美妆，4：居家日用，5：鞋品，6：美食，7：文娱车品，8：数码家电，9：男装，10：内衣，11：箱包，12：配饰，13：户外运动，14：家装家纺 |

**成功返回示例：**/\*重要事情说三遍！重要事情说三遍！重要事情说三遍！api\_all/api\_dongdong/api\_quantian/api\_shishi/api\_videos/api\_xiaoshi/api\_yongjin接口所有参数都可以自由组合。\*/

var tmp = { "status": 200, /\*状态\*/ "content": \[{ "code": "6646", /\*折淘客编号\*/ "type\_one\_id": "1", /\*分类ID，可参考折淘客分类\*/ "tao\_id": "554832820990", /\*商品ID\*/ "title": "储物箱整理抽屉式收纳柜", /\*商品短标题\*/ "jianjie": "【爆款返场，出口日本】进口PP材质，可叠加透明可视抽屉收纳盒，分格抽屉物品分类齐全，区域分化一目了然，自由组合，大容量，让你居家生活更舒心，收纳你的生活！【赠运费险】", /\*商品简介\*/ "pict\_url": "https://img.alicdn.com/bao/......jpg", /\*商品主图\*/ "user\_type": "0", /\*是否天猫，0是淘宝，1是天猫\*/ "seller\_id": "3073204721", /\*卖家ID\*/ /\*店铺地址可自行拼接，http://store.taobao.com/shop/view\_shop.htm?user\_number\_id=3073204721 \*/ "shop\_dsr": "4.80", /\*商品描述分\*/ "volume": "169", /\*年销量\*/ "size": "26.90", /\*折扣价\*/ "quanhou\_jiage": "16.90", /\*券后价\*/ "date\_time\_yongjin": "2018/11/8 14:45:22", /\*数据更新时间\*/ "tkrate3": "30.00", /\*佣金比率\*/ "yongjin\_type": "MKT", /\*佣金类型\*/ "coupon\_id": "fe31d6c8132c462ba59bd72e03d2eae0", /\*优惠券ID 该商品如无优惠券，此字段为空\*/ "coupon\_start\_time": "2018-11-05 00:00:00", /\*优惠券开始时间 该商品如无优惠券，此字段为空\*/ "coupon\_end\_time": "2018-11-10 23:59:59", /\*优惠券结束时间 该商品如无优惠券，此字段为空\*/ "coupon\_info\_money": "10", /\*优惠券金额 该商品如无优惠券，此字段为0\*/ "coupon\_total\_count": "100000", /\*优惠券总数量 该商品如无优惠券，此字段为0\*/ "coupon\_remain\_count": "99965", /\*优惠券剩余数量 该商品如无优惠券，此字段为0\*/ "coupon\_info": "满80.00元减10元", /\*优惠券信息\*/ "juhuasuan": "0", /\*是否聚划算，1是\*/ "taoqianggou": "0", /\*是否淘抢购，1是\*/ "haitao": "0", /\*是否海淘，1是\*/ "jiyoujia": "0", /\*是否极有家，1是\*/ "jinpaimaijia": "0", /\*是否金牌卖家，1是\*/ "pinpai": "0", /\*是否精选品牌，1是\*/ "pinpai\_name": "", /\*品牌名称\*/ "yunfeixian": "0", /\*是否有运费险，1有\*/ "nick": "\*\*\*旗舰店", /\*卖家昵称\*/ "small\_images": "https://img.alicdn.com/bao/.......jpg|https://img.alicdn.com/bao/.......jpg", /\*商品小图列表\*/ "white\_image": "https://img.alicdn.com/bao/......jpg", /\*商品白底图\*/ "tao\_title": "抽屉式收纳箱塑料储物箱整理箱衣服物家用宿舍衣柜内衣内裤收纳盒", /\*商品长标题\*/ "provcity": "浙江 杭州", /\*宝贝所在地\*/ "shop\_title": "\*\*\*旗舰店", /\*店铺名称\*/ "zhibo\_url": "https://cloud.video.taobao.com/play/u/849090736/p/2/e/6/t/1/219853683676.mp4", /\*视频地址\*/ "sellCount": "860000", /\*淘宝网页实时总销量\*/ "commentCount": "250000", /\*评论数量\*/ "favcount": "180000", /\*收藏数量\*/ "score1": "4.9", /\*宝贝描述分\*/ "score2": "4.9", /\*卖家服务分\*/ "score3": "4.9", /\*物流服务\*/ "creditLevel": "15", /\*店铺等级，一星 二星 三星 四星 五星 一钻 二钻 三钻 四钻 五钻 一皇冠 二皇冠 三皇冠 四皇冠 五皇冠 一金冠 二金冠 三金冠 四金冠 五金冠\*/ "shopIcon": "//img.alicdn.com/imgextra/de/a6/TB10dC8SX.jpg", /\*店铺logo\*/ "pcDescContent": "1.jpg|2.jpg|3.jpg|4.jpg|5.jpg", /\*图文详情图片地址\*/ "item\_url": "https://item.taobao.com/item.htm?id=581801364036", /\*商品url\*/ "category\_id": "50006126", /\*叶子类目id\*/ "category\_name": "收纳盒", /\*叶子类目name\*/ "level\_one\_category\_id": "122928002", /\*一级类目id\*/ "level\_one\_category\_name": "收纳整理", /\*一级类目name\*/ "volume\_shishi": "56000", /\*两小时销量\*/ "volume\_quantian": "26000", /\*全天销量\*/ "tk\_sale\_count": "260000", /\*人气值\*/ "tkfee3": "5.07", /\*返佣金额\*/ "biaoqian": "满300元,省30元", /\*店铺活动\*/ "tag": "赶紧拯救你厨房的油腻吧😂<br>植护🌿加厚厨房抽纸<br>高密度纤维纸张，吸油💧很不错<br>5包装|￥14.8元💰<br>✔厨房小帮手，用处多多", /\*朋友圈文案，需要自己进行urldecode\*/ "date\_time": "2018/11/8 14:45:22" /\*数据添加时间\*/ }\] };
```json
{
	"status": 200,                      /\*状态\*/
	"content": \[{
		"code": "6646",                 /\*折淘客编号\*/
		"type\_one\_id": "1",             /\*分类ID，可参考折淘客分类\*/
		"tao\_id": "554832820990",       /\*商品ID\*/
		"title": "储物箱整理抽屉式收纳柜",                   /\*商品短标题\*/
		"jianjie": "【爆款返场，出口日本】进口PP材质，可叠加透明可视抽屉收纳盒，分格抽屉物品分类齐全，区域分化一目了然，自由组合，大容量，让你居家生活更舒心，收纳你的生活！【赠运费险】", /\*商品简介\*/
		"pict\_url": "https://img.alicdn.com/bao/......jpg",                   /\*商品主图\*/
		"user\_type": "0",                   /\*是否天猫，0是淘宝，1是天猫\*/
		"seller\_id": "3073204721",                   /\*卖家ID\*/ /\*店铺地址可自行拼接，http://store.taobao.com/shop/view\_shop.htm?user\_number\_id=3073204721 \*/
		"shop\_dsr": "4.80",                   /\*商品描述分\*/
		"volume": "169",                   /\*年销量\*/
		"size": "26.90",                   /\*折扣价\*/
		"quanhou\_jiage": "16.90",                   /\*券后价\*/
		"date\_time\_yongjin": "2018/11/8 14:45:22",                   /\*数据更新时间\*/
		"tkrate3": "30.00",                   /\*佣金比率\*/
		"yongjin\_type": "MKT",                   /\*佣金类型\*/
		"coupon\_id": "fe31d6c8132c462ba59bd72e03d2eae0",                   /\*优惠券ID 该商品如无优惠券，此字段为空\*/
		"coupon\_start\_time": "2018-11-05 00:00:00",                   /\*优惠券开始时间 该商品如无优惠券，此字段为空\*/
		"coupon\_end\_time": "2018-11-10 23:59:59",                   /\*优惠券结束时间 该商品如无优惠券，此字段为空\*/
		"coupon\_info\_money": "10",                   /\*优惠券金额 该商品如无优惠券，此字段为0\*/
		"coupon\_total\_count": "100000",                   /\*优惠券总数量 该商品如无优惠券，此字段为0\*/
		"coupon\_remain\_count": "99965",                   /\*优惠券剩余数量 该商品如无优惠券，此字段为0\*/
		"coupon\_info": "满80.00元减10元",                   /\*优惠券信息\*/
		"juhuasuan": "0",                       /\*是否聚划算，1是\*/
		"taoqianggou": "0",                     /\*是否淘抢购，1是\*/
		"haitao": "0",                          /\*是否海淘，1是\*/
		"jiyoujia": "0",                        /\*是否极有家，1是\*/
		"jinpaimaijia": "0",                    /\*是否金牌卖家，1是\*/
		"pinpai": "0",                          /\*是否精选品牌，1是\*/
		"pinpai\_name": "",                      /\*品牌名称\*/
		"yunfeixian": "0",                      /\*是否有运费险，1有\*/
		"nick": "\*\*\*旗舰店",                    /\*卖家昵称\*/
		"small\_images": "https://img.alicdn.com/bao/.......jpg|https://img.alicdn.com/bao/.......jpg",                   /\*商品小图列表\*/
		"white\_image": "https://img.alicdn.com/bao/......jpg",                   /\*商品白底图\*/
		"tao\_title": "抽屉式收纳箱塑料储物箱整理箱衣服物家用宿舍衣柜内衣内裤收纳盒",                   /\*商品长标题\*/
		"provcity": "浙江 杭州",                   /\*宝贝所在地\*/
		"shop\_title": "\*\*\*旗舰店",                   /\*店铺名称\*/
		"zhibo\_url": "https://cloud.video.taobao.com/play/u/849090736/p/2/e/6/t/1/219853683676.mp4",                   /\*视频地址\*/
		"sellCount": "860000",                   /\*淘宝网页实时总销量\*/
		"commentCount": "250000",                   /\*评论数量\*/
		"favcount": "180000",                   /\*收藏数量\*/
		"score1": "4.9",                   /\*宝贝描述分\*/
		"score2": "4.9",                   /\*卖家服务分\*/
		"score3": "4.9",                   /\*物流服务分\*/
		"creditLevel": "15",                   /\*店铺等级（1-20），一星 二星 三星 四星 五星 一钻 二钻 三钻 四钻 五钻 一皇冠 二皇冠 三皇冠 四皇冠 五皇冠  一金冠 二金冠 三金冠 四金冠 五金冠\*/
		"shopIcon": "//img.alicdn.com/imgextra/de/a6/TB10dC8SX.jpg",       /\*店铺logo\*/
		"pcDescContent": "1.jpg|2.jpg|3.jpg|4.jpg|5.jpg",  /\*图文详情图片地址\*/
		"item\_url": "https://item.taobao.com/item.htm?id=581801364036",  /\*商品url\*/
		"category\_id": "50006126",                   /\*叶子类目id\*/
		"category\_name": "收纳盒",                   /\*叶子类目name\*/
		"level\_one\_category\_id": "122928002",                   /\*一级类目id\*/
		"level\_one\_category\_name": "收纳整理",                   /\*一级类目name\*/
		"volume\_shishi": "56000",                   /\*两小时销量\*/
		"volume\_quantian": "26000",                   /\*全天销量\*/
		"tk\_sale\_count": "260000",                   /\*人气值\*/
		"tkfee3": "5.07",                   /\*返佣金额\*/
		"biaoqian": ""满300元,省30元"",                   /\*店铺活动\*/
		"tag": "赶紧拯救你厨房的油腻吧😂植护🌿加厚厨房抽纸高密度纤维纸张，吸油💧很不错5包装|￥14.8元💰✔厨房小帮手，用处多多",                   /\*朋友圈文案，需要自己进行urldecode\*/
		"date\_time": "2018/11/8 14:45:22"                   /\*数据添加时间\*/
	}\]
}
```

**失败返回示例：**

var tmp = { "status": 301 };
```json
{
	"status": 301                      /\*状态\*/
}
```

[效果展示](http://cms.zhetaoke.com:10000/m/item/list.aspx?top=1)

---

## 全天销量榜API接口

**原始链接**: [全天销量榜API接口](https://www.zhetaoke.com/user/extend/extend_lingquan_quantian.aspx)

**全天销量榜API---本接口采用12台16核心64G内存服务器，江苏和北京双机房，负载均衡技术，1G独享带宽，放心调用。**

全天销量榜API：返回24小时内销量榜单商品列表（前600个），返回佣金≥15%，动态描述分≥4.6的商品列表。

**接口地址：** https://api.zhetaoke.com:10001/api/api\_quantian.ashx?appkey=0a565628b2fb4712902712734cec2feb&page=1&page\_size=20&sort=new&cid=

**返回数据：**json

**请求方式：**get

**请求参数说明：**

|  | 名称 | 类型 | 必填 | 说明 |
| --- | --- | --- | --- | --- |
|   | appkey | string | 是 | 折淘客的对接秘钥appkey |
|   | page | int | 否 | 分页获取数据,第几页 |
|   | page\_size | int | 否 | 每页数据条数（默认每页20条），可自定义1-50之间 |
|   | sort | string | 否 | 商品排序方式，new：按照全天销量从高到低排序，  
total\_sale\_num\_asc：按照总销量从小到大排序，total\_sale\_num\_desc：按照总销量从大到小排序，  
sale\_num\_asc：按照年销量从小到大排序，sale\_num\_desc：按照年销量从大到小排序，  
commission\_rate\_asc：按照佣金比例从小到大排序，commission\_rate\_desc：按照佣金比例从大到小排序，  
price\_asc：按照价格从小到大排序，price\_desc：按照价格从大到小排序，  
coupon\_info\_money\_asc：按照优惠券金额从小到大排序，coupon\_info\_money\_desc：按照优惠券金额从大到小排序，  
shop\_level\_asc：按照店铺等级从低到高排序，shop\_level\_desc：按照店铺等级从高到低排序，  
tkfee\_asc：按照返佣金额从低到高排序，tkfee\_desc：按照返佣金额从高到低排序，  
code：按照code值从大到小排序，date\_time：按照更新时间排序，random：按照随机排序 |
|   | cid | int | 否 | 一级商品分类，值为空：全部商品，1：女装，2：母婴，3：美妆，4：居家日用，5：鞋品，6：美食，7：文娱车品，8：数码家电，9：男装，10：内衣，11：箱包，12：配饰，13：户外运动，14：家装家纺 |

**成功返回示例：**/\*重要事情说三遍！重要事情说三遍！重要事情说三遍！api\_all/api\_dongdong/api\_quantian/api\_shishi/api\_videos/api\_xiaoshi/api\_yongjin接口所有参数都可以自由组合。\*/

var tmp = { "status": 200, /\*状态\*/ "content": \[{ "code": "6646", /\*折淘客编号\*/ "type\_one\_id": "1", /\*分类ID，可参考折淘客分类\*/ "tao\_id": "554832820990", /\*商品ID\*/ "title": "储物箱整理抽屉式收纳柜", /\*商品短标题\*/ "jianjie": "【爆款返场，出口日本】进口PP材质，可叠加透明可视抽屉收纳盒，分格抽屉物品分类齐全，区域分化一目了然，自由组合，大容量，让你居家生活更舒心，收纳你的生活！【赠运费险】", /\*商品简介\*/ "pict\_url": "https://img.alicdn.com/bao/......jpg", /\*商品主图\*/ "user\_type": "0", /\*是否天猫，0是淘宝，1是天猫\*/ "seller\_id": "3073204721", /\*卖家ID\*/ /\*店铺地址可自行拼接，http://store.taobao.com/shop/view\_shop.htm?user\_number\_id=3073204721 \*/ "shop\_dsr": "4.80", /\*商品描述分\*/ "volume": "169", /\*年销量\*/ "size": "26.90", /\*折扣价\*/ "quanhou\_jiage": "16.90", /\*券后价\*/ "date\_time\_yongjin": "2018/11/8 14:45:22", /\*数据更新时间\*/ "tkrate3": "30.00", /\*佣金比率\*/ "yongjin\_type": "MKT", /\*佣金类型\*/ "coupon\_id": "fe31d6c8132c462ba59bd72e03d2eae0", /\*优惠券ID 该商品如无优惠券，此字段为空\*/ "coupon\_start\_time": "2018-11-05 00:00:00", /\*优惠券开始时间 该商品如无优惠券，此字段为空\*/ "coupon\_end\_time": "2018-11-10 23:59:59", /\*优惠券结束时间 该商品如无优惠券，此字段为空\*/ "coupon\_info\_money": "10", /\*优惠券金额 该商品如无优惠券，此字段为0\*/ "coupon\_total\_count": "100000", /\*优惠券总数量 该商品如无优惠券，此字段为0\*/ "coupon\_remain\_count": "99965", /\*优惠券剩余数量 该商品如无优惠券，此字段为0\*/ "coupon\_info": "满80.00元减10元", /\*优惠券信息\*/ "juhuasuan": "0", /\*是否聚划算，1是\*/ "taoqianggou": "0", /\*是否淘抢购，1是\*/ "haitao": "0", /\*是否海淘，1是\*/ "jiyoujia": "0", /\*是否极有家，1是\*/ "jinpaimaijia": "0", /\*是否金牌卖家，1是\*/ "pinpai": "0", /\*是否精选品牌，1是\*/ "pinpai\_name": "", /\*品牌名称\*/ "yunfeixian": "0", /\*是否有运费险，1有\*/ "nick": "\*\*\*旗舰店", /\*卖家昵称\*/ "small\_images": "https://img.alicdn.com/bao/.......jpg|https://img.alicdn.com/bao/.......jpg", /\*商品小图列表\*/ "white\_image": "https://img.alicdn.com/bao/......jpg", /\*商品白底图\*/ "tao\_title": "抽屉式收纳箱塑料储物箱整理箱衣服物家用宿舍衣柜内衣内裤收纳盒", /\*商品长标题\*/ "provcity": "浙江 杭州", /\*宝贝所在地\*/ "shop\_title": "\*\*\*旗舰店", /\*店铺名称\*/ "zhibo\_url": "https://cloud.video.taobao.com/play/u/849090736/p/2/e/6/t/1/219853683676.mp4", /\*视频地址\*/ "sellCount": "860000", /\*淘宝网页实时总销量\*/ "commentCount": "250000", /\*评论数量\*/ "favcount": "180000", /\*收藏数量\*/ "score1": "4.9", /\*宝贝描述分\*/ "score2": "4.9", /\*卖家服务分\*/ "score3": "4.9", /\*物流服务\*/ "creditLevel": "15", /\*店铺等级，一星 二星 三星 四星 五星 一钻 二钻 三钻 四钻 五钻 一皇冠 二皇冠 三皇冠 四皇冠 五皇冠 一金冠 二金冠 三金冠 四金冠 五金冠\*/ "shopIcon": "//img.alicdn.com/imgextra/de/a6/TB10dC8SX.jpg", /\*店铺logo\*/ "pcDescContent": "1.jpg|2.jpg|3.jpg|4.jpg|5.jpg", /\*图文详情图片地址\*/ "item\_url": "https://item.taobao.com/item.htm?id=581801364036", /\*商品url\*/ "category\_id": "50006126", /\*叶子类目id\*/ "category\_name": "收纳盒", /\*叶子类目name\*/ "level\_one\_category\_id": "122928002", /\*一级类目id\*/ "level\_one\_category\_name": "收纳整理", /\*一级类目name\*/ "volume\_shishi": "56000", /\*两小时销量\*/ "volume\_quantian": "26000", /\*全天销量\*/ "tk\_sale\_count": "260000", /\*人气值\*/ "tkfee3": "5.07", /\*返佣金额\*/ "biaoqian": "满300元,省30元", /\*店铺活动\*/ "tag": "赶紧拯救你厨房的油腻吧😂<br>植护🌿加厚厨房抽纸<br>高密度纤维纸张，吸油💧很不错<br>5包装|￥14.8元💰<br>✔厨房小帮手，用处多多", /\*朋友圈文案，需要自己进行urldecode\*/ "date\_time": "2018/11/8 14:45:22" /\*数据添加时间\*/ }\] };
```json
{
	"status": 200,                      /\*状态\*/
	"content": \[{
		"code": "6646",                 /\*折淘客编号\*/
		"type\_one\_id": "1",             /\*分类ID，可参考折淘客分类\*/
		"tao\_id": "554832820990",       /\*商品ID\*/
		"title": "储物箱整理抽屉式收纳柜",                   /\*商品短标题\*/
		"jianjie": "【爆款返场，出口日本】进口PP材质，可叠加透明可视抽屉收纳盒，分格抽屉物品分类齐全，区域分化一目了然，自由组合，大容量，让你居家生活更舒心，收纳你的生活！【赠运费险】", /\*商品简介\*/
		"pict\_url": "https://img.alicdn.com/bao/......jpg",                   /\*商品主图\*/
		"user\_type": "0",                   /\*是否天猫，0是淘宝，1是天猫\*/
		"seller\_id": "3073204721",                   /\*卖家ID\*/ /\*店铺地址可自行拼接，http://store.taobao.com/shop/view\_shop.htm?user\_number\_id=3073204721 \*/
		"shop\_dsr": "4.80",                   /\*商品描述分\*/
		"volume": "169",                   /\*年销量\*/
		"size": "26.90",                   /\*折扣价\*/
		"quanhou\_jiage": "16.90",                   /\*券后价\*/
		"date\_time\_yongjin": "2018/11/8 14:45:22",                   /\*数据更新时间\*/
		"tkrate3": "30.00",                   /\*佣金比率\*/
		"yongjin\_type": "MKT",                   /\*佣金类型\*/
		"coupon\_id": "fe31d6c8132c462ba59bd72e03d2eae0",                   /\*优惠券ID 该商品如无优惠券，此字段为空\*/
		"coupon\_start\_time": "2018-11-05 00:00:00",                   /\*优惠券开始时间 该商品如无优惠券，此字段为空\*/
		"coupon\_end\_time": "2018-11-10 23:59:59",                   /\*优惠券结束时间 该商品如无优惠券，此字段为空\*/
		"coupon\_info\_money": "10",                   /\*优惠券金额 该商品如无优惠券，此字段为0\*/
		"coupon\_total\_count": "100000",                   /\*优惠券总数量 该商品如无优惠券，此字段为0\*/
		"coupon\_remain\_count": "99965",                   /\*优惠券剩余数量 该商品如无优惠券，此字段为0\*/
		"coupon\_info": "满80.00元减10元",                   /\*优惠券信息\*/
		"juhuasuan": "0",                       /\*是否聚划算，1是\*/
		"taoqianggou": "0",                     /\*是否淘抢购，1是\*/
		"haitao": "0",                          /\*是否海淘，1是\*/
		"jiyoujia": "0",                        /\*是否极有家，1是\*/
		"jinpaimaijia": "0",                    /\*是否金牌卖家，1是\*/
		"pinpai": "0",                          /\*是否精选品牌，1是\*/
		"pinpai\_name": "",                      /\*品牌名称\*/
		"yunfeixian": "0",                      /\*是否有运费险，1有\*/
		"nick": "\*\*\*旗舰店",                    /\*卖家昵称\*/
		"small\_images": "https://img.alicdn.com/bao/.......jpg|https://img.alicdn.com/bao/.......jpg",                   /\*商品小图列表\*/
		"white\_image": "https://img.alicdn.com/bao/......jpg",                   /\*商品白底图\*/
		"tao\_title": "抽屉式收纳箱塑料储物箱整理箱衣服物家用宿舍衣柜内衣内裤收纳盒",                   /\*商品长标题\*/
		"provcity": "浙江 杭州",                   /\*宝贝所在地\*/
		"shop\_title": "\*\*\*旗舰店",                   /\*店铺名称\*/
		"zhibo\_url": "https://cloud.video.taobao.com/play/u/849090736/p/2/e/6/t/1/219853683676.mp4",                   /\*视频地址\*/
		"sellCount": "860000",                   /\*淘宝网页实时总销量\*/
		"commentCount": "250000",                   /\*评论数量\*/
		"favcount": "180000",                   /\*收藏数量\*/
		"score1": "4.9",                   /\*宝贝描述分\*/
		"score2": "4.9",                   /\*卖家服务分\*/
		"score3": "4.9",                   /\*物流服务分\*/
		"creditLevel": "15",                   /\*店铺等级（1-20），一星 二星 三星 四星 五星 一钻 二钻 三钻 四钻 五钻 一皇冠 二皇冠 三皇冠 四皇冠 五皇冠  一金冠 二金冠 三金冠 四金冠 五金冠\*/
		"shopIcon": "//img.alicdn.com/imgextra/de/a6/TB10dC8SX.jpg",       /\*店铺logo\*/
		"pcDescContent": "1.jpg|2.jpg|3.jpg|4.jpg|5.jpg",  /\*图文详情图片地址\*/
		"item\_url": "https://item.taobao.com/item.htm?id=581801364036",  /\*商品url\*/
		"category\_id": "50006126",                   /\*叶子类目id\*/
		"category\_name": "收纳盒",                   /\*叶子类目name\*/
		"level\_one\_category\_id": "122928002",                   /\*一级类目id\*/
		"level\_one\_category\_name": "收纳整理",                   /\*一级类目name\*/
		"volume\_shishi": "56000",                   /\*两小时销量\*/
		"volume\_quantian": "26000",                   /\*全天销量\*/
		"tk\_sale\_count": "260000",                   /\*人气值\*/
		"tkfee3": "5.07",                   /\*返佣金额\*/
		"biaoqian": ""满300元,省30元"",                   /\*店铺活动\*/
		"tag": "赶紧拯救你厨房的油腻吧😂植护🌿加厚厨房抽纸高密度纤维纸张，吸油💧很不错5包装|￥14.8元💰✔厨房小帮手，用处多多",                   /\*朋友圈文案，需要自己进行urldecode\*/
		"date\_time": "2018/11/8 14:45:22"                   /\*数据添加时间\*/
	}\]
}
```

**失败返回示例：**

var tmp = { "status": 301 };
```json
{
	"status": 301                      /\*状态\*/
}
```

[效果展示](http://cms.zhetaoke.com:10000/m/item/list.aspx?top=2)

---

## 实时人气榜API接口

**原始链接**: [实时人气榜API接口](https://www.zhetaoke.com/user/extend/extend_lingquan_shishi.aspx)

**实时人气榜API---本接口采用12台16核心64G内存服务器，江苏和北京双机房，负载均衡技术，1G独享带宽，放心调用。**

实时人气榜API：实时返回人气榜单商品列表（前600个），返回佣金≥15%，动态描述分≥4.6的商品列表。

**接口地址：** https://api.zhetaoke.com:10001/api/api\_shishi.ashx?appkey=0a565628b2fb4712902712734cec2feb&page=1&page\_size=20&sort=new&cid=

**返回数据：**json

**请求方式：**get

**请求参数说明：**

|  | 名称 | 类型 | 必填 | 说明 |
| --- | --- | --- | --- | --- |
|   | appkey | string | 是 | 折淘客的对接秘钥appkey |
|   | page | int | 否 | 分页获取数据,第几页 |
|   | page\_size | int | 否 | 每页数据条数（默认每页20条），可自定义1-50之间 |
|   | sort | string | 否 | 商品排序方式，new：按照人气值从高到低进行排序，  
total\_sale\_num\_asc：按照总销量从小到大排序，total\_sale\_num\_desc：按照总销量从大到小排序，  
sale\_num\_asc：按照年销量从小到大排序，sale\_num\_desc：按照年销量从大到小排序，  
commission\_rate\_asc：按照佣金比例从小到大排序，commission\_rate\_desc：按照佣金比例从大到小排序，  
price\_asc：按照价格从小到大排序，price\_desc：按照价格从大到小排序，  
coupon\_info\_money\_asc：按照优惠券金额从小到大排序，coupon\_info\_money\_desc：按照优惠券金额从大到小排序，  
shop\_level\_asc：按照店铺等级从低到高排序，shop\_level\_desc：按照店铺等级从高到低排序，  
tkfee\_asc：按照返佣金额从低到高排序，tkfee\_desc：按照返佣金额从高到低排序，  
code：按照code值从大到小排序，date\_time：按照更新时间排序，random：按照随机排序 |
|   | cid | int | 否 | 一级商品分类，值为空：全部商品，1：女装，2：母婴，3：美妆，4：居家日用，5：鞋品，6：美食，7：文娱车品，8：数码家电，9：男装，10：内衣，11：箱包，12：配饰，13：户外运动，14：家装家纺 |

**成功返回示例：**/\*重要事情说三遍！重要事情说三遍！重要事情说三遍！api\_all/api\_dongdong/api\_quantian/api\_shishi/api\_videos/api\_xiaoshi/api\_yongjin接口所有参数都可以自由组合。\*/

var tmp = { "status": 200, /\*状态\*/ "content": \[{ "code": "6646", /\*折淘客编号\*/ "type\_one\_id": "1", /\*分类ID，可参考折淘客分类\*/ "tao\_id": "554832820990", /\*商品ID\*/ "title": "储物箱整理抽屉式收纳柜", /\*商品短标题\*/ "jianjie": "【爆款返场，出口日本】进口PP材质，可叠加透明可视抽屉收纳盒，分格抽屉物品分类齐全，区域分化一目了然，自由组合，大容量，让你居家生活更舒心，收纳你的生活！【赠运费险】", /\*商品简介\*/ "pict\_url": "https://img.alicdn.com/bao/......jpg", /\*商品主图\*/ "user\_type": "0", /\*是否天猫，0是淘宝，1是天猫\*/ "seller\_id": "3073204721", /\*卖家ID\*/ /\*店铺地址可自行拼接，http://store.taobao.com/shop/view\_shop.htm?user\_number\_id=3073204721 \*/ "shop\_dsr": "4.80", /\*商品描述分\*/ "volume": "169", /\*年销量\*/ "size": "26.90", /\*折扣价\*/ "quanhou\_jiage": "16.90", /\*券后价\*/ "date\_time\_yongjin": "2018/11/8 14:45:22", /\*数据更新时间\*/ "tkrate3": "30.00", /\*佣金比率\*/ "yongjin\_type": "MKT", /\*佣金类型\*/ "coupon\_id": "fe31d6c8132c462ba59bd72e03d2eae0", /\*优惠券ID 该商品如无优惠券，此字段为空\*/ "coupon\_start\_time": "2018-11-05 00:00:00", /\*优惠券开始时间 该商品如无优惠券，此字段为空\*/ "coupon\_end\_time": "2018-11-10 23:59:59", /\*优惠券结束时间 该商品如无优惠券，此字段为空\*/ "coupon\_info\_money": "10", /\*优惠券金额 该商品如无优惠券，此字段为0\*/ "coupon\_total\_count": "100000", /\*优惠券总数量 该商品如无优惠券，此字段为0\*/ "coupon\_remain\_count": "99965", /\*优惠券剩余数量 该商品如无优惠券，此字段为0\*/ "coupon\_info": "满80.00元减10元", /\*优惠券信息\*/ "juhuasuan": "0", /\*是否聚划算，1是\*/ "taoqianggou": "0", /\*是否淘抢购，1是\*/ "haitao": "0", /\*是否海淘，1是\*/ "jiyoujia": "0", /\*是否极有家，1是\*/ "jinpaimaijia": "0", /\*是否金牌卖家，1是\*/ "pinpai": "0", /\*是否精选品牌，1是\*/ "pinpai\_name": "", /\*品牌名称\*/ "yunfeixian": "0", /\*是否有运费险，1有\*/ "nick": "\*\*\*旗舰店", /\*卖家昵称\*/ "small\_images": "https://img.alicdn.com/bao/.......jpg|https://img.alicdn.com/bao/.......jpg", /\*商品小图列表\*/ "white\_image": "https://img.alicdn.com/bao/......jpg", /\*商品白底图\*/ "tao\_title": "抽屉式收纳箱塑料储物箱整理箱衣服物家用宿舍衣柜内衣内裤收纳盒", /\*商品长标题\*/ "provcity": "浙江 杭州", /\*宝贝所在地\*/ "shop\_title": "\*\*\*旗舰店", /\*店铺名称\*/ "zhibo\_url": "https://cloud.video.taobao.com/play/u/849090736/p/2/e/6/t/1/219853683676.mp4", /\*视频地址\*/ "sellCount": "860000", /\*淘宝网页实时总销量\*/ "commentCount": "250000", /\*评论数量\*/ "favcount": "180000", /\*收藏数量\*/ "score1": "4.9", /\*宝贝描述分\*/ "score2": "4.9", /\*卖家服务分\*/ "score3": "4.9", /\*物流服务\*/ "creditLevel": "15", /\*店铺等级，一星 二星 三星 四星 五星 一钻 二钻 三钻 四钻 五钻 一皇冠 二皇冠 三皇冠 四皇冠 五皇冠 一金冠 二金冠 三金冠 四金冠 五金冠\*/ "shopIcon": "//img.alicdn.com/imgextra/de/a6/TB10dC8SX.jpg", /\*店铺logo\*/ "pcDescContent": "1.jpg|2.jpg|3.jpg|4.jpg|5.jpg", /\*图文详情图片地址\*/ "item\_url": "https://item.taobao.com/item.htm?id=581801364036", /\*商品url\*/ "category\_id": "50006126", /\*叶子类目id\*/ "category\_name": "收纳盒", /\*叶子类目name\*/ "level\_one\_category\_id": "122928002", /\*一级类目id\*/ "level\_one\_category\_name": "收纳整理", /\*一级类目name\*/ "volume\_shishi": "56000", /\*两小时销量\*/ "volume\_quantian": "26000", /\*全天销量\*/ "tk\_sale\_count": "260000", /\*人气值\*/ "tkfee3": "5.07", /\*返佣金额\*/ "biaoqian": "满300元,省30元", /\*店铺活动\*/ "tag": "赶紧拯救你厨房的油腻吧😂<br>植护🌿加厚厨房抽纸<br>高密度纤维纸张，吸油💧很不错<br>5包装|￥14.8元💰<br>✔厨房小帮手，用处多多", /\*朋友圈文案，需要自己进行urldecode\*/ "date\_time": "2018/11/8 14:45:22" /\*数据添加时间\*/ }\] };
```json
{
	"status": 200,                      /\*状态\*/
	"content": \[{
		"code": "6646",                 /\*折淘客编号\*/
		"type\_one\_id": "1",             /\*分类ID，可参考折淘客分类\*/
		"tao\_id": "554832820990",       /\*商品ID\*/
		"title": "储物箱整理抽屉式收纳柜",                   /\*商品短标题\*/
		"jianjie": "【爆款返场，出口日本】进口PP材质，可叠加透明可视抽屉收纳盒，分格抽屉物品分类齐全，区域分化一目了然，自由组合，大容量，让你居家生活更舒心，收纳你的生活！【赠运费险】", /\*商品简介\*/
		"pict\_url": "https://img.alicdn.com/bao/......jpg",                   /\*商品主图\*/
		"user\_type": "0",                   /\*是否天猫，0是淘宝，1是天猫\*/
		"seller\_id": "3073204721",                   /\*卖家ID\*/ /\*店铺地址可自行拼接，http://store.taobao.com/shop/view\_shop.htm?user\_number\_id=3073204721 \*/
		"shop\_dsr": "4.80",                   /\*商品描述分\*/
		"volume": "169",                   /\*年销量\*/
		"size": "26.90",                   /\*折扣价\*/
		"quanhou\_jiage": "16.90",                   /\*券后价\*/
		"date\_time\_yongjin": "2018/11/8 14:45:22",                   /\*数据更新时间\*/
		"tkrate3": "30.00",                   /\*佣金比率\*/
		"yongjin\_type": "MKT",                   /\*佣金类型\*/
		"coupon\_id": "fe31d6c8132c462ba59bd72e03d2eae0",                   /\*优惠券ID 该商品如无优惠券，此字段为空\*/
		"coupon\_start\_time": "2018-11-05 00:00:00",                   /\*优惠券开始时间 该商品如无优惠券，此字段为空\*/
		"coupon\_end\_time": "2018-11-10 23:59:59",                   /\*优惠券结束时间 该商品如无优惠券，此字段为空\*/
		"coupon\_info\_money": "10",                   /\*优惠券金额 该商品如无优惠券，此字段为0\*/
		"coupon\_total\_count": "100000",                   /\*优惠券总数量 该商品如无优惠券，此字段为0\*/
		"coupon\_remain\_count": "99965",                   /\*优惠券剩余数量 该商品如无优惠券，此字段为0\*/
		"coupon\_info": "满80.00元减10元",                   /\*优惠券信息\*/
		"juhuasuan": "0",                       /\*是否聚划算，1是\*/
		"taoqianggou": "0",                     /\*是否淘抢购，1是\*/
		"haitao": "0",                          /\*是否海淘，1是\*/
		"jiyoujia": "0",                        /\*是否极有家，1是\*/
		"jinpaimaijia": "0",                    /\*是否金牌卖家，1是\*/
		"pinpai": "0",                          /\*是否精选品牌，1是\*/
		"pinpai\_name": "",                      /\*品牌名称\*/
		"yunfeixian": "0",                      /\*是否有运费险，1有\*/
		"nick": "\*\*\*旗舰店",                    /\*卖家昵称\*/
		"small\_images": "https://img.alicdn.com/bao/.......jpg|https://img.alicdn.com/bao/.......jpg",                   /\*商品小图列表\*/
		"white\_image": "https://img.alicdn.com/bao/......jpg",                   /\*商品白底图\*/
		"tao\_title": "抽屉式收纳箱塑料储物箱整理箱衣服物家用宿舍衣柜内衣内裤收纳盒",                   /\*商品长标题\*/
		"provcity": "浙江 杭州",                   /\*宝贝所在地\*/
		"shop\_title": "\*\*\*旗舰店",                   /\*店铺名称\*/
		"zhibo\_url": "https://cloud.video.taobao.com/play/u/849090736/p/2/e/6/t/1/219853683676.mp4",                   /\*视频地址\*/
		"sellCount": "860000",                   /\*淘宝网页实时总销量\*/
		"commentCount": "250000",                   /\*评论数量\*/
		"favcount": "180000",                   /\*收藏数量\*/
		"score1": "4.9",                   /\*宝贝描述分\*/
		"score2": "4.9",                   /\*卖家服务分\*/
		"score3": "4.9",                   /\*物流服务分\*/
		"creditLevel": "15",                   /\*店铺等级（1-20），一星 二星 三星 四星 五星 一钻 二钻 三钻 四钻 五钻 一皇冠 二皇冠 三皇冠 四皇冠 五皇冠  一金冠 二金冠 三金冠 四金冠 五金冠\*/
		"shopIcon": "//img.alicdn.com/imgextra/de/a6/TB10dC8SX.jpg",       /\*店铺logo\*/
		"pcDescContent": "1.jpg|2.jpg|3.jpg|4.jpg|5.jpg",  /\*图文详情图片地址\*/
		"item\_url": "https://item.taobao.com/item.htm?id=581801364036",  /\*商品url\*/
		"category\_id": "50006126",                   /\*叶子类目id\*/
		"category\_name": "收纳盒",                   /\*叶子类目name\*/
		"level\_one\_category\_id": "122928002",                   /\*一级类目id\*/
		"level\_one\_category\_name": "收纳整理",                   /\*一级类目name\*/
		"volume\_shishi": "56000",                   /\*两小时销量\*/
		"volume\_quantian": "26000",                   /\*全天销量\*/
		"tk\_sale\_count": "260000",                   /\*人气值\*/
		"tkfee3": "5.07",                   /\*返佣金额\*/
		"biaoqian": ""满300元,省30元"",                   /\*店铺活动\*/
		"tag": "赶紧拯救你厨房的油腻吧😂植护🌿加厚厨房抽纸高密度纤维纸张，吸油💧很不错5包装|￥14.8元💰✔厨房小帮手，用处多多",                   /\*朋友圈文案，需要自己进行urldecode\*/
		"date\_time": "2018/11/8 14:45:22"                   /\*数据添加时间\*/
	}\]
}
```

**失败返回示例：**

var tmp = { "status": 301 };
```json
{
	"status": 301                      /\*状态\*/
}
```

[效果展示](http://cms.zhetaoke.com:10000/m/item/list.aspx?top=3)

---

## 实时支出佣金榜API接口

**原始链接**: [实时支出佣金榜API接口](https://www.zhetaoke.com/user/extend/extend_lingquan_yongjin.aspx)

**实时支出佣金榜API---本接口采用12台16核心64G内存服务器，江苏和北京双机房，负载均衡技术，1G独享带宽，放心调用。**

实时支出佣金榜API：实时返回支出佣金榜单商品列表（前600个），返回佣金≥15%，动态描述分≥4.6的商品列表。

**接口地址：** https://api.zhetaoke.com:10001/api/api\_yongjin.ashx?appkey=0a565628b2fb4712902712734cec2feb&page=1&page\_size=20&sort=new&cid=

**返回数据：**json

**请求方式：**get

**请求参数说明：**

|  | 名称 | 类型 | 必填 | 说明 |
| --- | --- | --- | --- | --- |
|   | appkey | string | 是 | 折淘客的对接秘钥appkey |
|   | page | int | 否 | 分页获取数据,第几页 |
|   | page\_size | int | 否 | 每页数据条数（默认每页20条），可自定义1-50之间 |
|   | sort | string | 否 | 商品排序方式，new：按照月支出佣金从高到低进行排序，  
total\_sale\_num\_asc：按照总销量从小到大排序，total\_sale\_num\_desc：按照总销量从大到小排序，  
sale\_num\_asc：按照年销量从小到大排序，sale\_num\_desc：按照年销量从大到小排序，  
commission\_rate\_asc：按照佣金比例从小到大排序，commission\_rate\_desc：按照佣金比例从大到小排序，  
price\_asc：按照价格从小到大排序，price\_desc：按照价格从大到小排序，  
coupon\_info\_money\_asc：按照优惠券金额从小到大排序，coupon\_info\_money\_desc：按照优惠券金额从大到小排序，  
shop\_level\_asc：按照店铺等级从低到高排序，shop\_level\_desc：按照店铺等级从高到低排序，  
tkfee\_asc：按照返佣金额从低到高排序，tkfee\_desc：按照返佣金额从高到低排序，  
code：按照code值从大到小排序，date\_time：按照更新时间排序，random：按照随机排序 |
|   | cid | int | 否 | 一级商品分类，值为空：全部商品，1：女装，2：母婴，3：美妆，4：居家日用，5：鞋品，6：美食，7：文娱车品，8：数码家电，9：男装，10：内衣，11：箱包，12：配饰，13：户外运动，14：家装家纺 |

**成功返回示例：**/\*重要事情说三遍！重要事情说三遍！重要事情说三遍！api\_all/api\_dongdong/api\_quantian/api\_shishi/api\_videos/api\_xiaoshi/api\_yongjin接口所有参数都可以自由组合。\*/

var tmp = { "status": 200, /\*状态\*/ "content": \[{ "code": "6646", /\*折淘客编号\*/ "type\_one\_id": "1", /\*分类ID，可参考折淘客分类\*/ "tao\_id": "554832820990", /\*商品ID\*/ "title": "储物箱整理抽屉式收纳柜", /\*商品短标题\*/ "jianjie": "【爆款返场，出口日本】进口PP材质，可叠加透明可视抽屉收纳盒，分格抽屉物品分类齐全，区域分化一目了然，自由组合，大容量，让你居家生活更舒心，收纳你的生活！【赠运费险】", /\*商品简介\*/ "pict\_url": "https://img.alicdn.com/bao/......jpg", /\*商品主图\*/ "user\_type": "0", /\*是否天猫，0是淘宝，1是天猫\*/ "seller\_id": "3073204721", /\*卖家ID\*/ /\*店铺地址可自行拼接，http://store.taobao.com/shop/view\_shop.htm?user\_number\_id=3073204721 \*/ "shop\_dsr": "4.80", /\*商品描述分\*/ "volume": "169", /\*年销量\*/ "size": "26.90", /\*折扣价\*/ "quanhou\_jiage": "16.90", /\*券后价\*/ "date\_time\_yongjin": "2018/11/8 14:45:22", /\*数据更新时间\*/ "tkrate3": "30.00", /\*佣金比率\*/ "yongjin\_type": "MKT", /\*佣金类型\*/ "coupon\_id": "fe31d6c8132c462ba59bd72e03d2eae0", /\*优惠券ID 该商品如无优惠券，此字段为空\*/ "coupon\_start\_time": "2018-11-05 00:00:00", /\*优惠券开始时间 该商品如无优惠券，此字段为空\*/ "coupon\_end\_time": "2018-11-10 23:59:59", /\*优惠券结束时间 该商品如无优惠券，此字段为空\*/ "coupon\_info\_money": "10", /\*优惠券金额 该商品如无优惠券，此字段为0\*/ "coupon\_total\_count": "100000", /\*优惠券总数量 该商品如无优惠券，此字段为0\*/ "coupon\_remain\_count": "99965", /\*优惠券剩余数量 该商品如无优惠券，此字段为0\*/ "coupon\_info": "满80.00元减10元", /\*优惠券信息\*/ "juhuasuan": "0", /\*是否聚划算，1是\*/ "taoqianggou": "0", /\*是否淘抢购，1是\*/ "haitao": "0", /\*是否海淘，1是\*/ "jiyoujia": "0", /\*是否极有家，1是\*/ "jinpaimaijia": "0", /\*是否金牌卖家，1是\*/ "pinpai": "0", /\*是否精选品牌，1是\*/ "pinpai\_name": "", /\*品牌名称\*/ "yunfeixian": "0", /\*是否有运费险，1有\*/ "nick": "\*\*\*旗舰店", /\*卖家昵称\*/ "small\_images": "https://img.alicdn.com/bao/.......jpg|https://img.alicdn.com/bao/.......jpg", /\*商品小图列表\*/ "white\_image": "https://img.alicdn.com/bao/......jpg", /\*商品白底图\*/ "tao\_title": "抽屉式收纳箱塑料储物箱整理箱衣服物家用宿舍衣柜内衣内裤收纳盒", /\*商品长标题\*/ "provcity": "浙江 杭州", /\*宝贝所在地\*/ "shop\_title": "\*\*\*旗舰店", /\*店铺名称\*/ "zhibo\_url": "https://cloud.video.taobao.com/play/u/849090736/p/2/e/6/t/1/219853683676.mp4", /\*视频地址\*/ "sellCount": "860000", /\*淘宝网页实时总销量\*/ "commentCount": "250000", /\*评论数量\*/ "favcount": "180000", /\*收藏数量\*/ "score1": "4.9", /\*宝贝描述分\*/ "score2": "4.9", /\*卖家服务分\*/ "score3": "4.9", /\*物流服务\*/ "creditLevel": "15", /\*店铺等级，一星 二星 三星 四星 五星 一钻 二钻 三钻 四钻 五钻 一皇冠 二皇冠 三皇冠 四皇冠 五皇冠 一金冠 二金冠 三金冠 四金冠 五金冠\*/ "shopIcon": "//img.alicdn.com/imgextra/de/a6/TB10dC8SX.jpg", /\*店铺logo\*/ "pcDescContent": "1.jpg|2.jpg|3.jpg|4.jpg|5.jpg", /\*图文详情图片地址\*/ "item\_url": "https://item.taobao.com/item.htm?id=581801364036", /\*商品url\*/ "category\_id": "50006126", /\*叶子类目id\*/ "category\_name": "收纳盒", /\*叶子类目name\*/ "level\_one\_category\_id": "122928002", /\*一级类目id\*/ "level\_one\_category\_name": "收纳整理", /\*一级类目name\*/ "volume\_shishi": "56000", /\*两小时销量\*/ "volume\_quantian": "26000", /\*全天销量\*/ "tk\_sale\_count": "260000", /\*人气值\*/ "tkfee3": "5.07", /\*返佣金额\*/ "biaoqian": "满300元,省30元", /\*店铺活动\*/ "tag": "赶紧拯救你厨房的油腻吧😂<br>植护🌿加厚厨房抽纸<br>高密度纤维纸张，吸油💧很不错<br>5包装|￥14.8元💰<br>✔厨房小帮手，用处多多", /\*朋友圈文案，需要自己进行urldecode\*/ "date\_time": "2018/11/8 14:45:22" /\*数据添加时间\*/ }\] };
```json
{
	"status": 200,                      /\*状态\*/
	"content": \[{
		"code": "6646",                 /\*折淘客编号\*/
		"type\_one\_id": "1",             /\*分类ID，可参考折淘客分类\*/
		"tao\_id": "554832820990",       /\*商品ID\*/
		"title": "储物箱整理抽屉式收纳柜",                   /\*商品短标题\*/
		"jianjie": "【爆款返场，出口日本】进口PP材质，可叠加透明可视抽屉收纳盒，分格抽屉物品分类齐全，区域分化一目了然，自由组合，大容量，让你居家生活更舒心，收纳你的生活！【赠运费险】", /\*商品简介\*/
		"pict\_url": "https://img.alicdn.com/bao/......jpg",                   /\*商品主图\*/
		"user\_type": "0",                   /\*是否天猫，0是淘宝，1是天猫\*/
		"seller\_id": "3073204721",                   /\*卖家ID\*/ /\*店铺地址可自行拼接，http://store.taobao.com/shop/view\_shop.htm?user\_number\_id=3073204721 \*/
		"shop\_dsr": "4.80",                   /\*商品描述分\*/
		"volume": "169",                   /\*年销量\*/
		"size": "26.90",                   /\*折扣价\*/
		"quanhou\_jiage": "16.90",                   /\*券后价\*/
		"date\_time\_yongjin": "2018/11/8 14:45:22",                   /\*数据更新时间\*/
		"tkrate3": "30.00",                   /\*佣金比率\*/
		"yongjin\_type": "MKT",                   /\*佣金类型\*/
		"coupon\_id": "fe31d6c8132c462ba59bd72e03d2eae0",                   /\*优惠券ID 该商品如无优惠券，此字段为空\*/
		"coupon\_start\_time": "2018-11-05 00:00:00",                   /\*优惠券开始时间 该商品如无优惠券，此字段为空\*/
		"coupon\_end\_time": "2018-11-10 23:59:59",                   /\*优惠券结束时间 该商品如无优惠券，此字段为空\*/
		"coupon\_info\_money": "10",                   /\*优惠券金额 该商品如无优惠券，此字段为0\*/
		"coupon\_total\_count": "100000",                   /\*优惠券总数量 该商品如无优惠券，此字段为0\*/
		"coupon\_remain\_count": "99965",                   /\*优惠券剩余数量 该商品如无优惠券，此字段为0\*/
		"coupon\_info": "满80.00元减10元",                   /\*优惠券信息\*/
		"juhuasuan": "0",                       /\*是否聚划算，1是\*/
		"taoqianggou": "0",                     /\*是否淘抢购，1是\*/
		"haitao": "0",                          /\*是否海淘，1是\*/
		"jiyoujia": "0",                        /\*是否极有家，1是\*/
		"jinpaimaijia": "0",                    /\*是否金牌卖家，1是\*/
		"pinpai": "0",                          /\*是否精选品牌，1是\*/
		"pinpai\_name": "",                      /\*品牌名称\*/
		"yunfeixian": "0",                      /\*是否有运费险，1有\*/
		"nick": "\*\*\*旗舰店",                    /\*卖家昵称\*/
		"small\_images": "https://img.alicdn.com/bao/.......jpg|https://img.alicdn.com/bao/.......jpg",                   /\*商品小图列表\*/
		"white\_image": "https://img.alicdn.com/bao/......jpg",                   /\*商品白底图\*/
		"tao\_title": "抽屉式收纳箱塑料储物箱整理箱衣服物家用宿舍衣柜内衣内裤收纳盒",                   /\*商品长标题\*/
		"provcity": "浙江 杭州",                   /\*宝贝所在地\*/
		"shop\_title": "\*\*\*旗舰店",                   /\*店铺名称\*/
		"zhibo\_url": "https://cloud.video.taobao.com/play/u/849090736/p/2/e/6/t/1/219853683676.mp4",                   /\*视频地址\*/
		"sellCount": "860000",                   /\*淘宝网页实时总销量\*/
		"commentCount": "250000",                   /\*评论数量\*/
		"favcount": "180000",                   /\*收藏数量\*/
		"score1": "4.9",                   /\*宝贝描述分\*/
		"score2": "4.9",                   /\*卖家服务分\*/
		"score3": "4.9",                   /\*物流服务分\*/
		"creditLevel": "15",                   /\*店铺等级（1-20），一星 二星 三星 四星 五星 一钻 二钻 三钻 四钻 五钻 一皇冠 二皇冠 三皇冠 四皇冠 五皇冠  一金冠 二金冠 三金冠 四金冠 五金冠\*/
		"shopIcon": "//img.alicdn.com/imgextra/de/a6/TB10dC8SX.jpg",       /\*店铺logo\*/
		"pcDescContent": "1.jpg|2.jpg|3.jpg|4.jpg|5.jpg",  /\*图文详情图片地址\*/
		"item\_url": "https://item.taobao.com/item.htm?id=581801364036",  /\*商品url\*/
		"category\_id": "50006126",                   /\*叶子类目id\*/
		"category\_name": "收纳盒",                   /\*叶子类目name\*/
		"level\_one\_category\_id": "122928002",                   /\*一级类目id\*/
		"level\_one\_category\_name": "收纳整理",                   /\*一级类目name\*/
		"volume\_shishi": "56000",                   /\*两小时销量\*/
		"volume\_quantian": "26000",                   /\*全天销量\*/
		"tk\_sale\_count": "260000",                   /\*人气值\*/
		"tkfee3": "5.07",                   /\*返佣金额\*/
		"biaoqian": ""满300元,省30元"",                   /\*店铺活动\*/
		"tag": "赶紧拯救你厨房的油腻吧😂植护🌿加厚厨房抽纸高密度纤维纸张，吸油💧很不错5包装|￥14.8元💰✔厨房小帮手，用处多多",                   /\*朋友圈文案，需要自己进行urldecode\*/
		"date\_time": "2018/11/8 14:45:22"                   /\*数据添加时间\*/
	}\]
}
```

**失败返回示例：**

var tmp = { "status": 301 };
```json
{
	"status": 301                      /\*状态\*/
}
```

[效果展示](http://cms.zhetaoke.com:10000/m/item/list.aspx?top=3)

---

## 9.9元商品API

**原始链接**: [9.9元商品API](https://www.zhetaoke.com/user/extend/extend_lingquan_jiu.aspx)

**9.9元商品API---本接口采用12台16核心64G内存服务器，江苏和北京双机房，负载均衡技术，1G独享带宽，放心调用。**

9.9元商品API：返回购买价格≤9.9元的商品列表，返回佣金≥15%，动态描述分≥4.6的商品列表。

**接口地址：** https://api.zhetaoke.com:10001/api/api\_all.ashx?appkey=0a565628b2fb4712902712734cec2feb&page=1&page\_size=20&sort=new&price=0.0-9.9

9.9元商品API：返回指定查询条件的商品总数。

**接口地址：** https://api.zhetaoke.com:10001/api/api\_all.ashx?appkey=0a565628b2fb4712902712734cec2feb&page=1&page\_size=20&sort=new&price=0.0-9.9&total\_count=1

**返回数据：**json

**请求方式：**get

**请求参数说明：**

|  | 名称 | 类型 | 必填 | 说明 |
| --- | --- | --- | --- | --- |
|   | appkey | string | 是 | 折淘客的对接秘钥appkey |
|   | page | int | 否 | 分页获取数据,第几页 |
|   | page\_size | int | 否 | 每页数据条数（默认每页20条），可自定义1-50之间 |
|   | sort | string | 否 | 商品排序方式，new：按照综合排序，  
total\_sale\_num\_asc：按照总销量从小到大排序，total\_sale\_num\_desc：按照总销量从大到小排序，  
sale\_num\_asc：按照年销量从小到大排序，sale\_num\_desc：按照年销量从大到小排序，  
commission\_rate\_asc：按照佣金比例从小到大排序，commission\_rate\_desc：按照佣金比例从大到小排序，  
price\_asc：按照价格从小到大排序，price\_desc：按照价格从大到小排序，  
coupon\_info\_money\_asc：按照优惠券金额从小到大排序，coupon\_info\_money\_desc：按照优惠券金额从大到小排序，  
shop\_level\_asc：按照店铺等级从低到高排序，shop\_level\_desc：按照店铺等级从高到低排序，  
tkfee\_asc：按照返佣金额从低到高排序，tkfee\_desc：按照返佣金额从高到低排序，  
code：按照code值从大到小排序，date\_time：按照更新时间排序，random：按照随机排序 |
|   | cid | int | 否 | 一级商品分类，值为空：全部商品，1：女装，2：母婴，3：美妆，4：居家日用，5：鞋品，6：美食，7：文娱车品，8：数码家电，9：男装，10：内衣，11：箱包，12：配饰，13：户外运动，14：家装家纺 |
|   | price | string | 否 | 券后价价格区间，值为空：全部商品，0.0-9.9：9.9元商品，0.0-19.9：19.9元商品 |

**成功返回示例：**/\*重要事情说三遍！重要事情说三遍！重要事情说三遍！api\_all/api\_dongdong/api\_quantian/api\_shishi/api\_videos/api\_xiaoshi/api\_yongjin接口所有参数都可以自由组合。\*/

var tmp = { "status": 200, /\*状态\*/ "content": \[{ "code": "6646", /\*折淘客编号\*/ "type\_one\_id": "1", /\*分类ID，可参考折淘客分类\*/ "tao\_id": "554832820990", /\*商品ID\*/ "title": "储物箱整理抽屉式收纳柜", /\*商品短标题\*/ "jianjie": "【爆款返场，出口日本】进口PP材质，可叠加透明可视抽屉收纳盒，分格抽屉物品分类齐全，区域分化一目了然，自由组合，大容量，让你居家生活更舒心，收纳你的生活！【赠运费险】", /\*商品简介\*/ "pict\_url": "https://img.alicdn.com/bao/......jpg", /\*商品主图\*/ "user\_type": "0", /\*是否天猫，0是淘宝，1是天猫\*/ "seller\_id": "3073204721", /\*卖家ID\*/ /\*店铺地址可自行拼接，http://store.taobao.com/shop/view\_shop.htm?user\_number\_id=3073204721 \*/ "shop\_dsr": "4.80", /\*商品描述分\*/ "volume": "169", /\*年销量\*/ "size": "26.90", /\*折扣价\*/ "quanhou\_jiage": "16.90", /\*券后价\*/ "date\_time\_yongjin": "2018/11/8 14:45:22", /\*数据更新时间\*/ "tkrate3": "30.00", /\*佣金比率\*/ "yongjin\_type": "MKT", /\*佣金类型\*/ "coupon\_id": "fe31d6c8132c462ba59bd72e03d2eae0", /\*优惠券ID 该商品如无优惠券，此字段为空\*/ "coupon\_start\_time": "2018-11-05 00:00:00", /\*优惠券开始时间 该商品如无优惠券，此字段为空\*/ "coupon\_end\_time": "2018-11-10 23:59:59", /\*优惠券结束时间 该商品如无优惠券，此字段为空\*/ "coupon\_info\_money": "10", /\*优惠券金额 该商品如无优惠券，此字段为0\*/ "coupon\_total\_count": "100000", /\*优惠券总数量 该商品如无优惠券，此字段为0\*/ "coupon\_remain\_count": "99965", /\*优惠券剩余数量 该商品如无优惠券，此字段为0\*/ "coupon\_info": "满80.00元减10元", /\*优惠券信息\*/ "juhuasuan": "0", /\*是否聚划算，1是\*/ "taoqianggou": "0", /\*是否淘抢购，1是\*/ "haitao": "0", /\*是否海淘，1是\*/ "jiyoujia": "0", /\*是否极有家，1是\*/ "jinpaimaijia": "0", /\*是否金牌卖家，1是\*/ "pinpai": "0", /\*是否精选品牌，1是\*/ "pinpai\_name": "", /\*品牌名称\*/ "yunfeixian": "0", /\*是否有运费险，1有\*/ "nick": "\*\*\*旗舰店", /\*卖家昵称\*/ "small\_images": "https://img.alicdn.com/bao/.......jpg|https://img.alicdn.com/bao/.......jpg", /\*商品小图列表\*/ "white\_image": "https://img.alicdn.com/bao/......jpg", /\*商品白底图\*/ "tao\_title": "抽屉式收纳箱塑料储物箱整理箱衣服物家用宿舍衣柜内衣内裤收纳盒", /\*商品长标题\*/ "provcity": "浙江 杭州", /\*宝贝所在地\*/ "shop\_title": "\*\*\*旗舰店", /\*店铺名称\*/ "zhibo\_url": "https://cloud.video.taobao.com/play/u/849090736/p/2/e/6/t/1/219853683676.mp4", /\*视频地址\*/ "sellCount": "860000", /\*淘宝网页实时总销量\*/ "commentCount": "250000", /\*评论数量\*/ "favcount": "180000", /\*收藏数量\*/ "score1": "4.9", /\*宝贝描述分\*/ "score2": "4.9", /\*卖家服务分\*/ "score3": "4.9", /\*物流服务\*/ "creditLevel": "15", /\*店铺等级，一星 二星 三星 四星 五星 一钻 二钻 三钻 四钻 五钻 一皇冠 二皇冠 三皇冠 四皇冠 五皇冠 一金冠 二金冠 三金冠 四金冠 五金冠\*/ "shopIcon": "//img.alicdn.com/imgextra/de/a6/TB10dC8SX.jpg", /\*店铺logo\*/ "pcDescContent": "1.jpg|2.jpg|3.jpg|4.jpg|5.jpg", /\*图文详情图片地址\*/ "item\_url": "https://item.taobao.com/item.htm?id=581801364036", /\*商品url\*/ "category\_id": "50006126", /\*叶子类目id\*/ "category\_name": "收纳盒", /\*叶子类目name\*/ "level\_one\_category\_id": "122928002", /\*一级类目id\*/ "level\_one\_category\_name": "收纳整理", /\*一级类目name\*/ "tkfee3": "5.07", /\*返佣金额\*/ "biaoqian": ""满300元,省30元"", /\*店铺活动\*/ "tag": "赶紧拯救你厨房的油腻吧😂<br>植护🌿加厚厨房抽纸<br>高密度纤维纸张，吸油💧很不错<br>5包装|￥14.8元💰<br>✔厨房小帮手，用处多多", /\*朋友圈文案，需要自己进行urldecode\*/ "date\_time": "2018/11/8 14:45:22" /\*数据添加时间\*/ }\] };
```json
{
	"status": 200,                      /\*状态\*/
	"content": \[{
		"code": "6646",                 /\*折淘客编号\*/
		"type\_one\_id": "1",             /\*分类ID，可参考折淘客分类\*/
		"tao\_id": "554832820990",       /\*商品ID\*/
		"title": "储物箱整理抽屉式收纳柜",                   /\*商品短标题\*/
		"jianjie": "【爆款返场，出口日本】进口PP材质，可叠加透明可视抽屉收纳盒，分格抽屉物品分类齐全，区域分化一目了然，自由组合，大容量，让你居家生活更舒心，收纳你的生活！【赠运费险】", /\*商品简介\*/
		"pict\_url": "https://img.alicdn.com/bao/......jpg",                   /\*商品主图\*/
		"user\_type": "0",                   /\*是否天猫，0是淘宝，1是天猫\*/
		"seller\_id": "3073204721",                   /\*卖家ID\*/ /\*店铺地址可自行拼接，http://store.taobao.com/shop/view\_shop.htm?user\_number\_id=3073204721 \*/
		"shop\_dsr": "4.80",                   /\*商品描述分\*/
		"volume": "169",                   /\*年销量\*/
		"size": "26.90",                   /\*折扣价\*/
		"quanhou\_jiage": "16.90",                   /\*券后价\*/
		"date\_time\_yongjin": "2018/11/8 14:45:22",                   /\*数据更新时间\*/
		"tkrate3": "30.00",                   /\*佣金比率\*/
		"yongjin\_type": "MKT",                   /\*佣金类型\*/
		"coupon\_id": "fe31d6c8132c462ba59bd72e03d2eae0",                   /\*优惠券ID 该商品如无优惠券，此字段为空\*/
		"coupon\_start\_time": "2018-11-05 00:00:00",                   /\*优惠券开始时间 该商品如无优惠券，此字段为空\*/
		"coupon\_end\_time": "2018-11-10 23:59:59",                   /\*优惠券结束时间 该商品如无优惠券，此字段为空\*/
		"coupon\_info\_money": "10",                   /\*优惠券金额 该商品如无优惠券，此字段为0\*/
		"coupon\_total\_count": "100000",                   /\*优惠券总数量 该商品如无优惠券，此字段为0\*/
		"coupon\_remain\_count": "99965",                   /\*优惠券剩余数量 该商品如无优惠券，此字段为0\*/
		"coupon\_info": "满80.00元减10元",                   /\*优惠券信息\*/
		"juhuasuan": "0",                       /\*是否聚划算，1是\*/
		"taoqianggou": "0",                     /\*是否淘抢购，1是\*/
		"haitao": "0",                          /\*是否海淘，1是\*/
		"jiyoujia": "0",                        /\*是否极有家，1是\*/
		"jinpaimaijia": "0",                    /\*是否金牌卖家，1是\*/
		"pinpai": "0",                          /\*是否精选品牌，1是\*/
		"pinpai\_name": "",                      /\*品牌名称\*/
		"yunfeixian": "0",                      /\*是否有运费险，1有\*/
		"nick": "\*\*\*旗舰店",                    /\*卖家昵称\*/
		"small\_images": "https://img.alicdn.com/bao/.......jpg|https://img.alicdn.com/bao/.......jpg",                   /\*商品小图列表\*/
		"white\_image": "https://img.alicdn.com/bao/......jpg",                   /\*商品白底图\*/
		"tao\_title": "抽屉式收纳箱塑料储物箱整理箱衣服物家用宿舍衣柜内衣内裤收纳盒",                   /\*商品长标题\*/
		"provcity": "浙江 杭州",                   /\*宝贝所在地\*/
		"shop\_title": "\*\*\*旗舰店",                   /\*店铺名称\*/
		"zhibo\_url": "https://cloud.video.taobao.com/play/u/849090736/p/2/e/6/t/1/219853683676.mp4",                   /\*视频地址\*/
		"sellCount": "860000",                   /\*淘宝网页实时总销量\*/
		"commentCount": "250000",                   /\*评论数量\*/
		"favcount": "180000",                   /\*收藏数量\*/
		"score1": "4.9",                   /\*宝贝描述分\*/
		"score2": "4.9",                   /\*卖家服务分\*/
		"score3": "4.9",                   /\*物流服务分\*/
		"creditLevel": "15",                   /\*店铺等级（1-20），一星 二星 三星 四星 五星 一钻 二钻 三钻 四钻 五钻 一皇冠 二皇冠 三皇冠 四皇冠 五皇冠  一金冠 二金冠 三金冠 四金冠 五金冠\*/
		"shopIcon": "//img.alicdn.com/imgextra/de/a6/TB10dC8SX.jpg",       /\*店铺logo\*/
		"pcDescContent": "1.jpg|2.jpg|3.jpg|4.jpg|5.jpg",  /\*图文详情图片地址\*/
		"item\_url": "https://item.taobao.com/item.htm?id=581801364036",  /\*商品url\*/
		"category\_id": "50006126",                   /\*叶子类目id\*/
		"category\_name": "收纳盒",                   /\*叶子类目name\*/
		"level\_one\_category\_id": "122928002",                   /\*一级类目id\*/
		"level\_one\_category\_name": "收纳整理",                   /\*一级类目name\*/
		"tkfee3": "5.07",                   /\*返佣金额\*/
		"biaoqian": ""满300元,省30元"",                   /\*店铺活动\*/
		"tag": "赶紧拯救你厨房的油腻吧😂植护🌿加厚厨房抽纸高密度纤维纸张，吸油💧很不错5包装|￥14.8元💰✔厨房小帮手，用处多多",                   /\*朋友圈文案，需要自己进行urldecode\*/
		"date\_time": "2018/11/8 14:45:22"                   /\*数据添加时间\*/
	}\]
}
```

**失败返回示例：**

var tmp = { "status": 301 };
```json
{
	"status": 301                      /\*状态\*/
}
```

[效果展示](http://cms.zhetaoke.com:10000/m/item/list.aspx?maxfee=9.9)

---

## 19.9元商品API

**原始链接**: [19.9元商品API](https://www.zhetaoke.com/user/extend/extend_lingquan_shijiu.aspx)

**19.9元商品API---本接口采用12台16核心64G内存服务器，江苏和北京双机房，负载均衡技术，1G独享带宽，放心调用。**

19.9元商品API：返回购买价格≤19.9元的商品列表，返回佣金≥15%，动态描述分≥4.6的商品列表。

**接口地址：** https://api.zhetaoke.com:10001/api/api\_all.ashx?appkey=0a565628b2fb4712902712734cec2feb&page=1&page\_size=20&sort=new&price=0.0-19.9

19.9元商品API：返回指定查询条件的商品总数。

**接口地址：** https://api.zhetaoke.com:10001/api/api\_all.ashx?appkey=0a565628b2fb4712902712734cec2feb&page=1&page\_size=20&sort=new&price=0.0-19.9&total\_count=1

**返回数据：**json

**请求方式：**get

**请求参数说明：**

|  | 名称 | 类型 | 必填 | 说明 |
| --- | --- | --- | --- | --- |
|   | appkey | string | 是 | 折淘客的对接秘钥appkey |
|   | page | int | 否 | 分页获取数据,第几页 |
|   | page\_size | int | 否 | 每页数据条数（默认每页20条），可自定义1-50之间 |
|   | sort | string | 否 | 商品排序方式，new：按照综合排序，  
total\_sale\_num\_asc：按照总销量从小到大排序，total\_sale\_num\_desc：按照总销量从大到小排序，  
sale\_num\_asc：按照年销量从小到大排序，sale\_num\_desc：按照年销量从大到小排序，  
commission\_rate\_asc：按照佣金比例从小到大排序，commission\_rate\_desc：按照佣金比例从大到小排序，  
price\_asc：按照价格从小到大排序，price\_desc：按照价格从大到小排序，  
coupon\_info\_money\_asc：按照优惠券金额从小到大排序，coupon\_info\_money\_desc：按照优惠券金额从大到小排序，  
shop\_level\_asc：按照店铺等级从低到高排序，shop\_level\_desc：按照店铺等级从高到低排序，  
tkfee\_asc：按照返佣金额从低到高排序，tkfee\_desc：按照返佣金额从高到低排序，  
code：按照code值从大到小排序，date\_time：按照更新时间排序，random：按照随机排序 |
|   | cid | int | 否 | 一级商品分类，值为空：全部商品，1：女装，2：母婴，3：美妆，4：居家日用，5：鞋品，6：美食，7：文娱车品，8：数码家电，9：男装，10：内衣，11：箱包，12：配饰，13：户外运动，14：家装家纺 |
|   | price | string | 否 | 价格区间，值为空：全部商品，0.0-9.9：9.9元商品，0.0-19.9：19.9元商品 |

**成功返回示例：**/\*重要事情说三遍！重要事情说三遍！重要事情说三遍！api\_all/api\_dongdong/api\_quantian/api\_shishi/api\_videos/api\_xiaoshi/api\_yongjin接口所有参数都可以自由组合。\*/

var tmp = { "status": 200, /\*状态\*/ "content": \[{ "code": "6646", /\*折淘客编号\*/ "type\_one\_id": "1", /\*分类ID，可参考折淘客分类\*/ "tao\_id": "554832820990", /\*商品ID\*/ "title": "储物箱整理抽屉式收纳柜", /\*商品短标题\*/ "jianjie": "【爆款返场，出口日本】进口PP材质，可叠加透明可视抽屉收纳盒，分格抽屉物品分类齐全，区域分化一目了然，自由组合，大容量，让你居家生活更舒心，收纳你的生活！【赠运费险】", /\*商品简介\*/ "pict\_url": "https://img.alicdn.com/bao/......jpg", /\*商品主图\*/ "user\_type": "0", /\*是否天猫，0是淘宝，1是天猫\*/ "seller\_id": "3073204721", /\*卖家ID\*/ /\*店铺地址可自行拼接，http://store.taobao.com/shop/view\_shop.htm?user\_number\_id=3073204721 \*/ "shop\_dsr": "4.80", /\*商品描述分\*/ "volume": "169", /\*年销量\*/ "size": "26.90", /\*折扣价\*/ "quanhou\_jiage": "16.90", /\*券后价\*/ "date\_time\_yongjin": "2018/11/8 14:45:22", /\*数据更新时间\*/ "tkrate3": "30.00", /\*佣金比率\*/ "yongjin\_type": "MKT", /\*佣金类型\*/ "coupon\_id": "fe31d6c8132c462ba59bd72e03d2eae0", /\*优惠券ID 该商品如无优惠券，此字段为空\*/ "coupon\_start\_time": "2018-11-05 00:00:00", /\*优惠券开始时间 该商品如无优惠券，此字段为空\*/ "coupon\_end\_time": "2018-11-10 23:59:59", /\*优惠券结束时间 该商品如无优惠券，此字段为空\*/ "coupon\_info\_money": "10", /\*优惠券金额 该商品如无优惠券，此字段为0\*/ "coupon\_total\_count": "100000", /\*优惠券总数量 该商品如无优惠券，此字段为0\*/ "coupon\_remain\_count": "99965", /\*优惠券剩余数量 该商品如无优惠券，此字段为0\*/ "coupon\_info": "满80.00元减10元", /\*优惠券信息\*/ "juhuasuan": "0", /\*是否聚划算，1是\*/ "taoqianggou": "0", /\*是否淘抢购，1是\*/ "haitao": "0", /\*是否海淘，1是\*/ "jiyoujia": "0", /\*是否极有家，1是\*/ "jinpaimaijia": "0", /\*是否金牌卖家，1是\*/ "pinpai": "0", /\*是否精选品牌，1是\*/ "pinpai\_name": "", /\*品牌名称\*/ "yunfeixian": "0", /\*是否有运费险，1有\*/ "nick": "\*\*\*旗舰店", /\*卖家昵称\*/ "small\_images": "https://img.alicdn.com/bao/.......jpg|https://img.alicdn.com/bao/.......jpg", /\*商品小图列表\*/ "white\_image": "https://img.alicdn.com/bao/......jpg", /\*商品白底图\*/ "tao\_title": "抽屉式收纳箱塑料储物箱整理箱衣服物家用宿舍衣柜内衣内裤收纳盒", /\*商品长标题\*/ "provcity": "浙江 杭州", /\*宝贝所在地\*/ "shop\_title": "\*\*\*旗舰店", /\*店铺名称\*/ "zhibo\_url": "https://cloud.video.taobao.com/play/u/849090736/p/2/e/6/t/1/219853683676.mp4", /\*视频地址\*/ "sellCount": "860000", /\*淘宝网页实时总销量\*/ "commentCount": "250000", /\*评论数量\*/ "favcount": "180000", /\*收藏数量\*/ "score1": "4.9", /\*宝贝描述分\*/ "score2": "4.9", /\*卖家服务分\*/ "score3": "4.9", /\*物流服务\*/ "creditLevel": "15", /\*店铺等级，一星 二星 三星 四星 五星 一钻 二钻 三钻 四钻 五钻 一皇冠 二皇冠 三皇冠 四皇冠 五皇冠 一金冠 二金冠 三金冠 四金冠 五金冠\*/ "shopIcon": "//img.alicdn.com/imgextra/de/a6/TB10dC8SX.jpg", /\*店铺logo\*/ "pcDescContent": "1.jpg|2.jpg|3.jpg|4.jpg|5.jpg", /\*图文详情图片地址\*/ "item\_url": "https://item.taobao.com/item.htm?id=581801364036", /\*商品url\*/ "category\_id": "50006126", /\*叶子类目id\*/ "category\_name": "收纳盒", /\*叶子类目name\*/ "level\_one\_category\_id": "122928002", /\*一级类目id\*/ "level\_one\_category\_name": "收纳整理", /\*一级类目name\*/ "tkfee3": "5.07", /\*返佣金额\*/ "biaoqian": ""满300元,省30元"", /\*店铺活动\*/ "tag": "赶紧拯救你厨房的油腻吧😂<br>植护🌿加厚厨房抽纸<br>高密度纤维纸张，吸油💧很不错<br>5包装|￥14.8元💰<br>✔厨房小帮手，用处多多", /\*朋友圈文案，需要自己进行urldecode\*/ "date\_time": "2018/11/8 14:45:22" /\*数据添加时间\*/ }\] };
```json
{
	"status": 200,                      /\*状态\*/
	"content": \[{
		"code": "6646",                 /\*折淘客编号\*/
		"type\_one\_id": "1",             /\*分类ID，可参考折淘客分类\*/
		"tao\_id": "554832820990",       /\*商品ID\*/
		"title": "储物箱整理抽屉式收纳柜",                   /\*商品短标题\*/
		"jianjie": "【爆款返场，出口日本】进口PP材质，可叠加透明可视抽屉收纳盒，分格抽屉物品分类齐全，区域分化一目了然，自由组合，大容量，让你居家生活更舒心，收纳你的生活！【赠运费险】", /\*商品简介\*/
		"pict\_url": "https://img.alicdn.com/bao/......jpg",                   /\*商品主图\*/
		"user\_type": "0",                   /\*是否天猫，0是淘宝，1是天猫\*/
		"seller\_id": "3073204721",                   /\*卖家ID\*/ /\*店铺地址可自行拼接，http://store.taobao.com/shop/view\_shop.htm?user\_number\_id=3073204721 \*/
		"shop\_dsr": "4.80",                   /\*商品描述分\*/
		"volume": "169",                   /\*年销量\*/
		"size": "26.90",                   /\*折扣价\*/
		"quanhou\_jiage": "16.90",                   /\*券后价\*/
		"date\_time\_yongjin": "2018/11/8 14:45:22",                   /\*数据更新时间\*/
		"tkrate3": "30.00",                   /\*佣金比率\*/
		"yongjin\_type": "MKT",                   /\*佣金类型\*/
		"coupon\_id": "fe31d6c8132c462ba59bd72e03d2eae0",                   /\*优惠券ID 该商品如无优惠券，此字段为空\*/
		"coupon\_start\_time": "2018-11-05 00:00:00",                   /\*优惠券开始时间 该商品如无优惠券，此字段为空\*/
		"coupon\_end\_time": "2018-11-10 23:59:59",                   /\*优惠券结束时间 该商品如无优惠券，此字段为空\*/
		"coupon\_info\_money": "10",                   /\*优惠券金额 该商品如无优惠券，此字段为0\*/
		"coupon\_total\_count": "100000",                   /\*优惠券总数量 该商品如无优惠券，此字段为0\*/
		"coupon\_remain\_count": "99965",                   /\*优惠券剩余数量 该商品如无优惠券，此字段为0\*/
		"coupon\_info": "满80.00元减10元",                   /\*优惠券信息\*/
		"juhuasuan": "0",                       /\*是否聚划算，1是\*/
		"taoqianggou": "0",                     /\*是否淘抢购，1是\*/
		"haitao": "0",                          /\*是否海淘，1是\*/
		"jiyoujia": "0",                        /\*是否极有家，1是\*/
		"jinpaimaijia": "0",                    /\*是否金牌卖家，1是\*/
		"pinpai": "0",                          /\*是否精选品牌，1是\*/
		"pinpai\_name": "",                      /\*品牌名称\*/
		"yunfeixian": "0",                      /\*是否有运费险，1有\*/
		"nick": "\*\*\*旗舰店",                    /\*卖家昵称\*/
		"small\_images": "https://img.alicdn.com/bao/.......jpg|https://img.alicdn.com/bao/.......jpg",                   /\*商品小图列表\*/
		"white\_image": "https://img.alicdn.com/bao/......jpg",                   /\*商品白底图\*/
		"tao\_title": "抽屉式收纳箱塑料储物箱整理箱衣服物家用宿舍衣柜内衣内裤收纳盒",                   /\*商品长标题\*/
		"provcity": "浙江 杭州",                   /\*宝贝所在地\*/
		"shop\_title": "\*\*\*旗舰店",                   /\*店铺名称\*/
		"zhibo\_url": "https://cloud.video.taobao.com/play/u/849090736/p/2/e/6/t/1/219853683676.mp4",                   /\*视频地址\*/
		"sellCount": "860000",                   /\*淘宝网页实时总销量\*/
		"commentCount": "250000",                   /\*评论数量\*/
		"favcount": "180000",                   /\*收藏数量\*/
		"score1": "4.9",                   /\*宝贝描述分\*/
		"score2": "4.9",                   /\*卖家服务分\*/
		"score3": "4.9",                   /\*物流服务分\*/
		"creditLevel": "15",                   /\*店铺等级（1-20），一星 二星 三星 四星 五星 一钻 二钻 三钻 四钻 五钻 一皇冠 二皇冠 三皇冠 四皇冠 五皇冠  一金冠 二金冠 三金冠 四金冠 五金冠\*/
		"shopIcon": "//img.alicdn.com/imgextra/de/a6/TB10dC8SX.jpg",       /\*店铺logo\*/
		"pcDescContent": "1.jpg|2.jpg|3.jpg|4.jpg|5.jpg",  /\*图文详情图片地址\*/
		"item\_url": "https://item.taobao.com/item.htm?id=581801364036",  /\*商品url\*/
		"category\_id": "50006126",                   /\*叶子类目id\*/
		"category\_name": "收纳盒",                   /\*叶子类目name\*/
		"level\_one\_category\_id": "122928002",                   /\*一级类目id\*/
		"level\_one\_category\_name": "收纳整理",                   /\*一级类目name\*/
		"tkfee3": "5.07",                   /\*返佣金额\*/
		"biaoqian": ""满300元,省30元"",                   /\*店铺活动\*/
		"tag": "赶紧拯救你厨房的油腻吧😂植护🌿加厚厨房抽纸高密度纤维纸张，吸油💧很不错5包装|￥14.8元💰✔厨房小帮手，用处多多",                   /\*朋友圈文案，需要自己进行urldecode\*/
		"date\_time": "2018/11/8 14:45:22"                   /\*数据添加时间\*/
	}\]
}
```

**失败返回示例：**

var tmp = { "status": 301 };
```json
{
	"status": 301                      /\*状态\*/
}
```

[效果展示](http://cms.zhetaoke.com:10000/m/item/list.aspx?maxfee=19.9)

---

## 视频(抖货)商品API接口

**原始链接**: [视频(抖货)商品API接口](https://www.zhetaoke.com/user/extend/extend_lingquan_videos.aspx)

**视频(抖货)商品API接口---本接口采用12台16核心64G内存服务器，江苏和北京双机房，负载均衡技术，1G独享带宽，放心调用。**

视频(抖货)商品API接口：返回全网视频商品列表，返回佣金≥15%，动态描述分≥4.6的商品列表。

**接口地址：** https://api.zhetaoke.com:10001/api/api\_videos.ashx?appkey=0a565628b2fb4712902712734cec2feb&page=1&page\_size=20&sort=new

视频(抖货)商品API接口：返回指定查询条件的商品总数。

**接口地址：** https://api.zhetaoke.com:10001/api/api\_videos.ashx?appkey=0a565628b2fb4712902712734cec2feb&page=1&page\_size=20&sort=new&total\_count=1

**返回数据：**json

**请求方式：**get

**请求参数说明：**

|  | 名称 | 类型 | 必填 | 说明 |
| --- | --- | --- | --- | --- |
|   | appkey | string | 是 | 折淘客的对接秘钥appkey |
|   | page | int | 否 | 分页获取数据,第几页 |
|   | page\_size | int | 否 | 每页数据条数（默认每页20条），可自定义1-50之间 |
|   | sort | string | 否 | 商品排序方式，new：按照综合排序，  
total\_sale\_num\_asc：按照总销量从小到大排序，total\_sale\_num\_desc：按照总销量从大到小排序，  
sale\_num\_asc：按照年销量从小到大排序，sale\_num\_desc：按照年销量从大到小排序，  
commission\_rate\_asc：按照佣金比例从小到大排序，commission\_rate\_desc：按照佣金比例从大到小排序，  
price\_asc：按照价格从小到大排序，price\_desc：按照价格从大到小排序，  
coupon\_info\_money\_asc：按照优惠券金额从小到大排序，coupon\_info\_money\_desc：按照优惠券金额从大到小排序，  
shop\_level\_asc：按照店铺等级从低到高排序，shop\_level\_desc：按照店铺等级从高到低排序，  
tkfee\_asc：按照返佣金额从低到高排序，tkfee\_desc：按照返佣金额从高到低排序，  
code：按照code值从大到小排序，date\_time：按照更新时间排序，random：按照随机排序 |
|   | cid | int | 否 | 一级商品分类，值为空：全部商品，1：女装，2：母婴，3：美妆，4：居家日用，5：鞋品，6：美食，7：文娱车品，8：数码家电，9：男装，10：内衣，11：箱包，12：配饰，13：户外运动，14：家装家纺 |
|   | sale\_num\_start | string | 否 | 年销量≥ 值为空：全部商品，=500：年销量≥500的商品，=10000：年销量≥10000的商品 |

**成功返回示例：**/\*重要事情说三遍！重要事情说三遍！重要事情说三遍！api\_all/api\_dongdong/api\_quantian/api\_shishi/api\_videos/api\_xiaoshi/api\_yongjin接口所有参数都可以自由组合。\*/

var tmp = { "status": 200, /\*状态\*/ "content": \[{ "code": "6646", /\*折淘客编号\*/ "type\_one\_id": "1", /\*分类ID，可参考折淘客分类\*/ "tao\_id": "554832820990", /\*商品ID\*/ "title": "储物箱整理抽屉式收纳柜", /\*商品短标题\*/ "jianjie": "【爆款返场，出口日本】进口PP材质，可叠加透明可视抽屉收纳盒，分格抽屉物品分类齐全，区域分化一目了然，自由组合，大容量，让你居家生活更舒心，收纳你的生活！【赠运费险】", /\*商品简介\*/ "pict\_url": "https://img.alicdn.com/bao/......jpg", /\*商品主图\*/ "user\_type": "0", /\*是否天猫，0是淘宝，1是天猫\*/ "seller\_id": "3073204721", /\*卖家ID\*/ /\*店铺地址可自行拼接，http://store.taobao.com/shop/view\_shop.htm?user\_number\_id=3073204721 \*/ "shop\_dsr": "4.80", /\*商品描述分\*/ "volume": "169", /\*年销量\*/ "size": "26.90", /\*折扣价\*/ "quanhou\_jiage": "16.90", /\*券后价\*/ "date\_time\_yongjin": "2018/11/8 14:45:22", /\*数据更新时间\*/ "tkrate3": "30.00", /\*佣金比率\*/ "yongjin\_type": "MKT", /\*佣金类型\*/ "coupon\_id": "fe31d6c8132c462ba59bd72e03d2eae0", /\*优惠券ID 该商品如无优惠券，此字段为空\*/ "coupon\_start\_time": "2018-11-05 00:00:00", /\*优惠券开始时间 该商品如无优惠券，此字段为空\*/ "coupon\_end\_time": "2018-11-10 23:59:59", /\*优惠券结束时间 该商品如无优惠券，此字段为空\*/ "coupon\_info\_money": "10", /\*优惠券金额 该商品如无优惠券，此字段为0\*/ "coupon\_total\_count": "100000", /\*优惠券总数量 该商品如无优惠券，此字段为0\*/ "coupon\_remain\_count": "99965", /\*优惠券剩余数量 该商品如无优惠券，此字段为0\*/ "coupon\_info": "满80.00元减10元", /\*优惠券信息\*/ "juhuasuan": "0", /\*是否聚划算，1是\*/ "taoqianggou": "0", /\*是否淘抢购，1是\*/ "haitao": "0", /\*是否海淘，1是\*/ "jiyoujia": "0", /\*是否极有家，1是\*/ "jinpaimaijia": "0", /\*是否金牌卖家，1是\*/ "pinpai": "0", /\*是否精选品牌，1是\*/ "pinpai\_name": "", /\*品牌名称\*/ "yunfeixian": "0", /\*是否有运费险，1有\*/ "nick": "\*\*\*旗舰店", /\*卖家昵称\*/ "small\_images": "https://img.alicdn.com/bao/.......jpg|https://img.alicdn.com/bao/.......jpg", /\*商品小图列表\*/ "white\_image": "https://img.alicdn.com/bao/......jpg", /\*商品白底图\*/ "tao\_title": "抽屉式收纳箱塑料储物箱整理箱衣服物家用宿舍衣柜内衣内裤收纳盒", /\*商品长标题\*/ "provcity": "浙江 杭州", /\*宝贝所在地\*/ "shop\_title": "\*\*\*旗舰店", /\*店铺名称\*/ "zhibo\_url": "https://cloud.video.taobao.com/play/u/849090736/p/2/e/6/t/1/219853683676.mp4", /\*视频地址\*/ "sellCount": "860000", /\*淘宝网页实时总销量\*/ "commentCount": "250000", /\*评论数量\*/ "favcount": "180000", /\*收藏数量\*/ "score1": "4.9", /\*宝贝描述分\*/ "score2": "4.9", /\*卖家服务分\*/ "score3": "4.9", /\*物流服务\*/ "creditLevel": "15", /\*店铺等级，一星 二星 三星 四星 五星 一钻 二钻 三钻 四钻 五钻 一皇冠 二皇冠 三皇冠 四皇冠 五皇冠 一金冠 二金冠 三金冠 四金冠 五金冠\*/ "shopIcon": "//img.alicdn.com/imgextra/de/a6/TB10dC8SX.jpg", /\*店铺logo\*/ "pcDescContent": "1.jpg|2.jpg|3.jpg|4.jpg|5.jpg", /\*图文详情图片地址\*/ "item\_url": "https://item.taobao.com/item.htm?id=581801364036", /\*商品url\*/ "category\_id": "50006126", /\*叶子类目id\*/ "category\_name": "收纳盒", /\*叶子类目name\*/ "level\_one\_category\_id": "122928002", /\*一级类目id\*/ "level\_one\_category\_name": "收纳整理", /\*一级类目name\*/ "tkfee3": "5.07", /\*返佣金额\*/ "biaoqian": ""满300元,省30元"", /\*店铺活动\*/ "tag": "赶紧拯救你厨房的油腻吧😂<br>植护🌿加厚厨房抽纸<br>高密度纤维纸张，吸油💧很不错<br>5包装|￥14.8元💰<br>✔厨房小帮手，用处多多", /\*朋友圈文案，需要自己进行urldecode\*/ "date\_time": "2018/11/8 14:45:22" /\*数据添加时间\*/ }\] };
```json
{
	"status": 200,                      /\*状态\*/
	"content": \[{
		"code": "6646",                 /\*折淘客编号\*/
		"type\_one\_id": "1",             /\*分类ID，可参考折淘客分类\*/
		"tao\_id": "554832820990",       /\*商品ID\*/
		"title": "储物箱整理抽屉式收纳柜",                   /\*商品短标题\*/
		"jianjie": "【爆款返场，出口日本】进口PP材质，可叠加透明可视抽屉收纳盒，分格抽屉物品分类齐全，区域分化一目了然，自由组合，大容量，让你居家生活更舒心，收纳你的生活！【赠运费险】", /\*商品简介\*/
		"pict\_url": "https://img.alicdn.com/bao/......jpg",                   /\*商品主图\*/
		"user\_type": "0",                   /\*是否天猫，0是淘宝，1是天猫\*/
		"seller\_id": "3073204721",                   /\*卖家ID\*/ /\*店铺地址可自行拼接，http://store.taobao.com/shop/view\_shop.htm?user\_number\_id=3073204721 \*/
		"shop\_dsr": "4.80",                   /\*商品描述分\*/
		"volume": "169",                   /\*年销量\*/
		"size": "26.90",                   /\*折扣价\*/
		"quanhou\_jiage": "16.90",                   /\*券后价\*/
		"date\_time\_yongjin": "2018/11/8 14:45:22",                   /\*数据更新时间\*/
		"tkrate3": "30.00",                   /\*佣金比率\*/
		"yongjin\_type": "MKT",                   /\*佣金类型\*/
		"coupon\_id": "fe31d6c8132c462ba59bd72e03d2eae0",                   /\*优惠券ID 该商品如无优惠券，此字段为空\*/
		"coupon\_start\_time": "2018-11-05 00:00:00",                   /\*优惠券开始时间 该商品如无优惠券，此字段为空\*/
		"coupon\_end\_time": "2018-11-10 23:59:59",                   /\*优惠券结束时间 该商品如无优惠券，此字段为空\*/
		"coupon\_info\_money": "10",                   /\*优惠券金额 该商品如无优惠券，此字段为0\*/
		"coupon\_total\_count": "100000",                   /\*优惠券总数量 该商品如无优惠券，此字段为0\*/
		"coupon\_remain\_count": "99965",                   /\*优惠券剩余数量 该商品如无优惠券，此字段为0\*/
		"coupon\_info": "满80.00元减10元",                   /\*优惠券信息\*/
		"juhuasuan": "0",                       /\*是否聚划算，1是\*/
		"taoqianggou": "0",                     /\*是否淘抢购，1是\*/
		"haitao": "0",                          /\*是否海淘，1是\*/
		"jiyoujia": "0",                        /\*是否极有家，1是\*/
		"jinpaimaijia": "0",                    /\*是否金牌卖家，1是\*/
		"pinpai": "0",                          /\*是否精选品牌，1是\*/
		"pinpai\_name": "",                      /\*品牌名称\*/
		"yunfeixian": "0",                      /\*是否有运费险，1有\*/
		"nick": "\*\*\*旗舰店",                    /\*卖家昵称\*/
		"small\_images": "https://img.alicdn.com/bao/.......jpg|https://img.alicdn.com/bao/.......jpg",                   /\*商品小图列表\*/
		"white\_image": "https://img.alicdn.com/bao/......jpg",                   /\*商品白底图\*/
		"tao\_title": "抽屉式收纳箱塑料储物箱整理箱衣服物家用宿舍衣柜内衣内裤收纳盒",                   /\*商品长标题\*/
		"provcity": "浙江 杭州",                   /\*宝贝所在地\*/
		"shop\_title": "\*\*\*旗舰店",                   /\*店铺名称\*/
		"zhibo\_url": "https://cloud.video.taobao.com/play/u/849090736/p/2/e/6/t/1/219853683676.mp4",                   /\*视频地址\*/
		"sellCount": "860000",                   /\*淘宝网页实时总销量\*/
		"commentCount": "250000",                   /\*评论数量\*/
		"favcount": "180000",                   /\*收藏数量\*/
		"score1": "4.9",                   /\*宝贝描述分\*/
		"score2": "4.9",                   /\*卖家服务分\*/
		"score3": "4.9",                   /\*物流服务分\*/
		"creditLevel": "15",                   /\*店铺等级（1-20），一星 二星 三星 四星 五星 一钻 二钻 三钻 四钻 五钻 一皇冠 二皇冠 三皇冠 四皇冠 五皇冠  一金冠 二金冠 三金冠 四金冠 五金冠\*/
		"shopIcon": "//img.alicdn.com/imgextra/de/a6/TB10dC8SX.jpg",       /\*店铺logo\*/
		"pcDescContent": "1.jpg|2.jpg|3.jpg|4.jpg|5.jpg",  /\*图文详情图片地址\*/
		"item\_url": "https://item.taobao.com/item.htm?id=581801364036",  /\*商品url\*/
		"category\_id": "50006126",                   /\*叶子类目id\*/
		"category\_name": "收纳盒",                   /\*叶子类目name\*/
		"level\_one\_category\_id": "122928002",                   /\*一级类目id\*/
		"level\_one\_category\_name": "收纳整理",                   /\*一级类目name\*/
		"tkfee3": "5.07",                   /\*返佣金额\*/
		"biaoqian": ""满300元,省30元"",                   /\*店铺活动\*/
		"tag": "赶紧拯救你厨房的油腻吧😂植护🌿加厚厨房抽纸高密度纤维纸张，吸油💧很不错5包装|￥14.8元💰✔厨房小帮手，用处多多",                   /\*朋友圈文案，需要自己进行urldecode\*/
		"date\_time": "2018/11/8 14:45:22"                   /\*数据添加时间\*/
	}\]
}
```

**失败返回示例：**

var tmp = { "status": 301 };
```json
{
	"status": 301                      /\*状态\*/
}
```

[效果展示](http://cms.zhetaoke.com:10000/m/item/list.aspx?live=1)

---

## 相似商品API接口

**原始链接**: [相似商品API接口](https://www.zhetaoke.com/user/open/open_item_guess_like.aspx)

**相似商品API---本接口采用12台16核心64G内存服务器，江苏和北京双机房，负载均衡技术，1G独享带宽，放心调用。**

'

重要通知！重要通知！重要通知！此接口新增淘客sid和pid两个参数，这两个字段为必填字段！！！  
重要通知！重要通知！重要通知！此接口新增淘客sid和pid两个参数，这两个字段为必填字段！！！  
重要通知！重要通知！重要通知！此接口新增淘客sid和pid两个参数，这两个字段为必填字段！！！  
重要通知！重要通知！重要通知！此接口新增淘客sid和pid两个参数，这两个字段为必填字段！！！  
2024年3月1日上午9点，sid和pid参数必须填写，不填将无法获取数据，该时间也可能根据官方要求提前，请大家尽快修改。  

相似商品API：传入商品ID，即可返回个性化的相似商品推荐结果，实现千人千面效果。返回佣金≥15%，动态描述分≥4.6的商品列表。

**IMEI接口地址：** https://api.zhetaoke.com:10001/api/open\_item\_guess\_like.ashx?appkey=0a565628b2fb4712902712734cec2feb&sid=&pid=&device\_value=&device\_encrypt=MD5&device\_type=IMEI

**item\_id接口地址：** https://api.zhetaoke.com:10001/api/open\_item\_guess\_like.ashx?appkey=0a565628b2fb4712902712734cec2feb&sid=&pid=&item\_id=554832820990

**返回数据：**json

**请求方式：**get或者post

**请求参数说明：**

|  | 名称 | 类型 | 必填 | 说明 |
| --- | --- | --- | --- | --- |
|   | appkey | string | 是 | 折淘客的对接秘钥appkey |
|   | sid | string | 是 | 对应的淘客账号授权ID[点击查看](https://www.zhetaoke.com/user/shouquan.html) |
|   | pid | string | 是 | 淘客PID，mm\_xxx\_xxx\_xxx,三段格式，必须与授权的账户相同，否则出错 |
|   | page | int | 否 | 分页获取数据,第几页 |
|   | page\_size | int | 否 | 每页数据条数（默认每页20条），可自定义1-50之间 |
|   | sort | string | 否 | 商品排序方式，new：按照综合排序，  
total\_sale\_num\_asc：按照总销量从小到大排序，total\_sale\_num\_desc：按照总销量从大到小排序，  
sale\_num\_asc：按照年销量从小到大排序，sale\_num\_desc：按照年销量从大到小排序，  
commission\_rate\_asc：按照佣金比例从小到大排序，commission\_rate\_desc：按照佣金比例从大到小排序，  
price\_asc：按照价格从小到大排序，price\_desc：按照价格从大到小排序，  
coupon\_info\_money\_asc：按照优惠券金额从小到大排序，coupon\_info\_money\_desc：按照优惠券金额从大到小排序，  
shop\_level\_asc：按照店铺等级从低到高排序，shop\_level\_desc：按照店铺等级从高到低排序，  
tkfee\_asc：按照返佣金额从低到高排序，tkfee\_desc：按照返佣金额从高到低排序，  
code：按照code值从大到小排序，date\_time：按照更新时间排序，random：按照随机排序 |
|   | item\_id | string | 是 | 商品ID：该字段必填，商品ID。 |

**成功返回示例：**/\*重要事情说三遍！重要事情说三遍！重要事情说三遍！api\_all/api\_dongdong/api\_quantian/api\_shishi/api\_videos/api\_xiaoshi/api\_yongjin接口所有参数都可以自由组合。\*/

var tmp = { "status": 200, /\*状态\*/ "content": \[{ "code": "6646", /\*折淘客编号\*/ "type\_one\_id": "1", /\*分类ID，可参考折淘客分类\*/ "tao\_id": "554832820990", /\*商品ID\*/ "title": "储物箱整理抽屉式收纳柜", /\*商品短标题\*/ "jianjie": "【爆款返场，出口日本】进口PP材质，可叠加透明可视抽屉收纳盒，分格抽屉物品分类齐全，区域分化一目了然，自由组合，大容量，让你居家生活更舒心，收纳你的生活！【赠运费险】", /\*商品简介\*/ "pict\_url": "https://img.alicdn.com/bao/......jpg", /\*商品主图\*/ "user\_type": "0", /\*是否天猫，0是淘宝，1是天猫\*/ "seller\_id": "3073204721", /\*卖家ID\*/ /\*店铺地址可自行拼接，http://store.taobao.com/shop/view\_shop.htm?user\_number\_id=3073204721 \*/ "shop\_dsr": "4.80", /\*商品描述分\*/ "volume": "169", /\*年销量\*/ "size": "26.90", /\*折扣价\*/ "quanhou\_jiage": "16.90", /\*券后价\*/ "date\_time\_yongjin": "2018/11/8 14:45:22", /\*数据更新时间\*/ "tkrate3": "30.00", /\*佣金比率\*/ "yongjin\_type": "MKT", /\*佣金类型\*/ "coupon\_id": "fe31d6c8132c462ba59bd72e03d2eae0", /\*优惠券ID 该商品如无优惠券，此字段为空\*/ "coupon\_start\_time": "2018-11-05 00:00:00", /\*优惠券开始时间 该商品如无优惠券，此字段为空\*/ "coupon\_end\_time": "2018-11-10 23:59:59", /\*优惠券结束时间 该商品如无优惠券，此字段为空\*/ "coupon\_info\_money": "10", /\*优惠券金额 该商品如无优惠券，此字段为0\*/ "coupon\_total\_count": "100000", /\*优惠券总数量 该商品如无优惠券，此字段为0\*/ "coupon\_remain\_count": "99965", /\*优惠券剩余数量 该商品如无优惠券，此字段为0\*/ "coupon\_info": "满80.00元减10元", /\*优惠券信息\*/ "juhuasuan": "0", /\*是否聚划算，1是\*/ "taoqianggou": "0", /\*是否淘抢购，1是\*/ "haitao": "0", /\*是否海淘，1是\*/ "jiyoujia": "0", /\*是否极有家，1是\*/ "jinpaimaijia": "0", /\*是否金牌卖家，1是\*/ "pinpai": "0", /\*是否精选品牌，1是\*/ "pinpai\_name": "", /\*品牌名称\*/ "yunfeixian": "0", /\*是否有运费险，1有\*/ "nick": "\*\*\*旗舰店", /\*卖家昵称\*/ "small\_images": "https://img.alicdn.com/bao/.......jpg|https://img.alicdn.com/bao/.......jpg", /\*商品小图列表\*/ "white\_image": "https://img.alicdn.com/bao/......jpg", /\*商品白底图\*/ "tao\_title": "抽屉式收纳箱塑料储物箱整理箱衣服物家用宿舍衣柜内衣内裤收纳盒", /\*商品长标题\*/ "provcity": "浙江 杭州", /\*宝贝所在地\*/ "shop\_title": "\*\*\*旗舰店", /\*店铺名称\*/ "zhibo\_url": "https://cloud.video.taobao.com/play/u/849090736/p/2/e/6/t/1/219853683676.mp4", /\*视频地址\*/ "sellCount": "860000", /\*淘宝网页实时总销量\*/ "commentCount": "250000", /\*评论数量\*/ "favcount": "180000", /\*收藏数量\*/ "score1": "4.9", /\*宝贝描述分\*/ "score2": "4.9", /\*卖家服务分\*/ "score3": "4.9", /\*物流服务\*/ "creditLevel": "15", /\*店铺等级，一星 二星 三星 四星 五星 一钻 二钻 三钻 四钻 五钻 一皇冠 二皇冠 三皇冠 四皇冠 五皇冠 一金冠 二金冠 三金冠 四金冠 五金冠\*/ "shopIcon": "//img.alicdn.com/imgextra/de/a6/TB10dC8SX.jpg", /\*店铺logo\*/ "pcDescContent": "1.jpg|2.jpg|3.jpg|4.jpg|5.jpg", /\*图文详情图片地址\*/ "item\_url": "https://item.taobao.com/item.htm?id=581801364036", /\*商品url\*/ "category\_id": "50006126", /\*叶子类目id\*/ "category\_name": "收纳盒", /\*叶子类目name\*/ "level\_one\_category\_id": "122928002", /\*一级类目id\*/ "level\_one\_category\_name": "收纳整理", /\*一级类目name\*/ "tkfee3": "5.07", /\*返佣金额\*/ "biaoqian": ""满300元,省30元"", /\*店铺活动\*/ "tag": "赶紧拯救你厨房的油腻吧😂<br>植护🌿加厚厨房抽纸<br>高密度纤维纸张，吸油💧很不错<br>5包装|￥14.8元💰<br>✔厨房小帮手，用处多多", /\*朋友圈文案，需要自己进行urldecode\*/ "date\_time": "2018/11/8 14:45:22" /\*数据添加时间\*/ }\] };
```json
{
	"status": 200,                      /\*状态\*/
	"content": \[{
		"code": "6646",                 /\*折淘客编号\*/
		"type\_one\_id": "1",             /\*分类ID，可参考折淘客分类\*/
		"tao\_id": "554832820990",       /\*商品ID\*/
		"title": "储物箱整理抽屉式收纳柜",                   /\*商品短标题\*/
		"jianjie": "【爆款返场，出口日本】进口PP材质，可叠加透明可视抽屉收纳盒，分格抽屉物品分类齐全，区域分化一目了然，自由组合，大容量，让你居家生活更舒心，收纳你的生活！【赠运费险】", /\*商品简介\*/
		"pict\_url": "https://img.alicdn.com/bao/......jpg",                   /\*商品主图\*/
		"user\_type": "0",                   /\*是否天猫，0是淘宝，1是天猫\*/
		"seller\_id": "3073204721",                   /\*卖家ID\*/ /\*店铺地址可自行拼接，http://store.taobao.com/shop/view\_shop.htm?user\_number\_id=3073204721 \*/
		"shop\_dsr": "4.80",                   /\*商品描述分\*/
		"volume": "169",                   /\*年销量\*/
		"size": "26.90",                   /\*折扣价\*/
		"quanhou\_jiage": "16.90",                   /\*券后价\*/
		"date\_time\_yongjin": "2018/11/8 14:45:22",                   /\*数据更新时间\*/
		"tkrate3": "30.00",                   /\*佣金比率\*/
		"yongjin\_type": "MKT",                   /\*佣金类型\*/
		"coupon\_id": "fe31d6c8132c462ba59bd72e03d2eae0",                   /\*优惠券ID 该商品如无优惠券，此字段为空\*/
		"coupon\_start\_time": "2018-11-05 00:00:00",                   /\*优惠券开始时间 该商品如无优惠券，此字段为空\*/
		"coupon\_end\_time": "2018-11-10 23:59:59",                   /\*优惠券结束时间 该商品如无优惠券，此字段为空\*/
		"coupon\_info\_money": "10",                   /\*优惠券金额 该商品如无优惠券，此字段为0\*/
		"coupon\_total\_count": "100000",                   /\*优惠券总数量 该商品如无优惠券，此字段为0\*/
		"coupon\_remain\_count": "99965",                   /\*优惠券剩余数量 该商品如无优惠券，此字段为0\*/
		"coupon\_info": "满80.00元减10元",                   /\*优惠券信息\*/
		"juhuasuan": "0",                       /\*是否聚划算，1是\*/
		"taoqianggou": "0",                     /\*是否淘抢购，1是\*/
		"haitao": "0",                          /\*是否海淘，1是\*/
		"jiyoujia": "0",                        /\*是否极有家，1是\*/
		"jinpaimaijia": "0",                    /\*是否金牌卖家，1是\*/
		"pinpai": "0",                          /\*是否精选品牌，1是\*/
		"pinpai\_name": "",                      /\*品牌名称\*/
		"yunfeixian": "0",                      /\*是否有运费险，1有\*/
		"nick": "\*\*\*旗舰店",                    /\*卖家昵称\*/
		"small\_images": "https://img.alicdn.com/bao/.......jpg|https://img.alicdn.com/bao/.......jpg",                   /\*商品小图列表\*/
		"white\_image": "https://img.alicdn.com/bao/......jpg",                   /\*商品白底图\*/
		"tao\_title": "抽屉式收纳箱塑料储物箱整理箱衣服物家用宿舍衣柜内衣内裤收纳盒",                   /\*商品长标题\*/
		"provcity": "浙江 杭州",                   /\*宝贝所在地\*/
		"shop\_title": "\*\*\*旗舰店",                   /\*店铺名称\*/
		"zhibo\_url": "https://cloud.video.taobao.com/play/u/849090736/p/2/e/6/t/1/219853683676.mp4",                   /\*视频地址\*/
		"sellCount": "860000",                   /\*淘宝网页实时总销量\*/
		"commentCount": "250000",                   /\*评论数量\*/
		"favcount": "180000",                   /\*收藏数量\*/
		"score1": "4.9",                   /\*宝贝描述分\*/
		"score2": "4.9",                   /\*卖家服务分\*/
		"score3": "4.9",                   /\*物流服务分\*/
		"creditLevel": "15",                   /\*店铺等级（1-20），一星 二星 三星 四星 五星 一钻 二钻 三钻 四钻 五钻 一皇冠 二皇冠 三皇冠 四皇冠 五皇冠  一金冠 二金冠 三金冠 四金冠 五金冠\*/
		"shopIcon": "//img.alicdn.com/imgextra/de/a6/TB10dC8SX.jpg",       /\*店铺logo\*/
		"pcDescContent": "1.jpg|2.jpg|3.jpg|4.jpg|5.jpg",  /\*图文详情图片地址\*/
		"item\_url": "https://item.taobao.com/item.htm?id=581801364036",  /\*商品url\*/
		"category\_id": "50006126",                   /\*叶子类目id\*/
		"category\_name": "收纳盒",                   /\*叶子类目name\*/
		"level\_one\_category\_id": "122928002",                   /\*一级类目id\*/
		"level\_one\_category\_name": "收纳整理",                   /\*一级类目name\*/
		"tkfee3": "5.07",                   /\*返佣金额\*/
		"biaoqian": ""满300元,省30元"",                   /\*店铺活动\*/
		"tag": "赶紧拯救你厨房的油腻吧😂植护🌿加厚厨房抽纸高密度纤维纸张，吸油💧很不错5包装|￥14.8元💰✔厨房小帮手，用处多多",                   /\*朋友圈文案，需要自己进行urldecode\*/
		"date\_time": "2018/11/8 14:45:22"                   /\*数据添加时间\*/
	}\]
}
```

**失败返回示例：**

var tmp = { "status": 301 };
```json
{
	"status": 301                      /\*状态\*/
}
```

[效果展示](http://cms.zhetaoke.com:10000/m/item/detail_id.aspx?code=544647865020)

---

## 二维码发单图API

**原始链接**: [二维码发单图API](https://www.zhetaoke.com/user/open/open_qrpic.aspx)

**二维码发单图API（朋友圈文案）---本接口采用12台16核心64G内存服务器，江苏和北京双机房，负载均衡技术，1G独享带宽，放心调用。**

二维码发单图API（朋友圈文案）：支持淘口令文案，可返回二维码发单图url、中间页url和朋友圈文案，完全防封

**接口地址：**https://api.zhetaoke.com:10002/api/open\_qrpic.ashx

**请求实例：**https://api.zhetaoke.com:10002/api/open\_qrpic.ashx?appkey=0a565628b2fb4712902712734cec2feb&sid=#sid#&content=￥kStwYaf2xiD￥&type=0

**返回数据：**json

**请求方式：**get或者post

**请求参数说明：**

|  | 名称 | 类型 | 必填 | 说明 |
| --- | --- | --- | --- | --- |
|   | appkey | string | 是 | 折淘客的对接秘钥appkey |
|   | sid | string | 是 | 对应的淘客账号授权ID[点击查看](https://www.zhetaoke.com/user/shouquan.html) |
|   | content | string | 是 | 淘口令文案。请注意，该参数需要进行Urlencode编码后传入。  
重要的事情说三遍：该参数需要进行Urlencode编码！该参数需要进行Urlencode编码！该参数需要进行Urlencode编码！ |
|   | type | string | 否 | 二维码发单图样式，目前有0和1两种样式，后期会扩充。 |
|   | pict\_url | string | 否 | 商品主图地址（字段：pict\_url）或者其它商品图（字段：small\_images中的某一张），默认800\*800大小。  
此参数如果输入值为空，返回的pic\_url也将为空，输入值为：moren，后台默认按照此商品主图地址返回二维码图。  
注意：输入图片地址或者固定值“moren”，才会返回二维码发单图！如果输入值为空，则不返回二维码发单图！ |
|   | copy\_tkl\_url2 | string | 否 | 中间页url，原始url。如：http://cms.zhetaoke.com:10000/m/item/detail.aspx?code=593335919876 |
|   | text | string | 否 | 二维码扫描文字内容。内容中如果带淘口令，将显示该内容，否则显示默认文字内容。  
默认文字内容为：  
点击右上角【复制链接】或长按复制这段文字，打开【手机淘宝APP】，【领取优惠券】并购买$Od92YdM1vlf$...皎洁电热蚊香液无味家用儿童驱蚊器 |
|   | wenan | string | 否 | 朋友圈文案模板。  
默认文案模板内容为：  
{标题}<br />【在售价】：{在售价}元<br />【券后价】：{券后价}元<br />{简介}【优惠券：{优惠券金额}元】<br />-------------------<br />复制这条信息，({淘口令})，打开【手机taobao】即可查看并下单<br />【详情链接】{短链接}<br />  
注意：注意：注意：以下是文案说明，不是文案内容。  
1、{标题}，表示推广商品标题  
2、{在售价}，表示推广商品的原始价格  
3、{券后价}，表示推广商品的券后价格  
4、{简介}，表示推广商品的简介  
5、{优惠券金额}，表示推广商品的优惠券金额  
6、{短链接}，表示推广商品的短连接，不需要此项，删掉文案所在行即可  
7、{淘口令}，表示推广商品的淘口令，格式默认是：￥{淘口令}￥，可设置成({淘口令}),手淘都能认出。 |

**成功返回示例**

var tmp = { "model": "￥kStwYaf2xiD￥", "item\_id": "570499167319", "pic\_url": "https://api.zhetaoke.com:10001/uploads\_qrcode/20190413/201904132231017909.jpg", "copy\_tkl\_url": "https://t.cn/EXLVNdG", "copy\_tkl\_url2": "http://rd.wechat.com/qrcode/confirm?", "wenan": "这里是朋友圈文案。", }; document.write("<pre>" + syntaxHighlight(tmp) + "</pre>")
```json
{
    "model":"￥kStwYaf2xiD￥",     /\*淘口令\*/
    "item\_id":"570499167319",     /\*商品ID\*/
    "pic\_url":"https://api.zhetaoke.com:10001/uploads\_qrcode/20190413/201904132231017909.jpg",     /\*二维码发单图url\*/
    "copy\_tkl\_url":"https://t.cn/EXLVNdG"     /\*中间页url，新浪短链\*/
    "copy\_tkl\_url2":"http://rd.wechat.com/qrcode/confirm?"     /\*中间页url，原始url\*/
    "wenan":"这里是朋友圈文案。"     /\*朋友圈文案\*/
}
```

**二维码发单图样式：**  
 ![](https://www.zhetaoke.com/uploads_qrcode/1001.jpg) ![](https://www.zhetaoke.com/uploads_qrcode/1002.jpg)

接口调用说明：  
1、所有接口均免费提供，方便大家开发和接入自己的产品早日成为大牛淘客！  
2、使用高佣转链接口，生成您的淘客链接会自动申请最高佣金，包括鹊桥、定向计划、和营销计划（建议用户点击商品时触发式调用，降低服务器压力）！  
3、如需指定优惠券，在生成的二合一链接后，加入 &activityId=xxx券ID即可走指定券  

  
  

[乐享生成图片](http://cms.zhetaoke.com:10000/m/item/detail.aspx?code=573323475370)

---

## 超级分类商品API

**原始链接**: [超级分类商品API](https://www.zhetaoke.com/user/extend/extend_lingquan_type.aspx)

**按一级或者二级分类查询商品API---本接口采用12台16核心64G内存服务器，江苏和北京双机房，负载均衡技术，1G独享带宽，放心调用。**

按一级或者二级分类查询商品API：根据商品一级或者二级分类进行商品查询，返回佣金≥15%，动态描述分≥4.6的商品列表。

**接口地址：** https://api.zhetaoke.com:10001/api/api\_all.ashx?appkey=0a565628b2fb4712902712734cec2feb&page=1&page\_size=20&sort=new&cid=1&q=

按一级或者二级分类查询商品API：返回指定查询条件的商品总数。

**接口地址：** https://api.zhetaoke.com:10001/api/api\_all.ashx?appkey=0a565628b2fb4712902712734cec2feb&page=1&page\_size=20&sort=new&cid=1&q=&total\_count=1

一级分类及分类图片：返回一级分类列表，包括分类图片。

**接口地址：** https://api.zhetaoke.com:10001/api/api\_type\_one.ashx?appkey=0a565628b2fb4712902712734cec2feb

二级分类及分类图片：返回二级分类列表，包括分类图片。

**接口地址：** https://api.zhetaoke.com:10001/api/api\_type.ashx?appkey=0a565628b2fb4712902712734cec2feb

**返回数据：**json

**请求方式：**get

**请求参数说明：**

|  | 名称 | 类型 | 必填 | 说明 |
| --- | --- | --- | --- | --- |
|   | appkey | string | 是 | 折淘客的对接秘钥appkey |
|   | page | int | 否 | 分页获取数据,第几页 |
|   | page\_size | int | 否 | 每页数据条数（默认每页20条），可自定义1-50之间 |
|   | sort | string | 否 | 商品排序方式，new：按照综合排序，  
total\_sale\_num\_asc：按照总销量从小到大排序，total\_sale\_num\_desc：按照总销量从大到小排序，  
sale\_num\_asc：按照年销量从小到大排序，sale\_num\_desc：按照年销量从大到小排序，  
commission\_rate\_asc：按照佣金比例从小到大排序，commission\_rate\_desc：按照佣金比例从大到小排序，  
price\_asc：按照价格从小到大排序，price\_desc：按照价格从大到小排序，  
coupon\_info\_money\_asc：按照优惠券金额从小到大排序，coupon\_info\_money\_desc：按照优惠券金额从大到小排序，  
shop\_level\_asc：按照店铺等级从低到高排序，shop\_level\_desc：按照店铺等级从高到低排序，  
tkfee\_asc：按照返佣金额从低到高排序，tkfee\_desc：按照返佣金额从高到低排序，  
code：按照code值从大到小排序，date\_time：按照更新时间排序，random：按照随机排序 |
|   | cid | int | 否 | 一级商品分类，值为空：全部商品，1：女装，2：母婴，3：美妆，4：居家日用，5：鞋品，6：美食，7：文娱车品，8：数码家电，9：男装，10：内衣，11：箱包，12：配饰，13：户外运动，14：家装家纺 |
|   | q | string | 否 | 二级商品分类，值为空：全部商品，其它值请参考二级分类接口数据，比如外套，连衣裙等 |
|   | level\_one\_category\_id | string | 否 | 一级类目ID(level\_one\_category\_id)。只支持单个ID，该ID可以加入折淘客开放平台API群来获取。接口返回商品列表中也有该字段信息。 |
|   | category\_id | string | 否 | 叶子类目ID(category\_id)。支持多个ID，用,分割，最大10个，该ID可以加入折淘客开放平台API群来获取。接口返回商品列表中也有该字段信息。 |
|   | sale\_num\_start | string | 否 | 年销量≥ 值为空：全部商品，=500：年销量≥500的商品，=10000：年销量≥10000的商品 |

**成功返回示例：**/\*重要事情说三遍！重要事情说三遍！重要事情说三遍！api\_all/api\_dongdong/api\_quantian/api\_shishi/api\_videos/api\_xiaoshi/api\_yongjin接口所有参数都可以自由组合。\*/

var tmp = { "status": 200, /\*状态\*/ "content": \[{ "code": "6646", /\*折淘客编号\*/ "type\_one\_id": "1", /\*分类ID，可参考折淘客分类\*/ "tao\_id": "554832820990", /\*商品ID\*/ "title": "储物箱整理抽屉式收纳柜", /\*商品短标题\*/ "jianjie": "【爆款返场，出口日本】进口PP材质，可叠加透明可视抽屉收纳盒，分格抽屉物品分类齐全，区域分化一目了然，自由组合，大容量，让你居家生活更舒心，收纳你的生活！【赠运费险】", /\*商品简介\*/ "pict\_url": "https://img.alicdn.com/bao/......jpg", /\*商品主图\*/ "user\_type": "0", /\*是否天猫，0是淘宝，1是天猫\*/ "seller\_id": "3073204721", /\*卖家ID\*/ /\*店铺地址可自行拼接，http://store.taobao.com/shop/view\_shop.htm?user\_number\_id=3073204721 \*/ "shop\_dsr": "4.80", /\*商品描述分\*/ "volume": "169", /\*年销量\*/ "size": "26.90", /\*折扣价\*/ "quanhou\_jiage": "16.90", /\*券后价\*/ "date\_time\_yongjin": "2018/11/8 14:45:22", /\*数据更新时间\*/ "tkrate3": "30.00", /\*佣金比率\*/ "yongjin\_type": "MKT", /\*佣金类型\*/ "coupon\_id": "fe31d6c8132c462ba59bd72e03d2eae0", /\*优惠券ID 该商品如无优惠券，此字段为空\*/ "coupon\_start\_time": "2018-11-05 00:00:00", /\*优惠券开始时间 该商品如无优惠券，此字段为空\*/ "coupon\_end\_time": "2018-11-10 23:59:59", /\*优惠券结束时间 该商品如无优惠券，此字段为空\*/ "coupon\_info\_money": "10", /\*优惠券金额 该商品如无优惠券，此字段为0\*/ "coupon\_total\_count": "100000", /\*优惠券总数量 该商品如无优惠券，此字段为0\*/ "coupon\_remain\_count": "99965", /\*优惠券剩余数量 该商品如无优惠券，此字段为0\*/ "coupon\_info": "满80.00元减10元", /\*优惠券信息\*/ "juhuasuan": "0", /\*是否聚划算，1是\*/ "taoqianggou": "0", /\*是否淘抢购，1是\*/ "haitao": "0", /\*是否海淘，1是\*/ "jiyoujia": "0", /\*是否极有家，1是\*/ "jinpaimaijia": "0", /\*是否金牌卖家，1是\*/ "pinpai": "0", /\*是否精选品牌，1是\*/ "pinpai\_name": "", /\*品牌名称\*/ "yunfeixian": "0", /\*是否有运费险，1有\*/ "nick": "\*\*\*旗舰店", /\*卖家昵称\*/ "small\_images": "https://img.alicdn.com/bao/.......jpg|https://img.alicdn.com/bao/.......jpg", /\*商品小图列表\*/ "white\_image": "https://img.alicdn.com/bao/......jpg", /\*商品白底图\*/ "tao\_title": "抽屉式收纳箱塑料储物箱整理箱衣服物家用宿舍衣柜内衣内裤收纳盒", /\*商品长标题\*/ "provcity": "浙江 杭州", /\*宝贝所在地\*/ "shop\_title": "\*\*\*旗舰店", /\*店铺名称\*/ "zhibo\_url": "https://cloud.video.taobao.com/play/u/849090736/p/2/e/6/t/1/219853683676.mp4", /\*视频地址\*/ "sellCount": "860000", /\*淘宝网页实时总销量\*/ "commentCount": "250000", /\*评论数量\*/ "favcount": "180000", /\*收藏数量\*/ "score1": "4.9", /\*宝贝描述分\*/ "score2": "4.9", /\*卖家服务分\*/ "score3": "4.9", /\*物流服务\*/ "creditLevel": "15", /\*店铺等级，一星 二星 三星 四星 五星 一钻 二钻 三钻 四钻 五钻 一皇冠 二皇冠 三皇冠 四皇冠 五皇冠 一金冠 二金冠 三金冠 四金冠 五金冠\*/ "shopIcon": "//img.alicdn.com/imgextra/de/a6/TB10dC8SX.jpg", /\*店铺logo\*/ "pcDescContent": "1.jpg|2.jpg|3.jpg|4.jpg|5.jpg", /\*图文详情图片地址\*/ "item\_url": "https://item.taobao.com/item.htm?id=581801364036", /\*商品url\*/ "category\_id": "50006126", /\*叶子类目id\*/ "category\_name": "收纳盒", /\*叶子类目name\*/ "level\_one\_category\_id": "122928002", /\*一级类目id\*/ "level\_one\_category\_name": "收纳整理", /\*一级类目name\*/ "tkfee3": "5.07", /\*返佣金额\*/ "biaoqian": ""满300元,省30元"", /\*店铺活动\*/ "tag": "赶紧拯救你厨房的油腻吧😂<br>植护🌿加厚厨房抽纸<br>高密度纤维纸张，吸油💧很不错<br>5包装|￥14.8元💰<br>✔厨房小帮手，用处多多", /\*朋友圈文案，需要自己进行urldecode\*/ "date\_time": "2018/11/8 14:45:22" /\*数据添加时间\*/ }\] };
```json
{
	"status": 200,                      /\*状态\*/
	"content": \[{
		"code": "6646",                 /\*折淘客编号\*/
		"type\_one\_id": "1",             /\*分类ID，可参考折淘客分类\*/
		"tao\_id": "554832820990",       /\*商品ID\*/
		"title": "储物箱整理抽屉式收纳柜",                   /\*商品短标题\*/
		"jianjie": "【爆款返场，出口日本】进口PP材质，可叠加透明可视抽屉收纳盒，分格抽屉物品分类齐全，区域分化一目了然，自由组合，大容量，让你居家生活更舒心，收纳你的生活！【赠运费险】", /\*商品简介\*/
		"pict\_url": "https://img.alicdn.com/bao/......jpg",                   /\*商品主图\*/
		"user\_type": "0",                   /\*是否天猫，0是淘宝，1是天猫\*/
		"seller\_id": "3073204721",                   /\*卖家ID\*/ /\*店铺地址可自行拼接，http://store.taobao.com/shop/view\_shop.htm?user\_number\_id=3073204721 \*/
		"shop\_dsr": "4.80",                   /\*商品描述分\*/
		"volume": "169",                   /\*年销量\*/
		"size": "26.90",                   /\*折扣价\*/
		"quanhou\_jiage": "16.90",                   /\*券后价\*/
		"date\_time\_yongjin": "2018/11/8 14:45:22",                   /\*数据更新时间\*/
		"tkrate3": "30.00",                   /\*佣金比率\*/
		"yongjin\_type": "MKT",                   /\*佣金类型\*/
		"coupon\_id": "fe31d6c8132c462ba59bd72e03d2eae0",                   /\*优惠券ID 该商品如无优惠券，此字段为空\*/
		"coupon\_start\_time": "2018-11-05 00:00:00",                   /\*优惠券开始时间 该商品如无优惠券，此字段为空\*/
		"coupon\_end\_time": "2018-11-10 23:59:59",                   /\*优惠券结束时间 该商品如无优惠券，此字段为空\*/
		"coupon\_info\_money": "10",                   /\*优惠券金额 该商品如无优惠券，此字段为0\*/
		"coupon\_total\_count": "100000",                   /\*优惠券总数量 该商品如无优惠券，此字段为0\*/
		"coupon\_remain\_count": "99965",                   /\*优惠券剩余数量 该商品如无优惠券，此字段为0\*/
		"coupon\_info": "满80.00元减10元",                   /\*优惠券信息\*/
		"juhuasuan": "0",                       /\*是否聚划算，1是\*/
		"taoqianggou": "0",                     /\*是否淘抢购，1是\*/
		"haitao": "0",                          /\*是否海淘，1是\*/
		"jiyoujia": "0",                        /\*是否极有家，1是\*/
		"jinpaimaijia": "0",                    /\*是否金牌卖家，1是\*/
		"pinpai": "0",                          /\*是否精选品牌，1是\*/
		"pinpai\_name": "",                      /\*品牌名称\*/
		"yunfeixian": "0",                      /\*是否有运费险，1有\*/
		"nick": "\*\*\*旗舰店",                    /\*卖家昵称\*/
		"small\_images": "https://img.alicdn.com/bao/.......jpg|https://img.alicdn.com/bao/.......jpg",                   /\*商品小图列表\*/
		"white\_image": "https://img.alicdn.com/bao/......jpg",                   /\*商品白底图\*/
		"tao\_title": "抽屉式收纳箱塑料储物箱整理箱衣服物家用宿舍衣柜内衣内裤收纳盒",                   /\*商品长标题\*/
		"provcity": "浙江 杭州",                   /\*宝贝所在地\*/
		"shop\_title": "\*\*\*旗舰店",                   /\*店铺名称\*/
		"zhibo\_url": "https://cloud.video.taobao.com/play/u/849090736/p/2/e/6/t/1/219853683676.mp4",                   /\*视频地址\*/
		"sellCount": "860000",                   /\*淘宝网页实时总销量\*/
		"commentCount": "250000",                   /\*评论数量\*/
		"favcount": "180000",                   /\*收藏数量\*/
		"score1": "4.9",                   /\*宝贝描述分\*/
		"score2": "4.9",                   /\*卖家服务分\*/
		"score3": "4.9",                   /\*物流服务分\*/
		"creditLevel": "15",                   /\*店铺等级（1-20），一星 二星 三星 四星 五星 一钻 二钻 三钻 四钻 五钻 一皇冠 二皇冠 三皇冠 四皇冠 五皇冠  一金冠 二金冠 三金冠 四金冠 五金冠\*/
		"shopIcon": "//img.alicdn.com/imgextra/de/a6/TB10dC8SX.jpg",       /\*店铺logo\*/
		"pcDescContent": "1.jpg|2.jpg|3.jpg|4.jpg|5.jpg",  /\*图文详情图片地址\*/
		"item\_url": "https://item.taobao.com/item.htm?id=581801364036",  /\*商品url\*/
		"category\_id": "50006126",                   /\*叶子类目id\*/
		"category\_name": "收纳盒",                   /\*叶子类目name\*/
		"level\_one\_category\_id": "122928002",                   /\*一级类目id\*/
		"level\_one\_category\_name": "收纳整理",                   /\*一级类目name\*/
		"tkfee3": "5.07",                   /\*返佣金额\*/
		"biaoqian": ""满300元,省30元"",                   /\*店铺活动\*/
		"tag": "赶紧拯救你厨房的油腻吧😂植护🌿加厚厨房抽纸高密度纤维纸张，吸油💧很不错5包装|￥14.8元💰✔厨房小帮手，用处多多",                   /\*朋友圈文案，需要自己进行urldecode\*/
		"date\_time": "2018/11/8 14:45:22"                   /\*数据添加时间\*/
	}\]
}
```

**失败返回示例：**

var tmp = { "status": 301 };
```json
{
	"status": 301                      /\*状态\*/
}
```

[效果展示](http://cms.zhetaoke.com:10000/m/item/type.aspx?cate_id=1)

---

## 天猫商品API

**原始链接**: [天猫商品API](https://www.zhetaoke.com/user/extend/extend_lingquan_tmall.aspx)

**天猫商品API---本接口采用12台16核心64G内存服务器，江苏和北京双机房，负载均衡技术，1G独享带宽，放心调用。**

天猫商品API：返回天猫商品列表，返回佣金≥15%，动态描述分≥4.6的商品列表。

**接口地址：** https://api.zhetaoke.com:10001/api/api\_all.ashx?appkey=0a565628b2fb4712902712734cec2feb&page=1&page\_size=20&sort=new&tj=tmall

天猫商品API：返回指定查询条件的商品总数。

**接口地址：** https://api.zhetaoke.com:10001/api/api\_all.ashx?appkey=0a565628b2fb4712902712734cec2feb&page=1&page\_size=20&sort=new&tj=tmall&total\_count=1

**返回数据：**json

**请求方式：**get

**请求参数说明：**

|  | 名称 | 类型 | 必填 | 说明 |
| --- | --- | --- | --- | --- |
|   | appkey | string | 是 | 折淘客的对接秘钥appkey |
|   | page | int | 否 | 分页获取数据,第几页 |
|   | page\_size | int | 否 | 每页数据条数（默认每页20条），可自定义1-50之间 |
|   | sort | string | 否 | 商品排序方式，new：按照综合排序，  
total\_sale\_num\_asc：按照总销量从小到大排序，total\_sale\_num\_desc：按照总销量从大到小排序，  
sale\_num\_asc：按照年销量从小到大排序，sale\_num\_desc：按照年销量从大到小排序，  
commission\_rate\_asc：按照佣金比例从小到大排序，commission\_rate\_desc：按照佣金比例从大到小排序，  
price\_asc：按照价格从小到大排序，price\_desc：按照价格从大到小排序，  
coupon\_info\_money\_asc：按照优惠券金额从小到大排序，coupon\_info\_money\_desc：按照优惠券金额从大到小排序，  
shop\_level\_asc：按照店铺等级从低到高排序，shop\_level\_desc：按照店铺等级从高到低排序，  
tkfee\_asc：按照返佣金额从低到高排序，tkfee\_desc：按照返佣金额从高到低排序，  
code：按照code值从大到小排序，date\_time：按照更新时间排序，random：按照随机排序 |
|   | cid | int | 否 | 一级商品分类，值为空：全部商品，1：女装，2：母婴，3：美妆，4：居家日用，5：鞋品，6：美食，7：文娱车品，8：数码家电，9：男装，10：内衣，11：箱包，12：配饰，13：户外运动，14：家装家纺 |
|   | tj | string | 否 | 是否天猫商品，值为空：全部商品，tmall：天猫商品，gold\_seller：金牌卖家，taobao：淘宝商品 |

**成功返回示例：**/\*重要事情说三遍！重要事情说三遍！重要事情说三遍！api\_all/api\_dongdong/api\_quantian/api\_shishi/api\_videos/api\_xiaoshi/api\_yongjin接口所有参数都可以自由组合。\*/

var tmp = { "status": 200, /\*状态\*/ "content": \[{ "code": "6646", /\*折淘客编号\*/ "type\_one\_id": "1", /\*分类ID，可参考折淘客分类\*/ "tao\_id": "554832820990", /\*商品ID\*/ "title": "储物箱整理抽屉式收纳柜", /\*商品短标题\*/ "jianjie": "【爆款返场，出口日本】进口PP材质，可叠加透明可视抽屉收纳盒，分格抽屉物品分类齐全，区域分化一目了然，自由组合，大容量，让你居家生活更舒心，收纳你的生活！【赠运费险】", /\*商品简介\*/ "pict\_url": "https://img.alicdn.com/bao/......jpg", /\*商品主图\*/ "user\_type": "0", /\*是否天猫，0是淘宝，1是天猫\*/ "seller\_id": "3073204721", /\*卖家ID\*/ /\*店铺地址可自行拼接，http://store.taobao.com/shop/view\_shop.htm?user\_number\_id=3073204721 \*/ "shop\_dsr": "4.80", /\*商品描述分\*/ "volume": "169", /\*年销量\*/ "size": "26.90", /\*折扣价\*/ "quanhou\_jiage": "16.90", /\*券后价\*/ "date\_time\_yongjin": "2018/11/8 14:45:22", /\*数据更新时间\*/ "tkrate3": "30.00", /\*佣金比率\*/ "yongjin\_type": "MKT", /\*佣金类型\*/ "coupon\_id": "fe31d6c8132c462ba59bd72e03d2eae0", /\*优惠券ID 该商品如无优惠券，此字段为空\*/ "coupon\_start\_time": "2018-11-05 00:00:00", /\*优惠券开始时间 该商品如无优惠券，此字段为空\*/ "coupon\_end\_time": "2018-11-10 23:59:59", /\*优惠券结束时间 该商品如无优惠券，此字段为空\*/ "coupon\_info\_money": "10", /\*优惠券金额 该商品如无优惠券，此字段为0\*/ "coupon\_total\_count": "100000", /\*优惠券总数量 该商品如无优惠券，此字段为0\*/ "coupon\_remain\_count": "99965", /\*优惠券剩余数量 该商品如无优惠券，此字段为0\*/ "coupon\_info": "满80.00元减10元", /\*优惠券信息\*/ "juhuasuan": "0", /\*是否聚划算，1是\*/ "taoqianggou": "0", /\*是否淘抢购，1是\*/ "haitao": "0", /\*是否海淘，1是\*/ "jiyoujia": "0", /\*是否极有家，1是\*/ "jinpaimaijia": "0", /\*是否金牌卖家，1是\*/ "pinpai": "0", /\*是否精选品牌，1是\*/ "pinpai\_name": "", /\*品牌名称\*/ "yunfeixian": "0", /\*是否有运费险，1有\*/ "nick": "\*\*\*旗舰店", /\*卖家昵称\*/ "small\_images": "https://img.alicdn.com/bao/.......jpg|https://img.alicdn.com/bao/.......jpg", /\*商品小图列表\*/ "white\_image": "https://img.alicdn.com/bao/......jpg", /\*商品白底图\*/ "tao\_title": "抽屉式收纳箱塑料储物箱整理箱衣服物家用宿舍衣柜内衣内裤收纳盒", /\*商品长标题\*/ "provcity": "浙江 杭州", /\*宝贝所在地\*/ "shop\_title": "\*\*\*旗舰店", /\*店铺名称\*/ "zhibo\_url": "https://cloud.video.taobao.com/play/u/849090736/p/2/e/6/t/1/219853683676.mp4", /\*视频地址\*/ "sellCount": "860000", /\*淘宝网页实时总销量\*/ "commentCount": "250000", /\*评论数量\*/ "favcount": "180000", /\*收藏数量\*/ "score1": "4.9", /\*宝贝描述分\*/ "score2": "4.9", /\*卖家服务分\*/ "score3": "4.9", /\*物流服务\*/ "creditLevel": "15", /\*店铺等级，一星 二星 三星 四星 五星 一钻 二钻 三钻 四钻 五钻 一皇冠 二皇冠 三皇冠 四皇冠 五皇冠 一金冠 二金冠 三金冠 四金冠 五金冠\*/ "shopIcon": "//img.alicdn.com/imgextra/de/a6/TB10dC8SX.jpg", /\*店铺logo\*/ "pcDescContent": "1.jpg|2.jpg|3.jpg|4.jpg|5.jpg", /\*图文详情图片地址\*/ "item\_url": "https://item.taobao.com/item.htm?id=581801364036", /\*商品url\*/ "category\_id": "50006126", /\*叶子类目id\*/ "category\_name": "收纳盒", /\*叶子类目name\*/ "level\_one\_category\_id": "122928002", /\*一级类目id\*/ "level\_one\_category\_name": "收纳整理", /\*一级类目name\*/ "tkfee3": "5.07", /\*返佣金额\*/ "biaoqian": ""满300元,省30元"", /\*店铺活动\*/ "tag": "赶紧拯救你厨房的油腻吧😂<br>植护🌿加厚厨房抽纸<br>高密度纤维纸张，吸油💧很不错<br>5包装|￥14.8元💰<br>✔厨房小帮手，用处多多", /\*朋友圈文案，需要自己进行urldecode\*/ "date\_time": "2018/11/8 14:45:22" /\*数据添加时间\*/ }\] };
```json
{
	"status": 200,                      /\*状态\*/
	"content": \[{
		"code": "6646",                 /\*折淘客编号\*/
		"type\_one\_id": "1",             /\*分类ID，可参考折淘客分类\*/
		"tao\_id": "554832820990",       /\*商品ID\*/
		"title": "储物箱整理抽屉式收纳柜",                   /\*商品短标题\*/
		"jianjie": "【爆款返场，出口日本】进口PP材质，可叠加透明可视抽屉收纳盒，分格抽屉物品分类齐全，区域分化一目了然，自由组合，大容量，让你居家生活更舒心，收纳你的生活！【赠运费险】", /\*商品简介\*/
		"pict\_url": "https://img.alicdn.com/bao/......jpg",                   /\*商品主图\*/
		"user\_type": "0",                   /\*是否天猫，0是淘宝，1是天猫\*/
		"seller\_id": "3073204721",                   /\*卖家ID\*/ /\*店铺地址可自行拼接，http://store.taobao.com/shop/view\_shop.htm?user\_number\_id=3073204721 \*/
		"shop\_dsr": "4.80",                   /\*商品描述分\*/
		"volume": "169",                   /\*年销量\*/
		"size": "26.90",                   /\*折扣价\*/
		"quanhou\_jiage": "16.90",                   /\*券后价\*/
		"date\_time\_yongjin": "2018/11/8 14:45:22",                   /\*数据更新时间\*/
		"tkrate3": "30.00",                   /\*佣金比率\*/
		"yongjin\_type": "MKT",                   /\*佣金类型\*/
		"coupon\_id": "fe31d6c8132c462ba59bd72e03d2eae0",                   /\*优惠券ID 该商品如无优惠券，此字段为空\*/
		"coupon\_start\_time": "2018-11-05 00:00:00",                   /\*优惠券开始时间 该商品如无优惠券，此字段为空\*/
		"coupon\_end\_time": "2018-11-10 23:59:59",                   /\*优惠券结束时间 该商品如无优惠券，此字段为空\*/
		"coupon\_info\_money": "10",                   /\*优惠券金额 该商品如无优惠券，此字段为0\*/
		"coupon\_total\_count": "100000",                   /\*优惠券总数量 该商品如无优惠券，此字段为0\*/
		"coupon\_remain\_count": "99965",                   /\*优惠券剩余数量 该商品如无优惠券，此字段为0\*/
		"coupon\_info": "满80.00元减10元",                   /\*优惠券信息\*/
		"juhuasuan": "0",                       /\*是否聚划算，1是\*/
		"taoqianggou": "0",                     /\*是否淘抢购，1是\*/
		"haitao": "0",                          /\*是否海淘，1是\*/
		"jiyoujia": "0",                        /\*是否极有家，1是\*/
		"jinpaimaijia": "0",                    /\*是否金牌卖家，1是\*/
		"pinpai": "0",                          /\*是否精选品牌，1是\*/
		"pinpai\_name": "",                      /\*品牌名称\*/
		"yunfeixian": "0",                      /\*是否有运费险，1有\*/
		"nick": "\*\*\*旗舰店",                    /\*卖家昵称\*/
		"small\_images": "https://img.alicdn.com/bao/.......jpg|https://img.alicdn.com/bao/.......jpg",                   /\*商品小图列表\*/
		"white\_image": "https://img.alicdn.com/bao/......jpg",                   /\*商品白底图\*/
		"tao\_title": "抽屉式收纳箱塑料储物箱整理箱衣服物家用宿舍衣柜内衣内裤收纳盒",                   /\*商品长标题\*/
		"provcity": "浙江 杭州",                   /\*宝贝所在地\*/
		"shop\_title": "\*\*\*旗舰店",                   /\*店铺名称\*/
		"zhibo\_url": "https://cloud.video.taobao.com/play/u/849090736/p/2/e/6/t/1/219853683676.mp4",                   /\*视频地址\*/
		"sellCount": "860000",                   /\*淘宝网页实时总销量\*/
		"commentCount": "250000",                   /\*评论数量\*/
		"favcount": "180000",                   /\*收藏数量\*/
		"score1": "4.9",                   /\*宝贝描述分\*/
		"score2": "4.9",                   /\*卖家服务分\*/
		"score3": "4.9",                   /\*物流服务分\*/
		"creditLevel": "15",                   /\*店铺等级（1-20），一星 二星 三星 四星 五星 一钻 二钻 三钻 四钻 五钻 一皇冠 二皇冠 三皇冠 四皇冠 五皇冠  一金冠 二金冠 三金冠 四金冠 五金冠\*/
		"shopIcon": "//img.alicdn.com/imgextra/de/a6/TB10dC8SX.jpg",       /\*店铺logo\*/
		"pcDescContent": "1.jpg|2.jpg|3.jpg|4.jpg|5.jpg",  /\*图文详情图片地址\*/
		"item\_url": "https://item.taobao.com/item.htm?id=581801364036",  /\*商品url\*/
		"category\_id": "50006126",                   /\*叶子类目id\*/
		"category\_name": "收纳盒",                   /\*叶子类目name\*/
		"level\_one\_category\_id": "122928002",                   /\*一级类目id\*/
		"level\_one\_category\_name": "收纳整理",                   /\*一级类目name\*/
		"tkfee3": "5.07",                   /\*返佣金额\*/
		"biaoqian": ""满300元,省30元"",                   /\*店铺活动\*/
		"tag": "赶紧拯救你厨房的油腻吧😂植护🌿加厚厨房抽纸高密度纤维纸张，吸油💧很不错5包装|￥14.8元💰✔厨房小帮手，用处多多",                   /\*朋友圈文案，需要自己进行urldecode\*/
		"date\_time": "2018/11/8 14:45:22"                   /\*数据添加时间\*/
	}\]
}
```

**失败返回示例：**

var tmp = { "status": 301 };
```json
{
	"status": 301                      /\*状态\*/
}
```

[效果展示](http://cms.zhetaoke.com:10000/m/item/list.aspx?tianmao=1)

---

## 金牌卖家商品API

**原始链接**: [金牌卖家商品API](https://www.zhetaoke.com/user/extend/extend_lingquan_gold.aspx)

**金牌卖家商品API---本接口采用12台16核心64G内存服务器，江苏和北京双机房，负载均衡技术，1G独享带宽，放心调用。**

金牌卖家商品API：返回金牌卖家商品列表，返回佣金≥15%，动态描述分≥4.6的商品列表。

**接口地址：** https://api.zhetaoke.com:10001/api/api\_all.ashx?appkey=0a565628b2fb4712902712734cec2feb&page=1&page\_size=20&sort=new&tj=gold\_seller

金牌卖家商品API：返回指定查询条件的商品总数。

**接口地址：** https://api.zhetaoke.com:10001/api/api\_all.ashx?appkey=0a565628b2fb4712902712734cec2feb&page=1&page\_size=20&sort=new&tj=gold\_seller&total\_count=1

**返回数据：**json

**请求方式：**get

**请求参数说明：**

|  | 名称 | 类型 | 必填 | 说明 |
| --- | --- | --- | --- | --- |
|   | appkey | string | 是 | 折淘客的对接秘钥appkey |
|   | page | int | 否 | 分页获取数据,第几页 |
|   | page\_size | int | 否 | 每页数据条数（默认每页20条），可自定义1-50之间 |
|   | sort | string | 否 | 商品排序方式，new：按照综合排序，  
total\_sale\_num\_asc：按照总销量从小到大排序，total\_sale\_num\_desc：按照总销量从大到小排序，  
sale\_num\_asc：按照年销量从小到大排序，sale\_num\_desc：按照年销量从大到小排序，  
commission\_rate\_asc：按照佣金比例从小到大排序，commission\_rate\_desc：按照佣金比例从大到小排序，  
price\_asc：按照价格从小到大排序，price\_desc：按照价格从大到小排序，  
coupon\_info\_money\_asc：按照优惠券金额从小到大排序，coupon\_info\_money\_desc：按照优惠券金额从大到小排序，  
shop\_level\_asc：按照店铺等级从低到高排序，shop\_level\_desc：按照店铺等级从高到低排序，  
tkfee\_asc：按照返佣金额从低到高排序，tkfee\_desc：按照返佣金额从高到低排序，  
code：按照code值从大到小排序，date\_time：按照更新时间排序，random：按照随机排序 |
|   | cid | int | 否 | 一级商品分类，值为空：全部商品，1：女装，2：母婴，3：美妆，4：居家日用，5：鞋品，6：美食，7：文娱车品，8：数码家电，9：男装，10：内衣，11：箱包，12：配饰，13：户外运动，14：家装家纺 |
|   | tj | string | 否 | 是否天猫商品，值为空：全部商品，tmall：天猫商品，gold\_seller：金牌卖家，taobao：淘宝商品 |

**成功返回示例：**/\*重要事情说三遍！重要事情说三遍！重要事情说三遍！api\_all/api\_dongdong/api\_quantian/api\_shishi/api\_videos/api\_xiaoshi/api\_yongjin接口所有参数都可以自由组合。\*/

var tmp = { "status": 200, /\*状态\*/ "content": \[{ "code": "6646", /\*折淘客编号\*/ "type\_one\_id": "1", /\*分类ID，可参考折淘客分类\*/ "tao\_id": "554832820990", /\*商品ID\*/ "title": "储物箱整理抽屉式收纳柜", /\*商品短标题\*/ "jianjie": "【爆款返场，出口日本】进口PP材质，可叠加透明可视抽屉收纳盒，分格抽屉物品分类齐全，区域分化一目了然，自由组合，大容量，让你居家生活更舒心，收纳你的生活！【赠运费险】", /\*商品简介\*/ "pict\_url": "https://img.alicdn.com/bao/......jpg", /\*商品主图\*/ "user\_type": "0", /\*是否天猫，0是淘宝，1是天猫\*/ "seller\_id": "3073204721", /\*卖家ID\*/ /\*店铺地址可自行拼接，http://store.taobao.com/shop/view\_shop.htm?user\_number\_id=3073204721 \*/ "shop\_dsr": "4.80", /\*商品描述分\*/ "volume": "169", /\*年销量\*/ "size": "26.90", /\*折扣价\*/ "quanhou\_jiage": "16.90", /\*券后价\*/ "date\_time\_yongjin": "2018/11/8 14:45:22", /\*数据更新时间\*/ "tkrate3": "30.00", /\*佣金比率\*/ "yongjin\_type": "MKT", /\*佣金类型\*/ "coupon\_id": "fe31d6c8132c462ba59bd72e03d2eae0", /\*优惠券ID 该商品如无优惠券，此字段为空\*/ "coupon\_start\_time": "2018-11-05 00:00:00", /\*优惠券开始时间 该商品如无优惠券，此字段为空\*/ "coupon\_end\_time": "2018-11-10 23:59:59", /\*优惠券结束时间 该商品如无优惠券，此字段为空\*/ "coupon\_info\_money": "10", /\*优惠券金额 该商品如无优惠券，此字段为0\*/ "coupon\_total\_count": "100000", /\*优惠券总数量 该商品如无优惠券，此字段为0\*/ "coupon\_remain\_count": "99965", /\*优惠券剩余数量 该商品如无优惠券，此字段为0\*/ "coupon\_info": "满80.00元减10元", /\*优惠券信息\*/ "juhuasuan": "0", /\*是否聚划算，1是\*/ "taoqianggou": "0", /\*是否淘抢购，1是\*/ "haitao": "0", /\*是否海淘，1是\*/ "jiyoujia": "0", /\*是否极有家，1是\*/ "jinpaimaijia": "0", /\*是否金牌卖家，1是\*/ "pinpai": "0", /\*是否精选品牌，1是\*/ "pinpai\_name": "", /\*品牌名称\*/ "yunfeixian": "0", /\*是否有运费险，1有\*/ "nick": "\*\*\*旗舰店", /\*卖家昵称\*/ "small\_images": "https://img.alicdn.com/bao/.......jpg|https://img.alicdn.com/bao/.......jpg", /\*商品小图列表\*/ "white\_image": "https://img.alicdn.com/bao/......jpg", /\*商品白底图\*/ "tao\_title": "抽屉式收纳箱塑料储物箱整理箱衣服物家用宿舍衣柜内衣内裤收纳盒", /\*商品长标题\*/ "provcity": "浙江 杭州", /\*宝贝所在地\*/ "shop\_title": "\*\*\*旗舰店", /\*店铺名称\*/ "zhibo\_url": "https://cloud.video.taobao.com/play/u/849090736/p/2/e/6/t/1/219853683676.mp4", /\*视频地址\*/ "sellCount": "860000", /\*淘宝网页实时总销量\*/ "commentCount": "250000", /\*评论数量\*/ "favcount": "180000", /\*收藏数量\*/ "score1": "4.9", /\*宝贝描述分\*/ "score2": "4.9", /\*卖家服务分\*/ "score3": "4.9", /\*物流服务\*/ "creditLevel": "15", /\*店铺等级，一星 二星 三星 四星 五星 一钻 二钻 三钻 四钻 五钻 一皇冠 二皇冠 三皇冠 四皇冠 五皇冠 一金冠 二金冠 三金冠 四金冠 五金冠\*/ "shopIcon": "//img.alicdn.com/imgextra/de/a6/TB10dC8SX.jpg", /\*店铺logo\*/ "pcDescContent": "1.jpg|2.jpg|3.jpg|4.jpg|5.jpg", /\*图文详情图片地址\*/ "item\_url": "https://item.taobao.com/item.htm?id=581801364036", /\*商品url\*/ "category\_id": "50006126", /\*叶子类目id\*/ "category\_name": "收纳盒", /\*叶子类目name\*/ "level\_one\_category\_id": "122928002", /\*一级类目id\*/ "level\_one\_category\_name": "收纳整理", /\*一级类目name\*/ "tkfee3": "5.07", /\*返佣金额\*/ "biaoqian": ""满300元,省30元"", /\*店铺活动\*/ "tag": "赶紧拯救你厨房的油腻吧😂<br>植护🌿加厚厨房抽纸<br>高密度纤维纸张，吸油💧很不错<br>5包装|￥14.8元💰<br>✔厨房小帮手，用处多多", /\*朋友圈文案，需要自己进行urldecode\*/ "date\_time": "2018/11/8 14:45:22" /\*数据添加时间\*/ }\] };
```json
{
	"status": 200,                      /\*状态\*/
	"content": \[{
		"code": "6646",                 /\*折淘客编号\*/
		"type\_one\_id": "1",             /\*分类ID，可参考折淘客分类\*/
		"tao\_id": "554832820990",       /\*商品ID\*/
		"title": "储物箱整理抽屉式收纳柜",                   /\*商品短标题\*/
		"jianjie": "【爆款返场，出口日本】进口PP材质，可叠加透明可视抽屉收纳盒，分格抽屉物品分类齐全，区域分化一目了然，自由组合，大容量，让你居家生活更舒心，收纳你的生活！【赠运费险】", /\*商品简介\*/
		"pict\_url": "https://img.alicdn.com/bao/......jpg",                   /\*商品主图\*/
		"user\_type": "0",                   /\*是否天猫，0是淘宝，1是天猫\*/
		"seller\_id": "3073204721",                   /\*卖家ID\*/ /\*店铺地址可自行拼接，http://store.taobao.com/shop/view\_shop.htm?user\_number\_id=3073204721 \*/
		"shop\_dsr": "4.80",                   /\*商品描述分\*/
		"volume": "169",                   /\*年销量\*/
		"size": "26.90",                   /\*折扣价\*/
		"quanhou\_jiage": "16.90",                   /\*券后价\*/
		"date\_time\_yongjin": "2018/11/8 14:45:22",                   /\*数据更新时间\*/
		"tkrate3": "30.00",                   /\*佣金比率\*/
		"yongjin\_type": "MKT",                   /\*佣金类型\*/
		"coupon\_id": "fe31d6c8132c462ba59bd72e03d2eae0",                   /\*优惠券ID 该商品如无优惠券，此字段为空\*/
		"coupon\_start\_time": "2018-11-05 00:00:00",                   /\*优惠券开始时间 该商品如无优惠券，此字段为空\*/
		"coupon\_end\_time": "2018-11-10 23:59:59",                   /\*优惠券结束时间 该商品如无优惠券，此字段为空\*/
		"coupon\_info\_money": "10",                   /\*优惠券金额 该商品如无优惠券，此字段为0\*/
		"coupon\_total\_count": "100000",                   /\*优惠券总数量 该商品如无优惠券，此字段为0\*/
		"coupon\_remain\_count": "99965",                   /\*优惠券剩余数量 该商品如无优惠券，此字段为0\*/
		"coupon\_info": "满80.00元减10元",                   /\*优惠券信息\*/
		"juhuasuan": "0",                       /\*是否聚划算，1是\*/
		"taoqianggou": "0",                     /\*是否淘抢购，1是\*/
		"haitao": "0",                          /\*是否海淘，1是\*/
		"jiyoujia": "0",                        /\*是否极有家，1是\*/
		"jinpaimaijia": "0",                    /\*是否金牌卖家，1是\*/
		"pinpai": "0",                          /\*是否精选品牌，1是\*/
		"pinpai\_name": "",                      /\*品牌名称\*/
		"yunfeixian": "0",                      /\*是否有运费险，1有\*/
		"nick": "\*\*\*旗舰店",                    /\*卖家昵称\*/
		"small\_images": "https://img.alicdn.com/bao/.......jpg|https://img.alicdn.com/bao/.......jpg",                   /\*商品小图列表\*/
		"white\_image": "https://img.alicdn.com/bao/......jpg",                   /\*商品白底图\*/
		"tao\_title": "抽屉式收纳箱塑料储物箱整理箱衣服物家用宿舍衣柜内衣内裤收纳盒",                   /\*商品长标题\*/
		"provcity": "浙江 杭州",                   /\*宝贝所在地\*/
		"shop\_title": "\*\*\*旗舰店",                   /\*店铺名称\*/
		"zhibo\_url": "https://cloud.video.taobao.com/play/u/849090736/p/2/e/6/t/1/219853683676.mp4",                   /\*视频地址\*/
		"sellCount": "860000",                   /\*淘宝网页实时总销量\*/
		"commentCount": "250000",                   /\*评论数量\*/
		"favcount": "180000",                   /\*收藏数量\*/
		"score1": "4.9",                   /\*宝贝描述分\*/
		"score2": "4.9",                   /\*卖家服务分\*/
		"score3": "4.9",                   /\*物流服务分\*/
		"creditLevel": "15",                   /\*店铺等级（1-20），一星 二星 三星 四星 五星 一钻 二钻 三钻 四钻 五钻 一皇冠 二皇冠 三皇冠 四皇冠 五皇冠  一金冠 二金冠 三金冠 四金冠 五金冠\*/
		"shopIcon": "//img.alicdn.com/imgextra/de/a6/TB10dC8SX.jpg",       /\*店铺logo\*/
		"pcDescContent": "1.jpg|2.jpg|3.jpg|4.jpg|5.jpg",  /\*图文详情图片地址\*/
		"item\_url": "https://item.taobao.com/item.htm?id=581801364036",  /\*商品url\*/
		"category\_id": "50006126",                   /\*叶子类目id\*/
		"category\_name": "收纳盒",                   /\*叶子类目name\*/
		"level\_one\_category\_id": "122928002",                   /\*一级类目id\*/
		"level\_one\_category\_name": "收纳整理",                   /\*一级类目name\*/
		"tkfee3": "5.07",                   /\*返佣金额\*/
		"biaoqian": ""满300元,省30元"",                   /\*店铺活动\*/
		"tag": "赶紧拯救你厨房的油腻吧😂植护🌿加厚厨房抽纸高密度纤维纸张，吸油💧很不错5包装|￥14.8元💰✔厨房小帮手，用处多多",                   /\*朋友圈文案，需要自己进行urldecode\*/
		"date\_time": "2018/11/8 14:45:22"                   /\*数据添加时间\*/
	}\]
}
```

**失败返回示例：**

var tmp = { "status": 301 };
```json
{
	"status": 301                      /\*状态\*/
}
```

[效果展示](http://cms.zhetaoke.com:10000/m/item/list.aspx?jinpaimaijia=1)

---

## 淘抢购商品API

**原始链接**: [淘抢购商品API](https://www.zhetaoke.com/user/extend/extend_lingquan_taoqianggou.aspx)

**淘抢购商品API---本接口采用12台16核心64G内存服务器，江苏和北京双机房，负载均衡技术，1G独享带宽，放心调用。**

淘抢购商品API：返回淘抢购商品列表，返回佣金≥15%，动态描述分≥4.6的商品列表。

**接口地址：** https://api.zhetaoke.com:10001/api/api\_all.ashx?appkey=0a565628b2fb4712902712734cec2feb&page=1&page\_size=20&sort=new&jt=taoqianggou

淘抢购商品API：返回指定查询条件的商品总数。

**接口地址：** https://api.zhetaoke.com:10001/api/api\_all.ashx?appkey=0a565628b2fb4712902712734cec2feb&page=1&page\_size=20&sort=new&jt=taoqianggou&total\_count=1

**返回数据：**json

**请求方式：**get

**请求参数说明：**

|  | 名称 | 类型 | 必填 | 说明 |
| --- | --- | --- | --- | --- |
|   | appkey | string | 是 | 折淘客的对接秘钥appkey |
|   | page | int | 否 | 分页获取数据,第几页 |
|   | page\_size | int | 否 | 每页数据条数（默认每页20条），可自定义1-50之间 |
|   | sort | string | 否 | 商品排序方式，new：按照综合排序，  
total\_sale\_num\_asc：按照总销量从小到大排序，total\_sale\_num\_desc：按照总销量从大到小排序，  
sale\_num\_asc：按照年销量从小到大排序，sale\_num\_desc：按照年销量从大到小排序，  
commission\_rate\_asc：按照佣金比例从小到大排序，commission\_rate\_desc：按照佣金比例从大到小排序，  
price\_asc：按照价格从小到大排序，price\_desc：按照价格从大到小排序，  
coupon\_info\_money\_asc：按照优惠券金额从小到大排序，coupon\_info\_money\_desc：按照优惠券金额从大到小排序，  
shop\_level\_asc：按照店铺等级从低到高排序，shop\_level\_desc：按照店铺等级从高到低排序，  
tkfee\_asc：按照返佣金额从低到高排序，tkfee\_desc：按照返佣金额从高到低排序，  
code：按照code值从大到小排序，date\_time：按照更新时间排序，random：按照随机排序 |
|   | cid | int | 否 | 一级商品分类，值为空：全部商品，1：女装，2：母婴，3：美妆，4：居家日用，5：鞋品，6：美食，7：文娱车品，8：数码家电，9：男装，10：内衣，11：箱包，12：配饰，13：户外运动，14：家装家纺 |
|   | jt | string | 否 | 淘抢购或者聚划算，值为空：全部商品，taoqianggou：淘抢购商品，juhuasuan：聚划算商品 |

**成功返回示例：**/\*重要事情说三遍！重要事情说三遍！重要事情说三遍！api\_all/api\_dongdong/api\_quantian/api\_shishi/api\_videos/api\_xiaoshi/api\_yongjin接口所有参数都可以自由组合。\*/

var tmp = { "status": 200, /\*状态\*/ "content": \[{ "code": "6646", /\*折淘客编号\*/ "type\_one\_id": "1", /\*分类ID，可参考折淘客分类\*/ "tao\_id": "554832820990", /\*商品ID\*/ "title": "储物箱整理抽屉式收纳柜", /\*商品短标题\*/ "jianjie": "【爆款返场，出口日本】进口PP材质，可叠加透明可视抽屉收纳盒，分格抽屉物品分类齐全，区域分化一目了然，自由组合，大容量，让你居家生活更舒心，收纳你的生活！【赠运费险】", /\*商品简介\*/ "pict\_url": "https://img.alicdn.com/bao/......jpg", /\*商品主图\*/ "user\_type": "0", /\*是否天猫，0是淘宝，1是天猫\*/ "seller\_id": "3073204721", /\*卖家ID\*/ /\*店铺地址可自行拼接，http://store.taobao.com/shop/view\_shop.htm?user\_number\_id=3073204721 \*/ "shop\_dsr": "4.80", /\*商品描述分\*/ "volume": "169", /\*年销量\*/ "size": "26.90", /\*折扣价\*/ "quanhou\_jiage": "16.90", /\*券后价\*/ "date\_time\_yongjin": "2018/11/8 14:45:22", /\*数据更新时间\*/ "tkrate3": "30.00", /\*佣金比率\*/ "yongjin\_type": "MKT", /\*佣金类型\*/ "coupon\_id": "fe31d6c8132c462ba59bd72e03d2eae0", /\*优惠券ID 该商品如无优惠券，此字段为空\*/ "coupon\_start\_time": "2018-11-05 00:00:00", /\*优惠券开始时间 该商品如无优惠券，此字段为空\*/ "coupon\_end\_time": "2018-11-10 23:59:59", /\*优惠券结束时间 该商品如无优惠券，此字段为空\*/ "coupon\_info\_money": "10", /\*优惠券金额 该商品如无优惠券，此字段为0\*/ "coupon\_total\_count": "100000", /\*优惠券总数量 该商品如无优惠券，此字段为0\*/ "coupon\_remain\_count": "99965", /\*优惠券剩余数量 该商品如无优惠券，此字段为0\*/ "coupon\_info": "满80.00元减10元", /\*优惠券信息\*/ "juhuasuan": "0", /\*是否聚划算，1是\*/ "taoqianggou": "0", /\*是否淘抢购，1是\*/ "haitao": "0", /\*是否海淘，1是\*/ "jiyoujia": "0", /\*是否极有家，1是\*/ "jinpaimaijia": "0", /\*是否金牌卖家，1是\*/ "pinpai": "0", /\*是否精选品牌，1是\*/ "pinpai\_name": "", /\*品牌名称\*/ "yunfeixian": "0", /\*是否有运费险，1有\*/ "nick": "\*\*\*旗舰店", /\*卖家昵称\*/ "small\_images": "https://img.alicdn.com/bao/.......jpg|https://img.alicdn.com/bao/.......jpg", /\*商品小图列表\*/ "white\_image": "https://img.alicdn.com/bao/......jpg", /\*商品白底图\*/ "tao\_title": "抽屉式收纳箱塑料储物箱整理箱衣服物家用宿舍衣柜内衣内裤收纳盒", /\*商品长标题\*/ "provcity": "浙江 杭州", /\*宝贝所在地\*/ "shop\_title": "\*\*\*旗舰店", /\*店铺名称\*/ "zhibo\_url": "https://cloud.video.taobao.com/play/u/849090736/p/2/e/6/t/1/219853683676.mp4", /\*视频地址\*/ "sellCount": "860000", /\*淘宝网页实时总销量\*/ "commentCount": "250000", /\*评论数量\*/ "favcount": "180000", /\*收藏数量\*/ "score1": "4.9", /\*宝贝描述分\*/ "score2": "4.9", /\*卖家服务分\*/ "score3": "4.9", /\*物流服务\*/ "creditLevel": "15", /\*店铺等级，一星 二星 三星 四星 五星 一钻 二钻 三钻 四钻 五钻 一皇冠 二皇冠 三皇冠 四皇冠 五皇冠 一金冠 二金冠 三金冠 四金冠 五金冠\*/ "shopIcon": "//img.alicdn.com/imgextra/de/a6/TB10dC8SX.jpg", /\*店铺logo\*/ "pcDescContent": "1.jpg|2.jpg|3.jpg|4.jpg|5.jpg", /\*图文详情图片地址\*/ "item\_url": "https://item.taobao.com/item.htm?id=581801364036", /\*商品url\*/ "category\_id": "50006126", /\*叶子类目id\*/ "category\_name": "收纳盒", /\*叶子类目name\*/ "level\_one\_category\_id": "122928002", /\*一级类目id\*/ "level\_one\_category\_name": "收纳整理", /\*一级类目name\*/ "tkfee3": "5.07", /\*返佣金额\*/ "biaoqian": ""满300元,省30元"", /\*店铺活动\*/ "tag": "赶紧拯救你厨房的油腻吧😂<br>植护🌿加厚厨房抽纸<br>高密度纤维纸张，吸油💧很不错<br>5包装|￥14.8元💰<br>✔厨房小帮手，用处多多", /\*朋友圈文案，需要自己进行urldecode\*/ "date\_time": "2018/11/8 14:45:22" /\*数据添加时间\*/ }\] };
```json
{
	"status": 200,                      /\*状态\*/
	"content": \[{
		"code": "6646",                 /\*折淘客编号\*/
		"type\_one\_id": "1",             /\*分类ID，可参考折淘客分类\*/
		"tao\_id": "554832820990",       /\*商品ID\*/
		"title": "储物箱整理抽屉式收纳柜",                   /\*商品短标题\*/
		"jianjie": "【爆款返场，出口日本】进口PP材质，可叠加透明可视抽屉收纳盒，分格抽屉物品分类齐全，区域分化一目了然，自由组合，大容量，让你居家生活更舒心，收纳你的生活！【赠运费险】", /\*商品简介\*/
		"pict\_url": "https://img.alicdn.com/bao/......jpg",                   /\*商品主图\*/
		"user\_type": "0",                   /\*是否天猫，0是淘宝，1是天猫\*/
		"seller\_id": "3073204721",                   /\*卖家ID\*/ /\*店铺地址可自行拼接，http://store.taobao.com/shop/view\_shop.htm?user\_number\_id=3073204721 \*/
		"shop\_dsr": "4.80",                   /\*商品描述分\*/
		"volume": "169",                   /\*年销量\*/
		"size": "26.90",                   /\*折扣价\*/
		"quanhou\_jiage": "16.90",                   /\*券后价\*/
		"date\_time\_yongjin": "2018/11/8 14:45:22",                   /\*数据更新时间\*/
		"tkrate3": "30.00",                   /\*佣金比率\*/
		"yongjin\_type": "MKT",                   /\*佣金类型\*/
		"coupon\_id": "fe31d6c8132c462ba59bd72e03d2eae0",                   /\*优惠券ID 该商品如无优惠券，此字段为空\*/
		"coupon\_start\_time": "2018-11-05 00:00:00",                   /\*优惠券开始时间 该商品如无优惠券，此字段为空\*/
		"coupon\_end\_time": "2018-11-10 23:59:59",                   /\*优惠券结束时间 该商品如无优惠券，此字段为空\*/
		"coupon\_info\_money": "10",                   /\*优惠券金额 该商品如无优惠券，此字段为0\*/
		"coupon\_total\_count": "100000",                   /\*优惠券总数量 该商品如无优惠券，此字段为0\*/
		"coupon\_remain\_count": "99965",                   /\*优惠券剩余数量 该商品如无优惠券，此字段为0\*/
		"coupon\_info": "满80.00元减10元",                   /\*优惠券信息\*/
		"juhuasuan": "0",                       /\*是否聚划算，1是\*/
		"taoqianggou": "0",                     /\*是否淘抢购，1是\*/
		"haitao": "0",                          /\*是否海淘，1是\*/
		"jiyoujia": "0",                        /\*是否极有家，1是\*/
		"jinpaimaijia": "0",                    /\*是否金牌卖家，1是\*/
		"pinpai": "0",                          /\*是否精选品牌，1是\*/
		"pinpai\_name": "",                      /\*品牌名称\*/
		"yunfeixian": "0",                      /\*是否有运费险，1有\*/
		"nick": "\*\*\*旗舰店",                    /\*卖家昵称\*/
		"small\_images": "https://img.alicdn.com/bao/.......jpg|https://img.alicdn.com/bao/.......jpg",                   /\*商品小图列表\*/
		"white\_image": "https://img.alicdn.com/bao/......jpg",                   /\*商品白底图\*/
		"tao\_title": "抽屉式收纳箱塑料储物箱整理箱衣服物家用宿舍衣柜内衣内裤收纳盒",                   /\*商品长标题\*/
		"provcity": "浙江 杭州",                   /\*宝贝所在地\*/
		"shop\_title": "\*\*\*旗舰店",                   /\*店铺名称\*/
		"zhibo\_url": "https://cloud.video.taobao.com/play/u/849090736/p/2/e/6/t/1/219853683676.mp4",                   /\*视频地址\*/
		"sellCount": "860000",                   /\*淘宝网页实时总销量\*/
		"commentCount": "250000",                   /\*评论数量\*/
		"favcount": "180000",                   /\*收藏数量\*/
		"score1": "4.9",                   /\*宝贝描述分\*/
		"score2": "4.9",                   /\*卖家服务分\*/
		"score3": "4.9",                   /\*物流服务分\*/
		"creditLevel": "15",                   /\*店铺等级（1-20），一星 二星 三星 四星 五星 一钻 二钻 三钻 四钻 五钻 一皇冠 二皇冠 三皇冠 四皇冠 五皇冠  一金冠 二金冠 三金冠 四金冠 五金冠\*/
		"shopIcon": "//img.alicdn.com/imgextra/de/a6/TB10dC8SX.jpg",       /\*店铺logo\*/
		"pcDescContent": "1.jpg|2.jpg|3.jpg|4.jpg|5.jpg",  /\*图文详情图片地址\*/
		"item\_url": "https://item.taobao.com/item.htm?id=581801364036",  /\*商品url\*/
		"category\_id": "50006126",                   /\*叶子类目id\*/
		"category\_name": "收纳盒",                   /\*叶子类目name\*/
		"level\_one\_category\_id": "122928002",                   /\*一级类目id\*/
		"level\_one\_category\_name": "收纳整理",                   /\*一级类目name\*/
		"tkfee3": "5.07",                   /\*返佣金额\*/
		"biaoqian": ""满300元,省30元"",                   /\*店铺活动\*/
		"tag": "赶紧拯救你厨房的油腻吧😂植护🌿加厚厨房抽纸高密度纤维纸张，吸油💧很不错5包装|￥14.8元💰✔厨房小帮手，用处多多",                   /\*朋友圈文案，需要自己进行urldecode\*/
		"date\_time": "2018/11/8 14:45:22"                   /\*数据添加时间\*/
	}\]
}
```

**失败返回示例：**

var tmp = { "status": 301 };
```json
{
	"status": 301                      /\*状态\*/
}
```

[效果展示](http://cms.zhetaoke.com:10000/m/item/list.aspx?taoqianggou=1)

---

## 聚划算商品API

**原始链接**: [聚划算商品API](https://www.zhetaoke.com/user/extend/extend_lingquan_juhuasuan.aspx)

**聚划算商品API---本接口采用12台16核心64G内存服务器，江苏和北京双机房，负载均衡技术，1G独享带宽，放心调用。**

聚划算商品API：返回聚划算商品列表，返回佣金≥15%，动态描述分≥4.6的商品列表。

**接口地址：** https://api.zhetaoke.com:10001/api/api\_all.ashx?appkey=0a565628b2fb4712902712734cec2feb&page=1&page\_size=20&sort=new&jt=juhuasuan

聚划算商品API：返回指定查询条件的商品总数。

**接口地址：** https://api.zhetaoke.com:10001/api/api\_all.ashx?appkey=0a565628b2fb4712902712734cec2feb&page=1&page\_size=20&sort=new&jt=juhuasuan&total\_count=1

**返回数据：**json

**请求方式：**get

**请求参数说明：**

|  | 名称 | 类型 | 必填 | 说明 |
| --- | --- | --- | --- | --- |
|   | appkey | string | 是 | 折淘客的对接秘钥appkey |
|   | page | int | 否 | 分页获取数据,第几页 |
|   | page\_size | int | 否 | 每页数据条数（默认每页20条），可自定义1-50之间 |
|   | sort | string | 否 | 商品排序方式，new：按照综合排序，  
total\_sale\_num\_asc：按照总销量从小到大排序，total\_sale\_num\_desc：按照总销量从大到小排序，  
sale\_num\_asc：按照年销量从小到大排序，sale\_num\_desc：按照年销量从大到小排序，  
commission\_rate\_asc：按照佣金比例从小到大排序，commission\_rate\_desc：按照佣金比例从大到小排序，  
price\_asc：按照价格从小到大排序，price\_desc：按照价格从大到小排序，  
coupon\_info\_money\_asc：按照优惠券金额从小到大排序，coupon\_info\_money\_desc：按照优惠券金额从大到小排序，  
shop\_level\_asc：按照店铺等级从低到高排序，shop\_level\_desc：按照店铺等级从高到低排序，  
tkfee\_asc：按照返佣金额从低到高排序，tkfee\_desc：按照返佣金额从高到低排序，  
code：按照code值从大到小排序，date\_time：按照更新时间排序，random：按照随机排序 |
|   | cid | int | 否 | 一级商品分类，值为空：全部商品，1：女装，2：母婴，3：美妆，4：居家日用，5：鞋品，6：美食，7：文娱车品，8：数码家电，9：男装，10：内衣，11：箱包，12：配饰，13：户外运动，14：家装家纺 |
|   | jt | string | 否 | 淘抢购或者聚划算，值为空：全部商品，taoqianggou：淘抢购商品，juhuasuan：聚划算商品 |

**成功返回示例：**/\*重要事情说三遍！重要事情说三遍！重要事情说三遍！api\_all/api\_dongdong/api\_quantian/api\_shishi/api\_videos/api\_xiaoshi/api\_yongjin接口所有参数都可以自由组合。\*/

var tmp = { "status": 200, /\*状态\*/ "content": \[{ "code": "6646", /\*折淘客编号\*/ "type\_one\_id": "1", /\*分类ID，可参考折淘客分类\*/ "tao\_id": "554832820990", /\*商品ID\*/ "title": "储物箱整理抽屉式收纳柜", /\*商品短标题\*/ "jianjie": "【爆款返场，出口日本】进口PP材质，可叠加透明可视抽屉收纳盒，分格抽屉物品分类齐全，区域分化一目了然，自由组合，大容量，让你居家生活更舒心，收纳你的生活！【赠运费险】", /\*商品简介\*/ "pict\_url": "https://img.alicdn.com/bao/......jpg", /\*商品主图\*/ "user\_type": "0", /\*是否天猫，0是淘宝，1是天猫\*/ "seller\_id": "3073204721", /\*卖家ID\*/ /\*店铺地址可自行拼接，http://store.taobao.com/shop/view\_shop.htm?user\_number\_id=3073204721 \*/ "shop\_dsr": "4.80", /\*商品描述分\*/ "volume": "169", /\*年销量\*/ "size": "26.90", /\*折扣价\*/ "quanhou\_jiage": "16.90", /\*券后价\*/ "date\_time\_yongjin": "2018/11/8 14:45:22", /\*数据更新时间\*/ "tkrate3": "30.00", /\*佣金比率\*/ "yongjin\_type": "MKT", /\*佣金类型\*/ "coupon\_id": "fe31d6c8132c462ba59bd72e03d2eae0", /\*优惠券ID 该商品如无优惠券，此字段为空\*/ "coupon\_start\_time": "2018-11-05 00:00:00", /\*优惠券开始时间 该商品如无优惠券，此字段为空\*/ "coupon\_end\_time": "2018-11-10 23:59:59", /\*优惠券结束时间 该商品如无优惠券，此字段为空\*/ "coupon\_info\_money": "10", /\*优惠券金额 该商品如无优惠券，此字段为0\*/ "coupon\_total\_count": "100000", /\*优惠券总数量 该商品如无优惠券，此字段为0\*/ "coupon\_remain\_count": "99965", /\*优惠券剩余数量 该商品如无优惠券，此字段为0\*/ "coupon\_info": "满80.00元减10元", /\*优惠券信息\*/ "juhuasuan": "0", /\*是否聚划算，1是\*/ "taoqianggou": "0", /\*是否淘抢购，1是\*/ "haitao": "0", /\*是否海淘，1是\*/ "jiyoujia": "0", /\*是否极有家，1是\*/ "jinpaimaijia": "0", /\*是否金牌卖家，1是\*/ "pinpai": "0", /\*是否精选品牌，1是\*/ "pinpai\_name": "", /\*品牌名称\*/ "yunfeixian": "0", /\*是否有运费险，1有\*/ "nick": "\*\*\*旗舰店", /\*卖家昵称\*/ "small\_images": "https://img.alicdn.com/bao/.......jpg|https://img.alicdn.com/bao/.......jpg", /\*商品小图列表\*/ "white\_image": "https://img.alicdn.com/bao/......jpg", /\*商品白底图\*/ "tao\_title": "抽屉式收纳箱塑料储物箱整理箱衣服物家用宿舍衣柜内衣内裤收纳盒", /\*商品长标题\*/ "provcity": "浙江 杭州", /\*宝贝所在地\*/ "shop\_title": "\*\*\*旗舰店", /\*店铺名称\*/ "zhibo\_url": "https://cloud.video.taobao.com/play/u/849090736/p/2/e/6/t/1/219853683676.mp4", /\*视频地址\*/ "sellCount": "860000", /\*淘宝网页实时总销量\*/ "commentCount": "250000", /\*评论数量\*/ "favcount": "180000", /\*收藏数量\*/ "score1": "4.9", /\*宝贝描述分\*/ "score2": "4.9", /\*卖家服务分\*/ "score3": "4.9", /\*物流服务\*/ "creditLevel": "15", /\*店铺等级，一星 二星 三星 四星 五星 一钻 二钻 三钻 四钻 五钻 一皇冠 二皇冠 三皇冠 四皇冠 五皇冠 一金冠 二金冠 三金冠 四金冠 五金冠\*/ "shopIcon": "//img.alicdn.com/imgextra/de/a6/TB10dC8SX.jpg", /\*店铺logo\*/ "pcDescContent": "1.jpg|2.jpg|3.jpg|4.jpg|5.jpg", /\*图文详情图片地址\*/ "item\_url": "https://item.taobao.com/item.htm?id=581801364036", /\*商品url\*/ "category\_id": "50006126", /\*叶子类目id\*/ "category\_name": "收纳盒", /\*叶子类目name\*/ "level\_one\_category\_id": "122928002", /\*一级类目id\*/ "level\_one\_category\_name": "收纳整理", /\*一级类目name\*/ "tkfee3": "5.07", /\*返佣金额\*/ "biaoqian": ""满300元,省30元"", /\*店铺活动\*/ "tag": "赶紧拯救你厨房的油腻吧😂<br>植护🌿加厚厨房抽纸<br>高密度纤维纸张，吸油💧很不错<br>5包装|￥14.8元💰<br>✔厨房小帮手，用处多多", /\*朋友圈文案，需要自己进行urldecode\*/ "date\_time": "2018/11/8 14:45:22" /\*数据添加时间\*/ }\] };
```json
{
	"status": 200,                      /\*状态\*/
	"content": \[{
		"code": "6646",                 /\*折淘客编号\*/
		"type\_one\_id": "1",             /\*分类ID，可参考折淘客分类\*/
		"tao\_id": "554832820990",       /\*商品ID\*/
		"title": "储物箱整理抽屉式收纳柜",                   /\*商品短标题\*/
		"jianjie": "【爆款返场，出口日本】进口PP材质，可叠加透明可视抽屉收纳盒，分格抽屉物品分类齐全，区域分化一目了然，自由组合，大容量，让你居家生活更舒心，收纳你的生活！【赠运费险】", /\*商品简介\*/
		"pict\_url": "https://img.alicdn.com/bao/......jpg",                   /\*商品主图\*/
		"user\_type": "0",                   /\*是否天猫，0是淘宝，1是天猫\*/
		"seller\_id": "3073204721",                   /\*卖家ID\*/ /\*店铺地址可自行拼接，http://store.taobao.com/shop/view\_shop.htm?user\_number\_id=3073204721 \*/
		"shop\_dsr": "4.80",                   /\*商品描述分\*/
		"volume": "169",                   /\*年销量\*/
		"size": "26.90",                   /\*折扣价\*/
		"quanhou\_jiage": "16.90",                   /\*券后价\*/
		"date\_time\_yongjin": "2018/11/8 14:45:22",                   /\*数据更新时间\*/
		"tkrate3": "30.00",                   /\*佣金比率\*/
		"yongjin\_type": "MKT",                   /\*佣金类型\*/
		"coupon\_id": "fe31d6c8132c462ba59bd72e03d2eae0",                   /\*优惠券ID 该商品如无优惠券，此字段为空\*/
		"coupon\_start\_time": "2018-11-05 00:00:00",                   /\*优惠券开始时间 该商品如无优惠券，此字段为空\*/
		"coupon\_end\_time": "2018-11-10 23:59:59",                   /\*优惠券结束时间 该商品如无优惠券，此字段为空\*/
		"coupon\_info\_money": "10",                   /\*优惠券金额 该商品如无优惠券，此字段为0\*/
		"coupon\_total\_count": "100000",                   /\*优惠券总数量 该商品如无优惠券，此字段为0\*/
		"coupon\_remain\_count": "99965",                   /\*优惠券剩余数量 该商品如无优惠券，此字段为0\*/
		"coupon\_info": "满80.00元减10元",                   /\*优惠券信息\*/
		"juhuasuan": "0",                       /\*是否聚划算，1是\*/
		"taoqianggou": "0",                     /\*是否淘抢购，1是\*/
		"haitao": "0",                          /\*是否海淘，1是\*/
		"jiyoujia": "0",                        /\*是否极有家，1是\*/
		"jinpaimaijia": "0",                    /\*是否金牌卖家，1是\*/
		"pinpai": "0",                          /\*是否精选品牌，1是\*/
		"pinpai\_name": "",                      /\*品牌名称\*/
		"yunfeixian": "0",                      /\*是否有运费险，1有\*/
		"nick": "\*\*\*旗舰店",                    /\*卖家昵称\*/
		"small\_images": "https://img.alicdn.com/bao/.......jpg|https://img.alicdn.com/bao/.......jpg",                   /\*商品小图列表\*/
		"white\_image": "https://img.alicdn.com/bao/......jpg",                   /\*商品白底图\*/
		"tao\_title": "抽屉式收纳箱塑料储物箱整理箱衣服物家用宿舍衣柜内衣内裤收纳盒",                   /\*商品长标题\*/
		"provcity": "浙江 杭州",                   /\*宝贝所在地\*/
		"shop\_title": "\*\*\*旗舰店",                   /\*店铺名称\*/
		"zhibo\_url": "https://cloud.video.taobao.com/play/u/849090736/p/2/e/6/t/1/219853683676.mp4",                   /\*视频地址\*/
		"sellCount": "860000",                   /\*淘宝网页实时总销量\*/
		"commentCount": "250000",                   /\*评论数量\*/
		"favcount": "180000",                   /\*收藏数量\*/
		"score1": "4.9",                   /\*宝贝描述分\*/
		"score2": "4.9",                   /\*卖家服务分\*/
		"score3": "4.9",                   /\*物流服务分\*/
		"creditLevel": "15",                   /\*店铺等级（1-20），一星 二星 三星 四星 五星 一钻 二钻 三钻 四钻 五钻 一皇冠 二皇冠 三皇冠 四皇冠 五皇冠  一金冠 二金冠 三金冠 四金冠 五金冠\*/
		"shopIcon": "//img.alicdn.com/imgextra/de/a6/TB10dC8SX.jpg",       /\*店铺logo\*/
		"pcDescContent": "1.jpg|2.jpg|3.jpg|4.jpg|5.jpg",  /\*图文详情图片地址\*/
		"item\_url": "https://item.taobao.com/item.htm?id=581801364036",  /\*商品url\*/
		"category\_id": "50006126",                   /\*叶子类目id\*/
		"category\_name": "收纳盒",                   /\*叶子类目name\*/
		"level\_one\_category\_id": "122928002",                   /\*一级类目id\*/
		"level\_one\_category\_name": "收纳整理",                   /\*一级类目name\*/
		"tkfee3": "5.07",                   /\*返佣金额\*/
		"biaoqian": ""满300元,省30元"",                   /\*店铺活动\*/
		"tag": "赶紧拯救你厨房的油腻吧😂植护🌿加厚厨房抽纸高密度纤维纸张，吸油💧很不错5包装|￥14.8元💰✔厨房小帮手，用处多多",                   /\*朋友圈文案，需要自己进行urldecode\*/
		"date\_time": "2018/11/8 14:45:22"                   /\*数据添加时间\*/
	}\]
}
```

**失败返回示例：**

var tmp = { "status": 301 };
```json
{
	"status": 301                      /\*状态\*/
}
```

[效果展示](http://cms.zhetaoke.com:10000/m/item/list.aspx?juhuasuan=1)

---

## 海淘(天猫国际)商品API

**原始链接**: [海淘(天猫国际)商品API](https://www.zhetaoke.com/user/extend/extend_lingquan_haitao.aspx)

**海淘商品API（天猫国际API）---本接口采用12台16核心64G内存服务器，江苏和北京双机房，负载均衡技术，1G独享带宽，放心调用。**

海淘商品API（天猫国际API）：返回海淘商品列表，返回佣金≥15%，动态描述分≥4.6的商品列表。

**接口地址：** https://api.zhetaoke.com:10001/api/api\_all.ashx?appkey=0a565628b2fb4712902712734cec2feb&page=1&page\_size=20&sort=new&jh=haitao

海淘商品API（天猫国际API）：返回指定查询条件的商品总数。

**接口地址：** https://api.zhetaoke.com:10001/api/api\_all.ashx?appkey=0a565628b2fb4712902712734cec2feb&page=1&page\_size=20&sort=new&jh=haitao&total\_count=1

**返回数据：**json

**请求方式：**get

**请求参数说明：**

|  | 名称 | 类型 | 必填 | 说明 |
| --- | --- | --- | --- | --- |
|   | appkey | string | 是 | 折淘客的对接秘钥appkey |
|   | page | int | 否 | 分页获取数据,第几页 |
|   | page\_size | int | 否 | 每页数据条数（默认每页20条），可自定义1-50之间 |
|   | sort | string | 否 | 商品排序方式，new：按照综合排序，  
total\_sale\_num\_asc：按照总销量从小到大排序，total\_sale\_num\_desc：按照总销量从大到小排序，  
sale\_num\_asc：按照年销量从小到大排序，sale\_num\_desc：按照年销量从大到小排序，  
commission\_rate\_asc：按照佣金比例从小到大排序，commission\_rate\_desc：按照佣金比例从大到小排序，  
price\_asc：按照价格从小到大排序，price\_desc：按照价格从大到小排序，  
coupon\_info\_money\_asc：按照优惠券金额从小到大排序，coupon\_info\_money\_desc：按照优惠券金额从大到小排序，  
shop\_level\_asc：按照店铺等级从低到高排序，shop\_level\_desc：按照店铺等级从高到低排序，  
tkfee\_asc：按照返佣金额从低到高排序，tkfee\_desc：按照返佣金额从高到低排序，  
code：按照code值从大到小排序，date\_time：按照更新时间排序，random：按照随机排序 |
|   | cid | int | 否 | 一级商品分类，值为空：全部商品，1：女装，2：母婴，3：美妆，4：居家日用，5：鞋品，6：美食，7：文娱车品，8：数码家电，9：男装，10：内衣，11：箱包，12：配饰，13：户外运动，14：家装家纺 |
|   | jh | string | 否 | 海淘或者极有家，值为空：全部商品，haitao：海淘商品，jiyoujia：极有家商品 |

**成功返回示例：**/\*重要事情说三遍！重要事情说三遍！重要事情说三遍！api\_all/api\_dongdong/api\_quantian/api\_shishi/api\_videos/api\_xiaoshi/api\_yongjin接口所有参数都可以自由组合。\*/

var tmp = { "status": 200, /\*状态\*/ "content": \[{ "code": "6646", /\*折淘客编号\*/ "type\_one\_id": "1", /\*分类ID，可参考折淘客分类\*/ "tao\_id": "554832820990", /\*商品ID\*/ "title": "储物箱整理抽屉式收纳柜", /\*商品短标题\*/ "jianjie": "【爆款返场，出口日本】进口PP材质，可叠加透明可视抽屉收纳盒，分格抽屉物品分类齐全，区域分化一目了然，自由组合，大容量，让你居家生活更舒心，收纳你的生活！【赠运费险】", /\*商品简介\*/ "pict\_url": "https://img.alicdn.com/bao/......jpg", /\*商品主图\*/ "user\_type": "0", /\*是否天猫，0是淘宝，1是天猫\*/ "seller\_id": "3073204721", /\*卖家ID\*/ /\*店铺地址可自行拼接，http://store.taobao.com/shop/view\_shop.htm?user\_number\_id=3073204721 \*/ "shop\_dsr": "4.80", /\*商品描述分\*/ "volume": "169", /\*年销量\*/ "size": "26.90", /\*折扣价\*/ "quanhou\_jiage": "16.90", /\*券后价\*/ "date\_time\_yongjin": "2018/11/8 14:45:22", /\*数据更新时间\*/ "tkrate3": "30.00", /\*佣金比率\*/ "yongjin\_type": "MKT", /\*佣金类型\*/ "coupon\_id": "fe31d6c8132c462ba59bd72e03d2eae0", /\*优惠券ID 该商品如无优惠券，此字段为空\*/ "coupon\_start\_time": "2018-11-05 00:00:00", /\*优惠券开始时间 该商品如无优惠券，此字段为空\*/ "coupon\_end\_time": "2018-11-10 23:59:59", /\*优惠券结束时间 该商品如无优惠券，此字段为空\*/ "coupon\_info\_money": "10", /\*优惠券金额 该商品如无优惠券，此字段为0\*/ "coupon\_total\_count": "100000", /\*优惠券总数量 该商品如无优惠券，此字段为0\*/ "coupon\_remain\_count": "99965", /\*优惠券剩余数量 该商品如无优惠券，此字段为0\*/ "coupon\_info": "满80.00元减10元", /\*优惠券信息\*/ "juhuasuan": "0", /\*是否聚划算，1是\*/ "taoqianggou": "0", /\*是否淘抢购，1是\*/ "haitao": "0", /\*是否海淘，1是\*/ "jiyoujia": "0", /\*是否极有家，1是\*/ "jinpaimaijia": "0", /\*是否金牌卖家，1是\*/ "pinpai": "0", /\*是否精选品牌，1是\*/ "pinpai\_name": "", /\*品牌名称\*/ "yunfeixian": "0", /\*是否有运费险，1有\*/ "nick": "\*\*\*旗舰店", /\*卖家昵称\*/ "small\_images": "https://img.alicdn.com/bao/.......jpg|https://img.alicdn.com/bao/.......jpg", /\*商品小图列表\*/ "white\_image": "https://img.alicdn.com/bao/......jpg", /\*商品白底图\*/ "tao\_title": "抽屉式收纳箱塑料储物箱整理箱衣服物家用宿舍衣柜内衣内裤收纳盒", /\*商品长标题\*/ "provcity": "浙江 杭州", /\*宝贝所在地\*/ "shop\_title": "\*\*\*旗舰店", /\*店铺名称\*/ "zhibo\_url": "https://cloud.video.taobao.com/play/u/849090736/p/2/e/6/t/1/219853683676.mp4", /\*视频地址\*/ "sellCount": "860000", /\*淘宝网页实时总销量\*/ "commentCount": "250000", /\*评论数量\*/ "favcount": "180000", /\*收藏数量\*/ "score1": "4.9", /\*宝贝描述分\*/ "score2": "4.9", /\*卖家服务分\*/ "score3": "4.9", /\*物流服务\*/ "creditLevel": "15", /\*店铺等级，一星 二星 三星 四星 五星 一钻 二钻 三钻 四钻 五钻 一皇冠 二皇冠 三皇冠 四皇冠 五皇冠 一金冠 二金冠 三金冠 四金冠 五金冠\*/ "shopIcon": "//img.alicdn.com/imgextra/de/a6/TB10dC8SX.jpg", /\*店铺logo\*/ "pcDescContent": "1.jpg|2.jpg|3.jpg|4.jpg|5.jpg", /\*图文详情图片地址\*/ "item\_url": "https://item.taobao.com/item.htm?id=581801364036", /\*商品url\*/ "category\_id": "50006126", /\*叶子类目id\*/ "category\_name": "收纳盒", /\*叶子类目name\*/ "level\_one\_category\_id": "122928002", /\*一级类目id\*/ "level\_one\_category\_name": "收纳整理", /\*一级类目name\*/ "tkfee3": "5.07", /\*返佣金额\*/ "biaoqian": ""满300元,省30元"", /\*店铺活动\*/ "tag": "赶紧拯救你厨房的油腻吧😂<br>植护🌿加厚厨房抽纸<br>高密度纤维纸张，吸油💧很不错<br>5包装|￥14.8元💰<br>✔厨房小帮手，用处多多", /\*朋友圈文案，需要自己进行urldecode\*/ "date\_time": "2018/11/8 14:45:22" /\*数据添加时间\*/ }\] };
```json
{
	"status": 200,                      /\*状态\*/
	"content": \[{
		"code": "6646",                 /\*折淘客编号\*/
		"type\_one\_id": "1",             /\*分类ID，可参考折淘客分类\*/
		"tao\_id": "554832820990",       /\*商品ID\*/
		"title": "储物箱整理抽屉式收纳柜",                   /\*商品短标题\*/
		"jianjie": "【爆款返场，出口日本】进口PP材质，可叠加透明可视抽屉收纳盒，分格抽屉物品分类齐全，区域分化一目了然，自由组合，大容量，让你居家生活更舒心，收纳你的生活！【赠运费险】", /\*商品简介\*/
		"pict\_url": "https://img.alicdn.com/bao/......jpg",                   /\*商品主图\*/
		"user\_type": "0",                   /\*是否天猫，0是淘宝，1是天猫\*/
		"seller\_id": "3073204721",                   /\*卖家ID\*/ /\*店铺地址可自行拼接，http://store.taobao.com/shop/view\_shop.htm?user\_number\_id=3073204721 \*/
		"shop\_dsr": "4.80",                   /\*商品描述分\*/
		"volume": "169",                   /\*年销量\*/
		"size": "26.90",                   /\*折扣价\*/
		"quanhou\_jiage": "16.90",                   /\*券后价\*/
		"date\_time\_yongjin": "2018/11/8 14:45:22",                   /\*数据更新时间\*/
		"tkrate3": "30.00",                   /\*佣金比率\*/
		"yongjin\_type": "MKT",                   /\*佣金类型\*/
		"coupon\_id": "fe31d6c8132c462ba59bd72e03d2eae0",                   /\*优惠券ID 该商品如无优惠券，此字段为空\*/
		"coupon\_start\_time": "2018-11-05 00:00:00",                   /\*优惠券开始时间 该商品如无优惠券，此字段为空\*/
		"coupon\_end\_time": "2018-11-10 23:59:59",                   /\*优惠券结束时间 该商品如无优惠券，此字段为空\*/
		"coupon\_info\_money": "10",                   /\*优惠券金额 该商品如无优惠券，此字段为0\*/
		"coupon\_total\_count": "100000",                   /\*优惠券总数量 该商品如无优惠券，此字段为0\*/
		"coupon\_remain\_count": "99965",                   /\*优惠券剩余数量 该商品如无优惠券，此字段为0\*/
		"coupon\_info": "满80.00元减10元",                   /\*优惠券信息\*/
		"juhuasuan": "0",                       /\*是否聚划算，1是\*/
		"taoqianggou": "0",                     /\*是否淘抢购，1是\*/
		"haitao": "0",                          /\*是否海淘，1是\*/
		"jiyoujia": "0",                        /\*是否极有家，1是\*/
		"jinpaimaijia": "0",                    /\*是否金牌卖家，1是\*/
		"pinpai": "0",                          /\*是否精选品牌，1是\*/
		"pinpai\_name": "",                      /\*品牌名称\*/
		"yunfeixian": "0",                      /\*是否有运费险，1有\*/
		"nick": "\*\*\*旗舰店",                    /\*卖家昵称\*/
		"small\_images": "https://img.alicdn.com/bao/.......jpg|https://img.alicdn.com/bao/.......jpg",                   /\*商品小图列表\*/
		"white\_image": "https://img.alicdn.com/bao/......jpg",                   /\*商品白底图\*/
		"tao\_title": "抽屉式收纳箱塑料储物箱整理箱衣服物家用宿舍衣柜内衣内裤收纳盒",                   /\*商品长标题\*/
		"provcity": "浙江 杭州",                   /\*宝贝所在地\*/
		"shop\_title": "\*\*\*旗舰店",                   /\*店铺名称\*/
		"zhibo\_url": "https://cloud.video.taobao.com/play/u/849090736/p/2/e/6/t/1/219853683676.mp4",                   /\*视频地址\*/
		"sellCount": "860000",                   /\*淘宝网页实时总销量\*/
		"commentCount": "250000",                   /\*评论数量\*/
		"favcount": "180000",                   /\*收藏数量\*/
		"score1": "4.9",                   /\*宝贝描述分\*/
		"score2": "4.9",                   /\*卖家服务分\*/
		"score3": "4.9",                   /\*物流服务分\*/
		"creditLevel": "15",                   /\*店铺等级（1-20），一星 二星 三星 四星 五星 一钻 二钻 三钻 四钻 五钻 一皇冠 二皇冠 三皇冠 四皇冠 五皇冠  一金冠 二金冠 三金冠 四金冠 五金冠\*/
		"shopIcon": "//img.alicdn.com/imgextra/de/a6/TB10dC8SX.jpg",       /\*店铺logo\*/
		"pcDescContent": "1.jpg|2.jpg|3.jpg|4.jpg|5.jpg",  /\*图文详情图片地址\*/
		"item\_url": "https://item.taobao.com/item.htm?id=581801364036",  /\*商品url\*/
		"category\_id": "50006126",                   /\*叶子类目id\*/
		"category\_name": "收纳盒",                   /\*叶子类目name\*/
		"level\_one\_category\_id": "122928002",                   /\*一级类目id\*/
		"level\_one\_category\_name": "收纳整理",                   /\*一级类目name\*/
		"tkfee3": "5.07",                   /\*返佣金额\*/
		"biaoqian": ""满300元,省30元"",                   /\*店铺活动\*/
		"tag": "赶紧拯救你厨房的油腻吧😂植护🌿加厚厨房抽纸高密度纤维纸张，吸油💧很不错5包装|￥14.8元💰✔厨房小帮手，用处多多",                   /\*朋友圈文案，需要自己进行urldecode\*/
		"date\_time": "2018/11/8 14:45:22"                   /\*数据添加时间\*/
	}\]
}
```

**失败返回示例：**

var tmp = { "status": 301 };
```json
{
	"status": 301                      /\*状态\*/
}
```

[效果展示](http://cms.zhetaoke.com:10000/m/item/list.aspx?haitao=1)

---

## 极有家商品API

**原始链接**: [极有家商品API](https://www.zhetaoke.com/user/extend/extend_lingquan_jiyoujia.aspx)

**极友家商品API---本接口采用12台16核心64G内存服务器，江苏和北京双机房，负载均衡技术，1G独享带宽，放心调用。**

极有家商品API：返回极有家商品列表，返回佣金≥15%，动态描述分≥4.6的商品列表。

**接口地址：** https://api.zhetaoke.com:10001/api/api\_all.ashx?appkey=0a565628b2fb4712902712734cec2feb&page=1&page\_size=20&sort=new&jh=jiyoujia

极有家商品API：返回指定查询条件的商品总数。

**接口地址：** https://api.zhetaoke.com:10001/api/api\_all.ashx?appkey=0a565628b2fb4712902712734cec2feb&page=1&page\_size=20&sort=new&jh=jiyoujia&total\_count=1

**返回数据：**json

**请求方式：**get

**请求参数说明：**

|  | 名称 | 类型 | 必填 | 说明 |
| --- | --- | --- | --- | --- |
|   | appkey | string | 是 | 折淘客的对接秘钥appkey |
|   | page | int | 否 | 分页获取数据,第几页 |
|   | page\_size | int | 否 | 每页数据条数（默认每页20条），可自定义1-50之间 |
|   | sort | string | 否 | 商品排序方式，new：按照综合排序，  
total\_sale\_num\_asc：按照总销量从小到大排序，total\_sale\_num\_desc：按照总销量从大到小排序，  
sale\_num\_asc：按照年销量从小到大排序，sale\_num\_desc：按照年销量从大到小排序，  
commission\_rate\_asc：按照佣金比例从小到大排序，commission\_rate\_desc：按照佣金比例从大到小排序，  
price\_asc：按照价格从小到大排序，price\_desc：按照价格从大到小排序，  
coupon\_info\_money\_asc：按照优惠券金额从小到大排序，coupon\_info\_money\_desc：按照优惠券金额从大到小排序，  
shop\_level\_asc：按照店铺等级从低到高排序，shop\_level\_desc：按照店铺等级从高到低排序，  
tkfee\_asc：按照返佣金额从低到高排序，tkfee\_desc：按照返佣金额从高到低排序，  
code：按照code值从大到小排序，date\_time：按照更新时间排序，random：按照随机排序 |
|   | jh | string | 否 | 海淘或者极有家，值为空：全部商品，haitao：海淘商品，jiyoujia：极有家商品 |

**成功返回示例：**/\*重要事情说三遍！重要事情说三遍！重要事情说三遍！api\_all/api\_dongdong/api\_quantian/api\_shishi/api\_videos/api\_xiaoshi/api\_yongjin接口所有参数都可以自由组合。\*/

var tmp = { "status": 200, /\*状态\*/ "content": \[{ "code": "6646", /\*折淘客编号\*/ "type\_one\_id": "1", /\*分类ID，可参考折淘客分类\*/ "tao\_id": "554832820990", /\*商品ID\*/ "title": "储物箱整理抽屉式收纳柜", /\*商品短标题\*/ "jianjie": "【爆款返场，出口日本】进口PP材质，可叠加透明可视抽屉收纳盒，分格抽屉物品分类齐全，区域分化一目了然，自由组合，大容量，让你居家生活更舒心，收纳你的生活！【赠运费险】", /\*商品简介\*/ "pict\_url": "https://img.alicdn.com/bao/......jpg", /\*商品主图\*/ "user\_type": "0", /\*是否天猫，0是淘宝，1是天猫\*/ "seller\_id": "3073204721", /\*卖家ID\*/ /\*店铺地址可自行拼接，http://store.taobao.com/shop/view\_shop.htm?user\_number\_id=3073204721 \*/ "shop\_dsr": "4.80", /\*商品描述分\*/ "volume": "169", /\*年销量\*/ "size": "26.90", /\*折扣价\*/ "quanhou\_jiage": "16.90", /\*券后价\*/ "date\_time\_yongjin": "2018/11/8 14:45:22", /\*数据更新时间\*/ "tkrate3": "30.00", /\*佣金比率\*/ "yongjin\_type": "MKT", /\*佣金类型\*/ "coupon\_id": "fe31d6c8132c462ba59bd72e03d2eae0", /\*优惠券ID 该商品如无优惠券，此字段为空\*/ "coupon\_start\_time": "2018-11-05 00:00:00", /\*优惠券开始时间 该商品如无优惠券，此字段为空\*/ "coupon\_end\_time": "2018-11-10 23:59:59", /\*优惠券结束时间 该商品如无优惠券，此字段为空\*/ "coupon\_info\_money": "10", /\*优惠券金额 该商品如无优惠券，此字段为0\*/ "coupon\_total\_count": "100000", /\*优惠券总数量 该商品如无优惠券，此字段为0\*/ "coupon\_remain\_count": "99965", /\*优惠券剩余数量 该商品如无优惠券，此字段为0\*/ "coupon\_info": "满80.00元减10元", /\*优惠券信息\*/ "juhuasuan": "0", /\*是否聚划算，1是\*/ "taoqianggou": "0", /\*是否淘抢购，1是\*/ "haitao": "0", /\*是否海淘，1是\*/ "jiyoujia": "0", /\*是否极有家，1是\*/ "jinpaimaijia": "0", /\*是否金牌卖家，1是\*/ "pinpai": "0", /\*是否精选品牌，1是\*/ "pinpai\_name": "", /\*品牌名称\*/ "yunfeixian": "0", /\*是否有运费险，1有\*/ "nick": "\*\*\*旗舰店", /\*卖家昵称\*/ "small\_images": "https://img.alicdn.com/bao/.......jpg|https://img.alicdn.com/bao/.......jpg", /\*商品小图列表\*/ "white\_image": "https://img.alicdn.com/bao/......jpg", /\*商品白底图\*/ "tao\_title": "抽屉式收纳箱塑料储物箱整理箱衣服物家用宿舍衣柜内衣内裤收纳盒", /\*商品长标题\*/ "provcity": "浙江 杭州", /\*宝贝所在地\*/ "shop\_title": "\*\*\*旗舰店", /\*店铺名称\*/ "zhibo\_url": "https://cloud.video.taobao.com/play/u/849090736/p/2/e/6/t/1/219853683676.mp4", /\*视频地址\*/ "sellCount": "860000", /\*淘宝网页实时总销量\*/ "commentCount": "250000", /\*评论数量\*/ "favcount": "180000", /\*收藏数量\*/ "score1": "4.9", /\*宝贝描述分\*/ "score2": "4.9", /\*卖家服务分\*/ "score3": "4.9", /\*物流服务\*/ "creditLevel": "15", /\*店铺等级，一星 二星 三星 四星 五星 一钻 二钻 三钻 四钻 五钻 一皇冠 二皇冠 三皇冠 四皇冠 五皇冠 一金冠 二金冠 三金冠 四金冠 五金冠\*/ "shopIcon": "//img.alicdn.com/imgextra/de/a6/TB10dC8SX.jpg", /\*店铺logo\*/ "pcDescContent": "1.jpg|2.jpg|3.jpg|4.jpg|5.jpg", /\*图文详情图片地址\*/ "item\_url": "https://item.taobao.com/item.htm?id=581801364036", /\*商品url\*/ "category\_id": "50006126", /\*叶子类目id\*/ "category\_name": "收纳盒", /\*叶子类目name\*/ "level\_one\_category\_id": "122928002", /\*一级类目id\*/ "level\_one\_category\_name": "收纳整理", /\*一级类目name\*/ "tkfee3": "5.07", /\*返佣金额\*/ "biaoqian": ""满300元,省30元"", /\*店铺活动\*/ "tag": "赶紧拯救你厨房的油腻吧😂<br>植护🌿加厚厨房抽纸<br>高密度纤维纸张，吸油💧很不错<br>5包装|￥14.8元💰<br>✔厨房小帮手，用处多多", /\*朋友圈文案，需要自己进行urldecode\*/ "date\_time": "2018/11/8 14:45:22" /\*数据添加时间\*/ }\] };
```json
{
	"status": 200,                      /\*状态\*/
	"content": \[{
		"code": "6646",                 /\*折淘客编号\*/
		"type\_one\_id": "1",             /\*分类ID，可参考折淘客分类\*/
		"tao\_id": "554832820990",       /\*商品ID\*/
		"title": "储物箱整理抽屉式收纳柜",                   /\*商品短标题\*/
		"jianjie": "【爆款返场，出口日本】进口PP材质，可叠加透明可视抽屉收纳盒，分格抽屉物品分类齐全，区域分化一目了然，自由组合，大容量，让你居家生活更舒心，收纳你的生活！【赠运费险】", /\*商品简介\*/
		"pict\_url": "https://img.alicdn.com/bao/......jpg",                   /\*商品主图\*/
		"user\_type": "0",                   /\*是否天猫，0是淘宝，1是天猫\*/
		"seller\_id": "3073204721",                   /\*卖家ID\*/ /\*店铺地址可自行拼接，http://store.taobao.com/shop/view\_shop.htm?user\_number\_id=3073204721 \*/
		"shop\_dsr": "4.80",                   /\*商品描述分\*/
		"volume": "169",                   /\*年销量\*/
		"size": "26.90",                   /\*折扣价\*/
		"quanhou\_jiage": "16.90",                   /\*券后价\*/
		"date\_time\_yongjin": "2018/11/8 14:45:22",                   /\*数据更新时间\*/
		"tkrate3": "30.00",                   /\*佣金比率\*/
		"yongjin\_type": "MKT",                   /\*佣金类型\*/
		"coupon\_id": "fe31d6c8132c462ba59bd72e03d2eae0",                   /\*优惠券ID 该商品如无优惠券，此字段为空\*/
		"coupon\_start\_time": "2018-11-05 00:00:00",                   /\*优惠券开始时间 该商品如无优惠券，此字段为空\*/
		"coupon\_end\_time": "2018-11-10 23:59:59",                   /\*优惠券结束时间 该商品如无优惠券，此字段为空\*/
		"coupon\_info\_money": "10",                   /\*优惠券金额 该商品如无优惠券，此字段为0\*/
		"coupon\_total\_count": "100000",                   /\*优惠券总数量 该商品如无优惠券，此字段为0\*/
		"coupon\_remain\_count": "99965",                   /\*优惠券剩余数量 该商品如无优惠券，此字段为0\*/
		"coupon\_info": "满80.00元减10元",                   /\*优惠券信息\*/
		"juhuasuan": "0",                       /\*是否聚划算，1是\*/
		"taoqianggou": "0",                     /\*是否淘抢购，1是\*/
		"haitao": "0",                          /\*是否海淘，1是\*/
		"jiyoujia": "0",                        /\*是否极有家，1是\*/
		"jinpaimaijia": "0",                    /\*是否金牌卖家，1是\*/
		"pinpai": "0",                          /\*是否精选品牌，1是\*/
		"pinpai\_name": "",                      /\*品牌名称\*/
		"yunfeixian": "0",                      /\*是否有运费险，1有\*/
		"nick": "\*\*\*旗舰店",                    /\*卖家昵称\*/
		"small\_images": "https://img.alicdn.com/bao/.......jpg|https://img.alicdn.com/bao/.......jpg",                   /\*商品小图列表\*/
		"white\_image": "https://img.alicdn.com/bao/......jpg",                   /\*商品白底图\*/
		"tao\_title": "抽屉式收纳箱塑料储物箱整理箱衣服物家用宿舍衣柜内衣内裤收纳盒",                   /\*商品长标题\*/
		"provcity": "浙江 杭州",                   /\*宝贝所在地\*/
		"shop\_title": "\*\*\*旗舰店",                   /\*店铺名称\*/
		"zhibo\_url": "https://cloud.video.taobao.com/play/u/849090736/p/2/e/6/t/1/219853683676.mp4",                   /\*视频地址\*/
		"sellCount": "860000",                   /\*淘宝网页实时总销量\*/
		"commentCount": "250000",                   /\*评论数量\*/
		"favcount": "180000",                   /\*收藏数量\*/
		"score1": "4.9",                   /\*宝贝描述分\*/
		"score2": "4.9",                   /\*卖家服务分\*/
		"score3": "4.9",                   /\*物流服务分\*/
		"creditLevel": "15",                   /\*店铺等级（1-20），一星 二星 三星 四星 五星 一钻 二钻 三钻 四钻 五钻 一皇冠 二皇冠 三皇冠 四皇冠 五皇冠  一金冠 二金冠 三金冠 四金冠 五金冠\*/
		"shopIcon": "//img.alicdn.com/imgextra/de/a6/TB10dC8SX.jpg",       /\*店铺logo\*/
		"pcDescContent": "1.jpg|2.jpg|3.jpg|4.jpg|5.jpg",  /\*图文详情图片地址\*/
		"item\_url": "https://item.taobao.com/item.htm?id=581801364036",  /\*商品url\*/
		"category\_id": "50006126",                   /\*叶子类目id\*/
		"category\_name": "收纳盒",                   /\*叶子类目name\*/
		"level\_one\_category\_id": "122928002",                   /\*一级类目id\*/
		"level\_one\_category\_name": "收纳整理",                   /\*一级类目name\*/
		"tkfee3": "5.07",                   /\*返佣金额\*/
		"biaoqian": ""满300元,省30元"",                   /\*店铺活动\*/
		"tag": "赶紧拯救你厨房的油腻吧😂植护🌿加厚厨房抽纸高密度纤维纸张，吸油💧很不错5包装|￥14.8元💰✔厨房小帮手，用处多多",                   /\*朋友圈文案，需要自己进行urldecode\*/
		"date\_time": "2018/11/8 14:45:22"                   /\*数据添加时间\*/
	}\]
}
```

**失败返回示例：**

var tmp = { "status": 301 };
```json
{
	"status": 301                      /\*状态\*/
}
```

[效果展示](http://cms.zhetaoke.com:10000/m/item/list.aspx?jiyoujia=1)

---

## 今日商品API

**原始链接**: [今日商品API](https://www.zhetaoke.com/user/extend/extend_lingquan_today.aspx)

**今日商品API---本接口采用12台16核心64G内存服务器，江苏和北京双机房，负载均衡技术，1G独享带宽，放心调用。**

今日商品API：返回今日更新商品列表，返回佣金≥15%，动态描述分≥4.6的商品列表。

**接口地址：** https://api.zhetaoke.com:10001/api/api\_all.ashx?appkey=0a565628b2fb4712902712734cec2feb&page=1&page\_size=20&sort=new&today=1

今日商品API：返回指定查询条件的商品总数。

**接口地址：** https://api.zhetaoke.com:10001/api/api\_all.ashx?appkey=0a565628b2fb4712902712734cec2feb&page=1&page\_size=20&sort=new&today=1&total\_count=1

**返回数据：**json

**请求方式：**get

**请求参数说明：**

|  | 名称 | 类型 | 必填 | 说明 |
| --- | --- | --- | --- | --- |
|   | appkey | string | 是 | 折淘客的对接秘钥appkey |
|   | page | int | 否 | 分页获取数据,第几页 |
|   | page\_size | int | 否 | 每页数据条数（默认每页20条），可自定义1-50之间 |
|   | sort | string | 否 | 商品排序方式，new：按照综合排序，  
total\_sale\_num\_asc：按照总销量从小到大排序，total\_sale\_num\_desc：按照总销量从大到小排序，  
sale\_num\_asc：按照年销量从小到大排序，sale\_num\_desc：按照年销量从大到小排序，  
commission\_rate\_asc：按照佣金比例从小到大排序，commission\_rate\_desc：按照佣金比例从大到小排序，  
price\_asc：按照价格从小到大排序，price\_desc：按照价格从大到小排序，  
coupon\_info\_money\_asc：按照优惠券金额从小到大排序，coupon\_info\_money\_desc：按照优惠券金额从大到小排序，  
shop\_level\_asc：按照店铺等级从低到高排序，shop\_level\_desc：按照店铺等级从高到低排序，  
tkfee\_asc：按照返佣金额从低到高排序，tkfee\_desc：按照返佣金额从高到低排序，  
code：按照code值从大到小排序，date\_time：按照更新时间排序，random：按照随机排序 |
|   | cid | int | 否 | 一级商品分类，值为空：全部商品，1：女装，2：母婴，3：美妆，4：居家日用，5：鞋品，6：美食，7：文娱车品，8：数码家电，9：男装，10：内衣，11：箱包，12：配饰，13：户外运动，14：家装家纺 |
|   | today | string | 否 | 今日商品，值为空：全部商品，1：今日商品 |
|   | start\_date\_time\_yongjin | string | 否 | 数据更新开始时间，格式为：2019-08-22 17:00:00 |
|   | end\_date\_time\_yongjin | string | 否 | 数据更新结束时间，格式为：2019-08-22 17:10:00 |

**成功返回示例：**/\*重要事情说三遍！重要事情说三遍！重要事情说三遍！api\_all/api\_dongdong/api\_quantian/api\_shishi/api\_videos/api\_xiaoshi/api\_yongjin接口所有参数都可以自由组合。\*/

var tmp = { "status": 200, /\*状态\*/ "content": \[{ "code": "6646", /\*折淘客编号\*/ "type\_one\_id": "1", /\*分类ID，可参考折淘客分类\*/ "tao\_id": "554832820990", /\*商品ID\*/ "title": "储物箱整理抽屉式收纳柜", /\*商品短标题\*/ "jianjie": "【爆款返场，出口日本】进口PP材质，可叠加透明可视抽屉收纳盒，分格抽屉物品分类齐全，区域分化一目了然，自由组合，大容量，让你居家生活更舒心，收纳你的生活！【赠运费险】", /\*商品简介\*/ "pict\_url": "https://img.alicdn.com/bao/......jpg", /\*商品主图\*/ "user\_type": "0", /\*是否天猫，0是淘宝，1是天猫\*/ "seller\_id": "3073204721", /\*卖家ID\*/ /\*店铺地址可自行拼接，http://store.taobao.com/shop/view\_shop.htm?user\_number\_id=3073204721 \*/ "shop\_dsr": "4.80", /\*商品描述分\*/ "volume": "169", /\*年销量\*/ "size": "26.90", /\*折扣价\*/ "quanhou\_jiage": "16.90", /\*券后价\*/ "date\_time\_yongjin": "2018/11/8 14:45:22", /\*数据更新时间\*/ "tkrate3": "30.00", /\*佣金比率\*/ "yongjin\_type": "MKT", /\*佣金类型\*/ "coupon\_id": "fe31d6c8132c462ba59bd72e03d2eae0", /\*优惠券ID 该商品如无优惠券，此字段为空\*/ "coupon\_start\_time": "2018-11-05 00:00:00", /\*优惠券开始时间 该商品如无优惠券，此字段为空\*/ "coupon\_end\_time": "2018-11-10 23:59:59", /\*优惠券结束时间 该商品如无优惠券，此字段为空\*/ "coupon\_info\_money": "10", /\*优惠券金额 该商品如无优惠券，此字段为0\*/ "coupon\_total\_count": "100000", /\*优惠券总数量 该商品如无优惠券，此字段为0\*/ "coupon\_remain\_count": "99965", /\*优惠券剩余数量 该商品如无优惠券，此字段为0\*/ "coupon\_info": "满80.00元减10元", /\*优惠券信息\*/ "juhuasuan": "0", /\*是否聚划算，1是\*/ "taoqianggou": "0", /\*是否淘抢购，1是\*/ "haitao": "0", /\*是否海淘，1是\*/ "jiyoujia": "0", /\*是否极有家，1是\*/ "jinpaimaijia": "0", /\*是否金牌卖家，1是\*/ "pinpai": "0", /\*是否精选品牌，1是\*/ "pinpai\_name": "", /\*品牌名称\*/ "yunfeixian": "0", /\*是否有运费险，1有\*/ "nick": "\*\*\*旗舰店", /\*卖家昵称\*/ "small\_images": "https://img.alicdn.com/bao/.......jpg|https://img.alicdn.com/bao/.......jpg", /\*商品小图列表\*/ "white\_image": "https://img.alicdn.com/bao/......jpg", /\*商品白底图\*/ "tao\_title": "抽屉式收纳箱塑料储物箱整理箱衣服物家用宿舍衣柜内衣内裤收纳盒", /\*商品长标题\*/ "provcity": "浙江 杭州", /\*宝贝所在地\*/ "shop\_title": "\*\*\*旗舰店", /\*店铺名称\*/ "zhibo\_url": "https://cloud.video.taobao.com/play/u/849090736/p/2/e/6/t/1/219853683676.mp4", /\*视频地址\*/ "sellCount": "860000", /\*淘宝网页实时总销量\*/ "commentCount": "250000", /\*评论数量\*/ "favcount": "180000", /\*收藏数量\*/ "score1": "4.9", /\*宝贝描述分\*/ "score2": "4.9", /\*卖家服务分\*/ "score3": "4.9", /\*物流服务\*/ "creditLevel": "15", /\*店铺等级，一星 二星 三星 四星 五星 一钻 二钻 三钻 四钻 五钻 一皇冠 二皇冠 三皇冠 四皇冠 五皇冠 一金冠 二金冠 三金冠 四金冠 五金冠\*/ "shopIcon": "//img.alicdn.com/imgextra/de/a6/TB10dC8SX.jpg", /\*店铺logo\*/ "pcDescContent": "1.jpg|2.jpg|3.jpg|4.jpg|5.jpg", /\*图文详情图片地址\*/ "item\_url": "https://item.taobao.com/item.htm?id=581801364036", /\*商品url\*/ "category\_id": "50006126", /\*叶子类目id\*/ "category\_name": "收纳盒", /\*叶子类目name\*/ "level\_one\_category\_id": "122928002", /\*一级类目id\*/ "level\_one\_category\_name": "收纳整理", /\*一级类目name\*/ "tkfee3": "5.07", /\*返佣金额\*/ "biaoqian": ""满300元,省30元"", /\*店铺活动\*/ "tag": "赶紧拯救你厨房的油腻吧😂<br>植护🌿加厚厨房抽纸<br>高密度纤维纸张，吸油💧很不错<br>5包装|￥14.8元💰<br>✔厨房小帮手，用处多多", /\*朋友圈文案，需要自己进行urldecode\*/ "date\_time": "2018/11/8 14:45:22" /\*数据添加时间\*/ }\] };
```json
{
	"status": 200,                      /\*状态\*/
	"content": \[{
		"code": "6646",                 /\*折淘客编号\*/
		"type\_one\_id": "1",             /\*分类ID，可参考折淘客分类\*/
		"tao\_id": "554832820990",       /\*商品ID\*/
		"title": "储物箱整理抽屉式收纳柜",                   /\*商品短标题\*/
		"jianjie": "【爆款返场，出口日本】进口PP材质，可叠加透明可视抽屉收纳盒，分格抽屉物品分类齐全，区域分化一目了然，自由组合，大容量，让你居家生活更舒心，收纳你的生活！【赠运费险】", /\*商品简介\*/
		"pict\_url": "https://img.alicdn.com/bao/......jpg",                   /\*商品主图\*/
		"user\_type": "0",                   /\*是否天猫，0是淘宝，1是天猫\*/
		"seller\_id": "3073204721",                   /\*卖家ID\*/ /\*店铺地址可自行拼接，http://store.taobao.com/shop/view\_shop.htm?user\_number\_id=3073204721 \*/
		"shop\_dsr": "4.80",                   /\*商品描述分\*/
		"volume": "169",                   /\*年销量\*/
		"size": "26.90",                   /\*折扣价\*/
		"quanhou\_jiage": "16.90",                   /\*券后价\*/
		"date\_time\_yongjin": "2018/11/8 14:45:22",                   /\*数据更新时间\*/
		"tkrate3": "30.00",                   /\*佣金比率\*/
		"yongjin\_type": "MKT",                   /\*佣金类型\*/
		"coupon\_id": "fe31d6c8132c462ba59bd72e03d2eae0",                   /\*优惠券ID 该商品如无优惠券，此字段为空\*/
		"coupon\_start\_time": "2018-11-05 00:00:00",                   /\*优惠券开始时间 该商品如无优惠券，此字段为空\*/
		"coupon\_end\_time": "2018-11-10 23:59:59",                   /\*优惠券结束时间 该商品如无优惠券，此字段为空\*/
		"coupon\_info\_money": "10",                   /\*优惠券金额 该商品如无优惠券，此字段为0\*/
		"coupon\_total\_count": "100000",                   /\*优惠券总数量 该商品如无优惠券，此字段为0\*/
		"coupon\_remain\_count": "99965",                   /\*优惠券剩余数量 该商品如无优惠券，此字段为0\*/
		"coupon\_info": "满80.00元减10元",                   /\*优惠券信息\*/
		"juhuasuan": "0",                       /\*是否聚划算，1是\*/
		"taoqianggou": "0",                     /\*是否淘抢购，1是\*/
		"haitao": "0",                          /\*是否海淘，1是\*/
		"jiyoujia": "0",                        /\*是否极有家，1是\*/
		"jinpaimaijia": "0",                    /\*是否金牌卖家，1是\*/
		"pinpai": "0",                          /\*是否精选品牌，1是\*/
		"pinpai\_name": "",                      /\*品牌名称\*/
		"yunfeixian": "0",                      /\*是否有运费险，1有\*/
		"nick": "\*\*\*旗舰店",                    /\*卖家昵称\*/
		"small\_images": "https://img.alicdn.com/bao/.......jpg|https://img.alicdn.com/bao/.......jpg",                   /\*商品小图列表\*/
		"white\_image": "https://img.alicdn.com/bao/......jpg",                   /\*商品白底图\*/
		"tao\_title": "抽屉式收纳箱塑料储物箱整理箱衣服物家用宿舍衣柜内衣内裤收纳盒",                   /\*商品长标题\*/
		"provcity": "浙江 杭州",                   /\*宝贝所在地\*/
		"shop\_title": "\*\*\*旗舰店",                   /\*店铺名称\*/
		"zhibo\_url": "https://cloud.video.taobao.com/play/u/849090736/p/2/e/6/t/1/219853683676.mp4",                   /\*视频地址\*/
		"sellCount": "860000",                   /\*淘宝网页实时总销量\*/
		"commentCount": "250000",                   /\*评论数量\*/
		"favcount": "180000",                   /\*收藏数量\*/
		"score1": "4.9",                   /\*宝贝描述分\*/
		"score2": "4.9",                   /\*卖家服务分\*/
		"score3": "4.9",                   /\*物流服务分\*/
		"creditLevel": "15",                   /\*店铺等级（1-20），一星 二星 三星 四星 五星 一钻 二钻 三钻 四钻 五钻 一皇冠 二皇冠 三皇冠 四皇冠 五皇冠  一金冠 二金冠 三金冠 四金冠 五金冠\*/
		"shopIcon": "//img.alicdn.com/imgextra/de/a6/TB10dC8SX.jpg",       /\*店铺logo\*/
		"pcDescContent": "1.jpg|2.jpg|3.jpg|4.jpg|5.jpg",  /\*图文详情图片地址\*/
		"item\_url": "https://item.taobao.com/item.htm?id=581801364036",  /\*商品url\*/
		"category\_id": "50006126",                   /\*叶子类目id\*/
		"category\_name": "收纳盒",                   /\*叶子类目name\*/
		"level\_one\_category\_id": "122928002",                   /\*一级类目id\*/
		"level\_one\_category\_name": "收纳整理",                   /\*一级类目name\*/
		"tkfee3": "5.07",                   /\*返佣金额\*/
		"biaoqian": ""满300元,省30元"",                   /\*店铺活动\*/
		"tag": "赶紧拯救你厨房的油腻吧😂植护🌿加厚厨房抽纸高密度纤维纸张，吸油💧很不错5包装|￥14.8元💰✔厨房小帮手，用处多多",                   /\*朋友圈文案，需要自己进行urldecode\*/
		"date\_time": "2018/11/8 14:45:22"                   /\*数据添加时间\*/
	}\]
}
```

**失败返回示例：**

var tmp = { "status": 301 };
```json
{
	"status": 301                      /\*状态\*/
}
```

[效果展示](http://cms.zhetaoke.com:10000/m/item/list.aspx?today=1)

---

## 精选品牌商品API

**原始链接**: [精选品牌商品API](https://www.zhetaoke.com/user/extend/extend_lingquan_pinpai.aspx)

**精选品牌商品API---本接口采用12台16核心64G内存服务器，江苏和北京双机房，负载均衡技术，1G独享带宽，放心调用。**

精选品牌商品API：返回精选品牌商品列表，返回佣金≥15%，动态描述分≥4.6的商品列表。

**接口地址：** https://api.zhetaoke.com:10001/api/api\_all.ashx?appkey=0a565628b2fb4712902712734cec2feb&page=1&page\_size=20&sort=new&cid=0&pinpai=1&pinpai\_name=

精选品牌商品API：返回指定查询条件的商品总数。

**接口地址：** https://api.zhetaoke.com:10001/api/api\_all.ashx?appkey=0a565628b2fb4712902712734cec2feb&page=1&page\_size=20&sort=new&cid=0&pinpai=1&pinpai\_name=&total\_count=1

品牌列表：返回前100名品牌名列表。

**接口地址：** https://api.zhetaoke.com:10001/api/api\_pinpai\_name.ashx?appkey=0a565628b2fb4712902712734cec2feb&page=1&page\_size=100&cid=0

**返回数据：**json

**请求方式：**get

**请求参数说明：**

|  | 名称 | 类型 | 必填 | 说明 |
| --- | --- | --- | --- | --- |
|   | appkey | string | 是 | 折淘客的对接秘钥appkey |
|   | page | int | 否 | 分页获取数据,第几页 |
|   | page\_size | int | 否 | 每页数据条数（默认每页20条），可自定义1-50之间 |
|   | sort | string | 否 | 商品排序方式，new：按照综合排序，  
total\_sale\_num\_asc：按照总销量从小到大排序，total\_sale\_num\_desc：按照总销量从大到小排序，  
sale\_num\_asc：按照年销量从小到大排序，sale\_num\_desc：按照年销量从大到小排序，  
commission\_rate\_asc：按照佣金比例从小到大排序，commission\_rate\_desc：按照佣金比例从大到小排序，  
price\_asc：按照价格从小到大排序，price\_desc：按照价格从大到小排序，  
coupon\_info\_money\_asc：按照优惠券金额从小到大排序，coupon\_info\_money\_desc：按照优惠券金额从大到小排序，  
shop\_level\_asc：按照店铺等级从低到高排序，shop\_level\_desc：按照店铺等级从高到低排序，  
tkfee\_asc：按照返佣金额从低到高排序，tkfee\_desc：按照返佣金额从高到低排序，  
code：按照code值从大到小排序，date\_time：按照更新时间排序，random：按照随机排序 |
|   | cid | int | 否 | 一级商品分类，值为空：全部商品，1：女装，2：母婴，3：美妆，4：居家日用，5：鞋品，6：美食，7：文娱车品，8：数码家电，9：男装，10：内衣，11：箱包，12：配饰，13：户外运动，14：家装家纺 |
|   | pinpai | string | 否 | 精选品牌，值为空：全部商品，1：精选品牌商品 |
|   | pinpai\_name | string | 否 | 品牌名称，如：南极人、苏泊尔、美的等品牌。 |

**成功返回示例：**/\*重要事情说三遍！重要事情说三遍！重要事情说三遍！api\_all/api\_dongdong/api\_quantian/api\_shishi/api\_videos/api\_xiaoshi/api\_yongjin接口所有参数都可以自由组合。\*/

var tmp = { "status": 200, /\*状态\*/ "content": \[{ "code": "6646", /\*折淘客编号\*/ "type\_one\_id": "1", /\*分类ID，可参考折淘客分类\*/ "tao\_id": "554832820990", /\*商品ID\*/ "title": "储物箱整理抽屉式收纳柜", /\*商品短标题\*/ "jianjie": "【爆款返场，出口日本】进口PP材质，可叠加透明可视抽屉收纳盒，分格抽屉物品分类齐全，区域分化一目了然，自由组合，大容量，让你居家生活更舒心，收纳你的生活！【赠运费险】", /\*商品简介\*/ "pict\_url": "https://img.alicdn.com/bao/......jpg", /\*商品主图\*/ "user\_type": "0", /\*是否天猫，0是淘宝，1是天猫\*/ "seller\_id": "3073204721", /\*卖家ID\*/ /\*店铺地址可自行拼接，http://store.taobao.com/shop/view\_shop.htm?user\_number\_id=3073204721 \*/ "shop\_dsr": "4.80", /\*商品描述分\*/ "volume": "169", /\*年销量\*/ "size": "26.90", /\*折扣价\*/ "quanhou\_jiage": "16.90", /\*券后价\*/ "date\_time\_yongjin": "2018/11/8 14:45:22", /\*数据更新时间\*/ "tkrate3": "30.00", /\*佣金比率\*/ "yongjin\_type": "MKT", /\*佣金类型\*/ "coupon\_id": "fe31d6c8132c462ba59bd72e03d2eae0", /\*优惠券ID 该商品如无优惠券，此字段为空\*/ "coupon\_start\_time": "2018-11-05 00:00:00", /\*优惠券开始时间 该商品如无优惠券，此字段为空\*/ "coupon\_end\_time": "2018-11-10 23:59:59", /\*优惠券结束时间 该商品如无优惠券，此字段为空\*/ "coupon\_info\_money": "10", /\*优惠券金额 该商品如无优惠券，此字段为0\*/ "coupon\_total\_count": "100000", /\*优惠券总数量 该商品如无优惠券，此字段为0\*/ "coupon\_remain\_count": "99965", /\*优惠券剩余数量 该商品如无优惠券，此字段为0\*/ "coupon\_info": "满80.00元减10元", /\*优惠券信息\*/ "juhuasuan": "0", /\*是否聚划算，1是\*/ "taoqianggou": "0", /\*是否淘抢购，1是\*/ "haitao": "0", /\*是否海淘，1是\*/ "jiyoujia": "0", /\*是否极有家，1是\*/ "jinpaimaijia": "0", /\*是否金牌卖家，1是\*/ "pinpai": "0", /\*是否精选品牌，1是\*/ "pinpai\_name": "", /\*品牌名称\*/ "yunfeixian": "0", /\*是否有运费险，1有\*/ "nick": "\*\*\*旗舰店", /\*卖家昵称\*/ "small\_images": "https://img.alicdn.com/bao/.......jpg|https://img.alicdn.com/bao/.......jpg", /\*商品小图列表\*/ "white\_image": "https://img.alicdn.com/bao/......jpg", /\*商品白底图\*/ "tao\_title": "抽屉式收纳箱塑料储物箱整理箱衣服物家用宿舍衣柜内衣内裤收纳盒", /\*商品长标题\*/ "provcity": "浙江 杭州", /\*宝贝所在地\*/ "shop\_title": "\*\*\*旗舰店", /\*店铺名称\*/ "zhibo\_url": "https://cloud.video.taobao.com/play/u/849090736/p/2/e/6/t/1/219853683676.mp4", /\*视频地址\*/ "sellCount": "860000", /\*淘宝网页实时总销量\*/ "commentCount": "250000", /\*评论数量\*/ "favcount": "180000", /\*收藏数量\*/ "score1": "4.9", /\*宝贝描述分\*/ "score2": "4.9", /\*卖家服务分\*/ "score3": "4.9", /\*物流服务\*/ "creditLevel": "15", /\*店铺等级，一星 二星 三星 四星 五星 一钻 二钻 三钻 四钻 五钻 一皇冠 二皇冠 三皇冠 四皇冠 五皇冠 一金冠 二金冠 三金冠 四金冠 五金冠\*/ "shopIcon": "//img.alicdn.com/imgextra/de/a6/TB10dC8SX.jpg", /\*店铺logo\*/ "pcDescContent": "1.jpg|2.jpg|3.jpg|4.jpg|5.jpg", /\*图文详情图片地址\*/ "item\_url": "https://item.taobao.com/item.htm?id=581801364036", /\*商品url\*/ "category\_id": "50006126", /\*叶子类目id\*/ "category\_name": "收纳盒", /\*叶子类目name\*/ "level\_one\_category\_id": "122928002", /\*一级类目id\*/ "level\_one\_category\_name": "收纳整理", /\*一级类目name\*/ "tkfee3": "5.07", /\*返佣金额\*/ "biaoqian": ""满300元,省30元"", /\*店铺活动\*/ "tag": "赶紧拯救你厨房的油腻吧😂<br>植护🌿加厚厨房抽纸<br>高密度纤维纸张，吸油💧很不错<br>5包装|￥14.8元💰<br>✔厨房小帮手，用处多多", /\*朋友圈文案，需要自己进行urldecode\*/ "date\_time": "2018/11/8 14:45:22" /\*数据添加时间\*/ }\] };
```json
{
	"status": 200,                      /\*状态\*/
	"content": \[{
		"code": "6646",                 /\*折淘客编号\*/
		"type\_one\_id": "1",             /\*分类ID，可参考折淘客分类\*/
		"tao\_id": "554832820990",       /\*商品ID\*/
		"title": "储物箱整理抽屉式收纳柜",                   /\*商品短标题\*/
		"jianjie": "【爆款返场，出口日本】进口PP材质，可叠加透明可视抽屉收纳盒，分格抽屉物品分类齐全，区域分化一目了然，自由组合，大容量，让你居家生活更舒心，收纳你的生活！【赠运费险】", /\*商品简介\*/
		"pict\_url": "https://img.alicdn.com/bao/......jpg",                   /\*商品主图\*/
		"user\_type": "0",                   /\*是否天猫，0是淘宝，1是天猫\*/
		"seller\_id": "3073204721",                   /\*卖家ID\*/ /\*店铺地址可自行拼接，http://store.taobao.com/shop/view\_shop.htm?user\_number\_id=3073204721 \*/
		"shop\_dsr": "4.80",                   /\*商品描述分\*/
		"volume": "169",                   /\*年销量\*/
		"size": "26.90",                   /\*折扣价\*/
		"quanhou\_jiage": "16.90",                   /\*券后价\*/
		"date\_time\_yongjin": "2018/11/8 14:45:22",                   /\*数据更新时间\*/
		"tkrate3": "30.00",                   /\*佣金比率\*/
		"yongjin\_type": "MKT",                   /\*佣金类型\*/
		"coupon\_id": "fe31d6c8132c462ba59bd72e03d2eae0",                   /\*优惠券ID 该商品如无优惠券，此字段为空\*/
		"coupon\_start\_time": "2018-11-05 00:00:00",                   /\*优惠券开始时间 该商品如无优惠券，此字段为空\*/
		"coupon\_end\_time": "2018-11-10 23:59:59",                   /\*优惠券结束时间 该商品如无优惠券，此字段为空\*/
		"coupon\_info\_money": "10",                   /\*优惠券金额 该商品如无优惠券，此字段为0\*/
		"coupon\_total\_count": "100000",                   /\*优惠券总数量 该商品如无优惠券，此字段为0\*/
		"coupon\_remain\_count": "99965",                   /\*优惠券剩余数量 该商品如无优惠券，此字段为0\*/
		"coupon\_info": "满80.00元减10元",                   /\*优惠券信息\*/
		"juhuasuan": "0",                       /\*是否聚划算，1是\*/
		"taoqianggou": "0",                     /\*是否淘抢购，1是\*/
		"haitao": "0",                          /\*是否海淘，1是\*/
		"jiyoujia": "0",                        /\*是否极有家，1是\*/
		"jinpaimaijia": "0",                    /\*是否金牌卖家，1是\*/
		"pinpai": "0",                          /\*是否精选品牌，1是\*/
		"pinpai\_name": "",                      /\*品牌名称\*/
		"yunfeixian": "0",                      /\*是否有运费险，1有\*/
		"nick": "\*\*\*旗舰店",                    /\*卖家昵称\*/
		"small\_images": "https://img.alicdn.com/bao/.......jpg|https://img.alicdn.com/bao/.......jpg",                   /\*商品小图列表\*/
		"white\_image": "https://img.alicdn.com/bao/......jpg",                   /\*商品白底图\*/
		"tao\_title": "抽屉式收纳箱塑料储物箱整理箱衣服物家用宿舍衣柜内衣内裤收纳盒",                   /\*商品长标题\*/
		"provcity": "浙江 杭州",                   /\*宝贝所在地\*/
		"shop\_title": "\*\*\*旗舰店",                   /\*店铺名称\*/
		"zhibo\_url": "https://cloud.video.taobao.com/play/u/849090736/p/2/e/6/t/1/219853683676.mp4",                   /\*视频地址\*/
		"sellCount": "860000",                   /\*淘宝网页实时总销量\*/
		"commentCount": "250000",                   /\*评论数量\*/
		"favcount": "180000",                   /\*收藏数量\*/
		"score1": "4.9",                   /\*宝贝描述分\*/
		"score2": "4.9",                   /\*卖家服务分\*/
		"score3": "4.9",                   /\*物流服务分\*/
		"creditLevel": "15",                   /\*店铺等级（1-20），一星 二星 三星 四星 五星 一钻 二钻 三钻 四钻 五钻 一皇冠 二皇冠 三皇冠 四皇冠 五皇冠  一金冠 二金冠 三金冠 四金冠 五金冠\*/
		"shopIcon": "//img.alicdn.com/imgextra/de/a6/TB10dC8SX.jpg",       /\*店铺logo\*/
		"pcDescContent": "1.jpg|2.jpg|3.jpg|4.jpg|5.jpg",  /\*图文详情图片地址\*/
		"item\_url": "https://item.taobao.com/item.htm?id=581801364036",  /\*商品url\*/
		"category\_id": "50006126",                   /\*叶子类目id\*/
		"category\_name": "收纳盒",                   /\*叶子类目name\*/
		"level\_one\_category\_id": "122928002",                   /\*一级类目id\*/
		"level\_one\_category\_name": "收纳整理",                   /\*一级类目name\*/
		"tkfee3": "5.07",                   /\*返佣金额\*/
		"biaoqian": ""满300元,省30元"",                   /\*店铺活动\*/
		"tag": "赶紧拯救你厨房的油腻吧😂植护🌿加厚厨房抽纸高密度纤维纸张，吸油💧很不错5包装|￥14.8元💰✔厨房小帮手，用处多多",                   /\*朋友圈文案，需要自己进行urldecode\*/
		"date\_time": "2018/11/8 14:45:22"                   /\*数据添加时间\*/
	}\]
}
```

**失败返回示例：**

var tmp = { "status": 301 };
```json
{
	"status": 301                      /\*状态\*/
}
```

---

## 天猫超市商品API

**原始链接**: [天猫超市商品API](https://www.zhetaoke.com/user/extend/extend_lingquan_tianmaochaoshi.aspx)

**天猫超市商品API---本接口采用12台16核心64G内存服务器，江苏和北京双机房，负载均衡技术，1G独享带宽，放心调用。**

天猫超市商品API：返回天猫超市商品列表，动态描述分≥4.6的商品列表，根据不同参数可返回天猫超市单件免邮商品。

**接口地址：** https://api.zhetaoke.com:10001/api/api\_all.ashx?appkey=0a565628b2fb4712902712734cec2feb&page=1&page\_size=20&sort=new&tianmaochaoshi=1

天猫超市商品API：返回指定查询条件的商品总数。

**接口地址：** https://api.zhetaoke.com:10001/api/api\_all.ashx?appkey=0a565628b2fb4712902712734cec2feb&page=1&page\_size=20&sort=new&tianmaochaoshi=1&total\_count=1

**返回数据：**json

**请求方式：**get

**请求参数说明：**

|  | 名称 | 类型 | 必填 | 说明 |
| --- | --- | --- | --- | --- |
|   | appkey | string | 是 | 折淘客的对接秘钥appkey |
|   | page | int | 否 | 分页获取数据,第几页 |
|   | page\_size | int | 否 | 每页数据条数（默认每页20条），可自定义1-50之间 |
|   | sort | string | 否 | 商品排序方式，new：按照综合排序，  
total\_sale\_num\_asc：按照总销量从小到大排序，total\_sale\_num\_desc：按照总销量从大到小排序，  
sale\_num\_asc：按照年销量从小到大排序，sale\_num\_desc：按照年销量从大到小排序，  
commission\_rate\_asc：按照佣金比例从小到大排序，commission\_rate\_desc：按照佣金比例从大到小排序，  
price\_asc：按照价格从小到大排序，price\_desc：按照价格从大到小排序，  
coupon\_info\_money\_asc：按照优惠券金额从小到大排序，coupon\_info\_money\_desc：按照优惠券金额从大到小排序，  
shop\_level\_asc：按照店铺等级从低到高排序，shop\_level\_desc：按照店铺等级从高到低排序，  
tkfee\_asc：按照返佣金额从低到高排序，tkfee\_desc：按照返佣金额从高到低排序，  
code：按照code值从大到小排序，date\_time：按照更新时间排序，random：按照随机排序 |
|   | cid | int | 否 | 一级商品分类，值为空：全部商品，1：女装，2：母婴，3：美妆，4：居家日用，5：鞋品，6：美食，7：文娱车品，8：数码家电，9：男装，10：内衣，11：箱包，12：配饰，13：户外运动，14：家装家纺 |
|   | tianmaochaoshi | string | 否 | 天猫超市，值为空：全部商品，1：天猫超市商品 |
|   | tianmaochaoshi2 | string | 否 | 天猫超市商品是否单件免邮，值为1：天猫超市商品单件免邮，此字段只针对天猫超市店铺商品有效，其它店铺商品无效 |

**成功返回示例：**/\*重要事情说三遍！重要事情说三遍！重要事情说三遍！api\_all/api\_dongdong/api\_quantian/api\_shishi/api\_videos/api\_xiaoshi/api\_yongjin接口所有参数都可以自由组合。\*/

var tmp = { "status": 200, /\*状态\*/ "content": \[{ "code": "6646", /\*折淘客编号\*/ "type\_one\_id": "1", /\*分类ID，可参考折淘客分类\*/ "tao\_id": "554832820990", /\*商品ID\*/ "title": "储物箱整理抽屉式收纳柜", /\*商品短标题\*/ "jianjie": "【爆款返场，出口日本】进口PP材质，可叠加透明可视抽屉收纳盒，分格抽屉物品分类齐全，区域分化一目了然，自由组合，大容量，让你居家生活更舒心，收纳你的生活！【赠运费险】", /\*商品简介\*/ "pict\_url": "https://img.alicdn.com/bao/......jpg", /\*商品主图\*/ "user\_type": "0", /\*是否天猫，0是淘宝，1是天猫\*/ "seller\_id": "3073204721", /\*卖家ID\*/ /\*店铺地址可自行拼接，http://store.taobao.com/shop/view\_shop.htm?user\_number\_id=3073204721 \*/ "shop\_dsr": "4.80", /\*商品描述分\*/ "volume": "169", /\*年销量\*/ "size": "26.90", /\*折扣价\*/ "quanhou\_jiage": "16.90", /\*券后价\*/ "date\_time\_yongjin": "2018/11/8 14:45:22", /\*数据更新时间\*/ "tkrate3": "30.00", /\*佣金比率\*/ "yongjin\_type": "MKT", /\*佣金类型\*/ "coupon\_id": "fe31d6c8132c462ba59bd72e03d2eae0", /\*优惠券ID 该商品如无优惠券，此字段为空\*/ "coupon\_start\_time": "2018-11-05 00:00:00", /\*优惠券开始时间 该商品如无优惠券，此字段为空\*/ "coupon\_end\_time": "2018-11-10 23:59:59", /\*优惠券结束时间 该商品如无优惠券，此字段为空\*/ "coupon\_info\_money": "10", /\*优惠券金额 该商品如无优惠券，此字段为0\*/ "coupon\_total\_count": "100000", /\*优惠券总数量 该商品如无优惠券，此字段为0\*/ "coupon\_remain\_count": "99965", /\*优惠券剩余数量 该商品如无优惠券，此字段为0\*/ "coupon\_info": "满80.00元减10元", /\*优惠券信息\*/ "juhuasuan": "0", /\*是否聚划算，1是\*/ "taoqianggou": "0", /\*是否淘抢购，1是\*/ "haitao": "0", /\*是否海淘，1是\*/ "jiyoujia": "0", /\*是否极有家，1是\*/ "jinpaimaijia": "0", /\*是否金牌卖家，1是\*/ "pinpai": "0", /\*是否精选品牌，1是\*/ "pinpai\_name": "", /\*品牌名称\*/ "yunfeixian": "0", /\*是否有运费险，1有\*/ "nick": "\*\*\*旗舰店", /\*卖家昵称\*/ "small\_images": "https://img.alicdn.com/bao/.......jpg|https://img.alicdn.com/bao/.......jpg", /\*商品小图列表\*/ "white\_image": "https://img.alicdn.com/bao/......jpg", /\*商品白底图\*/ "tao\_title": "抽屉式收纳箱塑料储物箱整理箱衣服物家用宿舍衣柜内衣内裤收纳盒", /\*商品长标题\*/ "provcity": "浙江 杭州", /\*宝贝所在地\*/ "shop\_title": "\*\*\*旗舰店", /\*店铺名称\*/ "zhibo\_url": "https://cloud.video.taobao.com/play/u/849090736/p/2/e/6/t/1/219853683676.mp4", /\*视频地址\*/ "sellCount": "860000", /\*淘宝网页实时总销量\*/ "commentCount": "250000", /\*评论数量\*/ "favcount": "180000", /\*收藏数量\*/ "score1": "4.9", /\*宝贝描述分\*/ "score2": "4.9", /\*卖家服务分\*/ "score3": "4.9", /\*物流服务\*/ "creditLevel": "15", /\*店铺等级，一星 二星 三星 四星 五星 一钻 二钻 三钻 四钻 五钻 一皇冠 二皇冠 三皇冠 四皇冠 五皇冠 一金冠 二金冠 三金冠 四金冠 五金冠\*/ "shopIcon": "//img.alicdn.com/imgextra/de/a6/TB10dC8SX.jpg", /\*店铺logo\*/ "pcDescContent": "1.jpg|2.jpg|3.jpg|4.jpg|5.jpg", /\*图文详情图片地址\*/ "item\_url": "https://item.taobao.com/item.htm?id=581801364036", /\*商品url\*/ "category\_id": "50006126", /\*叶子类目id\*/ "category\_name": "收纳盒", /\*叶子类目name\*/ "level\_one\_category\_id": "122928002", /\*一级类目id\*/ "level\_one\_category\_name": "收纳整理", /\*一级类目name\*/ "tkfee3": "5.07", /\*返佣金额\*/ "biaoqian": ""满300元,省30元"", /\*店铺活动\*/ "tag": "赶紧拯救你厨房的油腻吧😂<br>植护🌿加厚厨房抽纸<br>高密度纤维纸张，吸油💧很不错<br>5包装|￥14.8元💰<br>✔厨房小帮手，用处多多", /\*朋友圈文案，需要自己进行urldecode\*/ "date\_time": "2018/11/8 14:45:22" /\*数据添加时间\*/ }\] };
```json
{
	"status": 200,                      /\*状态\*/
	"content": \[{
		"code": "6646",                 /\*折淘客编号\*/
		"type\_one\_id": "1",             /\*分类ID，可参考折淘客分类\*/
		"tao\_id": "554832820990",       /\*商品ID\*/
		"title": "储物箱整理抽屉式收纳柜",                   /\*商品短标题\*/
		"jianjie": "【爆款返场，出口日本】进口PP材质，可叠加透明可视抽屉收纳盒，分格抽屉物品分类齐全，区域分化一目了然，自由组合，大容量，让你居家生活更舒心，收纳你的生活！【赠运费险】", /\*商品简介\*/
		"pict\_url": "https://img.alicdn.com/bao/......jpg",                   /\*商品主图\*/
		"user\_type": "0",                   /\*是否天猫，0是淘宝，1是天猫\*/
		"seller\_id": "3073204721",                   /\*卖家ID\*/ /\*店铺地址可自行拼接，http://store.taobao.com/shop/view\_shop.htm?user\_number\_id=3073204721 \*/
		"shop\_dsr": "4.80",                   /\*商品描述分\*/
		"volume": "169",                   /\*年销量\*/
		"size": "26.90",                   /\*折扣价\*/
		"quanhou\_jiage": "16.90",                   /\*券后价\*/
		"date\_time\_yongjin": "2018/11/8 14:45:22",                   /\*数据更新时间\*/
		"tkrate3": "30.00",                   /\*佣金比率\*/
		"yongjin\_type": "MKT",                   /\*佣金类型\*/
		"coupon\_id": "fe31d6c8132c462ba59bd72e03d2eae0",                   /\*优惠券ID 该商品如无优惠券，此字段为空\*/
		"coupon\_start\_time": "2018-11-05 00:00:00",                   /\*优惠券开始时间 该商品如无优惠券，此字段为空\*/
		"coupon\_end\_time": "2018-11-10 23:59:59",                   /\*优惠券结束时间 该商品如无优惠券，此字段为空\*/
		"coupon\_info\_money": "10",                   /\*优惠券金额 该商品如无优惠券，此字段为0\*/
		"coupon\_total\_count": "100000",                   /\*优惠券总数量 该商品如无优惠券，此字段为0\*/
		"coupon\_remain\_count": "99965",                   /\*优惠券剩余数量 该商品如无优惠券，此字段为0\*/
		"coupon\_info": "满80.00元减10元",                   /\*优惠券信息\*/
		"juhuasuan": "0",                       /\*是否聚划算，1是\*/
		"taoqianggou": "0",                     /\*是否淘抢购，1是\*/
		"haitao": "0",                          /\*是否海淘，1是\*/
		"jiyoujia": "0",                        /\*是否极有家，1是\*/
		"jinpaimaijia": "0",                    /\*是否金牌卖家，1是\*/
		"pinpai": "0",                          /\*是否精选品牌，1是\*/
		"pinpai\_name": "",                      /\*品牌名称\*/
		"yunfeixian": "0",                      /\*是否有运费险，1有\*/
		"nick": "\*\*\*旗舰店",                    /\*卖家昵称\*/
		"small\_images": "https://img.alicdn.com/bao/.......jpg|https://img.alicdn.com/bao/.......jpg",                   /\*商品小图列表\*/
		"white\_image": "https://img.alicdn.com/bao/......jpg",                   /\*商品白底图\*/
		"tao\_title": "抽屉式收纳箱塑料储物箱整理箱衣服物家用宿舍衣柜内衣内裤收纳盒",                   /\*商品长标题\*/
		"provcity": "浙江 杭州",                   /\*宝贝所在地\*/
		"shop\_title": "\*\*\*旗舰店",                   /\*店铺名称\*/
		"zhibo\_url": "https://cloud.video.taobao.com/play/u/849090736/p/2/e/6/t/1/219853683676.mp4",                   /\*视频地址\*/
		"sellCount": "860000",                   /\*淘宝网页实时总销量\*/
		"commentCount": "250000",                   /\*评论数量\*/
		"favcount": "180000",                   /\*收藏数量\*/
		"score1": "4.9",                   /\*宝贝描述分\*/
		"score2": "4.9",                   /\*卖家服务分\*/
		"score3": "4.9",                   /\*物流服务分\*/
		"creditLevel": "15",                   /\*店铺等级（1-20），一星 二星 三星 四星 五星 一钻 二钻 三钻 四钻 五钻 一皇冠 二皇冠 三皇冠 四皇冠 五皇冠  一金冠 二金冠 三金冠 四金冠 五金冠\*/
		"shopIcon": "//img.alicdn.com/imgextra/de/a6/TB10dC8SX.jpg",       /\*店铺logo\*/
		"pcDescContent": "1.jpg|2.jpg|3.jpg|4.jpg|5.jpg",  /\*图文详情图片地址\*/
		"item\_url": "https://item.taobao.com/item.htm?id=581801364036",  /\*商品url\*/
		"category\_id": "50006126",                   /\*叶子类目id\*/
		"category\_name": "收纳盒",                   /\*叶子类目name\*/
		"level\_one\_category\_id": "122928002",                   /\*一级类目id\*/
		"level\_one\_category\_name": "收纳整理",                   /\*一级类目name\*/
		"tkfee3": "5.07",                   /\*返佣金额\*/
		"biaoqian": ""满300元,省30元"",                   /\*店铺活动\*/
		"tag": "赶紧拯救你厨房的油腻吧😂植护🌿加厚厨房抽纸高密度纤维纸张，吸油💧很不错5包装|￥14.8元💰✔厨房小帮手，用处多多",                   /\*朋友圈文案，需要自己进行urldecode\*/
		"date\_time": "2018/11/8 14:45:22"                   /\*数据添加时间\*/
	}\]
}
```

**失败返回示例：**

var tmp = { "status": 301 };
```json
{
	"status": 301                      /\*状态\*/
}
```

[效果展示](http://cms.zhetaoke.com:10000/m/item/list.aspx?tianmaochaoshi=1)

[订单查询说明](http://www.zhetaoke.com/help_detail_3_23.html)

---

## 预告商品API

**原始链接**: [预告商品API](https://www.zhetaoke.com/user/extend/extend_lingquan_yugao.aspx)

**预告商品API---本接口采用12台16核心64G内存服务器，江苏和北京双机房，负载均衡技术，1G独享带宽，放心调用。**

预告商品API：返回预告商品列表（优惠券开始时间还未生效，优惠券只能领不能用的商品列表），动态描述分≥4.6的商品列表。

**接口地址：** https://api.zhetaoke.com:10001/api/api\_all.ashx?appkey=0a565628b2fb4712902712734cec2feb&page=1&page\_size=20&sort=new&live=2

预告商品API：返回指定查询条件的商品总数。

**接口地址：** https://api.zhetaoke.com:10001/api/api\_all.ashx?appkey=0a565628b2fb4712902712734cec2feb&page=1&page\_size=20&sort=new&live=2&total\_count=1

**返回数据：**json

**请求方式：**get

**请求参数说明：**

|  | 名称 | 类型 | 必填 | 说明 |
| --- | --- | --- | --- | --- |
|   | appkey | string | 是 | 折淘客的对接秘钥appkey |
|   | page | int | 否 | 分页获取数据,第几页 |
|   | page\_size | int | 否 | 每页数据条数（默认每页20条），可自定义1-50之间 |
|   | sort | string | 否 | 商品排序方式，new：按照综合排序，  
total\_sale\_num\_asc：按照总销量从小到大排序，total\_sale\_num\_desc：按照总销量从大到小排序，  
sale\_num\_asc：按照年销量从小到大排序，sale\_num\_desc：按照年销量从大到小排序，  
commission\_rate\_asc：按照佣金比例从小到大排序，commission\_rate\_desc：按照佣金比例从大到小排序，  
price\_asc：按照价格从小到大排序，price\_desc：按照价格从大到小排序，  
coupon\_info\_money\_asc：按照优惠券金额从小到大排序，coupon\_info\_money\_desc：按照优惠券金额从大到小排序，  
shop\_level\_asc：按照店铺等级从低到高排序，shop\_level\_desc：按照店铺等级从高到低排序，  
tkfee\_asc：按照返佣金额从低到高排序，tkfee\_desc：按照返佣金额从高到低排序，  
code：按照code值从大到小排序，date\_time：按照更新时间排序，random：按照随机排序 |
|   | cid | int | 否 | 一级商品分类，值为空：全部商品，1：女装，2：母婴，3：美妆，4：居家日用，5：鞋品，6：美食，7：文娱车品，8：数码家电，9：男装，10：内衣，11：箱包，12：配饰，13：户外运动，14：家装家纺 |
|   | live | string | 否 | 预告商品，值为2：预告商品 |

**成功返回示例：**/\*重要事情说三遍！重要事情说三遍！重要事情说三遍！api\_all/api\_dongdong/api\_quantian/api\_shishi/api\_videos/api\_xiaoshi/api\_yongjin接口所有参数都可以自由组合。\*/

var tmp = { "status": 200, /\*状态\*/ "content": \[{ "code": "6646", /\*折淘客编号\*/ "type\_one\_id": "1", /\*分类ID，可参考折淘客分类\*/ "tao\_id": "554832820990", /\*商品ID\*/ "title": "储物箱整理抽屉式收纳柜", /\*商品短标题\*/ "jianjie": "【爆款返场，出口日本】进口PP材质，可叠加透明可视抽屉收纳盒，分格抽屉物品分类齐全，区域分化一目了然，自由组合，大容量，让你居家生活更舒心，收纳你的生活！【赠运费险】", /\*商品简介\*/ "pict\_url": "https://img.alicdn.com/bao/......jpg", /\*商品主图\*/ "user\_type": "0", /\*是否天猫，0是淘宝，1是天猫\*/ "seller\_id": "3073204721", /\*卖家ID\*/ /\*店铺地址可自行拼接，http://store.taobao.com/shop/view\_shop.htm?user\_number\_id=3073204721 \*/ "shop\_dsr": "4.80", /\*商品描述分\*/ "volume": "169", /\*年销量\*/ "size": "26.90", /\*折扣价\*/ "quanhou\_jiage": "16.90", /\*券后价\*/ "date\_time\_yongjin": "2018/11/8 14:45:22", /\*数据更新时间\*/ "tkrate3": "30.00", /\*佣金比率\*/ "yongjin\_type": "MKT", /\*佣金类型\*/ "coupon\_id": "fe31d6c8132c462ba59bd72e03d2eae0", /\*优惠券ID 该商品如无优惠券，此字段为空\*/ "coupon\_start\_time": "2018-11-05 00:00:00", /\*优惠券开始时间 该商品如无优惠券，此字段为空\*/ "coupon\_end\_time": "2018-11-10 23:59:59", /\*优惠券结束时间 该商品如无优惠券，此字段为空\*/ "coupon\_info\_money": "10", /\*优惠券金额 该商品如无优惠券，此字段为0\*/ "coupon\_total\_count": "100000", /\*优惠券总数量 该商品如无优惠券，此字段为0\*/ "coupon\_remain\_count": "99965", /\*优惠券剩余数量 该商品如无优惠券，此字段为0\*/ "coupon\_info": "满80.00元减10元", /\*优惠券信息\*/ "juhuasuan": "0", /\*是否聚划算，1是\*/ "taoqianggou": "0", /\*是否淘抢购，1是\*/ "haitao": "0", /\*是否海淘，1是\*/ "jiyoujia": "0", /\*是否极有家，1是\*/ "jinpaimaijia": "0", /\*是否金牌卖家，1是\*/ "pinpai": "0", /\*是否精选品牌，1是\*/ "pinpai\_name": "", /\*品牌名称\*/ "yunfeixian": "0", /\*是否有运费险，1有\*/ "nick": "\*\*\*旗舰店", /\*卖家昵称\*/ "small\_images": "https://img.alicdn.com/bao/.......jpg|https://img.alicdn.com/bao/.......jpg", /\*商品小图列表\*/ "white\_image": "https://img.alicdn.com/bao/......jpg", /\*商品白底图\*/ "tao\_title": "抽屉式收纳箱塑料储物箱整理箱衣服物家用宿舍衣柜内衣内裤收纳盒", /\*商品长标题\*/ "provcity": "浙江 杭州", /\*宝贝所在地\*/ "shop\_title": "\*\*\*旗舰店", /\*店铺名称\*/ "zhibo\_url": "https://cloud.video.taobao.com/play/u/849090736/p/2/e/6/t/1/219853683676.mp4", /\*视频地址\*/ "sellCount": "860000", /\*淘宝网页实时总销量\*/ "commentCount": "250000", /\*评论数量\*/ "favcount": "180000", /\*收藏数量\*/ "score1": "4.9", /\*宝贝描述分\*/ "score2": "4.9", /\*卖家服务分\*/ "score3": "4.9", /\*物流服务\*/ "creditLevel": "15", /\*店铺等级，一星 二星 三星 四星 五星 一钻 二钻 三钻 四钻 五钻 一皇冠 二皇冠 三皇冠 四皇冠 五皇冠 一金冠 二金冠 三金冠 四金冠 五金冠\*/ "shopIcon": "//img.alicdn.com/imgextra/de/a6/TB10dC8SX.jpg", /\*店铺logo\*/ "pcDescContent": "1.jpg|2.jpg|3.jpg|4.jpg|5.jpg", /\*图文详情图片地址\*/ "item\_url": "https://item.taobao.com/item.htm?id=581801364036", /\*商品url\*/ "category\_id": "50006126", /\*叶子类目id\*/ "category\_name": "收纳盒", /\*叶子类目name\*/ "level\_one\_category\_id": "122928002", /\*一级类目id\*/ "level\_one\_category\_name": "收纳整理", /\*一级类目name\*/ "tkfee3": "5.07", /\*返佣金额\*/ "biaoqian": ""满300元,省30元"", /\*店铺活动\*/ "tag": "赶紧拯救你厨房的油腻吧😂<br>植护🌿加厚厨房抽纸<br>高密度纤维纸张，吸油💧很不错<br>5包装|￥14.8元💰<br>✔厨房小帮手，用处多多", /\*朋友圈文案，需要自己进行urldecode\*/ "date\_time": "2018/11/8 14:45:22" /\*数据添加时间\*/ }\] };
```json
{
	"status": 200,                      /\*状态\*/
	"content": \[{
		"code": "6646",                 /\*折淘客编号\*/
		"type\_one\_id": "1",             /\*分类ID，可参考折淘客分类\*/
		"tao\_id": "554832820990",       /\*商品ID\*/
		"title": "储物箱整理抽屉式收纳柜",                   /\*商品短标题\*/
		"jianjie": "【爆款返场，出口日本】进口PP材质，可叠加透明可视抽屉收纳盒，分格抽屉物品分类齐全，区域分化一目了然，自由组合，大容量，让你居家生活更舒心，收纳你的生活！【赠运费险】", /\*商品简介\*/
		"pict\_url": "https://img.alicdn.com/bao/......jpg",                   /\*商品主图\*/
		"user\_type": "0",                   /\*是否天猫，0是淘宝，1是天猫\*/
		"seller\_id": "3073204721",                   /\*卖家ID\*/ /\*店铺地址可自行拼接，http://store.taobao.com/shop/view\_shop.htm?user\_number\_id=3073204721 \*/
		"shop\_dsr": "4.80",                   /\*商品描述分\*/
		"volume": "169",                   /\*年销量\*/
		"size": "26.90",                   /\*折扣价\*/
		"quanhou\_jiage": "16.90",                   /\*券后价\*/
		"date\_time\_yongjin": "2018/11/8 14:45:22",                   /\*数据更新时间\*/
		"tkrate3": "30.00",                   /\*佣金比率\*/
		"yongjin\_type": "MKT",                   /\*佣金类型\*/
		"coupon\_id": "fe31d6c8132c462ba59bd72e03d2eae0",                   /\*优惠券ID 该商品如无优惠券，此字段为空\*/
		"coupon\_start\_time": "2018-11-05 00:00:00",                   /\*优惠券开始时间 该商品如无优惠券，此字段为空\*/
		"coupon\_end\_time": "2018-11-10 23:59:59",                   /\*优惠券结束时间 该商品如无优惠券，此字段为空\*/
		"coupon\_info\_money": "10",                   /\*优惠券金额 该商品如无优惠券，此字段为0\*/
		"coupon\_total\_count": "100000",                   /\*优惠券总数量 该商品如无优惠券，此字段为0\*/
		"coupon\_remain\_count": "99965",                   /\*优惠券剩余数量 该商品如无优惠券，此字段为0\*/
		"coupon\_info": "满80.00元减10元",                   /\*优惠券信息\*/
		"juhuasuan": "0",                       /\*是否聚划算，1是\*/
		"taoqianggou": "0",                     /\*是否淘抢购，1是\*/
		"haitao": "0",                          /\*是否海淘，1是\*/
		"jiyoujia": "0",                        /\*是否极有家，1是\*/
		"jinpaimaijia": "0",                    /\*是否金牌卖家，1是\*/
		"pinpai": "0",                          /\*是否精选品牌，1是\*/
		"pinpai\_name": "",                      /\*品牌名称\*/
		"yunfeixian": "0",                      /\*是否有运费险，1有\*/
		"nick": "\*\*\*旗舰店",                    /\*卖家昵称\*/
		"small\_images": "https://img.alicdn.com/bao/.......jpg|https://img.alicdn.com/bao/.......jpg",                   /\*商品小图列表\*/
		"white\_image": "https://img.alicdn.com/bao/......jpg",                   /\*商品白底图\*/
		"tao\_title": "抽屉式收纳箱塑料储物箱整理箱衣服物家用宿舍衣柜内衣内裤收纳盒",                   /\*商品长标题\*/
		"provcity": "浙江 杭州",                   /\*宝贝所在地\*/
		"shop\_title": "\*\*\*旗舰店",                   /\*店铺名称\*/
		"zhibo\_url": "https://cloud.video.taobao.com/play/u/849090736/p/2/e/6/t/1/219853683676.mp4",                   /\*视频地址\*/
		"sellCount": "860000",                   /\*淘宝网页实时总销量\*/
		"commentCount": "250000",                   /\*评论数量\*/
		"favcount": "180000",                   /\*收藏数量\*/
		"score1": "4.9",                   /\*宝贝描述分\*/
		"score2": "4.9",                   /\*卖家服务分\*/
		"score3": "4.9",                   /\*物流服务分\*/
		"creditLevel": "15",                   /\*店铺等级（1-20），一星 二星 三星 四星 五星 一钻 二钻 三钻 四钻 五钻 一皇冠 二皇冠 三皇冠 四皇冠 五皇冠  一金冠 二金冠 三金冠 四金冠 五金冠\*/
		"shopIcon": "//img.alicdn.com/imgextra/de/a6/TB10dC8SX.jpg",       /\*店铺logo\*/
		"pcDescContent": "1.jpg|2.jpg|3.jpg|4.jpg|5.jpg",  /\*图文详情图片地址\*/
		"item\_url": "https://item.taobao.com/item.htm?id=581801364036",  /\*商品url\*/
		"category\_id": "50006126",                   /\*叶子类目id\*/
		"category\_name": "收纳盒",                   /\*叶子类目name\*/
		"level\_one\_category\_id": "122928002",                   /\*一级类目id\*/
		"level\_one\_category\_name": "收纳整理",                   /\*一级类目name\*/
		"tkfee3": "5.07",                   /\*返佣金额\*/
		"biaoqian": ""满300元,省30元"",                   /\*店铺活动\*/
		"tag": "赶紧拯救你厨房的油腻吧😂植护🌿加厚厨房抽纸高密度纤维纸张，吸油💧很不错5包装|￥14.8元💰✔厨房小帮手，用处多多",                   /\*朋友圈文案，需要自己进行urldecode\*/
		"date\_time": "2018/11/8 14:45:22"                   /\*数据添加时间\*/
	}\]
}
```

**失败返回示例：**

var tmp = { "status": 301 };
```json
{
	"status": 301                      /\*状态\*/
}
```

[效果展示](http://cms.zhetaoke.com:10000/m/item/list.aspx?tianmaochaoshi=1)

---

## 店铺商品API

**原始链接**: [店铺商品API](https://www.zhetaoke.com/user/extend/extend_lingquan_dianpu.aspx)

**店铺商品API---本接口采用12台16核心64G内存服务器，江苏和北京双机房，负载均衡技术，1G独享带宽，放心调用。**

店铺商品API：返回指定店铺商品列表，动态描述分≥4.6的商品列表。

**接口地址：** https://api.zhetaoke.com:10001/api/api\_all.ashx?appkey=0a565628b2fb4712902712734cec2feb&page=1&page\_size=20&sort=new&seller\_id=

店铺商品API：返回指定查询条件的商品总数。

**接口地址：** https://api.zhetaoke.com:10001/api/api\_all.ashx?appkey=0a565628b2fb4712902712734cec2feb&page=1&page\_size=20&sort=new&seller\_id=&total\_count=1

**返回数据：**json

**请求方式：**get

**请求参数说明：**

|  | 名称 | 类型 | 必填 | 说明 |
| --- | --- | --- | --- | --- |
|   | appkey | string | 是 | 折淘客的对接秘钥appkey |
|   | page | int | 否 | 分页获取数据,第几页 |
|   | page\_size | int | 否 | 每页数据条数（默认每页20条），可自定义1-50之间 |
|   | sort | string | 否 | 商品排序方式，new：按照综合排序，  
total\_sale\_num\_asc：按照总销量从小到大排序，total\_sale\_num\_desc：按照总销量从大到小排序，  
sale\_num\_asc：按照年销量从小到大排序，sale\_num\_desc：按照年销量从大到小排序，  
commission\_rate\_asc：按照佣金比例从小到大排序，commission\_rate\_desc：按照佣金比例从大到小排序，  
price\_asc：按照价格从小到大排序，price\_desc：按照价格从大到小排序，  
coupon\_info\_money\_asc：按照优惠券金额从小到大排序，coupon\_info\_money\_desc：按照优惠券金额从大到小排序，  
shop\_level\_asc：按照店铺等级从低到高排序，shop\_level\_desc：按照店铺等级从高到低排序，  
tkfee\_asc：按照返佣金额从低到高排序，tkfee\_desc：按照返佣金额从高到低排序，  
code：按照code值从大到小排序，date\_time：按照更新时间排序，random：按照随机排序 |
|   | q | string | 否 | 店铺名称 |

**成功返回示例：**/\*重要事情说三遍！重要事情说三遍！重要事情说三遍！api\_all/api\_dongdong/api\_quantian/api\_shishi/api\_videos/api\_xiaoshi/api\_yongjin接口所有参数都可以自由组合。\*/

var tmp = { "status": 200, /\*状态\*/ "content": \[{ "code": "6646", /\*折淘客编号\*/ "type\_one\_id": "1", /\*分类ID，可参考折淘客分类\*/ "tao\_id": "554832820990", /\*商品ID\*/ "title": "储物箱整理抽屉式收纳柜", /\*商品短标题\*/ "jianjie": "【爆款返场，出口日本】进口PP材质，可叠加透明可视抽屉收纳盒，分格抽屉物品分类齐全，区域分化一目了然，自由组合，大容量，让你居家生活更舒心，收纳你的生活！【赠运费险】", /\*商品简介\*/ "pict\_url": "https://img.alicdn.com/bao/......jpg", /\*商品主图\*/ "user\_type": "0", /\*是否天猫，0是淘宝，1是天猫\*/ "seller\_id": "3073204721", /\*卖家ID\*/ /\*店铺地址可自行拼接，http://store.taobao.com/shop/view\_shop.htm?user\_number\_id=3073204721 \*/ "shop\_dsr": "4.80", /\*商品描述分\*/ "volume": "169", /\*年销量\*/ "size": "26.90", /\*折扣价\*/ "quanhou\_jiage": "16.90", /\*券后价\*/ "date\_time\_yongjin": "2018/11/8 14:45:22", /\*数据更新时间\*/ "tkrate3": "30.00", /\*佣金比率\*/ "yongjin\_type": "MKT", /\*佣金类型\*/ "coupon\_id": "fe31d6c8132c462ba59bd72e03d2eae0", /\*优惠券ID 该商品如无优惠券，此字段为空\*/ "coupon\_start\_time": "2018-11-05 00:00:00", /\*优惠券开始时间 该商品如无优惠券，此字段为空\*/ "coupon\_end\_time": "2018-11-10 23:59:59", /\*优惠券结束时间 该商品如无优惠券，此字段为空\*/ "coupon\_info\_money": "10", /\*优惠券金额 该商品如无优惠券，此字段为0\*/ "coupon\_total\_count": "100000", /\*优惠券总数量 该商品如无优惠券，此字段为0\*/ "coupon\_remain\_count": "99965", /\*优惠券剩余数量 该商品如无优惠券，此字段为0\*/ "coupon\_info": "满80.00元减10元", /\*优惠券信息\*/ "juhuasuan": "0", /\*是否聚划算，1是\*/ "taoqianggou": "0", /\*是否淘抢购，1是\*/ "haitao": "0", /\*是否海淘，1是\*/ "jiyoujia": "0", /\*是否极有家，1是\*/ "jinpaimaijia": "0", /\*是否金牌卖家，1是\*/ "pinpai": "0", /\*是否精选品牌，1是\*/ "pinpai\_name": "", /\*品牌名称\*/ "yunfeixian": "0", /\*是否有运费险，1有\*/ "nick": "\*\*\*旗舰店", /\*卖家昵称\*/ "small\_images": "https://img.alicdn.com/bao/.......jpg|https://img.alicdn.com/bao/.......jpg", /\*商品小图列表\*/ "white\_image": "https://img.alicdn.com/bao/......jpg", /\*商品白底图\*/ "tao\_title": "抽屉式收纳箱塑料储物箱整理箱衣服物家用宿舍衣柜内衣内裤收纳盒", /\*商品长标题\*/ "provcity": "浙江 杭州", /\*宝贝所在地\*/ "shop\_title": "\*\*\*旗舰店", /\*店铺名称\*/ "zhibo\_url": "https://cloud.video.taobao.com/play/u/849090736/p/2/e/6/t/1/219853683676.mp4", /\*视频地址\*/ "sellCount": "860000", /\*淘宝网页实时总销量\*/ "commentCount": "250000", /\*评论数量\*/ "favcount": "180000", /\*收藏数量\*/ "score1": "4.9", /\*宝贝描述分\*/ "score2": "4.9", /\*卖家服务分\*/ "score3": "4.9", /\*物流服务\*/ "creditLevel": "15", /\*店铺等级，一星 二星 三星 四星 五星 一钻 二钻 三钻 四钻 五钻 一皇冠 二皇冠 三皇冠 四皇冠 五皇冠 一金冠 二金冠 三金冠 四金冠 五金冠\*/ "shopIcon": "//img.alicdn.com/imgextra/de/a6/TB10dC8SX.jpg", /\*店铺logo\*/ "pcDescContent": "1.jpg|2.jpg|3.jpg|4.jpg|5.jpg", /\*图文详情图片地址\*/ "item\_url": "https://item.taobao.com/item.htm?id=581801364036", /\*商品url\*/ "category\_id": "50006126", /\*叶子类目id\*/ "category\_name": "收纳盒", /\*叶子类目name\*/ "level\_one\_category\_id": "122928002", /\*一级类目id\*/ "level\_one\_category\_name": "收纳整理", /\*一级类目name\*/ "tkfee3": "5.07", /\*返佣金额\*/ "biaoqian": ""满300元,省30元"", /\*店铺活动\*/ "tag": "赶紧拯救你厨房的油腻吧😂<br>植护🌿加厚厨房抽纸<br>高密度纤维纸张，吸油💧很不错<br>5包装|￥14.8元💰<br>✔厨房小帮手，用处多多", /\*朋友圈文案，需要自己进行urldecode\*/ "date\_time": "2018/11/8 14:45:22" /\*数据添加时间\*/ }\] };
```json
{
	"status": 200,                      /\*状态\*/
	"content": \[{
		"code": "6646",                 /\*折淘客编号\*/
		"type\_one\_id": "1",             /\*分类ID，可参考折淘客分类\*/
		"tao\_id": "554832820990",       /\*商品ID\*/
		"title": "储物箱整理抽屉式收纳柜",                   /\*商品短标题\*/
		"jianjie": "【爆款返场，出口日本】进口PP材质，可叠加透明可视抽屉收纳盒，分格抽屉物品分类齐全，区域分化一目了然，自由组合，大容量，让你居家生活更舒心，收纳你的生活！【赠运费险】", /\*商品简介\*/
		"pict\_url": "https://img.alicdn.com/bao/......jpg",                   /\*商品主图\*/
		"user\_type": "0",                   /\*是否天猫，0是淘宝，1是天猫\*/
		"seller\_id": "3073204721",                   /\*卖家ID\*/ /\*店铺地址可自行拼接，http://store.taobao.com/shop/view\_shop.htm?user\_number\_id=3073204721 \*/
		"shop\_dsr": "4.80",                   /\*商品描述分\*/
		"volume": "169",                   /\*年销量\*/
		"size": "26.90",                   /\*折扣价\*/
		"quanhou\_jiage": "16.90",                   /\*券后价\*/
		"date\_time\_yongjin": "2018/11/8 14:45:22",                   /\*数据更新时间\*/
		"tkrate3": "30.00",                   /\*佣金比率\*/
		"yongjin\_type": "MKT",                   /\*佣金类型\*/
		"coupon\_id": "fe31d6c8132c462ba59bd72e03d2eae0",                   /\*优惠券ID 该商品如无优惠券，此字段为空\*/
		"coupon\_start\_time": "2018-11-05 00:00:00",                   /\*优惠券开始时间 该商品如无优惠券，此字段为空\*/
		"coupon\_end\_time": "2018-11-10 23:59:59",                   /\*优惠券结束时间 该商品如无优惠券，此字段为空\*/
		"coupon\_info\_money": "10",                   /\*优惠券金额 该商品如无优惠券，此字段为0\*/
		"coupon\_total\_count": "100000",                   /\*优惠券总数量 该商品如无优惠券，此字段为0\*/
		"coupon\_remain\_count": "99965",                   /\*优惠券剩余数量 该商品如无优惠券，此字段为0\*/
		"coupon\_info": "满80.00元减10元",                   /\*优惠券信息\*/
		"juhuasuan": "0",                       /\*是否聚划算，1是\*/
		"taoqianggou": "0",                     /\*是否淘抢购，1是\*/
		"haitao": "0",                          /\*是否海淘，1是\*/
		"jiyoujia": "0",                        /\*是否极有家，1是\*/
		"jinpaimaijia": "0",                    /\*是否金牌卖家，1是\*/
		"pinpai": "0",                          /\*是否精选品牌，1是\*/
		"pinpai\_name": "",                      /\*品牌名称\*/
		"yunfeixian": "0",                      /\*是否有运费险，1有\*/
		"nick": "\*\*\*旗舰店",                    /\*卖家昵称\*/
		"small\_images": "https://img.alicdn.com/bao/.......jpg|https://img.alicdn.com/bao/.......jpg",                   /\*商品小图列表\*/
		"white\_image": "https://img.alicdn.com/bao/......jpg",                   /\*商品白底图\*/
		"tao\_title": "抽屉式收纳箱塑料储物箱整理箱衣服物家用宿舍衣柜内衣内裤收纳盒",                   /\*商品长标题\*/
		"provcity": "浙江 杭州",                   /\*宝贝所在地\*/
		"shop\_title": "\*\*\*旗舰店",                   /\*店铺名称\*/
		"zhibo\_url": "https://cloud.video.taobao.com/play/u/849090736/p/2/e/6/t/1/219853683676.mp4",                   /\*视频地址\*/
		"sellCount": "860000",                   /\*淘宝网页实时总销量\*/
		"commentCount": "250000",                   /\*评论数量\*/
		"favcount": "180000",                   /\*收藏数量\*/
		"score1": "4.9",                   /\*宝贝描述分\*/
		"score2": "4.9",                   /\*卖家服务分\*/
		"score3": "4.9",                   /\*物流服务分\*/
		"creditLevel": "15",                   /\*店铺等级（1-20），一星 二星 三星 四星 五星 一钻 二钻 三钻 四钻 五钻 一皇冠 二皇冠 三皇冠 四皇冠 五皇冠  一金冠 二金冠 三金冠 四金冠 五金冠\*/
		"shopIcon": "//img.alicdn.com/imgextra/de/a6/TB10dC8SX.jpg",       /\*店铺logo\*/
		"pcDescContent": "1.jpg|2.jpg|3.jpg|4.jpg|5.jpg",  /\*图文详情图片地址\*/
		"item\_url": "https://item.taobao.com/item.htm?id=581801364036",  /\*商品url\*/
		"category\_id": "50006126",                   /\*叶子类目id\*/
		"category\_name": "收纳盒",                   /\*叶子类目name\*/
		"level\_one\_category\_id": "122928002",                   /\*一级类目id\*/
		"level\_one\_category\_name": "收纳整理",                   /\*一级类目name\*/
		"tkfee3": "5.07",                   /\*返佣金额\*/
		"biaoqian": ""满300元,省30元"",                   /\*店铺活动\*/
		"tag": "赶紧拯救你厨房的油腻吧😂植护🌿加厚厨房抽纸高密度纤维纸张，吸油💧很不错5包装|￥14.8元💰✔厨房小帮手，用处多多",                   /\*朋友圈文案，需要自己进行urldecode\*/
		"date\_time": "2018/11/8 14:45:22"                   /\*数据添加时间\*/
	}\]
}
```

**失败返回示例：**

var tmp = { "status": 301 };
```json
{
	"status": 301                      /\*状态\*/
}
```

[效果展示](http://cms.zhetaoke.com:10000/m/item/list.aspx?tianmaochaoshi=1)

---

## 超高佣金商品API

**原始链接**: [超高佣金商品API](https://www.zhetaoke.com/user/extend/extend_lingquan_gaoyongjin.aspx)

**超高佣金商品API---本接口采用12台16核心64G内存服务器，江苏和北京双机房，负载均衡技术，1G独享带宽，放心调用。**

超高佣金商品API：返回佣金比例≥50的商品列表，返回佣金≥15%，动态描述分≥4.6的商品列表。

**接口地址：** https://api.zhetaoke.com:10001/api/api\_all.ashx?appkey=0a565628b2fb4712902712734cec2feb&page=1&page\_size=20&sort=new&commission\_rate\_start=50

超高佣金商品API：返回指定查询条件的商品总数。

**接口地址：** https://api.zhetaoke.com:10001/api/api\_all.ashx?appkey=0a565628b2fb4712902712734cec2feb&page=1&page\_size=20&sort=new&commission\_rate\_start=50&total\_count=1

**返回数据：**json

**请求方式：**get

**请求参数说明：**

|  | 名称 | 类型 | 必填 | 说明 |
| --- | --- | --- | --- | --- |
|   | appkey | string | 是 | 折淘客的对接秘钥appkey |
|   | page | int | 否 | 分页获取数据,第几页 |
|   | page\_size | int | 否 | 每页数据条数（默认每页20条），可自定义1-50之间 |
|   | sort | string | 否 | 商品排序方式，new：按照综合排序，  
total\_sale\_num\_asc：按照总销量从小到大排序，total\_sale\_num\_desc：按照总销量从大到小排序，  
sale\_num\_asc：按照年销量从小到大排序，sale\_num\_desc：按照年销量从大到小排序，  
commission\_rate\_asc：按照佣金比例从小到大排序，commission\_rate\_desc：按照佣金比例从大到小排序，  
price\_asc：按照价格从小到大排序，price\_desc：按照价格从大到小排序，  
coupon\_info\_money\_asc：按照优惠券金额从小到大排序，coupon\_info\_money\_desc：按照优惠券金额从大到小排序，  
shop\_level\_asc：按照店铺等级从低到高排序，shop\_level\_desc：按照店铺等级从高到低排序，  
tkfee\_asc：按照返佣金额从低到高排序，tkfee\_desc：按照返佣金额从高到低排序，  
code：按照code值从大到小排序，date\_time：按照更新时间排序，random：按照随机排序 |
|   | cid | int | 否 | 一级商品分类，值为空：全部商品，1：女装，2：母婴，3：美妆，4：居家日用，5：鞋品，6：美食，7：文娱车品，8：数码家电，9：男装，10：内衣，11：箱包，12：配饰，13：户外运动，14：家装家纺 |
|   | commission\_rate\_start | string | 否 | 佣金比例≥，值为空：全部商品，=20：佣金比例≥20的商品，=50：佣金比例≥50的商品 |

**成功返回示例：**/\*重要事情说三遍！重要事情说三遍！重要事情说三遍！api\_all/api\_dongdong/api\_quantian/api\_shishi/api\_videos/api\_xiaoshi/api\_yongjin接口所有参数都可以自由组合。\*/

var tmp = { "status": 200, /\*状态\*/ "content": \[{ "code": "6646", /\*折淘客编号\*/ "type\_one\_id": "1", /\*分类ID，可参考折淘客分类\*/ "tao\_id": "554832820990", /\*商品ID\*/ "title": "储物箱整理抽屉式收纳柜", /\*商品短标题\*/ "jianjie": "【爆款返场，出口日本】进口PP材质，可叠加透明可视抽屉收纳盒，分格抽屉物品分类齐全，区域分化一目了然，自由组合，大容量，让你居家生活更舒心，收纳你的生活！【赠运费险】", /\*商品简介\*/ "pict\_url": "https://img.alicdn.com/bao/......jpg", /\*商品主图\*/ "user\_type": "0", /\*是否天猫，0是淘宝，1是天猫\*/ "seller\_id": "3073204721", /\*卖家ID\*/ /\*店铺地址可自行拼接，http://store.taobao.com/shop/view\_shop.htm?user\_number\_id=3073204721 \*/ "shop\_dsr": "4.80", /\*商品描述分\*/ "volume": "169", /\*年销量\*/ "size": "26.90", /\*折扣价\*/ "quanhou\_jiage": "16.90", /\*券后价\*/ "date\_time\_yongjin": "2018/11/8 14:45:22", /\*数据更新时间\*/ "tkrate3": "30.00", /\*佣金比率\*/ "yongjin\_type": "MKT", /\*佣金类型\*/ "coupon\_id": "fe31d6c8132c462ba59bd72e03d2eae0", /\*优惠券ID 该商品如无优惠券，此字段为空\*/ "coupon\_start\_time": "2018-11-05 00:00:00", /\*优惠券开始时间 该商品如无优惠券，此字段为空\*/ "coupon\_end\_time": "2018-11-10 23:59:59", /\*优惠券结束时间 该商品如无优惠券，此字段为空\*/ "coupon\_info\_money": "10", /\*优惠券金额 该商品如无优惠券，此字段为0\*/ "coupon\_total\_count": "100000", /\*优惠券总数量 该商品如无优惠券，此字段为0\*/ "coupon\_remain\_count": "99965", /\*优惠券剩余数量 该商品如无优惠券，此字段为0\*/ "coupon\_info": "满80.00元减10元", /\*优惠券信息\*/ "juhuasuan": "0", /\*是否聚划算，1是\*/ "taoqianggou": "0", /\*是否淘抢购，1是\*/ "haitao": "0", /\*是否海淘，1是\*/ "jiyoujia": "0", /\*是否极有家，1是\*/ "jinpaimaijia": "0", /\*是否金牌卖家，1是\*/ "pinpai": "0", /\*是否精选品牌，1是\*/ "pinpai\_name": "", /\*品牌名称\*/ "yunfeixian": "0", /\*是否有运费险，1有\*/ "nick": "\*\*\*旗舰店", /\*卖家昵称\*/ "small\_images": "https://img.alicdn.com/bao/.......jpg|https://img.alicdn.com/bao/.......jpg", /\*商品小图列表\*/ "white\_image": "https://img.alicdn.com/bao/......jpg", /\*商品白底图\*/ "tao\_title": "抽屉式收纳箱塑料储物箱整理箱衣服物家用宿舍衣柜内衣内裤收纳盒", /\*商品长标题\*/ "provcity": "浙江 杭州", /\*宝贝所在地\*/ "shop\_title": "\*\*\*旗舰店", /\*店铺名称\*/ "zhibo\_url": "https://cloud.video.taobao.com/play/u/849090736/p/2/e/6/t/1/219853683676.mp4", /\*视频地址\*/ "sellCount": "860000", /\*淘宝网页实时总销量\*/ "commentCount": "250000", /\*评论数量\*/ "favcount": "180000", /\*收藏数量\*/ "score1": "4.9", /\*宝贝描述分\*/ "score2": "4.9", /\*卖家服务分\*/ "score3": "4.9", /\*物流服务\*/ "creditLevel": "15", /\*店铺等级，一星 二星 三星 四星 五星 一钻 二钻 三钻 四钻 五钻 一皇冠 二皇冠 三皇冠 四皇冠 五皇冠 一金冠 二金冠 三金冠 四金冠 五金冠\*/ "shopIcon": "//img.alicdn.com/imgextra/de/a6/TB10dC8SX.jpg", /\*店铺logo\*/ "pcDescContent": "1.jpg|2.jpg|3.jpg|4.jpg|5.jpg", /\*图文详情图片地址\*/ "item\_url": "https://item.taobao.com/item.htm?id=581801364036", /\*商品url\*/ "category\_id": "50006126", /\*叶子类目id\*/ "category\_name": "收纳盒", /\*叶子类目name\*/ "level\_one\_category\_id": "122928002", /\*一级类目id\*/ "level\_one\_category\_name": "收纳整理", /\*一级类目name\*/ "tkfee3": "5.07", /\*返佣金额\*/ "biaoqian": ""满300元,省30元"", /\*店铺活动\*/ "tag": "赶紧拯救你厨房的油腻吧😂<br>植护🌿加厚厨房抽纸<br>高密度纤维纸张，吸油💧很不错<br>5包装|￥14.8元💰<br>✔厨房小帮手，用处多多", /\*朋友圈文案，需要自己进行urldecode\*/ "date\_time": "2018/11/8 14:45:22" /\*数据添加时间\*/ }\] };
```json
{
	"status": 200,                      /\*状态\*/
	"content": \[{
		"code": "6646",                 /\*折淘客编号\*/
		"type\_one\_id": "1",             /\*分类ID，可参考折淘客分类\*/
		"tao\_id": "554832820990",       /\*商品ID\*/
		"title": "储物箱整理抽屉式收纳柜",                   /\*商品短标题\*/
		"jianjie": "【爆款返场，出口日本】进口PP材质，可叠加透明可视抽屉收纳盒，分格抽屉物品分类齐全，区域分化一目了然，自由组合，大容量，让你居家生活更舒心，收纳你的生活！【赠运费险】", /\*商品简介\*/
		"pict\_url": "https://img.alicdn.com/bao/......jpg",                   /\*商品主图\*/
		"user\_type": "0",                   /\*是否天猫，0是淘宝，1是天猫\*/
		"seller\_id": "3073204721",                   /\*卖家ID\*/ /\*店铺地址可自行拼接，http://store.taobao.com/shop/view\_shop.htm?user\_number\_id=3073204721 \*/
		"shop\_dsr": "4.80",                   /\*商品描述分\*/
		"volume": "169",                   /\*年销量\*/
		"size": "26.90",                   /\*折扣价\*/
		"quanhou\_jiage": "16.90",                   /\*券后价\*/
		"date\_time\_yongjin": "2018/11/8 14:45:22",                   /\*数据更新时间\*/
		"tkrate3": "30.00",                   /\*佣金比率\*/
		"yongjin\_type": "MKT",                   /\*佣金类型\*/
		"coupon\_id": "fe31d6c8132c462ba59bd72e03d2eae0",                   /\*优惠券ID 该商品如无优惠券，此字段为空\*/
		"coupon\_start\_time": "2018-11-05 00:00:00",                   /\*优惠券开始时间 该商品如无优惠券，此字段为空\*/
		"coupon\_end\_time": "2018-11-10 23:59:59",                   /\*优惠券结束时间 该商品如无优惠券，此字段为空\*/
		"coupon\_info\_money": "10",                   /\*优惠券金额 该商品如无优惠券，此字段为0\*/
		"coupon\_total\_count": "100000",                   /\*优惠券总数量 该商品如无优惠券，此字段为0\*/
		"coupon\_remain\_count": "99965",                   /\*优惠券剩余数量 该商品如无优惠券，此字段为0\*/
		"coupon\_info": "满80.00元减10元",                   /\*优惠券信息\*/
		"juhuasuan": "0",                       /\*是否聚划算，1是\*/
		"taoqianggou": "0",                     /\*是否淘抢购，1是\*/
		"haitao": "0",                          /\*是否海淘，1是\*/
		"jiyoujia": "0",                        /\*是否极有家，1是\*/
		"jinpaimaijia": "0",                    /\*是否金牌卖家，1是\*/
		"pinpai": "0",                          /\*是否精选品牌，1是\*/
		"pinpai\_name": "",                      /\*品牌名称\*/
		"yunfeixian": "0",                      /\*是否有运费险，1有\*/
		"nick": "\*\*\*旗舰店",                    /\*卖家昵称\*/
		"small\_images": "https://img.alicdn.com/bao/.......jpg|https://img.alicdn.com/bao/.......jpg",                   /\*商品小图列表\*/
		"white\_image": "https://img.alicdn.com/bao/......jpg",                   /\*商品白底图\*/
		"tao\_title": "抽屉式收纳箱塑料储物箱整理箱衣服物家用宿舍衣柜内衣内裤收纳盒",                   /\*商品长标题\*/
		"provcity": "浙江 杭州",                   /\*宝贝所在地\*/
		"shop\_title": "\*\*\*旗舰店",                   /\*店铺名称\*/
		"zhibo\_url": "https://cloud.video.taobao.com/play/u/849090736/p/2/e/6/t/1/219853683676.mp4",                   /\*视频地址\*/
		"sellCount": "860000",                   /\*淘宝网页实时总销量\*/
		"commentCount": "250000",                   /\*评论数量\*/
		"favcount": "180000",                   /\*收藏数量\*/
		"score1": "4.9",                   /\*宝贝描述分\*/
		"score2": "4.9",                   /\*卖家服务分\*/
		"score3": "4.9",                   /\*物流服务分\*/
		"creditLevel": "15",                   /\*店铺等级（1-20），一星 二星 三星 四星 五星 一钻 二钻 三钻 四钻 五钻 一皇冠 二皇冠 三皇冠 四皇冠 五皇冠  一金冠 二金冠 三金冠 四金冠 五金冠\*/
		"shopIcon": "//img.alicdn.com/imgextra/de/a6/TB10dC8SX.jpg",       /\*店铺logo\*/
		"pcDescContent": "1.jpg|2.jpg|3.jpg|4.jpg|5.jpg",  /\*图文详情图片地址\*/
		"item\_url": "https://item.taobao.com/item.htm?id=581801364036",  /\*商品url\*/
		"category\_id": "50006126",                   /\*叶子类目id\*/
		"category\_name": "收纳盒",                   /\*叶子类目name\*/
		"level\_one\_category\_id": "122928002",                   /\*一级类目id\*/
		"level\_one\_category\_name": "收纳整理",                   /\*一级类目name\*/
		"tkfee3": "5.07",                   /\*返佣金额\*/
		"biaoqian": ""满300元,省30元"",                   /\*店铺活动\*/
		"tag": "赶紧拯救你厨房的油腻吧😂植护🌿加厚厨房抽纸高密度纤维纸张，吸油💧很不错5包装|￥14.8元💰✔厨房小帮手，用处多多",                   /\*朋友圈文案，需要自己进行urldecode\*/
		"date\_time": "2018/11/8 14:45:22"                   /\*数据添加时间\*/
	}\]
}
```

**失败返回示例：**

var tmp = { "status": 301 };
```json
{
	"status": 301                      /\*状态\*/
}
```

---

## 超高销量商品API

**原始链接**: [超高销量商品API](https://www.zhetaoke.com/user/extend/extend_lingquan_gaoxiaoliang.aspx)

**超高销量商品API---本接口采用12台16核心64G内存服务器，江苏和北京双机房，负载均衡技术，1G独享带宽，放心调用。**

超高销量商品API：返回年销量≥10000的商品列表，返回佣金≥15%，动态描述分≥4.6的商品列表。

**接口地址：** https://api.zhetaoke.com:10001/api/api\_all.ashx?appkey=0a565628b2fb4712902712734cec2feb&page=1&page\_size=20&sort=new&sale\_num\_start=10000

超高销量商品API：返回指定查询条件的商品总数。

**接口地址：** https://api.zhetaoke.com:10001/api/api\_all.ashx?appkey=0a565628b2fb4712902712734cec2feb&page=1&page\_size=20&sort=new&sale\_num\_start=10000&total\_count=1

**返回数据：**json

**请求方式：**get

**请求参数说明：**

|  | 名称 | 类型 | 必填 | 说明 |
| --- | --- | --- | --- | --- |
|   | appkey | string | 是 | 折淘客的对接秘钥appkey |
|   | page | int | 否 | 分页获取数据,第几页 |
|   | page\_size | int | 否 | 每页数据条数（默认每页20条），可自定义1-50之间 |
|   | sort | string | 否 | 商品排序方式，new：按照综合排序，  
total\_sale\_num\_asc：按照总销量从小到大排序，total\_sale\_num\_desc：按照总销量从大到小排序，  
sale\_num\_asc：按照年销量从小到大排序，sale\_num\_desc：按照年销量从大到小排序，  
commission\_rate\_asc：按照佣金比例从小到大排序，commission\_rate\_desc：按照佣金比例从大到小排序，  
price\_asc：按照价格从小到大排序，price\_desc：按照价格从大到小排序，  
coupon\_info\_money\_asc：按照优惠券金额从小到大排序，coupon\_info\_money\_desc：按照优惠券金额从大到小排序，  
shop\_level\_asc：按照店铺等级从低到高排序，shop\_level\_desc：按照店铺等级从高到低排序，  
tkfee\_asc：按照返佣金额从低到高排序，tkfee\_desc：按照返佣金额从高到低排序，  
code：按照code值从大到小排序，date\_time：按照更新时间排序，random：按照随机排序 |
|   | sale\_num\_start | string | 否 | 年销量≥ 值为空：全部商品，=500：年销量≥500的商品，=10000：年销量≥10000的商品 |

**成功返回示例：**/\*重要事情说三遍！重要事情说三遍！重要事情说三遍！api\_all/api\_dongdong/api\_quantian/api\_shishi/api\_videos/api\_xiaoshi/api\_yongjin接口所有参数都可以自由组合。\*/

var tmp = { "status": 200, /\*状态\*/ "content": \[{ "code": "6646", /\*折淘客编号\*/ "type\_one\_id": "1", /\*分类ID，可参考折淘客分类\*/ "tao\_id": "554832820990", /\*商品ID\*/ "title": "储物箱整理抽屉式收纳柜", /\*商品短标题\*/ "jianjie": "【爆款返场，出口日本】进口PP材质，可叠加透明可视抽屉收纳盒，分格抽屉物品分类齐全，区域分化一目了然，自由组合，大容量，让你居家生活更舒心，收纳你的生活！【赠运费险】", /\*商品简介\*/ "pict\_url": "https://img.alicdn.com/bao/......jpg", /\*商品主图\*/ "user\_type": "0", /\*是否天猫，0是淘宝，1是天猫\*/ "seller\_id": "3073204721", /\*卖家ID\*/ /\*店铺地址可自行拼接，http://store.taobao.com/shop/view\_shop.htm?user\_number\_id=3073204721 \*/ "shop\_dsr": "4.80", /\*商品描述分\*/ "volume": "169", /\*年销量\*/ "size": "26.90", /\*折扣价\*/ "quanhou\_jiage": "16.90", /\*券后价\*/ "date\_time\_yongjin": "2018/11/8 14:45:22", /\*数据更新时间\*/ "tkrate3": "30.00", /\*佣金比率\*/ "yongjin\_type": "MKT", /\*佣金类型\*/ "coupon\_id": "fe31d6c8132c462ba59bd72e03d2eae0", /\*优惠券ID 该商品如无优惠券，此字段为空\*/ "coupon\_start\_time": "2018-11-05 00:00:00", /\*优惠券开始时间 该商品如无优惠券，此字段为空\*/ "coupon\_end\_time": "2018-11-10 23:59:59", /\*优惠券结束时间 该商品如无优惠券，此字段为空\*/ "coupon\_info\_money": "10", /\*优惠券金额 该商品如无优惠券，此字段为0\*/ "coupon\_total\_count": "100000", /\*优惠券总数量 该商品如无优惠券，此字段为0\*/ "coupon\_remain\_count": "99965", /\*优惠券剩余数量 该商品如无优惠券，此字段为0\*/ "coupon\_info": "满80.00元减10元", /\*优惠券信息\*/ "juhuasuan": "0", /\*是否聚划算，1是\*/ "taoqianggou": "0", /\*是否淘抢购，1是\*/ "haitao": "0", /\*是否海淘，1是\*/ "jiyoujia": "0", /\*是否极有家，1是\*/ "jinpaimaijia": "0", /\*是否金牌卖家，1是\*/ "pinpai": "0", /\*是否精选品牌，1是\*/ "pinpai\_name": "", /\*品牌名称\*/ "yunfeixian": "0", /\*是否有运费险，1有\*/ "nick": "\*\*\*旗舰店", /\*卖家昵称\*/ "small\_images": "https://img.alicdn.com/bao/.......jpg|https://img.alicdn.com/bao/.......jpg", /\*商品小图列表\*/ "white\_image": "https://img.alicdn.com/bao/......jpg", /\*商品白底图\*/ "tao\_title": "抽屉式收纳箱塑料储物箱整理箱衣服物家用宿舍衣柜内衣内裤收纳盒", /\*商品长标题\*/ "provcity": "浙江 杭州", /\*宝贝所在地\*/ "shop\_title": "\*\*\*旗舰店", /\*店铺名称\*/ "zhibo\_url": "https://cloud.video.taobao.com/play/u/849090736/p/2/e/6/t/1/219853683676.mp4", /\*视频地址\*/ "sellCount": "860000", /\*淘宝网页实时总销量\*/ "commentCount": "250000", /\*评论数量\*/ "favcount": "180000", /\*收藏数量\*/ "score1": "4.9", /\*宝贝描述分\*/ "score2": "4.9", /\*卖家服务分\*/ "score3": "4.9", /\*物流服务\*/ "creditLevel": "15", /\*店铺等级，一星 二星 三星 四星 五星 一钻 二钻 三钻 四钻 五钻 一皇冠 二皇冠 三皇冠 四皇冠 五皇冠 一金冠 二金冠 三金冠 四金冠 五金冠\*/ "shopIcon": "//img.alicdn.com/imgextra/de/a6/TB10dC8SX.jpg", /\*店铺logo\*/ "pcDescContent": "1.jpg|2.jpg|3.jpg|4.jpg|5.jpg", /\*图文详情图片地址\*/ "item\_url": "https://item.taobao.com/item.htm?id=581801364036", /\*商品url\*/ "category\_id": "50006126", /\*叶子类目id\*/ "category\_name": "收纳盒", /\*叶子类目name\*/ "level\_one\_category\_id": "122928002", /\*一级类目id\*/ "level\_one\_category\_name": "收纳整理", /\*一级类目name\*/ "tkfee3": "5.07", /\*返佣金额\*/ "biaoqian": ""满300元,省30元"", /\*店铺活动\*/ "tag": "赶紧拯救你厨房的油腻吧😂<br>植护🌿加厚厨房抽纸<br>高密度纤维纸张，吸油💧很不错<br>5包装|￥14.8元💰<br>✔厨房小帮手，用处多多", /\*朋友圈文案，需要自己进行urldecode\*/ "date\_time": "2018/11/8 14:45:22" /\*数据添加时间\*/ }\] };
```json
{
	"status": 200,                      /\*状态\*/
	"content": \[{
		"code": "6646",                 /\*折淘客编号\*/
		"type\_one\_id": "1",             /\*分类ID，可参考折淘客分类\*/
		"tao\_id": "554832820990",       /\*商品ID\*/
		"title": "储物箱整理抽屉式收纳柜",                   /\*商品短标题\*/
		"jianjie": "【爆款返场，出口日本】进口PP材质，可叠加透明可视抽屉收纳盒，分格抽屉物品分类齐全，区域分化一目了然，自由组合，大容量，让你居家生活更舒心，收纳你的生活！【赠运费险】", /\*商品简介\*/
		"pict\_url": "https://img.alicdn.com/bao/......jpg",                   /\*商品主图\*/
		"user\_type": "0",                   /\*是否天猫，0是淘宝，1是天猫\*/
		"seller\_id": "3073204721",                   /\*卖家ID\*/ /\*店铺地址可自行拼接，http://store.taobao.com/shop/view\_shop.htm?user\_number\_id=3073204721 \*/
		"shop\_dsr": "4.80",                   /\*商品描述分\*/
		"volume": "169",                   /\*年销量\*/
		"size": "26.90",                   /\*折扣价\*/
		"quanhou\_jiage": "16.90",                   /\*券后价\*/
		"date\_time\_yongjin": "2018/11/8 14:45:22",                   /\*数据更新时间\*/
		"tkrate3": "30.00",                   /\*佣金比率\*/
		"yongjin\_type": "MKT",                   /\*佣金类型\*/
		"coupon\_id": "fe31d6c8132c462ba59bd72e03d2eae0",                   /\*优惠券ID 该商品如无优惠券，此字段为空\*/
		"coupon\_start\_time": "2018-11-05 00:00:00",                   /\*优惠券开始时间 该商品如无优惠券，此字段为空\*/
		"coupon\_end\_time": "2018-11-10 23:59:59",                   /\*优惠券结束时间 该商品如无优惠券，此字段为空\*/
		"coupon\_info\_money": "10",                   /\*优惠券金额 该商品如无优惠券，此字段为0\*/
		"coupon\_total\_count": "100000",                   /\*优惠券总数量 该商品如无优惠券，此字段为0\*/
		"coupon\_remain\_count": "99965",                   /\*优惠券剩余数量 该商品如无优惠券，此字段为0\*/
		"coupon\_info": "满80.00元减10元",                   /\*优惠券信息\*/
		"juhuasuan": "0",                       /\*是否聚划算，1是\*/
		"taoqianggou": "0",                     /\*是否淘抢购，1是\*/
		"haitao": "0",                          /\*是否海淘，1是\*/
		"jiyoujia": "0",                        /\*是否极有家，1是\*/
		"jinpaimaijia": "0",                    /\*是否金牌卖家，1是\*/
		"pinpai": "0",                          /\*是否精选品牌，1是\*/
		"pinpai\_name": "",                      /\*品牌名称\*/
		"yunfeixian": "0",                      /\*是否有运费险，1有\*/
		"nick": "\*\*\*旗舰店",                    /\*卖家昵称\*/
		"small\_images": "https://img.alicdn.com/bao/.......jpg|https://img.alicdn.com/bao/.......jpg",                   /\*商品小图列表\*/
		"white\_image": "https://img.alicdn.com/bao/......jpg",                   /\*商品白底图\*/
		"tao\_title": "抽屉式收纳箱塑料储物箱整理箱衣服物家用宿舍衣柜内衣内裤收纳盒",                   /\*商品长标题\*/
		"provcity": "浙江 杭州",                   /\*宝贝所在地\*/
		"shop\_title": "\*\*\*旗舰店",                   /\*店铺名称\*/
		"zhibo\_url": "https://cloud.video.taobao.com/play/u/849090736/p/2/e/6/t/1/219853683676.mp4",                   /\*视频地址\*/
		"sellCount": "860000",                   /\*淘宝网页实时总销量\*/
		"commentCount": "250000",                   /\*评论数量\*/
		"favcount": "180000",                   /\*收藏数量\*/
		"score1": "4.9",                   /\*宝贝描述分\*/
		"score2": "4.9",                   /\*卖家服务分\*/
		"score3": "4.9",                   /\*物流服务分\*/
		"creditLevel": "15",                   /\*店铺等级（1-20），一星 二星 三星 四星 五星 一钻 二钻 三钻 四钻 五钻 一皇冠 二皇冠 三皇冠 四皇冠 五皇冠  一金冠 二金冠 三金冠 四金冠 五金冠\*/
		"shopIcon": "//img.alicdn.com/imgextra/de/a6/TB10dC8SX.jpg",       /\*店铺logo\*/
		"pcDescContent": "1.jpg|2.jpg|3.jpg|4.jpg|5.jpg",  /\*图文详情图片地址\*/
		"item\_url": "https://item.taobao.com/item.htm?id=581801364036",  /\*商品url\*/
		"category\_id": "50006126",                   /\*叶子类目id\*/
		"category\_name": "收纳盒",                   /\*叶子类目name\*/
		"level\_one\_category\_id": "122928002",                   /\*一级类目id\*/
		"level\_one\_category\_name": "收纳整理",                   /\*一级类目name\*/
		"tkfee3": "5.07",                   /\*返佣金额\*/
		"biaoqian": ""满300元,省30元"",                   /\*店铺活动\*/
		"tag": "赶紧拯救你厨房的油腻吧😂植护🌿加厚厨房抽纸高密度纤维纸张，吸油💧很不错5包装|￥14.8元💰✔厨房小帮手，用处多多",                   /\*朋友圈文案，需要自己进行urldecode\*/
		"date\_time": "2018/11/8 14:45:22"                   /\*数据添加时间\*/
	}\]
}
```

**失败返回示例：**

var tmp = { "status": 301 };
```json
{
	"status": 301                      /\*状态\*/
}
```

---

## 超高评分商品API

**原始链接**: [超高评分商品API](https://www.zhetaoke.com/user/extend/extend_lingquan_gaopingfen.aspx)

**超高评分商品API---本接口采用12台16核心64G内存服务器，江苏和北京双机房，负载均衡技术，1G独享带宽，放心调用。**

超高评分商品API：返回动态评分≥4.9的商品列表，返回佣金≥15%，动态描述分≥4.6的商品列表。

**接口地址：** https://api.zhetaoke.com:10001/api/api\_all.ashx?appkey=0a565628b2fb4712902712734cec2feb&page=1&page\_size=20&sort=new&dsr\_start=4.9

超高评分商品API：返回指定查询条件的商品总数。

**接口地址：** https://api.zhetaoke.com:10001/api/api\_all.ashx?appkey=0a565628b2fb4712902712734cec2feb&page=1&page\_size=20&sort=new&dsr\_start=4.9&total\_count=1

**返回数据：**json

**请求方式：**get

**请求参数说明：**

|  | 名称 | 类型 | 必填 | 说明 |
| --- | --- | --- | --- | --- |
|   | appkey | string | 是 | 折淘客的对接秘钥appkey |
|   | page | int | 否 | 分页获取数据,第几页 |
|   | page\_size | int | 否 | 每页数据条数（默认每页20条），可自定义1-50之间 |
|   | sort | string | 否 | 商品排序方式，new：按照综合排序，  
total\_sale\_num\_asc：按照总销量从小到大排序，total\_sale\_num\_desc：按照总销量从大到小排序，  
sale\_num\_asc：按照年销量从小到大排序，sale\_num\_desc：按照年销量从大到小排序，  
commission\_rate\_asc：按照佣金比例从小到大排序，commission\_rate\_desc：按照佣金比例从大到小排序，  
price\_asc：按照价格从小到大排序，price\_desc：按照价格从大到小排序，  
coupon\_info\_money\_asc：按照优惠券金额从小到大排序，coupon\_info\_money\_desc：按照优惠券金额从大到小排序，  
shop\_level\_asc：按照店铺等级从低到高排序，shop\_level\_desc：按照店铺等级从高到低排序，  
tkfee\_asc：按照返佣金额从低到高排序，tkfee\_desc：按照返佣金额从高到低排序，  
code：按照code值从大到小排序，date\_time：按照更新时间排序，random：按照随机排序 |
|   | cid | int | 否 | 一级商品分类，值为空：全部商品，1：女装，2：母婴，3：美妆，4：居家日用，5：鞋品，6：美食，7：文娱车品，8：数码家电，9：男装，10：内衣，11：箱包，12：配饰，13：户外运动，14：家装家纺 |
|   | dsr\_start | string | 否 | 动态评分≥ 值为空：全部商品，=4.6：动态评分≥4.6的商品，=4.9：动态评分≥4.9的商品 |

**成功返回示例：**/\*重要事情说三遍！重要事情说三遍！重要事情说三遍！api\_all/api\_dongdong/api\_quantian/api\_shishi/api\_videos/api\_xiaoshi/api\_yongjin接口所有参数都可以自由组合。\*/

var tmp = { "status": 200, /\*状态\*/ "content": \[{ "code": "6646", /\*折淘客编号\*/ "type\_one\_id": "1", /\*分类ID，可参考折淘客分类\*/ "tao\_id": "554832820990", /\*商品ID\*/ "title": "储物箱整理抽屉式收纳柜", /\*商品短标题\*/ "jianjie": "【爆款返场，出口日本】进口PP材质，可叠加透明可视抽屉收纳盒，分格抽屉物品分类齐全，区域分化一目了然，自由组合，大容量，让你居家生活更舒心，收纳你的生活！【赠运费险】", /\*商品简介\*/ "pict\_url": "https://img.alicdn.com/bao/......jpg", /\*商品主图\*/ "user\_type": "0", /\*是否天猫，0是淘宝，1是天猫\*/ "seller\_id": "3073204721", /\*卖家ID\*/ /\*店铺地址可自行拼接，http://store.taobao.com/shop/view\_shop.htm?user\_number\_id=3073204721 \*/ "shop\_dsr": "4.80", /\*商品描述分\*/ "volume": "169", /\*年销量\*/ "size": "26.90", /\*折扣价\*/ "quanhou\_jiage": "16.90", /\*券后价\*/ "date\_time\_yongjin": "2018/11/8 14:45:22", /\*数据更新时间\*/ "tkrate3": "30.00", /\*佣金比率\*/ "yongjin\_type": "MKT", /\*佣金类型\*/ "coupon\_id": "fe31d6c8132c462ba59bd72e03d2eae0", /\*优惠券ID 该商品如无优惠券，此字段为空\*/ "coupon\_start\_time": "2018-11-05 00:00:00", /\*优惠券开始时间 该商品如无优惠券，此字段为空\*/ "coupon\_end\_time": "2018-11-10 23:59:59", /\*优惠券结束时间 该商品如无优惠券，此字段为空\*/ "coupon\_info\_money": "10", /\*优惠券金额 该商品如无优惠券，此字段为0\*/ "coupon\_total\_count": "100000", /\*优惠券总数量 该商品如无优惠券，此字段为0\*/ "coupon\_remain\_count": "99965", /\*优惠券剩余数量 该商品如无优惠券，此字段为0\*/ "coupon\_info": "满80.00元减10元", /\*优惠券信息\*/ "juhuasuan": "0", /\*是否聚划算，1是\*/ "taoqianggou": "0", /\*是否淘抢购，1是\*/ "haitao": "0", /\*是否海淘，1是\*/ "jiyoujia": "0", /\*是否极有家，1是\*/ "jinpaimaijia": "0", /\*是否金牌卖家，1是\*/ "pinpai": "0", /\*是否精选品牌，1是\*/ "pinpai\_name": "", /\*品牌名称\*/ "yunfeixian": "0", /\*是否有运费险，1有\*/ "nick": "\*\*\*旗舰店", /\*卖家昵称\*/ "small\_images": "https://img.alicdn.com/bao/.......jpg|https://img.alicdn.com/bao/.......jpg", /\*商品小图列表\*/ "white\_image": "https://img.alicdn.com/bao/......jpg", /\*商品白底图\*/ "tao\_title": "抽屉式收纳箱塑料储物箱整理箱衣服物家用宿舍衣柜内衣内裤收纳盒", /\*商品长标题\*/ "provcity": "浙江 杭州", /\*宝贝所在地\*/ "shop\_title": "\*\*\*旗舰店", /\*店铺名称\*/ "zhibo\_url": "https://cloud.video.taobao.com/play/u/849090736/p/2/e/6/t/1/219853683676.mp4", /\*视频地址\*/ "sellCount": "860000", /\*淘宝网页实时总销量\*/ "commentCount": "250000", /\*评论数量\*/ "favcount": "180000", /\*收藏数量\*/ "score1": "4.9", /\*宝贝描述分\*/ "score2": "4.9", /\*卖家服务分\*/ "score3": "4.9", /\*物流服务\*/ "creditLevel": "15", /\*店铺等级，一星 二星 三星 四星 五星 一钻 二钻 三钻 四钻 五钻 一皇冠 二皇冠 三皇冠 四皇冠 五皇冠 一金冠 二金冠 三金冠 四金冠 五金冠\*/ "shopIcon": "//img.alicdn.com/imgextra/de/a6/TB10dC8SX.jpg", /\*店铺logo\*/ "pcDescContent": "1.jpg|2.jpg|3.jpg|4.jpg|5.jpg", /\*图文详情图片地址\*/ "item\_url": "https://item.taobao.com/item.htm?id=581801364036", /\*商品url\*/ "category\_id": "50006126", /\*叶子类目id\*/ "category\_name": "收纳盒", /\*叶子类目name\*/ "level\_one\_category\_id": "122928002", /\*一级类目id\*/ "level\_one\_category\_name": "收纳整理", /\*一级类目name\*/ "tkfee3": "5.07", /\*返佣金额\*/ "biaoqian": ""满300元,省30元"", /\*店铺活动\*/ "tag": "赶紧拯救你厨房的油腻吧😂<br>植护🌿加厚厨房抽纸<br>高密度纤维纸张，吸油💧很不错<br>5包装|￥14.8元💰<br>✔厨房小帮手，用处多多", /\*朋友圈文案，需要自己进行urldecode\*/ "date\_time": "2018/11/8 14:45:22" /\*数据添加时间\*/ }\] };
```json
{
	"status": 200,                      /\*状态\*/
	"content": \[{
		"code": "6646",                 /\*折淘客编号\*/
		"type\_one\_id": "1",             /\*分类ID，可参考折淘客分类\*/
		"tao\_id": "554832820990",       /\*商品ID\*/
		"title": "储物箱整理抽屉式收纳柜",                   /\*商品短标题\*/
		"jianjie": "【爆款返场，出口日本】进口PP材质，可叠加透明可视抽屉收纳盒，分格抽屉物品分类齐全，区域分化一目了然，自由组合，大容量，让你居家生活更舒心，收纳你的生活！【赠运费险】", /\*商品简介\*/
		"pict\_url": "https://img.alicdn.com/bao/......jpg",                   /\*商品主图\*/
		"user\_type": "0",                   /\*是否天猫，0是淘宝，1是天猫\*/
		"seller\_id": "3073204721",                   /\*卖家ID\*/ /\*店铺地址可自行拼接，http://store.taobao.com/shop/view\_shop.htm?user\_number\_id=3073204721 \*/
		"shop\_dsr": "4.80",                   /\*商品描述分\*/
		"volume": "169",                   /\*年销量\*/
		"size": "26.90",                   /\*折扣价\*/
		"quanhou\_jiage": "16.90",                   /\*券后价\*/
		"date\_time\_yongjin": "2018/11/8 14:45:22",                   /\*数据更新时间\*/
		"tkrate3": "30.00",                   /\*佣金比率\*/
		"yongjin\_type": "MKT",                   /\*佣金类型\*/
		"coupon\_id": "fe31d6c8132c462ba59bd72e03d2eae0",                   /\*优惠券ID 该商品如无优惠券，此字段为空\*/
		"coupon\_start\_time": "2018-11-05 00:00:00",                   /\*优惠券开始时间 该商品如无优惠券，此字段为空\*/
		"coupon\_end\_time": "2018-11-10 23:59:59",                   /\*优惠券结束时间 该商品如无优惠券，此字段为空\*/
		"coupon\_info\_money": "10",                   /\*优惠券金额 该商品如无优惠券，此字段为0\*/
		"coupon\_total\_count": "100000",                   /\*优惠券总数量 该商品如无优惠券，此字段为0\*/
		"coupon\_remain\_count": "99965",                   /\*优惠券剩余数量 该商品如无优惠券，此字段为0\*/
		"coupon\_info": "满80.00元减10元",                   /\*优惠券信息\*/
		"juhuasuan": "0",                       /\*是否聚划算，1是\*/
		"taoqianggou": "0",                     /\*是否淘抢购，1是\*/
		"haitao": "0",                          /\*是否海淘，1是\*/
		"jiyoujia": "0",                        /\*是否极有家，1是\*/
		"jinpaimaijia": "0",                    /\*是否金牌卖家，1是\*/
		"pinpai": "0",                          /\*是否精选品牌，1是\*/
		"pinpai\_name": "",                      /\*品牌名称\*/
		"yunfeixian": "0",                      /\*是否有运费险，1有\*/
		"nick": "\*\*\*旗舰店",                    /\*卖家昵称\*/
		"small\_images": "https://img.alicdn.com/bao/.......jpg|https://img.alicdn.com/bao/.......jpg",                   /\*商品小图列表\*/
		"white\_image": "https://img.alicdn.com/bao/......jpg",                   /\*商品白底图\*/
		"tao\_title": "抽屉式收纳箱塑料储物箱整理箱衣服物家用宿舍衣柜内衣内裤收纳盒",                   /\*商品长标题\*/
		"provcity": "浙江 杭州",                   /\*宝贝所在地\*/
		"shop\_title": "\*\*\*旗舰店",                   /\*店铺名称\*/
		"zhibo\_url": "https://cloud.video.taobao.com/play/u/849090736/p/2/e/6/t/1/219853683676.mp4",                   /\*视频地址\*/
		"sellCount": "860000",                   /\*淘宝网页实时总销量\*/
		"commentCount": "250000",                   /\*评论数量\*/
		"favcount": "180000",                   /\*收藏数量\*/
		"score1": "4.9",                   /\*宝贝描述分\*/
		"score2": "4.9",                   /\*卖家服务分\*/
		"score3": "4.9",                   /\*物流服务分\*/
		"creditLevel": "15",                   /\*店铺等级（1-20），一星 二星 三星 四星 五星 一钻 二钻 三钻 四钻 五钻 一皇冠 二皇冠 三皇冠 四皇冠 五皇冠  一金冠 二金冠 三金冠 四金冠 五金冠\*/
		"shopIcon": "//img.alicdn.com/imgextra/de/a6/TB10dC8SX.jpg",       /\*店铺logo\*/
		"pcDescContent": "1.jpg|2.jpg|3.jpg|4.jpg|5.jpg",  /\*图文详情图片地址\*/
		"item\_url": "https://item.taobao.com/item.htm?id=581801364036",  /\*商品url\*/
		"category\_id": "50006126",                   /\*叶子类目id\*/
		"category\_name": "收纳盒",                   /\*叶子类目name\*/
		"level\_one\_category\_id": "122928002",                   /\*一级类目id\*/
		"level\_one\_category\_name": "收纳整理",                   /\*一级类目name\*/
		"tkfee3": "5.07",                   /\*返佣金额\*/
		"biaoqian": ""满300元,省30元"",                   /\*店铺活动\*/
		"tag": "赶紧拯救你厨房的油腻吧😂植护🌿加厚厨房抽纸高密度纤维纸张，吸油💧很不错5包装|￥14.8元💰✔厨房小帮手，用处多多",                   /\*朋友圈文案，需要自己进行urldecode\*/
		"date\_time": "2018/11/8 14:45:22"                   /\*数据添加时间\*/
	}\]
}
```

**失败返回示例：**

var tmp = { "status": 301 };
```json
{
	"status": 301                      /\*状态\*/
}
```

---

## 超高券面额商品API

**原始链接**: [超高券面额商品API](https://www.zhetaoke.com/user/extend/extend_lingquan_gaomiane.aspx)

**超高券面额商品API---本接口采用12台16核心64G内存服务器，江苏和北京双机房，负载均衡技术，1G独享带宽，放心调用。**

超高券面额商品API：返回优惠券面额≥100元的商品列表，返回佣金≥15%，动态描述分≥4.6的商品列表。

**接口地址：** https://api.zhetaoke.com:10001/api/api\_all.ashx?appkey=0a565628b2fb4712902712734cec2feb&page=1&page\_size=20&sort=new&coupon\_amount\_start=100

超高券面额商品API：返回指定查询条件的商品总数。

**接口地址：** https://api.zhetaoke.com:10001/api/api\_all.ashx?appkey=0a565628b2fb4712902712734cec2feb&page=1&page\_size=20&sort=new&coupon\_amount\_start=100&total\_count=1

**返回数据：**json

**请求方式：**get

**请求参数说明：**

|  | 名称 | 类型 | 必填 | 说明 |
| --- | --- | --- | --- | --- |
|   | appkey | string | 是 | 折淘客的对接秘钥appkey |
|   | page | int | 否 | 分页获取数据,第几页 |
|   | page\_size | int | 否 | 每页数据条数（默认每页20条），可自定义1-50之间 |
|   | sort | string | 否 | 商品排序方式，new：按照综合排序，  
total\_sale\_num\_asc：按照总销量从小到大排序，total\_sale\_num\_desc：按照总销量从大到小排序，  
sale\_num\_asc：按照年销量从小到大排序，sale\_num\_desc：按照年销量从大到小排序，  
commission\_rate\_asc：按照佣金比例从小到大排序，commission\_rate\_desc：按照佣金比例从大到小排序，  
price\_asc：按照价格从小到大排序，price\_desc：按照价格从大到小排序，  
coupon\_info\_money\_asc：按照优惠券金额从小到大排序，coupon\_info\_money\_desc：按照优惠券金额从大到小排序，  
shop\_level\_asc：按照店铺等级从低到高排序，shop\_level\_desc：按照店铺等级从高到低排序，  
tkfee\_asc：按照返佣金额从低到高排序，tkfee\_desc：按照返佣金额从高到低排序，  
code：按照code值从大到小排序，date\_time：按照更新时间排序，random：按照随机排序 |
|   | cid | int | 否 | 一级商品分类，值为空：全部商品，1：女装，2：母婴，3：美妆，4：居家日用，5：鞋品，6：美食，7：文娱车品，8：数码家电，9：男装，10：内衣，11：箱包，12：配饰，13：户外运动，14：家装家纺 |
|   | coupon\_amount\_start | string | 否 | 券面额≥ 值为空：全部商品，=5：券面额≥5的商品，=20：券面额≥20的商品，=100：券面额≥100的商品 |

**成功返回示例：**/\*重要事情说三遍！重要事情说三遍！重要事情说三遍！api\_all/api\_dongdong/api\_quantian/api\_shishi/api\_videos/api\_xiaoshi/api\_yongjin接口所有参数都可以自由组合。\*/

var tmp = { "status": 200, /\*状态\*/ "content": \[{ "code": "6646", /\*折淘客编号\*/ "type\_one\_id": "1", /\*分类ID，可参考折淘客分类\*/ "tao\_id": "554832820990", /\*商品ID\*/ "title": "储物箱整理抽屉式收纳柜", /\*商品短标题\*/ "jianjie": "【爆款返场，出口日本】进口PP材质，可叠加透明可视抽屉收纳盒，分格抽屉物品分类齐全，区域分化一目了然，自由组合，大容量，让你居家生活更舒心，收纳你的生活！【赠运费险】", /\*商品简介\*/ "pict\_url": "https://img.alicdn.com/bao/......jpg", /\*商品主图\*/ "user\_type": "0", /\*是否天猫，0是淘宝，1是天猫\*/ "seller\_id": "3073204721", /\*卖家ID\*/ /\*店铺地址可自行拼接，http://store.taobao.com/shop/view\_shop.htm?user\_number\_id=3073204721 \*/ "shop\_dsr": "4.80", /\*商品描述分\*/ "volume": "169", /\*年销量\*/ "size": "26.90", /\*折扣价\*/ "quanhou\_jiage": "16.90", /\*券后价\*/ "date\_time\_yongjin": "2018/11/8 14:45:22", /\*数据更新时间\*/ "tkrate3": "30.00", /\*佣金比率\*/ "yongjin\_type": "MKT", /\*佣金类型\*/ "coupon\_id": "fe31d6c8132c462ba59bd72e03d2eae0", /\*优惠券ID 该商品如无优惠券，此字段为空\*/ "coupon\_start\_time": "2018-11-05 00:00:00", /\*优惠券开始时间 该商品如无优惠券，此字段为空\*/ "coupon\_end\_time": "2018-11-10 23:59:59", /\*优惠券结束时间 该商品如无优惠券，此字段为空\*/ "coupon\_info\_money": "10", /\*优惠券金额 该商品如无优惠券，此字段为0\*/ "coupon\_total\_count": "100000", /\*优惠券总数量 该商品如无优惠券，此字段为0\*/ "coupon\_remain\_count": "99965", /\*优惠券剩余数量 该商品如无优惠券，此字段为0\*/ "coupon\_info": "满80.00元减10元", /\*优惠券信息\*/ "juhuasuan": "0", /\*是否聚划算，1是\*/ "taoqianggou": "0", /\*是否淘抢购，1是\*/ "haitao": "0", /\*是否海淘，1是\*/ "jiyoujia": "0", /\*是否极有家，1是\*/ "jinpaimaijia": "0", /\*是否金牌卖家，1是\*/ "pinpai": "0", /\*是否精选品牌，1是\*/ "pinpai\_name": "", /\*品牌名称\*/ "yunfeixian": "0", /\*是否有运费险，1有\*/ "nick": "\*\*\*旗舰店", /\*卖家昵称\*/ "small\_images": "https://img.alicdn.com/bao/.......jpg|https://img.alicdn.com/bao/.......jpg", /\*商品小图列表\*/ "white\_image": "https://img.alicdn.com/bao/......jpg", /\*商品白底图\*/ "tao\_title": "抽屉式收纳箱塑料储物箱整理箱衣服物家用宿舍衣柜内衣内裤收纳盒", /\*商品长标题\*/ "provcity": "浙江 杭州", /\*宝贝所在地\*/ "shop\_title": "\*\*\*旗舰店", /\*店铺名称\*/ "zhibo\_url": "https://cloud.video.taobao.com/play/u/849090736/p/2/e/6/t/1/219853683676.mp4", /\*视频地址\*/ "sellCount": "860000", /\*淘宝网页实时总销量\*/ "commentCount": "250000", /\*评论数量\*/ "favcount": "180000", /\*收藏数量\*/ "score1": "4.9", /\*宝贝描述分\*/ "score2": "4.9", /\*卖家服务分\*/ "score3": "4.9", /\*物流服务\*/ "creditLevel": "15", /\*店铺等级，一星 二星 三星 四星 五星 一钻 二钻 三钻 四钻 五钻 一皇冠 二皇冠 三皇冠 四皇冠 五皇冠 一金冠 二金冠 三金冠 四金冠 五金冠\*/ "shopIcon": "//img.alicdn.com/imgextra/de/a6/TB10dC8SX.jpg", /\*店铺logo\*/ "pcDescContent": "1.jpg|2.jpg|3.jpg|4.jpg|5.jpg", /\*图文详情图片地址\*/ "item\_url": "https://item.taobao.com/item.htm?id=581801364036", /\*商品url\*/ "category\_id": "50006126", /\*叶子类目id\*/ "category\_name": "收纳盒", /\*叶子类目name\*/ "level\_one\_category\_id": "122928002", /\*一级类目id\*/ "level\_one\_category\_name": "收纳整理", /\*一级类目name\*/ "tkfee3": "5.07", /\*返佣金额\*/ "biaoqian": ""满300元,省30元"", /\*店铺活动\*/ "tag": "赶紧拯救你厨房的油腻吧😂<br>植护🌿加厚厨房抽纸<br>高密度纤维纸张，吸油💧很不错<br>5包装|￥14.8元💰<br>✔厨房小帮手，用处多多", /\*朋友圈文案，需要自己进行urldecode\*/ "date\_time": "2018/11/8 14:45:22" /\*数据添加时间\*/ }\] };
```json
{
	"status": 200,                      /\*状态\*/
	"content": \[{
		"code": "6646",                 /\*折淘客编号\*/
		"type\_one\_id": "1",             /\*分类ID，可参考折淘客分类\*/
		"tao\_id": "554832820990",       /\*商品ID\*/
		"title": "储物箱整理抽屉式收纳柜",                   /\*商品短标题\*/
		"jianjie": "【爆款返场，出口日本】进口PP材质，可叠加透明可视抽屉收纳盒，分格抽屉物品分类齐全，区域分化一目了然，自由组合，大容量，让你居家生活更舒心，收纳你的生活！【赠运费险】", /\*商品简介\*/
		"pict\_url": "https://img.alicdn.com/bao/......jpg",                   /\*商品主图\*/
		"user\_type": "0",                   /\*是否天猫，0是淘宝，1是天猫\*/
		"seller\_id": "3073204721",                   /\*卖家ID\*/ /\*店铺地址可自行拼接，http://store.taobao.com/shop/view\_shop.htm?user\_number\_id=3073204721 \*/
		"shop\_dsr": "4.80",                   /\*商品描述分\*/
		"volume": "169",                   /\*年销量\*/
		"size": "26.90",                   /\*折扣价\*/
		"quanhou\_jiage": "16.90",                   /\*券后价\*/
		"date\_time\_yongjin": "2018/11/8 14:45:22",                   /\*数据更新时间\*/
		"tkrate3": "30.00",                   /\*佣金比率\*/
		"yongjin\_type": "MKT",                   /\*佣金类型\*/
		"coupon\_id": "fe31d6c8132c462ba59bd72e03d2eae0",                   /\*优惠券ID 该商品如无优惠券，此字段为空\*/
		"coupon\_start\_time": "2018-11-05 00:00:00",                   /\*优惠券开始时间 该商品如无优惠券，此字段为空\*/
		"coupon\_end\_time": "2018-11-10 23:59:59",                   /\*优惠券结束时间 该商品如无优惠券，此字段为空\*/
		"coupon\_info\_money": "10",                   /\*优惠券金额 该商品如无优惠券，此字段为0\*/
		"coupon\_total\_count": "100000",                   /\*优惠券总数量 该商品如无优惠券，此字段为0\*/
		"coupon\_remain\_count": "99965",                   /\*优惠券剩余数量 该商品如无优惠券，此字段为0\*/
		"coupon\_info": "满80.00元减10元",                   /\*优惠券信息\*/
		"juhuasuan": "0",                       /\*是否聚划算，1是\*/
		"taoqianggou": "0",                     /\*是否淘抢购，1是\*/
		"haitao": "0",                          /\*是否海淘，1是\*/
		"jiyoujia": "0",                        /\*是否极有家，1是\*/
		"jinpaimaijia": "0",                    /\*是否金牌卖家，1是\*/
		"pinpai": "0",                          /\*是否精选品牌，1是\*/
		"pinpai\_name": "",                      /\*品牌名称\*/
		"yunfeixian": "0",                      /\*是否有运费险，1有\*/
		"nick": "\*\*\*旗舰店",                    /\*卖家昵称\*/
		"small\_images": "https://img.alicdn.com/bao/.......jpg|https://img.alicdn.com/bao/.......jpg",                   /\*商品小图列表\*/
		"white\_image": "https://img.alicdn.com/bao/......jpg",                   /\*商品白底图\*/
		"tao\_title": "抽屉式收纳箱塑料储物箱整理箱衣服物家用宿舍衣柜内衣内裤收纳盒",                   /\*商品长标题\*/
		"provcity": "浙江 杭州",                   /\*宝贝所在地\*/
		"shop\_title": "\*\*\*旗舰店",                   /\*店铺名称\*/
		"zhibo\_url": "https://cloud.video.taobao.com/play/u/849090736/p/2/e/6/t/1/219853683676.mp4",                   /\*视频地址\*/
		"sellCount": "860000",                   /\*淘宝网页实时总销量\*/
		"commentCount": "250000",                   /\*评论数量\*/
		"favcount": "180000",                   /\*收藏数量\*/
		"score1": "4.9",                   /\*宝贝描述分\*/
		"score2": "4.9",                   /\*卖家服务分\*/
		"score3": "4.9",                   /\*物流服务分\*/
		"creditLevel": "15",                   /\*店铺等级（1-20），一星 二星 三星 四星 五星 一钻 二钻 三钻 四钻 五钻 一皇冠 二皇冠 三皇冠 四皇冠 五皇冠  一金冠 二金冠 三金冠 四金冠 五金冠\*/
		"shopIcon": "//img.alicdn.com/imgextra/de/a6/TB10dC8SX.jpg",       /\*店铺logo\*/
		"pcDescContent": "1.jpg|2.jpg|3.jpg|4.jpg|5.jpg",  /\*图文详情图片地址\*/
		"item\_url": "https://item.taobao.com/item.htm?id=581801364036",  /\*商品url\*/
		"category\_id": "50006126",                   /\*叶子类目id\*/
		"category\_name": "收纳盒",                   /\*叶子类目name\*/
		"level\_one\_category\_id": "122928002",                   /\*一级类目id\*/
		"level\_one\_category\_name": "收纳整理",                   /\*一级类目name\*/
		"tkfee3": "5.07",                   /\*返佣金额\*/
		"biaoqian": ""满300元,省30元"",                   /\*店铺活动\*/
		"tag": "赶紧拯救你厨房的油腻吧😂植护🌿加厚厨房抽纸高密度纤维纸张，吸油💧很不错5包装|￥14.8元💰✔厨房小帮手，用处多多",                   /\*朋友圈文案，需要自己进行urldecode\*/
		"date\_time": "2018/11/8 14:45:22"                   /\*数据添加时间\*/
	}\]
}
```

**失败返回示例：**

var tmp = { "status": 301 };
```json
{
	"status": 301                      /\*状态\*/
}
```

---

## 偏远地区包邮商品API

**原始链接**: [偏远地区包邮商品API](https://www.zhetaoke.com/user/extend/extend_lingquan_pybaoyou.aspx)

**偏远地区包邮商品API---本接口采用12台16核心64G内存服务器，江苏和北京双机房，负载均衡技术，1G独享带宽，放心调用。**

偏远地区包邮商品API：返回偏远地区包邮商品列表，动态描述分≥4.6的商品列表。

**接口地址：** https://api.zhetaoke.com:10001/api/api\_all.ashx?appkey=0a565628b2fb4712902712734cec2feb&page=1&page\_size=20&sort=new&py\_baoyou=1

偏远地区包邮商品API：返回指定查询条件的商品总数。

**接口地址：** https://api.zhetaoke.com:10001/api/api\_all.ashx?appkey=0a565628b2fb4712902712734cec2feb&page=1&page\_size=20&sort=new&py\_baoyou=1&total\_count=1

**返回数据：**json

**请求方式：**get

**请求参数说明：**

|  | 名称 | 类型 | 必填 | 说明 |
| --- | --- | --- | --- | --- |
|   | appkey | string | 是 | 折淘客的对接秘钥appkey |
|   | page | int | 否 | 分页获取数据,第几页 |
|   | page\_size | int | 否 | 每页数据条数（默认每页20条），可自定义1-50之间 |
|   | sort | string | 否 | 商品排序方式，new：按照综合排序，  
total\_sale\_num\_asc：按照总销量从小到大排序，total\_sale\_num\_desc：按照总销量从大到小排序，  
sale\_num\_asc：按照年销量从小到大排序，sale\_num\_desc：按照年销量从大到小排序，  
commission\_rate\_asc：按照佣金比例从小到大排序，commission\_rate\_desc：按照佣金比例从大到小排序，  
price\_asc：按照价格从小到大排序，price\_desc：按照价格从大到小排序，  
coupon\_info\_money\_asc：按照优惠券金额从小到大排序，coupon\_info\_money\_desc：按照优惠券金额从大到小排序，  
shop\_level\_asc：按照店铺等级从低到高排序，shop\_level\_desc：按照店铺等级从高到低排序，  
tkfee\_asc：按照返佣金额从低到高排序，tkfee\_desc：按照返佣金额从高到低排序，  
code：按照code值从大到小排序，date\_time：按照更新时间排序，random：按照随机排序 |
|   | cid | int | 否 | 一级商品分类，值为空：全部商品，1：女装，2：母婴，3：美妆，4：居家日用，5：鞋品，6：美食，7：文娱车品，8：数码家电，9：男装，10：内衣，11：箱包，12：配饰，13：户外运动，14：家装家纺 |
|   | py\_baoyou | string | 否 | 偏远地区包邮商品，值为空：全部商品，1：偏远地区包邮商品（新疆等） |

**成功返回示例：**/\*重要事情说三遍！重要事情说三遍！重要事情说三遍！api\_all/api\_dongdong/api\_quantian/api\_shishi/api\_videos/api\_xiaoshi/api\_yongjin接口所有参数都可以自由组合。\*/

var tmp = { "status": 200, /\*状态\*/ "content": \[{ "code": "6646", /\*折淘客编号\*/ "type\_one\_id": "1", /\*分类ID，可参考折淘客分类\*/ "tao\_id": "554832820990", /\*商品ID\*/ "title": "储物箱整理抽屉式收纳柜", /\*商品短标题\*/ "jianjie": "【爆款返场，出口日本】进口PP材质，可叠加透明可视抽屉收纳盒，分格抽屉物品分类齐全，区域分化一目了然，自由组合，大容量，让你居家生活更舒心，收纳你的生活！【赠运费险】", /\*商品简介\*/ "pict\_url": "https://img.alicdn.com/bao/......jpg", /\*商品主图\*/ "user\_type": "0", /\*是否天猫，0是淘宝，1是天猫\*/ "seller\_id": "3073204721", /\*卖家ID\*/ /\*店铺地址可自行拼接，http://store.taobao.com/shop/view\_shop.htm?user\_number\_id=3073204721 \*/ "shop\_dsr": "4.80", /\*商品描述分\*/ "volume": "169", /\*年销量\*/ "size": "26.90", /\*折扣价\*/ "quanhou\_jiage": "16.90", /\*券后价\*/ "date\_time\_yongjin": "2018/11/8 14:45:22", /\*数据更新时间\*/ "tkrate3": "30.00", /\*佣金比率\*/ "yongjin\_type": "MKT", /\*佣金类型\*/ "coupon\_id": "fe31d6c8132c462ba59bd72e03d2eae0", /\*优惠券ID 该商品如无优惠券，此字段为空\*/ "coupon\_start\_time": "2018-11-05 00:00:00", /\*优惠券开始时间 该商品如无优惠券，此字段为空\*/ "coupon\_end\_time": "2018-11-10 23:59:59", /\*优惠券结束时间 该商品如无优惠券，此字段为空\*/ "coupon\_info\_money": "10", /\*优惠券金额 该商品如无优惠券，此字段为0\*/ "coupon\_total\_count": "100000", /\*优惠券总数量 该商品如无优惠券，此字段为0\*/ "coupon\_remain\_count": "99965", /\*优惠券剩余数量 该商品如无优惠券，此字段为0\*/ "coupon\_info": "满80.00元减10元", /\*优惠券信息\*/ "juhuasuan": "0", /\*是否聚划算，1是\*/ "taoqianggou": "0", /\*是否淘抢购，1是\*/ "haitao": "0", /\*是否海淘，1是\*/ "jiyoujia": "0", /\*是否极有家，1是\*/ "jinpaimaijia": "0", /\*是否金牌卖家，1是\*/ "pinpai": "0", /\*是否精选品牌，1是\*/ "pinpai\_name": "", /\*品牌名称\*/ "yunfeixian": "0", /\*是否有运费险，1有\*/ "nick": "\*\*\*旗舰店", /\*卖家昵称\*/ "small\_images": "https://img.alicdn.com/bao/.......jpg|https://img.alicdn.com/bao/.......jpg", /\*商品小图列表\*/ "white\_image": "https://img.alicdn.com/bao/......jpg", /\*商品白底图\*/ "tao\_title": "抽屉式收纳箱塑料储物箱整理箱衣服物家用宿舍衣柜内衣内裤收纳盒", /\*商品长标题\*/ "provcity": "浙江 杭州", /\*宝贝所在地\*/ "shop\_title": "\*\*\*旗舰店", /\*店铺名称\*/ "zhibo\_url": "https://cloud.video.taobao.com/play/u/849090736/p/2/e/6/t/1/219853683676.mp4", /\*视频地址\*/ "sellCount": "860000", /\*淘宝网页实时总销量\*/ "commentCount": "250000", /\*评论数量\*/ "favcount": "180000", /\*收藏数量\*/ "score1": "4.9", /\*宝贝描述分\*/ "score2": "4.9", /\*卖家服务分\*/ "score3": "4.9", /\*物流服务\*/ "creditLevel": "15", /\*店铺等级，一星 二星 三星 四星 五星 一钻 二钻 三钻 四钻 五钻 一皇冠 二皇冠 三皇冠 四皇冠 五皇冠 一金冠 二金冠 三金冠 四金冠 五金冠\*/ "shopIcon": "//img.alicdn.com/imgextra/de/a6/TB10dC8SX.jpg", /\*店铺logo\*/ "pcDescContent": "1.jpg|2.jpg|3.jpg|4.jpg|5.jpg", /\*图文详情图片地址\*/ "item\_url": "https://item.taobao.com/item.htm?id=581801364036", /\*商品url\*/ "category\_id": "50006126", /\*叶子类目id\*/ "category\_name": "收纳盒", /\*叶子类目name\*/ "level\_one\_category\_id": "122928002", /\*一级类目id\*/ "level\_one\_category\_name": "收纳整理", /\*一级类目name\*/ "tkfee3": "5.07", /\*返佣金额\*/ "biaoqian": ""满300元,省30元"", /\*店铺活动\*/ "tag": "赶紧拯救你厨房的油腻吧😂<br>植护🌿加厚厨房抽纸<br>高密度纤维纸张，吸油💧很不错<br>5包装|￥14.8元💰<br>✔厨房小帮手，用处多多", /\*朋友圈文案，需要自己进行urldecode\*/ "date\_time": "2018/11/8 14:45:22" /\*数据添加时间\*/ }\] };
```json
{
	"status": 200,                      /\*状态\*/
	"content": \[{
		"code": "6646",                 /\*折淘客编号\*/
		"type\_one\_id": "1",             /\*分类ID，可参考折淘客分类\*/
		"tao\_id": "554832820990",       /\*商品ID\*/
		"title": "储物箱整理抽屉式收纳柜",                   /\*商品短标题\*/
		"jianjie": "【爆款返场，出口日本】进口PP材质，可叠加透明可视抽屉收纳盒，分格抽屉物品分类齐全，区域分化一目了然，自由组合，大容量，让你居家生活更舒心，收纳你的生活！【赠运费险】", /\*商品简介\*/
		"pict\_url": "https://img.alicdn.com/bao/......jpg",                   /\*商品主图\*/
		"user\_type": "0",                   /\*是否天猫，0是淘宝，1是天猫\*/
		"seller\_id": "3073204721",                   /\*卖家ID\*/ /\*店铺地址可自行拼接，http://store.taobao.com/shop/view\_shop.htm?user\_number\_id=3073204721 \*/
		"shop\_dsr": "4.80",                   /\*商品描述分\*/
		"volume": "169",                   /\*年销量\*/
		"size": "26.90",                   /\*折扣价\*/
		"quanhou\_jiage": "16.90",                   /\*券后价\*/
		"date\_time\_yongjin": "2018/11/8 14:45:22",                   /\*数据更新时间\*/
		"tkrate3": "30.00",                   /\*佣金比率\*/
		"yongjin\_type": "MKT",                   /\*佣金类型\*/
		"coupon\_id": "fe31d6c8132c462ba59bd72e03d2eae0",                   /\*优惠券ID 该商品如无优惠券，此字段为空\*/
		"coupon\_start\_time": "2018-11-05 00:00:00",                   /\*优惠券开始时间 该商品如无优惠券，此字段为空\*/
		"coupon\_end\_time": "2018-11-10 23:59:59",                   /\*优惠券结束时间 该商品如无优惠券，此字段为空\*/
		"coupon\_info\_money": "10",                   /\*优惠券金额 该商品如无优惠券，此字段为0\*/
		"coupon\_total\_count": "100000",                   /\*优惠券总数量 该商品如无优惠券，此字段为0\*/
		"coupon\_remain\_count": "99965",                   /\*优惠券剩余数量 该商品如无优惠券，此字段为0\*/
		"coupon\_info": "满80.00元减10元",                   /\*优惠券信息\*/
		"juhuasuan": "0",                       /\*是否聚划算，1是\*/
		"taoqianggou": "0",                     /\*是否淘抢购，1是\*/
		"haitao": "0",                          /\*是否海淘，1是\*/
		"jiyoujia": "0",                        /\*是否极有家，1是\*/
		"jinpaimaijia": "0",                    /\*是否金牌卖家，1是\*/
		"pinpai": "0",                          /\*是否精选品牌，1是\*/
		"pinpai\_name": "",                      /\*品牌名称\*/
		"yunfeixian": "0",                      /\*是否有运费险，1有\*/
		"nick": "\*\*\*旗舰店",                    /\*卖家昵称\*/
		"small\_images": "https://img.alicdn.com/bao/.......jpg|https://img.alicdn.com/bao/.......jpg",                   /\*商品小图列表\*/
		"white\_image": "https://img.alicdn.com/bao/......jpg",                   /\*商品白底图\*/
		"tao\_title": "抽屉式收纳箱塑料储物箱整理箱衣服物家用宿舍衣柜内衣内裤收纳盒",                   /\*商品长标题\*/
		"provcity": "浙江 杭州",                   /\*宝贝所在地\*/
		"shop\_title": "\*\*\*旗舰店",                   /\*店铺名称\*/
		"zhibo\_url": "https://cloud.video.taobao.com/play/u/849090736/p/2/e/6/t/1/219853683676.mp4",                   /\*视频地址\*/
		"sellCount": "860000",                   /\*淘宝网页实时总销量\*/
		"commentCount": "250000",                   /\*评论数量\*/
		"favcount": "180000",                   /\*收藏数量\*/
		"score1": "4.9",                   /\*宝贝描述分\*/
		"score2": "4.9",                   /\*卖家服务分\*/
		"score3": "4.9",                   /\*物流服务分\*/
		"creditLevel": "15",                   /\*店铺等级（1-20），一星 二星 三星 四星 五星 一钻 二钻 三钻 四钻 五钻 一皇冠 二皇冠 三皇冠 四皇冠 五皇冠  一金冠 二金冠 三金冠 四金冠 五金冠\*/
		"shopIcon": "//img.alicdn.com/imgextra/de/a6/TB10dC8SX.jpg",       /\*店铺logo\*/
		"pcDescContent": "1.jpg|2.jpg|3.jpg|4.jpg|5.jpg",  /\*图文详情图片地址\*/
		"item\_url": "https://item.taobao.com/item.htm?id=581801364036",  /\*商品url\*/
		"category\_id": "50006126",                   /\*叶子类目id\*/
		"category\_name": "收纳盒",                   /\*叶子类目name\*/
		"level\_one\_category\_id": "122928002",                   /\*一级类目id\*/
		"level\_one\_category\_name": "收纳整理",                   /\*一级类目name\*/
		"tkfee3": "5.07",                   /\*返佣金额\*/
		"biaoqian": ""满300元,省30元"",                   /\*店铺活动\*/
		"tag": "赶紧拯救你厨房的油腻吧😂植护🌿加厚厨房抽纸高密度纤维纸张，吸油💧很不错5包装|￥14.8元💰✔厨房小帮手，用处多多",                   /\*朋友圈文案，需要自己进行urldecode\*/
		"date\_time": "2018/11/8 14:45:22"                   /\*数据添加时间\*/
	}\]
}
```

**失败返回示例：**

var tmp = { "status": 301 };
```json
{
	"status": 301                      /\*状态\*/
}
```

[效果展示](http://cms.zhetaoke.com:10000/m/item/list.aspx?baodan=1)

[偏远地区包邮说明](http://www.zhetaoke.com/help_detail_3_19.html)

---

## 极品爆单商品API接口

**原始链接**: [极品爆单商品API接口](https://www.zhetaoke.com/user/extend/extend_lingquan_baodan.aspx)

**极品爆单商品API---本接口采用12台16核心64G内存服务器，江苏和北京双机房，负载均衡技术，1G独享带宽，放心调用。**

极品爆单商品API：返回极品爆单商品列表，动态描述分≥4.6的商品列表。

**接口地址：** https://api.zhetaoke.com:10001/api/api\_all.ashx?appkey=0a565628b2fb4712902712734cec2feb&page=1&page\_size=20&sort=new&baodan=1

极品爆单商品API：返回指定查询条件的商品总数。

**接口地址：** https://api.zhetaoke.com:10001/api/api\_all.ashx?appkey=0a565628b2fb4712902712734cec2feb&page=1&page\_size=20&sort=new&baodan=1&total\_count=1

**返回数据：**json

**请求方式：**get

**请求参数说明：**

|  | 名称 | 类型 | 必填 | 说明 |
| --- | --- | --- | --- | --- |
|   | appkey | string | 是 | 折淘客的对接秘钥appkey |
|   | page | int | 否 | 分页获取数据,第几页 |
|   | page\_size | int | 否 | 每页数据条数（默认每页20条），可自定义1-50之间 |
|   | sort | string | 否 | 商品排序方式，new：按照综合排序，  
total\_sale\_num\_asc：按照总销量从小到大排序，total\_sale\_num\_desc：按照总销量从大到小排序，  
sale\_num\_asc：按照年销量从小到大排序，sale\_num\_desc：按照年销量从大到小排序，  
commission\_rate\_asc：按照佣金比例从小到大排序，commission\_rate\_desc：按照佣金比例从大到小排序，  
price\_asc：按照价格从小到大排序，price\_desc：按照价格从大到小排序，  
coupon\_info\_money\_asc：按照优惠券金额从小到大排序，coupon\_info\_money\_desc：按照优惠券金额从大到小排序，  
shop\_level\_asc：按照店铺等级从低到高排序，shop\_level\_desc：按照店铺等级从高到低排序，  
tkfee\_asc：按照返佣金额从低到高排序，tkfee\_desc：按照返佣金额从高到低排序，  
code：按照code值从大到小排序，date\_time：按照更新时间排序，random：按照随机排序 |
|   | cid | int | 否 | 一级商品分类，值为空：全部商品，1：女装，2：母婴，3：美妆，4：居家日用，5：鞋品，6：美食，7：文娱车品，8：数码家电，9：男装，10：内衣，11：箱包，12：配饰，13：户外运动，14：家装家纺 |
|   | baodan | string | 否 | 极品爆单商品，值为空：全部商品，1：极品爆单商品（拍2件、拍3件、拍4件、拍5件、拍6件等） |

**成功返回示例：**/\*重要事情说三遍！重要事情说三遍！重要事情说三遍！api\_all/api\_dongdong/api\_quantian/api\_shishi/api\_videos/api\_xiaoshi/api\_yongjin接口所有参数都可以自由组合。\*/

var tmp = { "status": 200, /\*状态\*/ "content": \[{ "code": "6646", /\*折淘客编号\*/ "type\_one\_id": "1", /\*分类ID，可参考折淘客分类\*/ "tao\_id": "554832820990", /\*商品ID\*/ "title": "储物箱整理抽屉式收纳柜", /\*商品短标题\*/ "jianjie": "【爆款返场，出口日本】进口PP材质，可叠加透明可视抽屉收纳盒，分格抽屉物品分类齐全，区域分化一目了然，自由组合，大容量，让你居家生活更舒心，收纳你的生活！【赠运费险】", /\*商品简介\*/ "pict\_url": "https://img.alicdn.com/bao/......jpg", /\*商品主图\*/ "user\_type": "0", /\*是否天猫，0是淘宝，1是天猫\*/ "seller\_id": "3073204721", /\*卖家ID\*/ /\*店铺地址可自行拼接，http://store.taobao.com/shop/view\_shop.htm?user\_number\_id=3073204721 \*/ "shop\_dsr": "4.80", /\*商品描述分\*/ "volume": "169", /\*年销量\*/ "size": "26.90", /\*折扣价\*/ "quanhou\_jiage": "16.90", /\*券后价\*/ "date\_time\_yongjin": "2018/11/8 14:45:22", /\*数据更新时间\*/ "tkrate3": "30.00", /\*佣金比率\*/ "yongjin\_type": "MKT", /\*佣金类型\*/ "coupon\_id": "fe31d6c8132c462ba59bd72e03d2eae0", /\*优惠券ID 该商品如无优惠券，此字段为空\*/ "coupon\_start\_time": "2018-11-05 00:00:00", /\*优惠券开始时间 该商品如无优惠券，此字段为空\*/ "coupon\_end\_time": "2018-11-10 23:59:59", /\*优惠券结束时间 该商品如无优惠券，此字段为空\*/ "coupon\_info\_money": "10", /\*优惠券金额 该商品如无优惠券，此字段为0\*/ "coupon\_total\_count": "100000", /\*优惠券总数量 该商品如无优惠券，此字段为0\*/ "coupon\_remain\_count": "99965", /\*优惠券剩余数量 该商品如无优惠券，此字段为0\*/ "coupon\_info": "满80.00元减10元", /\*优惠券信息\*/ "juhuasuan": "0", /\*是否聚划算，1是\*/ "taoqianggou": "0", /\*是否淘抢购，1是\*/ "haitao": "0", /\*是否海淘，1是\*/ "jiyoujia": "0", /\*是否极有家，1是\*/ "jinpaimaijia": "0", /\*是否金牌卖家，1是\*/ "pinpai": "0", /\*是否精选品牌，1是\*/ "pinpai\_name": "", /\*品牌名称\*/ "yunfeixian": "0", /\*是否有运费险，1有\*/ "nick": "\*\*\*旗舰店", /\*卖家昵称\*/ "small\_images": "https://img.alicdn.com/bao/.......jpg|https://img.alicdn.com/bao/.......jpg", /\*商品小图列表\*/ "white\_image": "https://img.alicdn.com/bao/......jpg", /\*商品白底图\*/ "tao\_title": "抽屉式收纳箱塑料储物箱整理箱衣服物家用宿舍衣柜内衣内裤收纳盒", /\*商品长标题\*/ "provcity": "浙江 杭州", /\*宝贝所在地\*/ "shop\_title": "\*\*\*旗舰店", /\*店铺名称\*/ "zhibo\_url": "https://cloud.video.taobao.com/play/u/849090736/p/2/e/6/t/1/219853683676.mp4", /\*视频地址\*/ "sellCount": "860000", /\*淘宝网页实时总销量\*/ "commentCount": "250000", /\*评论数量\*/ "favcount": "180000", /\*收藏数量\*/ "score1": "4.9", /\*宝贝描述分\*/ "score2": "4.9", /\*卖家服务分\*/ "score3": "4.9", /\*物流服务\*/ "creditLevel": "15", /\*店铺等级，一星 二星 三星 四星 五星 一钻 二钻 三钻 四钻 五钻 一皇冠 二皇冠 三皇冠 四皇冠 五皇冠 一金冠 二金冠 三金冠 四金冠 五金冠\*/ "shopIcon": "//img.alicdn.com/imgextra/de/a6/TB10dC8SX.jpg", /\*店铺logo\*/ "pcDescContent": "1.jpg|2.jpg|3.jpg|4.jpg|5.jpg", /\*图文详情图片地址\*/ "item\_url": "https://item.taobao.com/item.htm?id=581801364036", /\*商品url\*/ "category\_id": "50006126", /\*叶子类目id\*/ "category\_name": "收纳盒", /\*叶子类目name\*/ "level\_one\_category\_id": "122928002", /\*一级类目id\*/ "level\_one\_category\_name": "收纳整理", /\*一级类目name\*/ "tkfee3": "5.07", /\*返佣金额\*/ "biaoqian": ""满300元,省30元"", /\*店铺活动\*/ "tag": "赶紧拯救你厨房的油腻吧😂<br>植护🌿加厚厨房抽纸<br>高密度纤维纸张，吸油💧很不错<br>5包装|￥14.8元💰<br>✔厨房小帮手，用处多多", /\*朋友圈文案，需要自己进行urldecode\*/ "date\_time": "2018/11/8 14:45:22" /\*数据添加时间\*/ }\] };
```json
{
	"status": 200,                      /\*状态\*/
	"content": \[{
		"code": "6646",                 /\*折淘客编号\*/
		"type\_one\_id": "1",             /\*分类ID，可参考折淘客分类\*/
		"tao\_id": "554832820990",       /\*商品ID\*/
		"title": "储物箱整理抽屉式收纳柜",                   /\*商品短标题\*/
		"jianjie": "【爆款返场，出口日本】进口PP材质，可叠加透明可视抽屉收纳盒，分格抽屉物品分类齐全，区域分化一目了然，自由组合，大容量，让你居家生活更舒心，收纳你的生活！【赠运费险】", /\*商品简介\*/
		"pict\_url": "https://img.alicdn.com/bao/......jpg",                   /\*商品主图\*/
		"user\_type": "0",                   /\*是否天猫，0是淘宝，1是天猫\*/
		"seller\_id": "3073204721",                   /\*卖家ID\*/ /\*店铺地址可自行拼接，http://store.taobao.com/shop/view\_shop.htm?user\_number\_id=3073204721 \*/
		"shop\_dsr": "4.80",                   /\*商品描述分\*/
		"volume": "169",                   /\*年销量\*/
		"size": "26.90",                   /\*折扣价\*/
		"quanhou\_jiage": "16.90",                   /\*券后价\*/
		"date\_time\_yongjin": "2018/11/8 14:45:22",                   /\*数据更新时间\*/
		"tkrate3": "30.00",                   /\*佣金比率\*/
		"yongjin\_type": "MKT",                   /\*佣金类型\*/
		"coupon\_id": "fe31d6c8132c462ba59bd72e03d2eae0",                   /\*优惠券ID 该商品如无优惠券，此字段为空\*/
		"coupon\_start\_time": "2018-11-05 00:00:00",                   /\*优惠券开始时间 该商品如无优惠券，此字段为空\*/
		"coupon\_end\_time": "2018-11-10 23:59:59",                   /\*优惠券结束时间 该商品如无优惠券，此字段为空\*/
		"coupon\_info\_money": "10",                   /\*优惠券金额 该商品如无优惠券，此字段为0\*/
		"coupon\_total\_count": "100000",                   /\*优惠券总数量 该商品如无优惠券，此字段为0\*/
		"coupon\_remain\_count": "99965",                   /\*优惠券剩余数量 该商品如无优惠券，此字段为0\*/
		"coupon\_info": "满80.00元减10元",                   /\*优惠券信息\*/
		"juhuasuan": "0",                       /\*是否聚划算，1是\*/
		"taoqianggou": "0",                     /\*是否淘抢购，1是\*/
		"haitao": "0",                          /\*是否海淘，1是\*/
		"jiyoujia": "0",                        /\*是否极有家，1是\*/
		"jinpaimaijia": "0",                    /\*是否金牌卖家，1是\*/
		"pinpai": "0",                          /\*是否精选品牌，1是\*/
		"pinpai\_name": "",                      /\*品牌名称\*/
		"yunfeixian": "0",                      /\*是否有运费险，1有\*/
		"nick": "\*\*\*旗舰店",                    /\*卖家昵称\*/
		"small\_images": "https://img.alicdn.com/bao/.......jpg|https://img.alicdn.com/bao/.......jpg",                   /\*商品小图列表\*/
		"white\_image": "https://img.alicdn.com/bao/......jpg",                   /\*商品白底图\*/
		"tao\_title": "抽屉式收纳箱塑料储物箱整理箱衣服物家用宿舍衣柜内衣内裤收纳盒",                   /\*商品长标题\*/
		"provcity": "浙江 杭州",                   /\*宝贝所在地\*/
		"shop\_title": "\*\*\*旗舰店",                   /\*店铺名称\*/
		"zhibo\_url": "https://cloud.video.taobao.com/play/u/849090736/p/2/e/6/t/1/219853683676.mp4",                   /\*视频地址\*/
		"sellCount": "860000",                   /\*淘宝网页实时总销量\*/
		"commentCount": "250000",                   /\*评论数量\*/
		"favcount": "180000",                   /\*收藏数量\*/
		"score1": "4.9",                   /\*宝贝描述分\*/
		"score2": "4.9",                   /\*卖家服务分\*/
		"score3": "4.9",                   /\*物流服务分\*/
		"creditLevel": "15",                   /\*店铺等级（1-20），一星 二星 三星 四星 五星 一钻 二钻 三钻 四钻 五钻 一皇冠 二皇冠 三皇冠 四皇冠 五皇冠  一金冠 二金冠 三金冠 四金冠 五金冠\*/
		"shopIcon": "//img.alicdn.com/imgextra/de/a6/TB10dC8SX.jpg",       /\*店铺logo\*/
		"pcDescContent": "1.jpg|2.jpg|3.jpg|4.jpg|5.jpg",  /\*图文详情图片地址\*/
		"item\_url": "https://item.taobao.com/item.htm?id=581801364036",  /\*商品url\*/
		"category\_id": "50006126",                   /\*叶子类目id\*/
		"category\_name": "收纳盒",                   /\*叶子类目name\*/
		"level\_one\_category\_id": "122928002",                   /\*一级类目id\*/
		"level\_one\_category\_name": "收纳整理",                   /\*一级类目name\*/
		"tkfee3": "5.07",                   /\*返佣金额\*/
		"biaoqian": ""满300元,省30元"",                   /\*店铺活动\*/
		"tag": "赶紧拯救你厨房的油腻吧😂植护🌿加厚厨房抽纸高密度纤维纸张，吸油💧很不错5包装|￥14.8元💰✔厨房小帮手，用处多多",                   /\*朋友圈文案，需要自己进行urldecode\*/
		"date\_time": "2018/11/8 14:45:22"                   /\*数据添加时间\*/
	}\]
}
```

**失败返回示例：**

var tmp = { "status": 301 };
```json
{
	"status": 301                      /\*状态\*/
}
```

[效果展示](http://cms.zhetaoke.com:10000/m/item/list.aspx?baodan=1)

[极品爆单说明](http://www.zhetaoke.com/help_detail_3_19.html)

---

## 失效商品API

**原始链接**: [失效商品API](https://www.zhetaoke.com/user/extend/extend_lingquan_shixiao.aspx)

**失效商品API---本接口采用12台16核心64G内存服务器，江苏和北京双机房，负载均衡技术，1G独享带宽，放心调用。**

失效商品API：返回站内失效商品列表，按照失效时间倒序排序。

**接口地址：** https://api.zhetaoke.com:10001/api/api\_shixiao.ashx?appkey=0a565628b2fb4712902712734cec2feb&page=1&page\_size=20

**返回数据：**json

**请求方式：**get

**请求参数说明：**

|  | 名称 | 类型 | 必填 | 说明 |
| --- | --- | --- | --- | --- |
|   | appkey | string | 是 | 折淘客的对接秘钥appkey |
|   | page | int | 否 | 分页获取数据,第几页 |
|   | page\_size | int | 否 | 每页数据条数（默认每页20条）,可自定义,最大100。 |
|   | start\_time | string | 否 | 失效开始时间，如：2019-07-01 00:00:00 |
|   | end\_time | string | 否 | 失效结束时间，如：2019-07-01 23:59:59 |

**成功返回示例：可以根据返回的code值进行循环获取列表，也可以根据失效开始时间和失效结束时间进行循环获取列表**

var tmp = { "status": 200, "content": \[ { "code": 8606, "id": 2897599, "tao\_id": 550079617480, "seller\_id": 3081566001, "activity\_id": "6a4120847b4e4406aded126c49d8fe4c", "reason\_type": 1, "date\_time": "2019/07/25 19:20:35" }, { "code": 8605, "id": 2897399, "tao\_id": 552658952962, "seller\_id": 2456251683, "activity\_id": "e6dce51e49fb45f7ae78e3dfcfb662da", "reason\_type": 1, "date\_time": "2019/07/25 19:19:58" }, { "code": 8604, "id": 2897396, "tao\_id": 552800167758, "seller\_id": 2456251683, "activity\_id": "e6dce51e49fb45f7ae78e3dfcfb662da", "reason\_type": 1, "date\_time": "2019/07/25 19:19:58" }, { "code": 8603, "id": 2897381, "tao\_id": 596989321908, "seller\_id": 2939572214, "activity\_id": "d540ac3565ca4193a4b3c0b708667b3a", "reason\_type": 1, "date\_time": "2019/07/25 19:19:55" }, { "code": 8602, "id": 2897161, "tao\_id": 580192638026, "seller\_id": 3980046019, "activity\_id": "7d02aba3841042b6b40200153fd68dd0", "reason\_type": 1, "date\_time": "2019/07/25 19:19:47" }, { "code": 8601, "id": 2897148, "tao\_id": 593894071030, "seller\_id": 1935567546, "activity\_id": "f5e3ebc63a3843d4b862a5f762966641", "reason\_type": 1, "date\_time": "2019/07/25 19:19:45" }, { "code": 8600, "id": 2897120, "tao\_id": 594079508990, "seller\_id": 3008205583, "activity\_id": "f4ee846cc03a4a86a08ccca9906dafb0", "reason\_type": 1, "date\_time": "2019/07/25 19:19:41" }, { "code": 8599, "id": 2896984, "tao\_id": 550366326482, "seller\_id": 756598406, "activity\_id": "eac0b2733e524308bc79462be0e638a1", "reason\_type": 1, "date\_time": "2019/07/25 19:19:21" }, { "code": 8598, "id": 2896958, "tao\_id": 543895861791, "seller\_id": 672771577, "activity\_id": "474addc513e64f79992d4af0b9106388", "reason\_type": 1, "date\_time": "2019/07/25 19:19:19" }, { "code": 8597, "id": 2896948, "tao\_id": 583298077187, "seller\_id": 4011436690, "activity\_id": "5b727ab9bca74f9e9fd105ac40b12e28", "reason\_type": 1, "date\_time": "2019/07/25 19:19:16" } \] }; document.write("<pre>" + syntaxHighlight(tmp) + "</pre>")
```json
{
    "status":200,
    "content":\[
        {
            "code": 8606,   /\* 自动增长列编号 \*/
            "id": 2897599,  /\* 原始商品编号code \*/
            "tao\_id": 550079617480,  /\* 商品编号 \*/
            "seller\_id": 3081566001,  /\* 店铺seller\_id \*/
            "activity\_id": "6a4120847b4e4406aded126c49d8fe4c",  /\* 优惠券id \*/
            "reason\_type": 1,  /\* 失效原因。1:优惠券无效，2:对应宝贝已下架或非淘客宝贝，3:商品上架时间超过7天，4:商家主动下架，5:黑名单下架，6:本券金额小于官券金额 \*/
            "date\_time": "2019/07/25 19:20:35"  /\* 失效时间 \*/
            },
        {
            "code":8605,
            "id":2897399,
            "tao\_id":552658952962,
            "seller\_id":2456251683,
            "activity\_id":"e6dce51e49fb45f7ae78e3dfcfb662da",
            "reason\_type":1,
            "date\_time":"2019/07/25 19:19:58"
        },
        {
            "code":8604,
            "id":2897396,
            "tao\_id":552800167758,
            "seller\_id":2456251683,
            "activity\_id":"e6dce51e49fb45f7ae78e3dfcfb662da",
            "reason\_type":1,
            "date\_time":"2019/07/25 19:19:58"
        },
        {
            "code":8603,
            "id":2897381,
            "tao\_id":596989321908,
            "seller\_id":2939572214,
            "activity\_id":"d540ac3565ca4193a4b3c0b708667b3a",
            "reason\_type":1,
            "date\_time":"2019/07/25 19:19:55"
        },
        {
            "code":8602,
            "id":2897161,
            "tao\_id":580192638026,
            "seller\_id":3980046019,
            "activity\_id":"7d02aba3841042b6b40200153fd68dd0",
            "reason\_type":1,
            "date\_time":"2019/07/25 19:19:47"
        },
        {
            "code":8601,
            "id":2897148,
            "tao\_id":593894071030,
            "seller\_id":1935567546,
            "activity\_id":"f5e3ebc63a3843d4b862a5f762966641",
            "reason\_type":1,
            "date\_time":"2019/07/25 19:19:45"
        },
        {
            "code":8600,
            "id":2897120,
            "tao\_id":594079508990,
            "seller\_id":3008205583,
            "activity\_id":"f4ee846cc03a4a86a08ccca9906dafb0",
            "reason\_type":1,
            "date\_time":"2019/07/25 19:19:41"
        },
        {
            "code":8599,
            "id":2896984,
            "tao\_id":550366326482,
            "seller\_id":756598406,
            "activity\_id":"eac0b2733e524308bc79462be0e638a1",
            "reason\_type":1,
            "date\_time":"2019/07/25 19:19:21"
        },
        {
            "code":8598,
            "id":2896958,
            "tao\_id":543895861791,
            "seller\_id":672771577,
            "activity\_id":"474addc513e64f79992d4af0b9106388",
            "reason\_type":1,
            "date\_time":"2019/07/25 19:19:19"
        },
        {
            "code":8597,
            "id":2896948,
            "tao\_id":583298077187,
            "seller\_id":4011436690,
            "activity\_id":"5b727ab9bca74f9e9fd105ac40b12e28",
            "reason\_type":1,
            "date\_time":"2019/07/25 19:19:16"
        }
    \]
}
```

  
  
  
  
  
  
  
  
  
  
  

[效果展示](http://cms.zhetaoke.com:10000/m/index_super.aspx)

---

## 热搜词词典API

**原始链接**: [热搜词词典API](https://www.zhetaoke.com/user/extend/extend_lingquan_guanjianci.aspx)

**热搜词词典API---本接口采用12台16核心64G内存服务器，江苏和北京双机房，负载均衡技术，1G独享带宽，放心调用。**

热搜词词典API：返回热搜词词典表，搜索量极大，曝光量极高的词（适合用来优化标题和了解买家实时需求），分淘客热搜词和买家热搜词。

**接口地址：** https://api.zhetaoke.com:10001/api/api\_guanjianci.ashx?appkey=0a565628b2fb4712902712734cec2feb&page=1&page\_size=1000&type=0

淘客热搜榜API：根据淘客热搜词，返回相应热搜词的商品列表。

**接口地址：** https://api.zhetaoke.com:10001/api/api\_all.ashx?appkey=0a565628b2fb4712902712734cec2feb&page=1&page\_size=20&sort=new&q=月饼

买家热搜榜API：根据买家热搜词，返回相应热搜词的商品列表。

**接口地址：** https://api.zhetaoke.com:10001/api/api\_all.ashx?appkey=0a565628b2fb4712902712734cec2feb&page=1&page\_size=20&sort=new&q=洗手液

**返回数据：**json

**请求方式：**get

**请求参数说明：**

|  | 名称 | 类型 | 必填 | 说明 |
| --- | --- | --- | --- | --- |
|   | appkey | string | 是 | 折淘客的对接秘钥appkey |
|   | page | int | 否 | 分页获取数据,第几页 |
|   | page\_size | int | 否 | 每页数据条数（默认每页1000条）,可自定义。 |
|   | type | string | 否 | 类型，0：淘客热搜词，1：买家热搜词 |

**成功返回示例：**

var tmp = { "status": 200, "content": \[ { "keywords": "连衣裙" }, { "keywords": "真丝连衣裙" }, { "keywords": "手机" }, { "keywords": "女装" }, { "keywords": "窗帘" }, { "keywords": "女装" }, { "keywords": "旗袍" }, { "keywords": "沙发" }, { "keywords": "鱼竿" }, { "keywords": "华为" }, { "keywords": "衣柜" }, { "keywords": "t恤" }, { "keywords": "半身裙" }, { "keywords": "蓝牙耳机" }, { "keywords": "男鞋" }, { "keywords": "行车记录仪" }, { "keywords": "真丝香云纱连衣裙" }, { "keywords": "汉服" }, { "keywords": "拉杆箱" }, { "keywords": "实木床" } \] }; document.write("<pre>" + syntaxHighlight(tmp) + "</pre>")
```json
{
    "status":200,
    "content":\[
        {
            "keywords":"连衣裙"
        },
        {
            "keywords":"真丝连衣裙"
        },
        {
            "keywords":"手机"
        },
        {
            "keywords":"女装"
        },
        {
            "keywords":"窗帘"
        },
        {
            "keywords":"女装"
        },
        {
            "keywords":"旗袍"
        },
        {
            "keywords":"沙发"
        },
        {
            "keywords":"鱼竿"
        },
        {
            "keywords":"华为"
        },
        {
            "keywords":"衣柜"
        },
        {
            "keywords":"t恤"
        },
        {
            "keywords":"半身裙"
        },
        {
            "keywords":"蓝牙耳机"
        },
        {
            "keywords":"男鞋"
        },
        {
            "keywords":"行车记录仪"
        },
        {
            "keywords":"真丝香云纱连衣裙"
        },
        {
            "keywords":"汉服"
        },
        {
            "keywords":"拉杆箱"
        },
        {
            "keywords":"实木床"
        }
    \]
}
```

  
  
  
  
  
  
  
  
  
  
  

[效果展示](http://cms.zhetaoke.com:10000/m/index_super.aspx)

---

## 联想词API

**原始链接**: [联想词API](https://www.zhetaoke.com/user/extend/extend_lingquan_suggest.aspx)

**联想词API---本接口采用12台16核心64G内存服务器，江苏和北京双机房，负载均衡技术，1G独享带宽，放心调用。**

联想词API：根据搜索关键词返回联想词，完善您的搜索功能，建议用户停止输入时进行接口请求。

**接口地址：** https://api.zhetaoke.com:10001/api/api\_suggest.ashx?appkey=0a565628b2fb4712902712734cec2feb&content=扇子

**返回数据：**json

**请求方式：**get

**请求参数说明：**

|  | 名称 | 类型 | 必填 | 说明 |
| --- | --- | --- | --- | --- |
|   | appkey | string | 是 | 折淘客的对接秘钥appkey |
|   | content | string | 是 | 搜索关键词。请注意，该参数需要进行Urlencode编码后传入。。  
重要的事情说三遍：该参数需要进行Urlencode编码！该参数需要进行Urlencode编码！该参数需要进行Urlencode编码！ |

**成功返回示例：**

var tmp = { "result": \[ \[ "扇子折扇 中国风", "253722.61181201233" \], \[ "扇子女 古风", "66873.67656752412" \], \[ "扇子舞演出服", "87467.25353959098" \], \[ "扇子折扇", "362131.40750769887" \], \[ "扇子夏季", "246631.23860046975" \], \[ "扇子女可爱便携", "24993.983839235527" \], \[ "扇子定制", "150270.18110635423" \], \[ "扇子男生霸气", "3605.16945738386" \], \[ "扇子折扇 古风 随身", "79041.21830118999" \], \[ "扇子舞蹈扇", "550753.9458154506" \] \] }; document.write("<pre>" + syntaxHighlight(tmp) + "</pre>")
```json
{
    "result":\[
        \[
            "扇子折扇 中国风",
            "253722.61181201233"
        \],
        \[
            "扇子女 古风",
            "66873.67656752412"
        \],
        \[
            "扇子舞演出服",
            "87467.25353959098"
        \],
        \[
            "扇子折扇",
            "362131.40750769887"
        \],
        \[
            "扇子夏季",
            "246631.23860046975"
        \],
        \[
            "扇子女可爱便携",
            "24993.983839235527"
        \],
        \[
            "扇子定制",
            "150270.18110635423"
        \],
        \[
            "扇子男生霸气",
            "3605.16945738386"
        \],
        \[
            "扇子折扇 古风 随身",
            "79041.21830118999"
        \],
        \[
            "扇子舞蹈扇",
            "550753.9458154506"
        \]
    \]
}
```

  
  
  
  
  
  
  
  
  
  
  

[效果展示](http://cms.zhetaoke.com:10000/m/index_super.aspx)

---

## 轮播图API

**原始链接**: [轮播图API](https://www.zhetaoke.com/user/extend/extend_lingquan_lunbo.aspx)

**轮播图API---本接口采用12台16核心64G内存服务器，江苏和北京双机房，负载均衡技术，1G独享带宽，放心调用。**

轮播图API：返回轮播图列表和相应的专题商品请求地址。

**接口地址：** https://api.zhetaoke.com:10001/api/api\_lunbo.ashx?appkey=0a565628b2fb4712902712734cec2feb&page=1&page\_size=20

**返回数据：**json

**请求方式：**get

**请求参数说明：**

|  | 名称 | 类型 | 必填 | 说明 |
| --- | --- | --- | --- | --- |
|   | appkey | string | 是 | 折淘客的对接秘钥appkey |
|   | page | int | 否 | 分页获取数据,第几页 |
|   | page\_size | int | 否 | 每页数据条数（默认每页20条），可自定义1-50之间 |

**成功返回示例：**

var tmp = { "status": 200, "content": \[ { "code": "5", "name": "9块9包邮", "pic": "https://api.zhetaoke.com:10001/api/banner/13.jpg", "get\_url": "https://api.zhetaoke.com:10001/api/api\_all.ashx?appkey={替换appkey}&page=1&page\_size=20&sort=sale\_num\_desc&price=0.0-9.9", "date\_time": "2019/08/03 17:25:08" }, { "code": "4", "name": "天猫超市", "pic": "https://api.zhetaoke.com:10001/api/banner/12.jpg", "get\_url": "https://api.zhetaoke.com:10001/api/api\_all.ashx?appkey={替换appkey}&page=1&page\_size=20&sort=sale\_num\_desc&tianmaochaoshi=1", "date\_time": "2019/08/03 17:25:02" }, { "code": "3", "name": "咚咚抢", "pic": "https://api.zhetaoke.com:10001/api/banner/11.jpg", "get\_url": "https://api.zhetaoke.com:10001/api/api\_dongdong.ashx?appkey={替换appkey}&page=1&page\_size=20&sort=sale\_num\_desc&t=0-23", "date\_time": "2019/08/03 17:24:54" }, { "code": "2", "name": "爆款商品排行", "pic": "https://api.zhetaoke.com:10001/api/banner/10.jpg", "get\_url": "https://api.zhetaoke.com:10001/api/api\_all.ashx?appkey={替换appkey}&page=1&page\_size=20&sort=new&baodan=1", "date\_time": "2019/08/03 17:24:47" }, { "code": "1", "name": "全网超级搜索", "pic": "https://api.zhetaoke.com:10001/api/banner/1.jpg", "get\_url": "https://api.zhetaoke.com:10003/api/api\_quanwang.ashx?appkey={替换appkey}&page=1&page\_size=20&sort=new&q=内衣", "date\_time": "2019/08/03 17:18:08" } \] }; document.write("<pre>" + syntaxHighlight(tmp) + "</pre>")
```json
{
    "status":200,
    "content":\[
        {
            "code":"6", /\*自动增长列编号\*/
            "name":"天猫国际",  /\*轮播图名称\*/
            "pic":"https://api.zhetaoke.com:10001/api/banner/20.jpg",   /\*图片地址\*/
            "get\_url":"https://api.zhetaoke.com:10001/api/api\_all.ashx?appkey={替换appkey}&page=1&page\_size=20&sort=sale\_num\_desc&jh=haitao", /\*商品请求地址\*/
            "date\_time":"2019/08/03 17:25:17"   /\*添加时间\*/
        },
        {
            "code":"5",
            "name":"9块9包邮",
            "pic":"https://api.zhetaoke.com:10001/api/banner/13.jpg",
            "get\_url":"https://api.zhetaoke.com:10001/api/api\_all.ashx?appkey={替换appkey}&page=1&page\_size=20&sort=sale\_num\_desc&price=0.0-9.9",
            "date\_time":"2019/08/03 17:25:08"
        },
        {
            "code":"4",
            "name":"天猫超市",
            "pic":"https://api.zhetaoke.com:10001/api/banner/12.jpg",
            "get\_url":"https://api.zhetaoke.com:10001/api/api\_all.ashx?appkey={替换appkey}&page=1&page\_size=20&sort=sale\_num\_desc&tianmaochaoshi=1",
            "date\_time":"2019/08/03 17:25:02"
        },
        {
            "code":"3",
            "name":"咚咚抢",
            "pic":"https://api.zhetaoke.com:10001/api/banner/11.jpg",
            "get\_url":"https://api.zhetaoke.com:10001/api/api\_dongdong.ashx?appkey={替换appkey}&page=1&page\_size=20&sort=sale\_num\_desc&t=0-23",
            "date\_time":"2019/08/03 17:24:54"
        },
        {
            "code":"2",
            "name":"爆款商品排行",
            "pic":"https://api.zhetaoke.com:10001/api/banner/10.jpg",
            "get\_url":"https://api.zhetaoke.com:10001/api/api\_all.ashx?appkey={替换appkey}&page=1&page\_size=20&sort=new&baodan=1",
            "date\_time":"2019/08/03 17:24:47"
        },
        {
            "code":"1",
            "name":"全网超级搜索",
            "pic":"https://api.zhetaoke.com:10001/api/banner/1.jpg",
            "get\_url":"https://api.zhetaoke.com:10003/api/api\_quanwang.ashx?appkey={替换appkey}&page=1&page\_size=20&sort=new&q=内衣",
            "date\_time":"2019/08/03 17:18:08"
        }
    \]
}
```

  
  
  
  
  
  
  
  
  
  
  

[效果展示](http://app.zhetaoke.com:10000/index.html)

---

## 精选礼物专题API

**原始链接**: [精选礼物专题API](https://www.zhetaoke.com/user/extend/extend_lingquan_liwu_zhuanti.aspx)

**精选礼物专题API---本接口采用12台16核心64G内存服务器，江苏和北京双机房，负载均衡技术，1G独享带宽，放心调用。**

精选礼物专题API：返回精选礼物专题列表和相应的专题商品请求地址。

**接口地址：** https://api.zhetaoke.com:10001/api/api\_liwu\_zhuanti.ashx?appkey=0a565628b2fb4712902712734cec2feb&page=1&page\_size=20&type\_id=0&sort=random

**返回数据：**json

**请求方式：**get

**请求参数说明：**

|  | 名称 | 类型 | 必填 | 说明 |
| --- | --- | --- | --- | --- |
|   | appkey | string | 是 | 折淘客的对接秘钥appkey |
|   | page | int | 否 | 分页获取数据,第几页 |
|   | page\_size | int | 否 | 每页数据条数（默认每页20条），可自定义1-50之间 |
|   | type\_id | int | 否 | 分类id，0表示全部，1表示综合类，以后会不断扩充分类 |
|   | sort | string | 否 | 排序方式，code\_asc：表示按照code从小到大排序，code\_desc：表示按照code从大到小排序，random：表示随机排序 |

**成功返回示例：**

var tmp = { "status": 200, "content": \[ { "code": "27", "type\_id": "1", "type\_name": "综合", "title": "卡通花束礼物", "keywords": "卡通花束", "jianjie": "小编为您提供大量精美的卡通花束礼物，包括各种优质的卡通花束礼物、珍贵的卡通花束礼物、特别的卡通花束礼物、实用的卡通花束礼物，以及各种让人感动的卡通花束礼物。希望您会喜欢并持续关注小编为您带来的各种精美的卡通花束礼物。", "sucai\_pic": "https://img.alicdn.com/bao/uploaded/i1/713945573/O1CN015TnUIW1r2TmJE72ei\_!!0-item\_pic.jpg\_300x300.jpg", "get\_url": "https://api.zhetaoke.com:10001/api/api\_all.ashx?appkey={替换appkey}&page=1&page\_size=20&sort=sale\_num\_desc&cid=0&q=卡通花束", "publish\_time": "2018/05/17 20:51:27" }, { "code": "16", "type\_id": "1", "type\_name": "综合", "title": "钢笔礼物", "keywords": "钢笔", "jianjie": "小编为您提供大量精美的钢笔礼物，包括各种优质的钢笔礼物、珍贵的钢笔礼物、特别的钢笔礼物、实用的钢笔礼物，以及各种让人感动的钢笔礼物。希望您会喜欢并持续关注小编为您带来的各种精美的钢笔礼物。", "sucai\_pic": "https://img.alicdn.com/bao/uploaded/TB1kKOrVOLaK1RjSZFxXXamPFXa.png\_300x300.jpg", "get\_url": "https://api.zhetaoke.com:10001/api/api\_all.ashx?appkey={替换appkey}&page=1&page\_size=20&sort=sale\_num\_desc&cid=0&q=钢笔", "publish\_time": "2018/05/17 20:49:47" }, { "code": "42", "type\_id": "1", "type\_name": "综合", "title": "机器人礼物", "keywords": "机器人", "jianjie": "小编为您提供大量精美的机器人礼物，包括各种优质的机器人礼物、珍贵的机器人礼物、特别的机器人礼物、实用的机器人礼物，以及各种让人感动的机器人礼物。希望您会喜欢并持续关注小编为您带来的各种精美的机器人礼物。", "sucai\_pic": "https://img.alicdn.com/bao/uploaded/TB1wcY4yAvoK1RjSZFwXXciCFXa.png\_300x300.jpg", "get\_url": "https://api.zhetaoke.com:10001/api/api\_all.ashx?appkey={替换appkey}&page=1&page\_size=20&sort=sale\_num\_desc&cid=0&q=机器人", "publish\_time": "2018/05/17 20:53:01" } \] }; document.write("<pre>" + syntaxHighlight(tmp) + "</pre>")
```json
{
    "status":200,
    "content":\[
        {
            "code":"27",
            "type\_id":"1",
            "type\_name":"综合",
            "title":"卡通花束礼物",
            "keywords":"卡通花束",
            "jianjie":"小编为您提供大量精美的卡通花束礼物，包括各种优质的卡通花束礼物、珍贵的卡通花束礼物、特别的卡通花束礼物、实用的卡通花束礼物，以及各种让人感动的卡通花束礼物。希望您会喜欢并持续关注小编为您带来的各种精美的卡通花束礼物。",
            "sucai\_pic":"https://img.alicdn.com/bao/uploaded/i1/713945573/O1CN015TnUIW1r2TmJE72ei\_!!0-item\_pic.jpg\_300x300.jpg",
            "get\_url":"https://api.zhetaoke.com:10001/api/api\_all.ashx?appkey={替换appkey}&page=1&page\_size=20&sort=sale\_num\_desc&cid=0&q=卡通花束",
            "publish\_time":"2019/05/17 20:51:27"
        },
        {
            "code":"16",
            "type\_id":"1",
            "type\_name":"综合",
            "title":"钢笔礼物",
            "keywords":"钢笔",
            "jianjie":"小编为您提供大量精美的钢笔礼物，包括各种优质的钢笔礼物、珍贵的钢笔礼物、特别的钢笔礼物、实用的钢笔礼物，以及各种让人感动的钢笔礼物。希望您会喜欢并持续关注小编为您带来的各种精美的钢笔礼物。",
            "sucai\_pic":"https://img.alicdn.com/bao/uploaded/TB1kKOrVOLaK1RjSZFxXXamPFXa.png\_300x300.jpg",
            "get\_url":"https://api.zhetaoke.com:10001/api/api\_all.ashx?appkey={替换appkey}&page=1&page\_size=20&sort=sale\_num\_desc&cid=0&q=钢笔",
            "publish\_time":"2019/05/17 20:49:47"
        },
        {
            "code":"42",
            "type\_id":"1",
            "type\_name":"综合",
            "title":"机器人礼物",
            "keywords":"机器人",
            "jianjie":"小编为您提供大量精美的机器人礼物，包括各种优质的机器人礼物、珍贵的机器人礼物、特别的机器人礼物、实用的机器人礼物，以及各种让人感动的机器人礼物。希望您会喜欢并持续关注小编为您带来的各种精美的机器人礼物。",
            "sucai\_pic":"https://img.alicdn.com/bao/uploaded/TB1wcY4yAvoK1RjSZFwXXciCFXa.png\_300x300.jpg",
            "get\_url":"https://api.zhetaoke.com:10001/api/api\_all.ashx?appkey={替换appkey}&page=1&page\_size=20&sort=sale\_num\_desc&cid=0&q=机器人",
            "publish\_time":"2019/05/17 20:53:01"
        }
    \]
}
```

  
  
  
  
  
  
  
  
  
  
  

[效果展示](http://app.zhetaoke.com:10000/index.html)

---

## 淘宝分词API

**原始链接**: [淘宝分词API](https://www.zhetaoke.com/user/extend/extend_lingquan_fenci.aspx)

**淘宝分词API---本接口采用12台16核心64G内存服务器，江苏和北京双机房，负载均衡技术，1G独享带宽，放心调用。**

淘宝分词API：根据搜索关键词返回一个最优的分词结果，完善您的搜索功能，在精确搜索找不到商品的情况下，可以使用该接口，适合模糊搜索。

**接口地址：** https://api.zhetaoke.com:10001/api/api\_fenci.ashx?appkey=0a565628b2fb4712902712734cec2feb&text=真学桌面看书架多功能书架儿童读书架桌上学生阅读架书立可伸缩书靠折叠书本支架简易夹书器

**返回数据：**json

**请求方式：**get/post

**请求参数说明：**

|  | 名称 | 类型 | 必填 | 说明 |
| --- | --- | --- | --- | --- |
|   | appkey | string | 是 | 折淘客的对接秘钥appkey |
|   | text | string | 是 | 搜索关键词，一般是淘宝商品长标题或者商品简介等内容。请注意，该参数需要进行Urlencode编码后传入。。  
重要的事情说三遍：该参数需要进行Urlencode编码！该参数需要进行Urlencode编码！该参数需要进行Urlencode编码！ |
|   | type | string | 否 | 分词模式，type=0关键词提取模式，表示提取最大前10个仅包含名词和动词的关键词，type=1表示精确模式分词结果，type=2表示第二种关键词提取模式。 |
|   | count | string | 否 | 返回最大前10个关键词，默认是1，可以输入1-10之间的任意整数，提取最大前10个仅包含名词和动词的关键词，该字段type=0有效。 |

**type=0成功返回示例：**

var tmp = { "status": 200, "content": "书架,书立,真学,夹书器,书本,桌面,伸缩,折叠,支架,读书" } document.write("<pre>" + syntaxHighlight(tmp) + "</pre>")
```json
{
    "status":200,
    "content":"书架,书立,真学,夹书器,书本,桌面,伸缩,折叠,支架,读书"
}
```

**type=1成功返回示例：**

var tmp = { "status": 200, "content": "真学,桌面,看,书架,多功能,书架,儿童,读书,架,桌上,学生,阅读,架,书立,可,伸缩,书靠,折叠,书本,支架,简易,夹书器" } document.write("<pre>" + syntaxHighlight(tmp) + "</pre>")
```json
{
    "status":200,
    "content":"真学,桌面,看,书架,多功能,书架,儿童,读书,架,桌上,学生,阅读,架,书立,可,伸缩,书靠,折叠,书本,支架,简易,夹书器"
}
```

---

