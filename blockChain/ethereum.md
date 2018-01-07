# Ethereum

#### Gas
- Gas is the way that fees are calculated  
- The fees are still paid in either, though, which is different from gas  
- The gas **cost** is the amount of hours of labour, whereas the gas **price** is like the hourly wage you pay for the work to be done. The combination of the two determines your total transaction fee  
- if gas price is too low, no one will process your transaction  
- if gas price is fine but the gas cost of transaction runs "over budget" the transaction fails but still goes into the blockchain, and you don't get the money backfor the work that the labourers did  
- This makes sure that nothing runs forever, and that people will be careful about the code that they run. It keeps both miners and users safe from bad code!  

## 区块链6层架构
1. 数据层  
  封装了底层数据区块的链式结构，以及相关的非对称公私钥数据加密计数和时间戳等技术
2. 网络层  
  包括分布式组网机制、数据传播机制和数据验证机制  
3. 共识层  
  封装网络节点的各类共识机制算法，共识机制算法是区块链技术的核心技术  
4. 激励层  
  将经济因素集成到区块链技术体系中来，主要包括经济激励的发行机制和分配机制  
5. 合约层  
  主要封装各类脚本、算法和智能合约，是区块链可编程特性的基础  
6. 应用层  
  封装了区块链的各种应用场景和案例，比如搭建在以太坊上的各类区块应用就是部署在应用层  


### 如何连接以太坊客户端  
Geth是客户端，完成了前5层的代码  
web、app等其他可以通过web3.js和Geth进行交互  

### 以太币和账户  
1. 以太坊最小单位是Wei.  
  - 1 ether = 1e18 wei.  

2. 以太坊有两种账户  
  - Externally Owned Accounts(EOAs) - 个人账户  
  - Contract Accounts - 合约账户  
  - 两种的却别：
    - EOA由私钥来控制，谁拥有私钥谁拥有这个账户  
    - 合约账户则由代码来决定，可以是创建合约的人，也可以是股份最大的，也可以是投票决定的  

3. Message  
  - 当一个交易发生时，会产生一个message  
  - A 向 B 交易10ether
    - A Message.sender  
    - 10ether Message.value  
  - A 调用智能合约C时  
    - A Message.sender  
    - Message.value 得看具体有没有交易额  
  - C 合约调用合约 D  
    - C Message.sender  
    - Message.value 同上

4. Gas  
  A调用contract C的function，需要支付Gas，一般是5位数的Gas  

5. Confirmation  
  解决双花问题  
  任何一笔TX，都需要区块的确认，如果失败会被回滚  

### 创建私有链  
- `geth --datadir './' init genesis.json `
  - `--datadir` 需要手动指定，否则就会被当成是是公有链的节点
- `geth --datadir "./" --nodiscover console 2>>geth.log`  
  - `--nodiscover` 防止其他节点查找，加入  
  - `console` 进入交互  
  - `2>>geth.log` 将日志输出  

### 挖矿  
挖矿的本质是挖block，所有的智能合约啥的调用都要放在交易中  

### 以太币
以太币的基本单位是wei  
```
web3.toWei(0.01) = 10000000000000000
web3.fromWei(10000000000000000) = 0.01
```
