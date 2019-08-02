#### 让master节点参与调度
- 参与  
`kubectl taint node k8s-master node-role.kubernetes.io/master-`
- 取消  
`kubectl taint node k8s-master node-role.kubernetes.io/master=""`