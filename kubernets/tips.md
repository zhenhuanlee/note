#### 让master节点参与调度
- 参与  
`kubectl taint node k8s-master node-role.kubernetes.io/master-`
- 取消  
`kubectl taint node k8s-master node-role.kubernetes.io/master=""`

#### 修改service nodePort范围
1. `vim /etc/kubernetes/manifests/kube-apiserver.yaml`  
2. 添加 `- --service-node-port-range=1-65535`  
3. `kubectl apply -f  /etc/kubernetes/manifests/kube-apiserver.yaml`  


#### 问题
`failed to migrate ipam, retrying... error=failed to get add IPIP tunnel addr 192.168.0.1: The provided IP address is not in a configured pool`  
将`/var/lib/cni/`下面的文件删除，然后重新安装calico  

