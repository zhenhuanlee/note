# [MySQL的`for update`](https://blog.csdn.net/u011957758/article/details/75212222)  
### 使用场景
- 高并发  
- 数据的准确性有要求  
原则：一锁二判三更新  

### 如何使用  
`select * from tablename where xxx for update`  

### for update的锁表  
- innoDB 默认的是行级别的锁，当有明确的主键时，是行级锁，否则是表级锁  

> 设表foods，存在id, name, status三个字段，id是主键，status有索引  
1. 明确指定主键/索引，并且有此记录，行级锁  
`SELECT * FROM foods WHERE id=1 FOR UPDATE;`  
`SELECT * FROM foods WHERE id=1 AND name="xxx" FOR UPDATE;`  

2. 明确指定主键/索引，若查无此记录，无锁  
`SELECT * FROM foods WHERE id=-1 FOR UPDATE;`  

3. 无主键索引，表级锁  
`SELECT * FROM foods WHERE name="xxx" FOR UPDATE;`  

4. 主键索引不明确，表级锁  
`SELECT * FROM foods WHERE id <> '3' FOR UPDATE`  
`SELECT * FROM foods WHERE id LIKE '3' FOR UPDATE`  

### 注意点  
- 仅适用于innoDB，并且不许开启事务，在begin与commit之间生效  
- 要测试for update的锁表情况，可以利用mysql的Command Mode开启第二个视窗来做测试  

### QA  
- 当开启一个事务进行for upate时，另一个事务也有for update的时候回一直等着，知道第一个事务结束么？  
  会的，除非第一个事务commit或者rollback或者断开连接，第二个事务回立马拿到锁进行和面操作  
- 如果没有查到记录会锁表么  
  会的。表级所时，不管是否查询到记录，都会锁定表  

