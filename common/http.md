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
