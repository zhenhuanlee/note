# 日常收集的面试题
<details>
  <summary>事务的ACID，并详细解释每一个</summary>
  <ul>
    <li>Atomicity</li>
    原子性，一个事务中的所有操作，要么都完成，要么都失败；失败会回滚到事务开始前的状态，事务不可分割，不可约简
    <li>Consistency</li>
    一致性，事务开始前和事务结束以后，数据库的完整性没有被破坏。这表示写入的数据必须完全符合预设的约束，触发器，级联回滚等  
    <li>Isolation</li>
    隔离性，数据库允许多个并发事务同时对其数据进行读写和修改的能力，隔离性可以防止多个并发事务交叉执行而导致数据不一致，事务隔离分为不同级别，包括读未提交(Read Uncommitted)，读提交(read committed)，可重复读(repeatable read)和串行化(Serialize)
    <li>Durability</li>
    持久性，事务处理结束后，对数据的修改是永久的，即使系统故障也不会丢失  
  </ul>
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
