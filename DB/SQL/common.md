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
> 如果第一个正确，第二个就不会执行  

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
 UNION
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
