# MySQL8 新特性
## 索引增强
1. 隐藏索引 (invisible index)
```sql
select @@optimizer_switch\G;   --查看 各种参数
set session optimizer_switch="use_invisible_indexes=on';   --在会话级别设置查询优化器可以看到隐藏索引
```
2. 降序索引，只有 InnoDB 存储索引支持降序索引，只支持 BTREE 降序索引
3. GROUP BY 不再进行隐式排序
4. 函数索引，通过虚拟列功能实现
```sql
create table t3(c1 varchar(10),c2 varchar(10));
create index idx_c1 on t3(c1);   --普通索引
create index func_idx on t3( (UPPER(c2)) );   --一个大写的函数索引
```
5. 支持 JSON 数据的索引

## 通用表达式 (common table expression)，即 with 子句
```sql
WITH recursive cte(n) as
( select 1
  union all
  select n+1 from cte where n<10
)
select * from cte;
```
## 窗口函数
- SUM, AVG, COUNT, MAX, MIN

-  专用窗口函数, --有空研究--
    - A: ROW_NUMBER, RANK, DENSE_RANK, PERCENT_RANK
    - B: FIRST_VALUE, LAST_VALUE, LEAD, LAG
    - C: CUM_DIST, NTH_VALUE, NTILE

```sql
-- rows 定义取值范围，默认是整个窗口, 即 rows between unbounded preceding and unbounded following
SELECT year, country, product, profit,
  first_value(profit) over wwww as 'first',
  last_value(profit) over wwww as 'last',
	sum(profile) over (partition by country order by profile rows UNBOUNDED preceding) running_total, -- 当前组中递增的和, 应该类似于 rows between unbounded preceding and current row
	avg(profile) OVER (PARTITION BY country ORDER BY profile rows BETWEEN 1 preceding and 1 FOLLOWING) running_avg -- 当前组中前一条，当前条和后一条的平均值
FROM sales
WINDOW wwww as (partition by country order by profit rows unbounded preceding) -- 上面的 first_value 和 last_value 提取公因数
order by profit
```
- UNBOUNDED PRECEDING -> 每个分区的最前沿，也就是第一行，实现了上面的分区累加的效果
- M preceding -> 之前已经处理过的的 M 行x
- CURRENT ROW -> 当前行
- N following -> 之后的 N 行
- UNBOUNDED FOLLOWING -> 当前分区的最后一行

## 锁语句选项
select ... for share / update 支持 NOWAIT & SKIP LOCKED 选项
- NOWAIT 如果请求的行被其他事务锁定，语句立刻返回
- SKIP LOCKED 从返回的结果集中移除被锁定的行

## 其他的一些优化
#### 快速 DDL
alter table ... ALGORITHM=INSTANT
增加字段的时候只修改数据字典上的信息，所以非常的快

#### 服务器的优化
新增静态变量 `innodb_dedicated_server`, 自动配置 InnoDB 内存参数 innodb_buffer_pool_size / innodb_log_file_size 等  
> 如果是数据库专用服务器的话，应该打开这个，会尽量多的占用资源

## JSON 增强
- 内联路径操作符: ->>  
  column ->> path 等价于
  JSON_UNQUOTE(column -> path) 等价于
  JSON_UNQUOTE(JSON_EXTRACT(column, path))
- JSON 聚合函数
- JSON 实用函数
- JSON 合并函数
- JSON 表函数 -> 将 JSON 扩展成表

