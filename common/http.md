# HTTP(HyperText Transfer Protocol)
## 简介
- HTTP是无连接的：每次只处理一个请求
- HTTP是媒体独立的：只要客户端和服务器知道如何处理数据，任何类型的数据都可以通过HTTP发送，客户端以及服务器指定使用合适的MIME-type内容类型
- HTTP是无状态协议：无状态是指协议对于事物处理没有记忆能力。这意味着后续处理如果需要前面的信息，则它必须重传

## HTTP消息结构
HTTP使用URI(Unifrom Resource Identifiers)来传输数据和建立连接
一旦建立连接后，数据消息就会通过类似Inetnet邮件所使用的格式[RFC5322]和多用途Internet邮件扩展(MIME)[RFC2045]来传送

### 客户端请求信息
![header](../assets/request_header.png)

Postman:
```
POST /api/v4/custom_remoters/update_custom_remoters?user_id=100001 HTTP/1.1
Host: 192.168.1.112:3000
Cache-Control: no-cache
Postman-Token: c9102c5b-0d77-7533-b613-d237aac31f89
Content-Type: multipart/form-data;
boundary=----WebKitFormBoundary7MA4YWxkTrZu0gW

------WebKitFormBoundary7MA4YWxkTrZu0gW
Content-Disposition: form-data; name="user_id"

100001
------WebKitFormBoundary7MA4YWxkTrZu0gW
Content-Disposition: form-data; name="customer_keys"

{key_id:1,group:1,order:1}
------WebKitFormBoundary7MA4YWxkTrZu0gW--
```

### HTTP 请求方法
&nbsp; |  方法    | 描述
------ | ------- | -----------------------------------------
1      | GET     | 请求指定的页面信息，并返回实体主体
2      | HEAD    | 类似于GET，只不过返回的响应中没有具体的内容，用于获取报头
3      | POST    | 向指定的资源提交数据进行处理请求(表单，文件)
4      | PUT     | 从客户端向服务器传送的数据取代指定的文档的内容
5      | DELETE  | 请求服务器删除指定的页面
6      | CONNECT | HTTP/1.1协议中预留给能够将连接改为管道方式的代理服务器
7      | OPTIONS | 允许客户端查看服务器的性能
8      | TRACE   | 回显服务器收到的请求，主要用于测试或诊断

### HTTP 响应头信息
HTTP请求头提供了关于请求，响应或者其他的发送实体的信息  
- Allow: 服务器支持哪些请求方法(GET POST ...)
- Content-Encoding:
  文档的编码(Encode)方法。只有在解码后才可以得到Content-Type头指定的内容类型
- Content-Length: 表示内容长度。只有当浏览器使用持久HTTP连接时才需要这个数据
- Content-Type: 表示后面的文档属于什么MIME类型。
- Date: 当前的GMT时间
- Expires: 应该在什么时候认为文档已过期，从而不再缓存它
- Last-Modified:
  文档最后的修改时间，只有当改动时间迟于指定时间的文档才会返回，否则返回304(Not
Modified)
- Location: 表示客户应当到哪里去提取文档
- Refresh: 表示浏览器应当在多少时间后刷新文档，秒单位。
可以通过`etHeader("Refresh", "5;URL=http://host/path")让浏览器读取指定的页面`
```
<META HTTP-EQUIV="Refresh" CONTENT="5;URL=http://host/path">
```
- Server: 服务器的名字
- Set-Cookie: 设置和页面关联的Cookie
- WWW-Authenticate: 授权信息

### HTTP 状态码
- 1**：信息，服务器收到请求，需要请求者继续执行操作
- 2**：成功，操作被成功接受并处理
- 3**：重定向，需要进一步的操作以完成请求
- 4**：客户端错误，请求包含语法错误或无法完成请求
- 5**：服务器错误，服务器在处理请求的过程中发生错误

### HTTP Content-Type
内容类型，一般指网页中存在的Content-Type，用于定义网络文件的类型和网页的编码，决定浏览器将以什么形式、什么编码来读取这个文件

#### 表单的发包方式
> 简单的来说，当表单中有文件`<input type='file'>`元素时，使用`multipart/form data`  
> 永远不要使用`text/plain`（黑人问号）  
> `application/x-www-form-urlencoded`或多或少的有点像URL后面的query参数  
> `multipart/form-data`明显更复杂，但是允许整个文件被包含在数据中  

**application/x-www-from-urlencoded**  
常用的表单发包方式，普通的表单提交，或者js发包，默认都是这种
```html
<form encrypt="application/x-www-form-urlencoded" action="" method="POST">
</form>
```
服务器收到的raw header:
```
Accept:text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8
Accept-Encoding:gzip, deflate
Accept-Language:zh-CN,zh;q=0.8,en;q=0.6,zh-TW;q=0.4,gl;q=0.2,de;q=0.2
Cache-Control:no-cache
Connection:keep-alive(附录)
Content-Length:17
Content-Type:application/x-www-form-urlencoded
```
**multiplepart/form-data**  
用在发送文件的POST包  
```python
data = { "key": "123" }
files = {'file': open('index.py', 'rb')}
res = request.post(url="http://localhost/upload", method="POST", data=data,
files=files)
print res
```
会发送如下数据
```
POST http://www.homeway.me HTTP/1.1
Content-Type:multipart/form-data;
boundary=------WebKitFormBoundaryOGkWPJsSaJCPWjZP

------WebKitFormBoundaryOGkWPJsSaJCPWjZP
Content-Disposition: form-data; name="key2"
456
------WebKitFormBoundaryOGkWPJsSaJCPWjZP
Content-Disposition: form-data; name="key1"
123
------WebKitFormBoundaryOGkWPJsSaJCPWjZP
Content-Disposition: form-data; name="file"; filename="index.py"
```

**text/html**  
它是一种使用HTTP作为传输协议，XML作为编码方式的远程调用规范  
微信用的是这种数据发送请求的
```
POST http://www/homeway.me HTTP/1.1
Content-Type: text/xml

<?xml version="1.0"?>
<resouce>
  <id>123</id>
  <params>
    <name>
      <value>homwway</value>
    </name>
    <age>
      <value>22</value>
    </age>
  </params>
</resouce>
```

~~**application/json**~~  
~~使用json形式将数据发送给服务器~~  
~~可以方便的提交复杂的结构化数据，特别适合RESTful的接口~~
has been abandoned.

# 附录
## Keep-Alive
普通的模式每次请求都要建立一个新的连接，完成后立刻断开；  
而`Keep-Alive`(持久连接/连接重用)使客户端到服务器的连接持续有效，这样就避免了重复建立连接的开销  
http1.1 默认开启，`Connection: close`关闭  
### 特点：  
1. 更少的建立和关闭连接，节省了CPU在的routers和hosts(clients,servers,proxies,gateway,tunels,caches)上面的时间开销  
2. HTTP的请求和响应可以在一个连接中进行管道传输(pipelining)，管道传输允许客户端发出多个请求而不必等每个返回，使用一个独立的TCP连接更有效率和节约时间  
3. 减少建立连接请求就减少了包的发送，从而减少了网络堵塞，and by allowing TCP sufficient time to determine the congestion state of the network.  
4. 潜在的也减少了随之而来的请求，因为没有了握手请求  
5. HTTP还需要改进，比如错误应该被提交而不是关闭连接，新的HTTP协议和老的服务器还有冲突，最好可以在错误被捕获后自动切换到老的协议  
> 单用户客户端与任何服务器或代理之间的连接数不应该超过两个，一个代理与其他服务器或代码之间应该使用不超过2*N的活跃并发连接，这是为了提高HTTP响应时间，避免堵塞（冗余的连接并不能带来代码执行能力的提升）  

### 如何判断消息内容/长度的大小
1. 使用`Content-Length`  
静态内容(静态页面或者图片)  
2. 使用`Transfer-Encoding`  
动态页面，一遍产生数据，一遍发给客户端`Transfer-Encoding: chunked`  
- Chunk编码将数据分成一块一块的发生  
- Chunked编码将使用若干个Chunk串连接而成，由一个标明长度是0的Chunk标志结束  
Chunk编码格式：  
```
Chunked-Body = *<strong>chunk </strong>
"0" CRLF
footer
CRLF
chunk = chunk-size [ chunk-ext ] CRLF
chunk-data CRLF

hex-no-zero = &lt;HEX excluding "0"&gt;

chunk-size = hex-no-zero *HEX
chunk-ext = *( ";" chunk-ext-name [ "=" chunk-ext-value ] )
chunk-ext-name = token
chunk-ext-val = token | quoted-string
chunk-data = chunk-size(OCTET)

footer = *entity-header
```
> 即Chunk编码由四部分组成:  
> 1. 0至N个Chunk块  
> 2. "0" CRLF  
> 3. footer     
> 4. CRLF . 而每个chunk块由：chunk-size、chunk-ext（可选）、CRLF、chunk-data、CRLF组成。

### 消息长度总结
一个消息的`transfer-length`(传输长度)是指消息中的`message-body`(消息体)的长度。当应用了`transfer-coding`(传输编码)，每个消息中的`message-body`的长度(transfer-legnth)由一下集中情况决定(优先级高到低)  
1. 任何不含消息体的内容(如1xx、204，304等响应消息和任何头(HEAD，首部)请求的响应消息)，总是由一个空行(CLRF)结束  
2. 如果出现了`Transfer-Encoding`字段，且值不是`identity`，那么`Transfer-Length`由`Chunked`传输编码定义，除非消息由于关闭连接而终止  
3. 出现了`Content-Length`头字段，它的值表示`Entity-Length`(实体长度)和`Transfer-Length`。如果这两个长度的大小不一样，那么将不能发送`Content-Length`字段，并且如果同时接收到了`Transfer-Encoding`和`Content-Lenght`字段，那么必须忽略`Content-Length`字段  
4. 如果消息体使用媒体类型`multipart/byteranges`，并且`transger-length`没有另外指定，那么会自鉴定(self-delimiting)媒体类型定义`Transfer-Length`。除非发送者知道接受者能够解析该种类型，否则不能使用该类型  
5. 服务器关闭连接确定消息长度  
为了兼容HTTP/1.0，必须包含一个合法的`Content-Lenght`字段，除非知道服务器兼容HTTP/1.1。一个请求包含消息体，但是`Content-Lenght`没有指定，如果不恩给你判断消息的长度，服务器应该返回400或411  

所有HTTP/1.1的接受者应用程序必须接受`chunked``transfer-coding`，因此当不能实现知道消息的长度，允许使用这种机制来传输信息

## HTTP头字段总结
1. Accept: 告诉服务器自己接受什么介质类型，`/`表示任何类型，`type/*`表示该类型下的所有子类型`type/subtype`  
2. Accept-Charset: 浏览器声明自己接受的字符集  
    - Accept-Encoding: 浏览器声明自己接受的编码方式，通常指定压缩方法，是否支持，支持哪些(gzip, deflate)  
    - Accept-Language: 浏览器声明自己接受的语言(中文是语言，中文有多种字符集big5, gb2312等)    
3. Accept-Ranges: 服务器表明自己是否接受获取某个实体的一部分的请求(如文件的一部分)。bytes：接受，none：不接受  
4. Age: 当代理服务器用自己缓存的实体去响应请求时，用该头部表明该实体从产生到现在经过了多长时间  
5. Authorization: 当客户端接收到来自服务器的`WWW-Authenticate`响应时，用该头部来回应自己的身份验证信息  
6. Cache-Control:   
    请求:  
    - no-chche: 不要缓存的实体  
    - max-age: 只接受Age小余max-age的  
    - min-fresh: 接受新鲜声明期大于当前Age和min-fresh值之和的缓存对象  
    响应:  
    - public: 可以用Cached内容回应任何用户  
    - private: 值能用缓存内容回应先前请求该内容的那个用户  
    - no-cache: 可以缓存，但是只有在和web服务器验证了其有效后才能返回给客户端  
    - max-age: 本响应包含的对象的过期时间  
    - ALL:no-store: 不允许缓存  
7. connection:  
    请求:
    - close: 告诉服务器或代理服务器，在完成本次请求的响应后断开连接  
    - keepalive: 告诉web服务器，保持连接，等待后续请求  
    响应:  
    - close: 连接已经关闭  
    - keepalive: 保持连接，等待后续请求  
    - Keep-Alive: 如果浏览器请求保持连接，则改头部表明希望WEB服务器保持连接多长时间，如`Keep-alive: 300`  
8. Content-Encoding: WEB服务器表明自己使用了什么压缩方法（gzip，deflate）压缩响应中的对象  
9. Content-Language: WEB 服务器告诉浏览器自己响应的对象的语言  
10. Content-Length: WEB 服务器告诉浏览器自己响应的对象的长度  
11. Content-Range: WEB 服务器表明该响应包含的部分对象为整个对象的哪个部分(例如：`Content-Range: bytes 21010-47021/47022`)  
12. Content-Type: WEB 服务器告诉浏览器自己响应的对象的类型(`Content-Type：application/xml`)  
13. ETag: 一个标志值，主要用于判断一个文件是否发生了改变  
14. Expired: WEB服务器表明该实体将在什么时候过期，对于过期了的对象，只有在跟WEB服务器验证了其有效性后，才能用来响应客户请求。是 HTTP/1.0 的头部(`Expires：Sat, 23 May 2009 10:02:12 GMT`)  
15. Host: 客户端指定自己想访问的WEB服务器的域名/IP 地址和端口号(`Host：rss.sina.com.cn`)  
16. If-Match: 如果对象的 ETag 没有改变，其实也就意味著对象没有改变，才执行请求的动作  
17. If-None-Match: 如果对象的 ETag 改变了，其实也就意味著对象也改变了，才执行请求的动作  
18. If-Modified-Since: 如果请求的对象在该头部指定的时间之后修改了，才执行请求的动作（比如返回对象），否则返回代码304，告诉浏览器 该对象没有修改(`If-Modified-Since：Thu, 10 Apr 2008 09:14:42 GMT`)  
19. If-Unmodified-Since: 如果请求的对象在该头部指定的时间之后没修改过，才执行请求的动作（比如返回对象）  
20. If-Range: 浏览器告诉 WEB 服务器，如果我请求的对象没有改变，就把我缺少的部分给我，如果对象改变了，就把整个对象给我。浏览器通过发送请求对象的 ETag 或者 自己所知道的最后修改时间给 WEB 服务器，让其判断对象是否改变了。总是跟 Range 头部一起使用  
21. Last-Modified: WEB 服务器认为对象的最后修改时间，比如文件的最后修改时间，动态页面的最后产生时间等等(`Last-Modified：Tue, 06 May 2008 02:42:43 GMT`)  
22. Location: WEB 服务器告诉浏览器，试图访问的对象已经被移到别的位置了，到该头部指定的位置去取(`Location:http://i0.sinaimg.cn/dy/deco/2008/0528/sinahome_0803_ws_005_text_0.gif`)  
23. Pramga: 主要使用 `Pramga: no-cache`，相当于 `Cache-Control: no-cache`  
24. Proxy-Authenticate: 代理服务器响应浏览器，要求其提供代理身份验证信息   
    Proxy-Authorization: 浏览器响应代理服务器的身份验证请求，提供自己的身份信息     
25. Range: 浏览器（比如 Flashget 多线程下载时）告诉 WEB 服务器自己想取对象的哪部分(`Range: bytes=1173546-`)  
26. Referer: 浏览器向 WEB 服务器表明自己是从哪个 网页/URL 获得/点击 当前请求中的网址/URL(`Referer：http://www.sina.com/`)
27. Server: WEB 服务器表明自己是什么软件及版本等信息(`Server: Apache/2.0.61 (Unix)`)    
28. User-Agent: 浏览器表明自己的身份（是哪种浏览器）(`User-Agent：Mozilla/5.0 (Windows; U; Windows NT 5.1; zh-CN; rv:1.8.1.14) Gecko/20080404 Firefox/2、0、0、14`)  
29. Transfer-Encoding: WEB 服务器表明自己对本响应消息体（不是消息体里面的对象）作了怎样的编码，比如是否分块（chunked）(`Transfer-Encoding: chunked`)  
30. Vary: WEB服务器用该头部的内容告诉 Cache 服务器，在什么条件下才能用本响应所返回的对象响应后续的请求。假如源WEB服务器在接到第一个请求消息时，其响应消息的头部为: `Content-Encoding: gzip; Vary: Content-Encoding`那么 Cache 服务器会分析后续请求消息的头部，检查其 `Accept-Encoding`，是否跟先前响应的 Vary 头部值一致，即是否使用相同的内容编码方法，这样就可以防止 Cache 服务器用自己 Cache 里面压缩后的实体响应给不具备解压能力的浏览器(`Vary：Accept-Encoding`)  
31. Via: 列出从客户端到 OCS 或者相反方向的响应经过了哪些代理服务器，他们用什么协议（和版本）发送的请求。当客户端请求到达第一个代理服务器时，该服务器会在自己发出的请求里面添 加 Via 头部，并填上自己的相关信息，当下一个代理服务器收到第一个代理服务器的请求时，会在自己发出的请求里面复制前一个代理服务器的请求的Via 头部，并把自己的相关信息加到后面，以此类推，当 OCS 收到最后一个代理服务器的请求时，检查 Via 头部，就知道该请求所经过的路由(`Via：1.0 236.D0707195.sina.com.cn:80 (squid/2.6.STABLE13)`)
