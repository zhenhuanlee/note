# [以太坊智能合约](http://ethfans.org/posts/101-noob-intro)  

#### 流程 
1. 使用Solidity语言开发  
2. 使用solc编译  

#### 部署流程
1. 启动一个以太坊节点  
2. 使用solc编译智能合约 => 获得二进制代码  
3. 将编译好的合约部署到网络(这一步会消耗以太币，还需要使用你的节点默认地址或者指定地址来给合约签名)  
4. 获得合约的区块链地址和ABI  
5. 使用web3.js提供的Javascript API来调用合约  

![flow](http://ethfans.org/uploads/photo/2015/1fc96327c8a1d60c8dc16f8ec1a2fe5d.png)


#### 编程  
##### 使用truffle部署智能合约的步骤  
1. `truffle init`在新目录中创建truffle项目目录结构  
2. 编写合约代码，保存到`contracts/name.sol`文件  
3. 把合约名字加到`config/app.json`的contracts部分(存疑)  
4. 启动以太坊节点(可以是testrpc)  
5. `truffle deploy`  
`truffle deploy`会把部署好的合约的地址和ABI(应用接口)加入到配置文件中，之后`truffle test`和`truffle build`步骤可以使用这些信息  

##### 基本数据类型
- `address`地址类型  
- `uint`无符号数  
- `public`这个关键字表明变量可以被合约之外的对象使用，`private`修饰则表示变量智能被本合约(或者衍生合约)内的对象使用  
- `mapping或数组(mapping类似Hash，Dictionary等数据类型)  
- 组织者地址 vs. 合约地址，部署好的合约在区块链上拥有自己的地址，在Solidity合约中可以使用`this`来访问这个地址，如`address myAddress = this`  
- `suicide` Solidity的好东西，转给合约的地址会保存于合约(地址)中，最终这些资金通过`destroy`函数被释放给了构造函数中设置的组织者地址。这是通过`suicide(organizer)`这行代码实现的  

> 模拟一个交易  
> `conference.byTicjet({ from: accounts[1], value: some_ticket_price_integer });`  
> 函数调用可以是交易。改变合约状态(修改变量值，添加记录等)的函数调用本身也是转账交易，隐式的包含了发送人的交易价值。因此web3.js的函数调用可以通过指定`{ from: _, value: _}`参数来发送以太币。在Solidity合约中，可以通过`msg.sender`和`msg.value`来获取这些信息  

- 事件(Event) 可选的功能，合约中的`Deposit`和`Send`事件是会被记录在以太坊虚拟机日志中的数据。他们实际上没有任何作用，但是用事件把交易记录进日志是好的做法  

- 构造函数 `Conference.new({ from: accounts[0] })`通过调用合约构造函数创造一个新的Conference实例。(默认就是accounts[0]，from可省略)  

- `Promise` 代码中的`then`和`return`就是`Promise`，写成一个深深的嵌套调用链的话是这样的：  
```javascript
conference.numRegistrants.call().then(
  function(num) {
    assert.equal(num, 0, "Registrants should be zero!");
  }
  conference.organizer.call().then(
    function(organizer) {
      assert.equal(organizer, accounts[0], "Owner doesn't match!");
    }
  ).then(
    function(...)
  )
).then(
  function(...)
)
```
> `Promise`减少嵌套，使代码扁平，允许用异步返回，简化了表达"成功时做这个"和"失败时做那个"的语法 > Web3.js通过回调函数实现异步调用，因此不需要等到交易完成就可以继续执行前端代码  

- `call` 使用`call`来检查变量的值，例如`conference.quota.call().then(...)`，还可以通过传参数，如`call(0)`，来获取mapping在index 0处的元素  

- 断言 标准JS测试中的断言，`assert.equal`最常用  

- `Wei` 以太币有很多种单位，在合约中常用的是`Wei`,最小的单位。Web3.js提供了在各单位与Wei之间互相转换的便利方法，如`web3.toWei(.05, 'ether')`。  

- 账户余额 `web3.eth.getBalance(some_address)`。记住发送给合约的资金会由合约自己持有，直到调用`suicide`  

- `toNumber()`将16进制结构转码，大数字有`web3.toBigNumber(numberOrHexString)`  

- 合约中发送以太币  
    `address myAddress = this` 获取该合约实例地址  
    `this.balance`  检查余额  
    合约通过`recipient.send(amount)`方法把资金发回了购票人  

- 在web3.js中，应该监听事件而不是返回值  
```javascript
event Deposit(address _from, uint _amount);
event Refund(address _to, uint _amount);
```

- `Filter` 监听事件可能会产生大量的轮询，作为替代可以使用过滤器。他们可以更灵活的开始或停止对事件的监听  
总的来说，使用事件和过滤器的组合比检查变量消耗的Gas更少，因而在验证正式网络的交易运行结果时非常有用  

- `Gas` 在交易函数调用中可以在`{from: _, value: _, gas: _}`对象内设置Gas参数  
    `web3.eth.gasPrice`调用来获取当前Gas的价格  

