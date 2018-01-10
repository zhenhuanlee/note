# Go环境变量  
1. GOROOT  
    就是go的安装路径  
2. GOPATH  
    - `go install`/`go get`和go的工具等会用到GOPATH环境变量  
    - GOPATH是作为编译后二进制的存放目的地和import包时的搜索路径，也就是工作目录，可以在src下创建你的go源文件，然后开始工作  
        - GOPATH下主要包含三个目录：bin  pkg  src  
        - bin目录主要存放可执行文件，pkg目录存放编译好的库文件，主要是.a文件，src目录存放go的源文件  

- 可以自己创建一个gopath目录  
- GOPATH可以是一个目录列表，`go get`下载的第三方库，一般会下载到列表的第一个目录里面  
- 需要把GOPATH中的可执行目录也配置到环境变量中，否则自行下载的第三方go工具就无法使用了  
    `export $PATH:$GOPATH/bin`   
- 
