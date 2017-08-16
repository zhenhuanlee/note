# Docker
一个Go开发的，基于Linux内核的cgroup, namespace, AUFS类的Union FS等技术  
对进程进行封装隔离，属于操作系统层面的虚拟化技术  

#### 优势
- 更高效的利用资源   
  不需要进行硬件虚拟以及运行完整操作系统  
- 更快的启动时间  
- 一致的运行环境  
- 持续交付和部署  
  Dockerfile管理更透明，可以持续集成，持续部署  

#### 基本概念
- 镜像  
- 容器  
- 仓库  

###### 利用commit理解镜像的构成  
镜像(image)是容器(container)的基础，每次执行`docker run`的时候都会制定哪个镜像作为容器运行的基础  

> 不要使用docker commit（有很多冗余的记录），使用Dockerfile定制镜像  

#### 为什么要安装virtualbox?
docker讲到底还是一个运行在操作系统上的一个程序，它需要内核特性支持，目前来看，Docker只能跑在Linux上，如果需要在MAC或者Win上跑，需要安装一个virtualbox这样的虚拟机工具  

#### docker-machine是干嘛的
为了run anywhere，出现了boot2docker这个工具，包含两部分：
  - 一个轻量级的基于tiny core linux(一种Linux发行版)的iso镜像  
  - 管理和设置vm的boot2docker工具  
然而boot2docker只支持virtualbox，很多像AWS，Microsoft Azure，VMware等云环境也想方便的跑docker  



### [Docker Machine, Compose, and Swarm: How They Work Together](https://blog.codeship.com/docker-machine-compose-and-swarm-how-they-work-together/)
simple to use tools to set up container hosts(Machine), manage multiple containers linked together(Compose), and treating your container hosts as a cluster(Swarm).  

#### Docker Machine
Machine可以帮助在许多流行服务器平台的上创建容器

#### Docker Compose
Compose 有一个简单的方法去描述一个涵盖多个containers的应用，他们有什么联系，应该开放哪些接口  
```yaml
web:
  build: .
  ports:
    - "5000:5000"
  volumes:
    - .:/code
  links:
    - redis
redis:
  image: redis
```
这个应用由两个containers组成，第一个是"web"，是由一个Docfile所构建  
第二行显示端口映射  
第三部分显示会映射一个volume  
最后会连接到另一个redis的container，这个container会使用标准的Redis image
然后运行`docker-compose up`
