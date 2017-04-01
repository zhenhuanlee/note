# 常用的SQL语句
## ROW NUMBEr() OVER ()
将表排序，并赋予一个序号  
```sql
SELECT row_number() OVER (PARTITION BY device_category_id ORDER BY id DESC), id, device_category_id
FROM remoters
-- 按device_category_id分组，并按id排序
```
row_number | id | device_category_id
---------- | -- | -----------------
2298       | 15 |   2
2299       | 13 |   2
2300       | 4  |   2
1          |8797|   4
2          |8796|   4
> 部分结果

## CASE WHEN THEN ELSE END
```sql
-- 简单case语句
CASE gender
WHEN '1' THEN 'male'
WHEN '2' THEN 'female'
ELSE 'other' END
-- case搜索函数，可以写判断了
CASE WHEN SEX='1' THEN 'male'
     WHEN SEX='2' THEN 'female'
     ELSE 'other' END
```
> 如果正确，后面的就不会执行  

1. 习题1  
country | population
------- | ----------
中国     | 600
美国     | 100
加拿大   | 100
英国     | 200
法国     | 300
日本     | 250
德国     | 200
墨西哥   | 50
印度     | 250

得到

continent | population
--------- | ----------
亚洲       | 1100
北美洲     | 250
其他       | 700

```sql
SELECT sum(population),
(CASE country
WHEN '中国'   THEN '亚洲'
WHEN '美国'   THEN '北美'
WHEN '加拿大' THEN '其他'
WHEN '英国'   THEN '北美'
ELSE '其他' END)
FROM countries
GROUP BY (CASE country
WHEN '中国'   THEN '亚洲'
WHEN '美国'   THEN '北美'
WHEN '加拿大' THEN '其他'
WHEN '英国'   THEN '北美'
ELSE '其他' END)
```

2. 习题2  
国家 | 性别 | 人口
--- | ---- | ---
中国 |  1  | 340
中国 |  2  | 260
美国 |  1  | 45
美国 |  2  | 55

国家 | 男   |  女
--- | ---- | ---
中国 |  340 | 260
美国 |  45  | 55

```sql
-- 方案1
SELECT country,
SUM(CASE sex WHEN 1 THEN population ELSE 0 END) man,
SUM(CASE sex WHEN 2 THEN population ELSE 0 END) woman
FROM countries
GROUP BY country

-- 方案2
SELECT res.country, SUM(男), SUM(女) FROM
(SELECT country, population 男, 0 女 FROM countries WHERE sex=1
 UNION ALL
 SELECT country, 0 男, population 女 FROM countries WHERE sex=2) res
GROUP BY res.country

-- 方案3
SELECT country,
SUM(population) FILTER (WHERE sex=1) AS man,
SUM(population) FILTER (WHERE sex=2) AS female
FROM countries
GROUP BY country
```

3. 在`CHECK`中使用`CASE`函数
```sql
CREATE TABLE users (
    name varchar(10),
    sex varchar(2),
    age INTEGER
    CHECK(age > 10 AND sex='1')
)

-- sex为2的需要工资大于1000
CONSTRAINT check_salary CHECK(
  CASE WHEN sex='2' THEN CASE
  WHEN salary > 1000 THEN 1 ELSE 0 END
  ELSE 1 END = 1
)
```

4. 有条件有选择的`UPDATE`
```sql
-- 大于5000，-10%，否则，+15%
UPDATE personnel SET salary = (
  CASE WHEN salary > 5000 THEN salary * 0.9
       WHEN salary > 2000 AND salary < 4500 THEN salary * 1.1
       ELSE salary END
)
```
> 不能分两个update语句执行，因为可能一个数据操作完了会落在另一个范围内，导致多次执行

5. 两个表数据是否一致的检查  
Case函数不同于Decode函数，在Case函数中，可以使用`BETWEEN`,`LIKE`,`IS NULL`,`IN`,`EXISTS`等  
```sql
-- 比较table_a和table_b两个表，都有keyCol字段，如果tab_a中的keyCol数据可以在tab_b中找到，则返回'matched'，否则返回'unmatched'
SELECT keyCol,
CASE WHEN keyCol IN (SELECT keyCol FROM tab_b)
THEN 'Matched'
ELSE 'Unmatched' END label
FROM tab_b

-- or

SELECT keyCol,
CASE WHEN EXISTS(SELECT * FROM tab_b WHERE tab_a.keyCol=tab_b.keyCol)
THEN 'Matched'
ELSE 'Unmatched' END label
FROM tab_a
```

6. 在case中使用合计函数  
学号(std_id) | 课程(class_id) | 课程名(class_name) | (main_class_flg)
-----------  |  ------------ | ----------------- | ---------------
100          |      1        |   经济学           | Y
100          |      2        |   历史学           | N
200          |      2        |   历史学           | N
200          |      3        |   考古学           | Y
200          |      4        |   计算机学          | N
300          |      4        |   计算机学          | N
400          |      5        |   化学             |  N
500          |      6        |   数学             | N
查询出：  
- 只选修一门的，返回那门课程id  
- 选修多门课程的人，返回所选的主课程id  

```sql
SELECT std_id,
CASE WHEN COUNT(1)=1 THEN MAX(class_id)
ELSE MAX(CASE WHEN main_class_flg='Y' THEN class_id END) END
FROM courses c
GROUP BY std_id
ORDER BY std_id
```

```sql
CASE col_1
WHEN 1    　   THEN 'Right'
WHEN NULL  THEN 'Wrong'
END
-- WHEN NULL 这一样只会返回unkonown，正确的用法是 WHEN col_1 IS NULL
```
