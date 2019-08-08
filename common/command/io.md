#### [网络io](https://os.51cto.com/art/201404/435279.htm)  
1. nload  
2. iftop  

#### [硬盘io](https://www.cnblogs.com/mfryf/archive/2012/03/12/2392012.html)  
1. iostat  
- `yum install sysstat`  
- `iostat -x 1 10`  
- %util 越大，说明产生的IO请求太多，IO系统已经满负荷  
- idle 小于70%，IO压力的越大  

2. top的`wa` IO等待所占用的CPU时间的百分比，超过30%，压力大  
