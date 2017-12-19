# Docker Machine
在远程机器上安装Docker，并像本地一样运行  

### 前提条件  
1. 在目标主机上创建一个用户并加入sudo组  
```shell
sudo adduser nick
sudo usermod -a -G sudo nick
```
2. 设置sudo操作不需要密码  
```shell
sudo visudo
# 加入 nick   ALL=(ALL:ALL) NOPASSWD: ALL
```
3. 在目标机器上加入public key  

### 安装  
```shell
docker-machine create -d generic \
  --generic-ip-address=xxx.xxx.xxx.xxx \
  --generic-ssh-user=nick \
  --generic-ssh-key ~/.ssh/id_rsa \
  name
``` 
> create 本是要创建虚拟主机，并安装docker，因为目标主机已存在，所以直接安装docker  

### 切换环境  
```shell
eval "$(docker-machine env name)"
eval "$(docker-machine env -u)"
```
> Docker客户端默认是访问本机的Docker daemon，但是当指定了DOCKER_HOST变量后，Docker就会访问这个变量中指定的docker daemon  
