#### Dockerfile定制镜像
创建`Dockerfile`，并输入  
```shell
FROM nginx
RUN echo '<h1>Hello, Docker!</h1>' > /usr/share/nginx/html/index.html
```
##### FROM 指定基础镜像
- 一般都是以`FROM`指定一个镜像为开头，自定义自己的镜像  
- 还可以以`FROM scratch`开头，这是个虚拟镜像，意味着不以任何虚拟镜像为基础，接下来缩写的指令将作为镜像第一层开始存在  
  不以任何系统为基础，直接将可执行文件复制进镜像的做法并不罕见，比如`swarm`...对于Linux下静态编译的程序来说，并不需要操作系统提供运行时支持，所需的一切库都已经在可执行文件里了，因此直接`FROM scratch`会让镜像体积更加小巧。

##### RUN执行命令  
RUN指令有两种格式  
1. shell格式：`RUN echo '13' > index.html`  
2. exec格式 ： `RUN ["可执行文件", "参数1", "参数2"]`  
但是不要每一个命令都一个RUN
```docker
FROM debian:jessie

RUN apt-get update
RUN apt-get install -y gcc libc6-dev make
...
```
因为Dockerfile中每一个指定都会建立一层，RUN也不例外。新建一层，执行结束后，commit这一层的修改，构成新的镜像  
这样，上面的这种写法就构成了2层镜像。这完全没有意义  
正确的写法：
```docker
FROM debian:jessie

RUN buildDeps='gcc libc6-dev make' \
    && apt-get update \
    && apt-get install -y $buildDeps
    ...
```

#### 构建镜像
在Docfile文件所在的目录执行
```shell
docker build -t nginx:v3 .
```
> . 表示当前目录，这个指的是上下文路径，详情见下  

#### 镜像构建上文
Docker在运行时分为Docker引擎(服务器守护进程)和客户端工具，Docker引擎提供一组REST API，被称为`Docker Remote API`，而如`docker`命令这样的客户端工具，则是通过这组API与Docker引擎交互，从而完成各种功能。因此，虽然表面上我们好像在本机执行各种`docker`命令，但实际上，一切都是使用的远程调用形式在服务端(Docker引擎)完成，也因为这种C/S设计，让我们操作远程服务的Docker引擎变的轻而易举  
`docker build`是在服务端(docker 引擎)完成的，当构建时，指定了上下文，docker build会将路径下所有的内容打包，然后传给Docker引擎  
如果在Dockerfile中这么写  
```
COPY ./package.json /app/
```
这并不是要复制执行`docker build`命令所在的目录下的`package.json`，也不是复制`Dockerfile`所在目录下的`package.json`，而是复制上下文目录下的`package.json`  

一般来说，会将`Dockerfile`置于一个空目录下，或者项目根目录下，如果该目录下没有所需文件，那么应该把所需文件复制一份过来  

#### Dockerfile详解  
`COPY`将从构建上下问目录中<source>的文件/目录复制到新的一层的镜像内的<target>位置  
```
COPY <source>... <target>
COPY ["<source1>",... "<target>"]
```
