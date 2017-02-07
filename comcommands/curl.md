# Command Line Url viewer
```shell
# 查看网页源码
$ curl www.baidu.com

# 可以使用 `-o`参数将网页保存，这样就相当于`wget`
$ curl -o [filename] www.baidu.com

# 有些网址是自动跳转的，使用`-L`参数
$ curl -L www.baidu.com

# 显示头信息 `-i`
$ curl -i www.baidu.com
# '-I' 是只显示头信息
$ curl -I www.baidu.com

# 显示通信过程 `-v` 
# 包括端口连接，http request头信息
$ curl -v www.baidu.com 

# 比上面更详细的通信过程
$ curl --trace output.txt www.baidu.com
# or
$ curl --trace-ascii output.txt www.baidu.com

# 发送表单信息
# GET 把数据附在网址后面
$ curl www.baodu.com/xxx?data=xxx
# POST 使用`--data`参数
$ curl -X POST --data "data=xxx" www.baidu.com/xxx
# 表单编码，`--data-urlencode`
$ curl -X POST--data-urlencode "data=April 1" www.baidu.com/xxx

# HTTP动词
# 默认 GET，使用 `-X`
$ curl -X DELETE www.baidu.com

# 文件上传
# <form method="POST" enctype='multipart/form-data' action="upload.cgi">
#  <input type=file name=upload>
#  <input type=submit name=press value="OK">
# </form>
# 假定文件上传的表单是这样的
# 使用curl 上传文件
$ curl --form upload=@localfilename --form press=OK [URL]

# Referer 字段
# 在http request 头信息中提供一个referer 字段，表明从哪里跳转过来
$ curl --referer http://www.example.com http://www.example.com

# User Agent字段
# 显示客户端的设备信息
$ curl --user-agent "[User Agent]" [URL]

# cookie
$ curl --cookie "name=xxx" www.baidu.com
# 具体的 cookie 值，可以从http response头信息的`Set-Cookie`字段中得到
# `-c cookie-file`可以保存服务器返回的cookie到文件
$ curl -c cookies http://example.com
# `-b cookie-file`可以使用这个文件作为cookie信息
$ curl -b cookies http://example.com

# 增加头信息
# 在http request之中，自行增加一个头信息，`--header`参数
$ curl --header "Content-Type:application/json" http://example.com

# HTTP认证 `--user`
$ curl --user name:password example.com
```

