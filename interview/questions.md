# 日常收集的面试题
<details>
  <summary>事务的ACID，并详细解释每一个</summary>
</details><br>

<details>
  <summary>脏读，幻影读，不可重复读</summary>
</details><br>

<details>
  <summary>红黑树，二叉树的算法</summary>
</details><br>

<details>
  <summary>如何解决hash冲突的，如果冲突了，怎么在hash表中找到目标值</summary>
</details><br>

<details>
  <summary>ThreadLocal是啥</summary>
</details><br>

<details>
  <summary>MySQL行锁是否有死锁的出现</summary>
</details><br>

<details>
  <summary>乐观锁悲观锁</summary>
</details><br>

<details>
  <summary>nginx负载均衡策略</summary>
</details><br>

<details>
  <summary>redis并发高的原因</summary>
</details><br>

<details>
  <summary>redis哨兵，复制，集群</summary>
</details><br>

<details>
  <summary>redis内存淘汰机制</summary>
</details><br>

<details>
  <summary>SQL优化</summary>
</details><br>

<details>
  <summary>分布式锁</summary>
</details><br>

<details>
  <summary>缓存穿透，缓存雪崩，缓存击穿</summary>
  <ul>
    <li>缓存穿透</li>
    * 查询一个不存在的数据，一般的缓存都不会将空的情况写入缓存。<br>
    * 可以用布隆过滤器，或将空也缓存，过期时间设置的短一点(5分钟)
    <li>缓存雪崩</li>
    * 大量的缓存设置了相同的过期时间，导致同一时间大量的请求到达数据库<br>
    * 缓存的时间可以分散开，比如在原有的过期时间上加点随机值
    <li>缓存击穿</li>
    * 某个数据在缓存过期的时候有大量的并发访问，这样就导致了大量针对该key的请求落到了数据库上<br>
    * 可以设置不过期，只更新；或使用互斥锁，在访问缓存过期的数据时，不是去撸数据库，而是先使用缓存工具的某些带成功返回值的操作(如Redis的SETNX)去set一个mutex key，当操作返回成功时，再进行load db的操作 
  </ul>
</details><br>
