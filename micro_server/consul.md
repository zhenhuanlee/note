#### 简单网络
1. 第1个server  
```s
consul agent --data-dir=/data --bootstrap-expect=3 -ui --server=true --client=0.0.0.0
```

2. 第2个server  
```s
consul agent --server=true --client=0.0.0.0 --join 172.17.0.2 --data-dir=/data
```

3. 第3个server  
```s
consul agent --server=true --join 172.17.0.2 --data-dir=/data
```

4. 第4个client  
```s
consul agent --server=false --join=172.17.0.3 --data-dir=/data --config-dir=/consul/config/services.json
```

5. 第5个client  
```s
consul agent --server=false --join=172.17.0.4 --data-dir=/data --config-dir=/consul/services.json
```

>> 其中services.json  
>>
```json
{
  "services": [
    {
      "id": "hello1",
      "name": "hello",
      "tags": [
        "primary"
      ],
      "address": "172.17.0.5",
      "port": 5000,
      "checks": [
        {
        "http": "http://localhost:5000/",
        "tls_skip_verify": false,
        "method": "Get",
        "interval": "10s",
        "timeout": "1s"
        }
      ]
    }
  ]
}
```


#### 请求节点服务信息  
```s
curl http://127.0.0.1:8500/v1/health/service/hello?passing=false
# curl http://hello.service.dc1.consul/v1/health/service/hello?passing=false 
// passing=false 是过滤不健康的节点
```
