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
