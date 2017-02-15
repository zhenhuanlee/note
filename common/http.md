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
   | 方法    | 描述
--- | ------ | -----------------------------------------
1  | GET     | 请求指定的页面信息，并返回实体主体
2  | HEAD    | 类似于GET，只不过返回的响应中没有具体的内容，用于获取报头 
3  | POST    | 向指定的资源提交数据进行处理请求(表单，文件) 
4  | PUT     | 从客户端向服务器传送的数据取代指定的文档的内容 
5  | DELETE  | 请求服务器删除指定的页面 
6  | CONNECT | HTTP/1.1协议中预留给能够将连接改为管道方式的代理服务器
7  | OPTIONS | 允许客户端查看服务器的性能
8  | TRACE   | 回显服务器收到的请求，主要用于测试或诊断 

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
可以通过setHeader("Refresh", "5;URL=http://host/path")让浏览器读取指定的页面
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
1. application/x-www-from-urlencoded  
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
Connection:keep-alive
Content-Length:17
Content-Type:application/x-www-form-urlencoded
```
2. multiplepart/form-data  
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

3. text/html  
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

4. application/json  
使用json形式将数据发送给服务器  
可以方便的提交复杂的结构化数据，特别适合RESTful的接口
