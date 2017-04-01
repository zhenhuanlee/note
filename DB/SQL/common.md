# 一些概念

## sql执行顺序
第一个被处理的是`FROM`子句  
`SELECT`最后被处理

每个步骤都会产生一个虚拟表，这个虚拟表会被作用到下一个步骤的输入。这些虚拟表对调用者不可用，最后一步生成的表才会返回给调用者.
如果没有在查询中指定某一个子句，将跳过相应的步骤。  
```sql
(8)SELECT (9)DISTINCE (11)<TOP Num> <SELECT list>
(1)FROM [left_table]
(3)<join_type> JOIN <right_table>
(2)ON <join_condition>
(4)WHERE <where_condition>
(5)GROUP BY <group_by_list>
(6)WITH <CUBE | RollUP>
(7)HAVING <having_condition>
(10)ORDER BY <order_by_list>
```
1. FROM: 对FROM子句中的前两个表执行笛卡儿积(Cartesian product)(交叉连接)，生成虚拟表VT1  
2. ON: 对VT1应用ON筛选，只有那些使<join_condition>为真的行才被插入VT2  
3. OUTER(JOIN): 如果指定了OUTER JOIN(相对于CROSS JOIN或INNER JOIN)，保留表(preserved table：左外部联接把表标记未保留表，右外部联接把右表标记为保留表，完全外部联接把两个表都标记未保留表)中找到匹配的行将作为外部行添加到VT2，生成VT3.如果FROM子句包含两个以上的表，则对上一个联接生成的结果表和下一个表重复执行步骤1到3，直到处理完所有的表为止。  
4. WHERE: 对VT3应用WHERE筛选器。只有使<where_condition>为true的行才被插入VT4  
5. GROUP BY: 按GROUP BY子句中的列对VT4中的行分组，生成VT5  
6. CUBE|ROLLUP: 把超组(Suppergroups)插入VT5，生成VT6  
7. HAVING: 对VT6应用HAVING筛选器，只有使<having_condition>为true的组才被插入VT7  
8. SELECT: 处理SELECT列表，产生VT8  
9. DISTINCT: 将重复的行从VT8中移除，产生VT9  
10. ORDER BY: 将VT9中的行按ORDER BY子句中的列列表排序，生成游标(VC10)  
11. TOP: 从VC10的开始处选择指定数量或比例的行，生成表VT11，并返回给调用者  
> 步骤10，按ORDER BY子句中的列列表排序上步返回的行，返回游标VC10.这一步是第一步也是唯一一步可以使用SELECT列表中的列别名的步骤。这一步不同于其他步骤的是，它不返回有效的表，而是返回一个游标。SQL的基于集合理论的，集合不会预先对它的行排序，它只是成员的逻辑集合，成员的顺序无关紧要。对表进行排序的查询可以返回一个对象，包含按特定物理顺序组织的行。ANSI吧这种对象称为游标。理解这一步是正确理解SQL的基础  
> 因为这一步不反悔表(而是返回游标)，使用ORDER BY子句的查询不能用作表表达式，表表达式包括：视图，内联表值函数，子查询，派生表和公用表达式。它的结果必须返回给期望得到物理记录的客户端应用程序。  

下面的派生表查询无效，并产生一个错误：  
```sql
SELECT *
FROM (SELECT orderid, customerid FROM orders ORDER BY orderid)
AS d
```
下面的视图也会产生错误
> 视图：存储在数据库中并具有名字的SQL语句，一个虚拟的表  
```sql
CREATE VIEW my_view
AS
SELECT * FROM orders ORDER BY orderid
```
