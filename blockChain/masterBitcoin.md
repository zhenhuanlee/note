# 精通比特币

## 6 交易
### 6.3 交易的输入和输出  
- 比特币交易中的基础构建单元是交易输出  
- 比特币完整节点跟踪所有可找到和可使用的输出，称为"未话费的交易输出"(UATXO)  
- 用户的比特币余额是指用户钱包中可用的UTXO总和  
- 如果一个UTXO比一笔交易所需量大，它仍会被当作一个整体而消耗掉，但同时会在交易中生成零头  
> 一个价值20比特币的UTXO，想要支付1个比特币，会产生两个输出，一个支付1个比特币给接收人，另一个支付了19个比特币的找零到自己的钱包  

#### 6.3.1 交易输出
交易输出包含两个部分：  
- 一定量的比特币，面值为聪(satoshis)，是最小的比特币单位  
- 确定话费输出所需条件的加密难题(cryptographoc puzzle)  
> 这个加密难题也被称为锁定脚本(locking script)，见证脚本(witness script)，或脚本公钥(scriptPubKey)  

```json
"vout": [
  {
    "value": 0.01500000,
    "scriptPubKey": "OP_DUP OP_HASH160 ab68025513c3dbd2f7b92a94e0581f5d50f654e7 OP_EQUALVERIFY
OP_CHECKSIG"
  },
  {
    "value": 0.08450000,
    "scriptPubKey": "OP_DUP OP_HASH160 7f9b1a7fb68d60c536c2fd8aeaa53a8f3cc025a8 OP_EQUALVERIFY OP_CHECKSIG",
  }
]
```
这个交易包含两个输出，每个输出都由一个值和一个加密难题来定义。在Bitcoin Core显示的编码中，该值显示已bitcoin为单位，但在交际本身，它被记录为以satoshis为单位的整数  
每个输出的第二部分是设定支出条件的加密难题。Bitcoin Core将其显示为scriptPubKey，并向我们展示了一个可读的脚本表示  

##### 6.3.1.1 交易序列化-输出  
交易在网络传输或者在程序之间交换时，他们被序列化。  
!(http://upload-images.jianshu.io/upload_images/1785959-d804ff8f61c1c86e.png?imageMogr2/auto-orient/strip%7CimageView2/2/w/1240)[seralize]  

#### 6.3.2 交易输入
交易输入将UTXO(通过引用)标记为将被消费，并通过解锁脚本提供所有权证明  
详细的看下输入组件：  
- 一个指向UTXO的指针：通过指向UTXO被记录在区块链中所在的交易的哈希值和序列号来实现  
- 解锁脚本：钱包构建它用以满足设定在UTXO中的支出条件，大多数情况下，解锁脚本是一个证明比特币所有权的数字签名和公钥，但并不是所有的解锁脚本都包含签名    
- 序列号  

交易输入是一个名为vin的数组(列表)：  
```json
"vin": [
  {
    "txid": "7957a35fe64f80d234d76d83a2a8f1a0d8149a41d81de548f0a65a8a999f6f18",
    "vout": 0,
    "scriptSig" : "3045022100884d142d86652a3f47ba4746ec719bbfbd040a570b1deccbb6498c75c4ae24cb02204b9f039ff08df09cbe9f6addac960298cad530a863ea8f53982c09db8f6e3813[ALL] 0484ecc0d46f1918b30928fa0e4ed99f16a0fb4fde0735e7ade8416ab9fe423cc5412336376789d172787ec3457eee41c04f4938de5cc17b4a10fa336a8d752adf",
    "sequence": 4294967295
  }
]
```
列表中只有一个输入(因为一个UTXO包含足够的值来完成支付)。输入包含四个元素：    
- 一个交易ID，引用包含正在使用的UTXO的交易  
- 一个输出索引(vout)，用于表示来自该交易的哪个UTXO被引用(第一个为零)    
- 一个scriptSig(解锁脚本)，满足放置在UTXO上的条件，解锁它用于支出  
- 一个序列号(稍后讨论)  

> - 在这笔交易中，输入指向的交易id是`7957a35fe64f80d234d76d83a2a8f1a0d8149a41d81de548f0a65a8a999f6f18`  
> - 输出索引是0(即由该交易创建的第一个UTXO)。解锁脚本由发送者的钱包构建，首先检索引用的UTXO，检查其锁定脚本，然后使用它来构建所需的解锁脚本以满足此要求  

##### 6.3.2.1
```json
"vin":
[
  {
    "txid": "7957a35fe64f80d234d76d83a2a8f1a0d8149a41d81de548f0a65a8a999f6f18",
    "vout": 0,
    "scriptSig" : "3045022100884d142d86652a3f47ba4746ec719bbfbd040a570b1deccbb6498c75c4ae24cb02204b9f039ff08df09cbe9f6addac960298cad530a863ea8f53982c09db8f6e3813[ALL] 0484ecc0d46f1918b30928fa0e4ed99f16a0fb4fde0735e7ade8416ab9fe423cc5412336376789d172787ec3457eee41c04f4938de5cc17b4a10fa336a8d752adf",
    "sequence": 4294967295
  }
]
```

#### 6.4.3 脚本构建(锁定与解锁) 
比特币的交易验证引擎依赖于两类脚本来验证比特币交易：锁定脚本和解锁脚本  
- 锁定脚本是一个放置在输出上面的话费条件：它指定了今后话费这笔输出必须要满足的条件，由于锁定脚本往往含有一个公钥或者比特币地址  
- 解锁脚本是一个解决或满足被锁定脚本在一个输出上设定的话费条件的脚本，它将允许输出被消费，解锁脚本是每一笔比特币交易输入的一部分，而且往往含有一个由比特币钱包(通过用户的私钥)生成的数字签名  
- 每个比特币节点会通过同时执行锁定和解锁脚本来验证一笔交易，每个输入都包含一个解锁脚本，并引用了之前存在的UTXO。验证软件将复制解锁脚本，检索输入所引用的UTXO，并从该UTXO复制锁定脚本，然后依次执行解锁和锁定脚本。  

#### 6.4.4 P2PKH (pat to public key hash)
比特币网络处理的大多数交易都是由"付款至公钥哈希"(或P2PKH)脚本锁定的输出，这些输出都含有一个锁定脚本，将输入锁定为一个公钥哈希(比特币地址)  
由P2PKH脚本锁定的输出可以通过提供一个公钥和由相应私钥创建的数字签名来解锁(使用)  
`OP_DUP OP_HASH160 <Cafe Public Key Hash> OP_EQUALVERIFY OP_CHECKSIG`  
相应的解锁脚本是：  
`<Cafe Signature> <Cafe Public Key>`  
将两个脚本结合起来可以形成如下组合验证脚本：  
```
<Cafe Signatur> <Cafe Public Key>OP_DUP OP_HASH160
<Cafe Public Key Hash> OP_EQUALVERIFY OP_CHECKSIG
```

