# Bitcoin一些参数  
## 创建一个新的区块链  
### chainparams.cpp  
- pchMessageStart  
最重要的，后面的16进制标识符，是区块链的一个ID，用于区别两条不一样的区块链  
```c++
// number 0~6 
// character a~f
pchMessageStart[0] = 0xf9;
pchMessageStart[1] = 0xbe;
pchMessageStart[2] = 0xb4;
pchMessageStart[3] = 0xd9;
```

- pszTimestamp  
用来记录区块链alive的那一天发生的大事情  

- nTime  
代表区块链发布的Unix时间  

- genesis.nNonce  
需要修改成`genesis.nNonce = 0`  
nNonce用来计算找到第一个block需要的区块数，一旦找到创始区块，将会重新计数block  

- 移除旧的创始区块和merkle的值    
删除unit256S后面的值，编译报错后，看debug.log的信息，将新的value填入  
```c++
assert(consensus.hashGenesisBlock == uint256S("0x000000000019d6689c085ae165831e934ff763ae46a2a6c172b3f1b60a8ce26f"));
assert(genesis.hashMerkleRoot == uint256S("0x4a5e1e4baab89f3a32518a88c31bc87f618f76673e2cc77ab2127b7afdeda33b"));
```

- nLastPOWBlock(cannot find in bitcoin)  
代表POW的最后一个block   
 
