# [例子](https://zhuanlan.zhihu.com/p/68476641)  
```yaml
# --- 是可选的分隔符
---
# 这个需要根据安装的k8s版本和资源类型进行变化
apiVersion: extensions/v1beta1
# 创建的类型，可以是Deployment, Job, Ingress, Service等
kind: Deployment
# 包含一些meta信息，比如name, namespace，label等信息
metadata:
  # 给pod打上标签, 定义service 需要用
  labels:
    run: first-web-size
  name: first-web-size
# 容器的定义
spec:
  # 实例个数
  replicas: 2
  selector:
    matchLabels:
      run: first-web-size
  strategy:
    rollingUpdate:
      maxSurge: 1
      maxUnavailable: 1
    type: RollingUpdate
  template:
    metadata:
      creationTimestamp: null
      labels:
        run: first-web-size
    spec:
      containers:
      # 微软 dotnet core 样例镜像
      - image: mcr.microsoft.com/dotnet/core/samples:aspnetapp
        imagePullPolicy: IfNotPresent
        name: first-web-size
        terminationMessagePath: /dev/termination-log
        livenessProbe:
          httpGet:
            path: /v1/health
            port: 80
          initialDelaySeconds: 120
          periodSeconds: 30
        # CPU + 内存限制
        resources:
          # 最高使用500m的CPU, 超过就限制CPU;最高用512M内存, 超过就炸应用
          limits:
            cpu: 500m
            memory: 512Mi
          # 要求给50m的CPU调度和128M内存才可以启动
          requests:
            cpu: 50m
            memory: 128Mi
    # 这里是私有镜像仓库需要自己配置的一下秘钥什么之类的, 配置过程见k8s官方文档
    #   imagePullSecrets:
    #     - name: regcred
      dnsPolicy: ClusterFirst
      restartPolicy: Always
      terminationGracePeriodSeconds: 30
---
apiVersion: v1
kind: Service
metadata:
  name: first-web-size
  annotations:
    # 这里:后面的值填腾讯云的子网id, 这样service就在腾讯云私有网络子网里面有自己的 ip了
    service.kubernetes.io/qcloud-loadbalancer-internal-subnetid: subnet-xxx
spec:
  selector:
    # 过滤标签, 在上面定义pod的时候也写了对应的label
    run: first-web-size
  ports:
  - protocol: TCP
    # service对外提供访问的端口
    port: 80
    # pod的端口
    targetPort: 80
  type: LoadBalancer
  externalTrafficPolicy: Cluster
```