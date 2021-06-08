## 什么是 lambda  
- 只需要关心代码，不需要管理服务器  
- 事件驱动  
- 自动扩展  
- 按量计费  
- AWS 各服务之间的胶水层  
- 一个函数最长时间 5 分钟  

## Lambda 编程模型，并发问题，版本控制  
#### 编程模型  
```js
exports.awsHandler = function(event, context, callback) {}

// awsHandler: 假设此代码保存并 helloworld.js，那么 helloworld.awsHandler 是处理函数
// event: 将事件数据传递到处理程序   
// context: 提供运行时信息，与 Lambda 执行环境作为交互，服务将此对象作为第二个参数传递给 Lambda 函数处理程序  
// callback: 回调函数是可选的，取决于是否希望将信息返回到调用方   
```

- context 有哪些内容  
```js
context.getRemainingTimeInMillis()  //
context.functionName
context.awsRequestId
context.logGroupName
context.logStreamName
context.clientContext
```

#### 并发  
根据前端并发数自动扩展，直到达到设置上限  

#### 版本控制和别名  
- 版本号单调递增  
- 多环境可用 alias 指定版本  

#### CloudFormation

#### API Gateway
将收到的 HTTP 请求转化成 Lambda 的调用  
- 无服务器需要管理，系统会根据流量自动伸缩  
- 集成安全认证管理功能  
- 自带 DDOS 保护、请求限流、保证后端安全  
- 支持 Rest 和 Websocket  
- 一键生成 Swagger 和 Client SDK  
- Stage 方式部署，非常容易部署或回滚  
- 支持安全策略，缓存，监控，访问日志  
- 自定义域名  

#### Lambda
- 事件驱动的无服务器的微服务  
- 对 Lambda 进行编排和调度：Step Function  
  1. 支持同步/异步执行  
  2. 并行与串行  
  3. 自带异常处理机制  
  4. 自动扩展  
  5. 状态保存  
  6. 长时间运行与等待  
  7. 审计与监控  

#### Cognito



## AWS 无服务应用模型 SAM(Serveless Application Model)
- 使用 SAM 文件描述无服务器应用程序，包括 Lambda 函数，API Gateway 和其他 AWS资源  
- 使用 CloudFormation 部署  

## Serveless 开发框架  
- SAM  
- Serveless  
- Claudia.js  
- Chalice  
- Sparta  
- apex  
- zappa  

## 监控 - Amazon CloudWatch  

## 计费  
512MB 内存，一个月执行 300 万次，每次运行 1 秒  
- 月度计算费用：  
price = 0.0000113477 rmb/GB
time = 3M * 1s = 3000000 s
sum = time * 512MB / 1024 = 1500000 GBs
res = sum * price = 170.2155  

- 月度请求费用  
每 100 万个请求 1.36 rmb  
res = 3M * 1.36 rmb/M = 4.08  

- 月度总费用  
170.2155 + 4.08 = 174.29  

