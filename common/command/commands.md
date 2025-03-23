## 一些常用命令

### 代理并连接
ssh -i ~/.ssh/xxx.pem -L 8877:xxx.yyy.com:3306 -p 9876 ubuntu@sshhost

mysqlbinlog \
    --read-from-remote-server \
    --host=127.0.0.1 \
    --port=8877  \
    --user fiton-20200924 \
    --password \
    --raw \
    --verbose \
    --result-file=/Users/jude/Downloads \
    mysql-bin-changelog.505911
