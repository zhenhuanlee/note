# [理解OAuth2.0](http://www.ruanyifeng.com/blog/2014/05/oauth_2_0.html)
OAuth是一个关于授权(authorization)的开放网络标准，目前版本2.0
### 思路
OAuth在客户端与服务器提供商之间设置一个授权层(authorization layer)。客户端不能直接登录服务提供商，只能登录授权层  
1. 用户打开客户端后，客户端要求用户给予授权  
2. 用户同意给予客户端授权  
3. 客户端使用上一步获得的授权，向服务器申请令牌  
4. 认证服务器对客户端进行认证后，确认无误，同意发放令牌  
5. 客户端使用令牌，向服务器申请获取资源  
6. 资源服务器确认令牌无误，同意向客户端开放资源  

### 客户端授权模式  
客户端必须得到用户的授权(authorization grant)，才能获得令牌(access token)。OAuth2.0定义了四种授权方式  
* 授权码模式(authorization code)  
* 简化模式(implicit)  
* 密码模式(resource owner password credentials)  
* 客户端模式(client credentials)  

#### 授权码模式
功能最完整、流程最严密的授权模式。它的特点就是通过客户端的后台服务器与服务提供商的认证服务器进行互动  
1. 用户访问客户端，后者将前者导向认证服务器  
2. 用户给予授权，认证服务器将用户导向客户端事先指定的'重定向URI'，同时附上一个授权码  
3. 客户端收到授权码，附上早先的"重定向URI"，向认证服务器申请令牌。(这一步在客户端的后台的服务器上完成，用户不可见)  
4. 认证服务器核对授权码和重定向URI，确认无误后，向客户端发送访问令牌(access token)和更新令牌(refresh token)  
1步骤中，客户端申请认证的URI包含以下参数：  
* response_type: 表示授权类型，必须，此处的值固定为'code'  
* client_id: 表示客户端的ID，必须项  
* redirect_uri: 表示重定向URI，可选项  
* scope: 表示申请的权限范围，可选项  
* state: 表示客户端的当前状态，可以指定任意值，认证服务器会返回这个值  
```
GET /authorize?response_type=code&client_id=s6BhdRkqt3&state=xyz
        &redirect_uri=https%3A%2F%2Fclient%2Eexample%2Ecom%2Fcb HTTP/1.1
Host: server.example.com
```

2步骤中，服务器响应客户端的URI，包含以下参数：
* code: 表示授权码，必须，该码的有效期应该很短，通常为10分钟，客户端只能使用该码一次，否则会被授权服务器拒绝。该码与客户端ID和重定向URI是一一对应关系  
* state: 如果客户端的请求中包含这个参数，认证服务器的回应也必须一模一样包含这个参数
```
HTTP/1.1 302 Found
Location: https://client.example.com/cb?code=SplxlOBeZQQYbYS6WxSbIA
          &state=xyz
```

3步骤中，客户端向认证服务器申请令牌的HTTP请求，包含以下参数:  
1. grant_type: 表示使用的授权模式，必须，此处的值固定为'authorization_code'  
2. code: 表示上一步获得的授权码，必选  
3. redirect_uri: 表示重定向URI，必选，且与1步骤中的该参数值保持一致  
4. client_id: 客户端ID，必须  
```
POST /token HTTP/1.1
Host: server.example.com
Authorization: Basic czZCaGRSa3F0MzpnWDFmQmF0M2JW
Content-Type: application/x-www-form-urlencoded

grant_type=authorization_code&code=SplxlOBeZQQYbYS6WxSbIA
&redirect_uri=https%3A%2F%2Fclient%2Eexample%2Ecom%2Fcb
```

4步骤中，认证服务器发送的HTTP回复，包含以下参数:  
* access_token: 表示访问令牌，必选  
* token_type: 表示令牌类型，该值大小写不敏感，必选，可以是bearer类型或mac类型  
* expires_in: 表示过期时间，单位为秒，如果省略该参数，必须其他方式设置过期时间  
* refresh_token: 表示更新令牌，用来获取下一次的访问令牌，可选  
* scope: 表示权限范围，如果与客户申请的范围一致，可省略  
```
HTTP/1.1 200 OK
Content-Type: application/json;charset=UTF-8
Cache-Control: no-store
Pragma: no-cache

{
  "access_token":"2YotnFZFEjr1zCsicMWpAA",
  "token_type":"example",
  "expires_in":3600,
  "refresh_token":"tGzv3JOkF0XG5Qx2TlKWIA",
  "example_parameter":"example_value"
}
```
相关参数使用JSON格式发送(Content-Type: application/json)。此外，HTTP头信息中明确指定不得缓存  

#### 简化模式  
不通过第三方应用程序的服务器，直接在浏览器中向认证服务器申请令牌，跳过了"授权码"这个步骤。所有步骤在浏览器中完成，令牌对访问者是可见的，且客户端不需要认证  
1. 客户端将用户导向认证服务器  
2. 用户决定是否给予客户端授权  
3. 认证服务器将用户导向客户端指定的重定向URI，并在URI的Hash部分包含了访问令牌  
4. 浏览器向资源服务器发出请求，其中不包含上一步收到的Hash值  
5. 资源服务器返回一个网页，其中包含的代码可以获取Hash值中的令牌  
6. 浏览器执行上一步获得的脚本，提取出令牌  
7. 浏览器将令牌发给客户端  
