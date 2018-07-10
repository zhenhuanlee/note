## 命令速查
1. 与容器交互  `docker run -it ubuntu /bin/bash`
  > - i 允许你对容器内的标准输入(STDIN)进行交互  
  > - t 在新容器中指定一个伪终端或终端  

2. 容器是否在运行 `docker ps`  

3. 容器后台运行 `docker run -d ubuntu /bin/sh -c "while true; do echo hello world; sleep 1; done"`
  > - 这是一个后台执行的，每一秒打印一个hello world的脚本  
  > - 要看打印，使用下面的命令  
  > - d 让程序在后台运行  

4. `docker logs -f <docker>`  
  > - f 相当于tail -f  

5. `docker run -d -p 5000:5000 training/webapp python app.py`  
  > - 运行一个web app  
  > - p 端口映射  

6. 查看指定容器端口映射 `docker port <docker>`

7. 查看应用进程 `docker top <docker>`

8. 查看Docker的底层信息。它会返回一个JSON `docker inspect <docker>`  

9. 获取一个新的image `docker pull <docker>`

10. commit镜像 `docker commit -m="has updated" -a="runoob" e218edb10161 runoob/ubuntu:v2`
  > - m: 提交的描述信息  
  > - a: 指定镜像的作者  
  > - e218edb10161: docker id  
  > - runoob/ubuntu:v2 指定要创建的目标镜像名  

11. build镜像(需要Dockerfile) `docker build -t runoob/centos:6.7`  
  > - t: 指定要创建的目标镜像名  
  > - .: Dockerile所在的目录  

12. 设置镜像标签 `docker tag 860c279d2fec runoop/centos:dev`  

13. 挂载 `docker run -p 3000:3000 --name mycontainer -v $PWD/conf/my.cnf:/etc/mysql/my.cnf -v $PWD/logs:/logs -v $PWD/data:mysql_data -e MYSQL_ROOT_PASSWORD=123456 -d mysql:5.6`
  > - -v $PWD/conf/my.cnf:/etc/mysql/my.cnf 将主机当面目录下的conf/my.cnf挂载到容器的/etc/mysql/my.cnf  

14. 如果一个连接正在操作，可以用`docker attach <container>`，那么两个连接就同步了  

15. 如果要像SSH一样独立分开的话，就用`docker exec <container> /bin/bash`  

16. 拷贝文件 `docker copy <file> <container>://usr/share/nginx/html`  

17. 查看container所有的属性 `docker inspect <container>`

18. mac上进入spine `screen ~/Library/Containers/com.docker.docker/Data/com.docker.driver.amd64-linux/tty`  

19. 创建一个volume `docker create -v $PWD/data:/var/mydata --name data_container ubuntu`
  > 这是一个仅有数据的容器

20. 加载volume的容器 `docker run -it --volumes-from data_container ubuntu /bin/bash`
  > data_container 是上面那个创建的容器的名字  
  > 在linux命令行运行mount可以查看   

21. push一个镜像
```
  docker tag my_image $DOCKER_ID_USER/my_image
  docker push $DOCKER_ID_USER/my_image
```

#### docker-compose

1. 运行docker-compose `docker-compose up`  

2. 停止所有compose的container `docker-compose stop`  

3. 移除所有compose的containers `docker-compose rm`  

4. 修改了配置后build就可以 `docker-compose build`  
  > 本地创建镜像  

5. `docker-compose ps`  

6. `docker-compose logs`
