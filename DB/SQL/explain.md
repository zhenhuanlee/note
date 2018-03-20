# Explain
explain 显示了MySQL如何使用索引来处理select语句，以及连接表  
#### explain能干嘛？
1. 表的读取顺序  
2. 数据读取操作的操作类型  
3. 哪些索引可以使用？  
4. 哪些索引被实际使用  
5. 表之间的引用  
6. 每张表有多少行被优化器查询  
mysql> explain select * from members as m left join two_factors as tf on tf.member_id = m.id;  
id | select_type | table | partitions | type | possible_keys | key | key_len | ref | rows | filtered | Extra |
-----|----|-----|----|---|------|-----|----|-----|---|--------|-------|
1 | SIMPLE | m | NULL | ALL | NULL | NULL | NULL | NULL | 8 | 100.00 | NULL |
1 | SIMPLE | tf | NULL | ALL | NULL | NULL | NULL | NULL | 16 | 100.00 | Using where; Using join buffer (Block Nested Loop) |  
2 rows in set, 1 warning (0.00 sec)  

- id决定了表的读取顺序  
执行select语句查询的序列号，包含一组数字，表示查询中执行select子句或操作表的顺序  
分三种情况：  
1. id相同，执行顺序由上至下  
2. id不同，如果是子查询，id的序号会递增，id越大优先级越高  
3. id相同和不同，同时存在，如果id相同，可以认为是一组，从上往下顺序执行，在所有组中，id值越大，优先级越高  

- select_type 查询的类型，也就是数据读取操作的操作类型，他一共有以下5种：  
1. simple：简单的select查询，查询中不包含子查询或者union  
2. primary: 查询中若包含任何复杂的子查询，最外层查询则被标记  
3. subquery：在select或者where列表中包含子查询  
4. derived：from列表中包含的子查询被标记为DERIVED（衍生表），mysql会递归执行这些子查询，把结果放到临时表中；  
union：若第二个select出现在union之后，则被标记为union，若union包含在from子句的子查询中，外层select将被标记为DERIVED；  
union result：从union表(即union合并的结果集)中获取select查询的结果  
5. type：访问类型排列显示查询使用了何种类型，从最好到最差依次是:system > const > eq_ref > ref > range > index > all  
- system：表只有一行记录(等于系统表)，这是const类型的特例，平时不会出现，可以忽略  
- const：表示通过索引一次就找到了，const用于比较primary key或者unique索引。因为只匹配一行记录，所以很快。如果将主键置于where列表中，mysql就能将该查询转换成一个常量  
- eq_ref：唯一性索引，对于每一个索引键，表中只有一条记录与之匹配，常用于主键或者唯一索引扫描  
- ref：非唯一性索引扫描，参会匹配某个单独值所得的所有行，本质上也是一种索引访问，它返回所有匹配某个单独值的行，然而，它可能会找到多个符合条件的行，所以它应该属于查询和扫描的混合体  
- range：只检索给定范围的行，使用一个索引来选择行，key列显示使用哪个索引，一般就是在你的where语句中出现了between,<,>,in等查询；这种范围索引扫描比全表扫描要好，因为它只需要开始于索引的某一个点，不用扫描全部索引  
- index：index于all区别为index类型只遍历缩印书，这通常比all快，因为索引文件通常比数据文件小，也就是说，虽然all和index都是读写表，但是index是从索引中读取的，而all是从硬盘中读取的  
- all：也就是全表扫描  
> 查询至少达到range级别，最好ref  


