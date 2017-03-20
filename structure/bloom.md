# [Bloom Filter By Example](http://llimllib.github.io/bloomfilter-tutorial/)
设计Bloom Filter是为了快速且内存友好的判断一个元素是否属于一个集合  
代价是Bloom Filter是概率性的: 它断定属于集合的元素，有可能是不属于的  
Bloom Filter 的基础数据结构是一个**位向量**，网页上有个小例子  
该例中，没一个空的代表一个比特位，下面的数字是它的索引

## Hash函数
Bloom
Filter的哈希函数应该是独立的、不统一的、分散的、越快越好，就像~~murmur~~SpiSHash、fnv系列和Jenkina  
一些BloomFiler的实现  
- Cassandra used  Murmur hashes
- [Hadoop](http://salsahpc.indiana.edu/tutorial/apps/hadoop-0.20.203.0/docs/api/org/apache/hadoop/util/hash/Hash.html) includes default implementions of Jenkins and Murmur hashes
- [python-bloomfilter](https://github.com/jaybaird/python-bloomfilter/blob/master/pybloom/pybloom.py) uses cryptographic hashes
- [Sdroege Bloom Filter] uses fnv1a
- [Squid]() uses MD5

## Double Hashing
不用选择2个或更多的hash函数，相反，可以创建任意数量的新的哈希函数通过[Double
Hashing](https://en.wikipedia.org/wiki/Double_hashing)两个函数  
通过两个独立的哈希函数a和b，和一个值x，可以创建一个新的哈希函数  
```
hash<sub>i</sub>(x, m) = (hash<sub>a</sub>(x) + i * hash<sub>b</sub>(x)) mod m
```

## Bloom Filter 多大比较好
Bloom Filter的一个好处是，你可以修改失误几率，大的filter会比较小，这个比例近似于(1-e<sup>-kn/m</sup>)<sup>k</sup>

## 多少个hash函数比较合适
哈希函数越多，filter就越慢，也满的更快，太少的话精确度太低，必须选择合适的k、n、m，公式是(m/n)ln2  
选择bloom filter大小的步骤是：  
1. 先大概决定`n`的值  
2. 决定`m`的值  
3. 计算出`k`的最佳值  
4. 计算失误比，如果不能接受，返回第二步  
