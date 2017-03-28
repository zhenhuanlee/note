# [网页性能管理详解](http://www.ruanyifeng.com/blog/2015/09/web-page-performance-in-depth.html)
应该杜绝响应缓慢，占用大量资源的网页  

## 网页生成的过程：  
1. HHTML代码转化成DOM  
2. CSS代码转换成CSSOM（CSS OBJECT MODEL）  
3. 结合DOM和CSSOM，生成一棵渲染树（包含每个节点的视觉信息）  
4. 生成布局（layout），即将所有渲染树的所有节点进行平面合成  
5. 将布局绘制（paint）在屏幕上  
这五步里，一到三都是非常快的，耗时的是第四步和第五步  
生成布局(flow)和绘制(paint)这两部，合成为"渲染"(render)(我还以为整个过程是渲染😔)  

## 重排和重绘
生成网页的时候，至少会渲染一次，访问过程中，还会不断渲染  
网页重新渲染的几种情况：  
1. 修改DOM  
2. 修改样式表  
3. 用户事件（鼠标悬停，页面滚动，输入框键入文字，改变窗口大小等）  
重新渲染，就需要重排(reflow)-重新生成新的布局，重绘（repaint）  
注意：重绘不一定会出发重排，如改变元素的颜色

## 对于性能的影响
重排和重绘是影响性能的根本原因  
```js
div.style.color = 'blue'
div.style.marginTop = '30px'
```
浏览器已足够智能，会尽量把所有的变动集中起来，排成队列，一次执行，上述情况只会触发一次重排和重绘
```js
div.style.color = 'blue'
var margin = parseInt(div.style.marginTop)
div.style.marginTop = (margin + 10) + 'px'
```
以上代码就属于烂代码，会迫使浏览器先重绘，再重排  
一般来说，样式的写操作之后，如有下面的这些读操作，都会让浏览器重新渲染  
- offsetTop / offsetLeft / offsetWidth / offsetHeight  
- scrollTop / scrollLeft / scrollWidth / scrollHeight  
- clientTop / clientLeft / clientWidth / clientHeight  
- getComputedStye()  
> 尽量不要把读操作和写操作放在一个语句里（先读再写？）
```
// bad
div.style.left = div.offsetLeft + 10 + "px"
div.style.top = div.offsetTop + 10 + "px"

// good
var left = div.offsetLeft
var top = div.offsetTop
div.style.left = left + 10 + "px"
div.style.top = top + 10 + "px"
```
> 样式表越简单，重排和重绘就越快
> DOM元素层级越高，成本就越高
> table 元素的重排和重绘陈本高于div(这就是div布局比较流行的原因？)

## 提高性能的9个技巧
1. DOM元素的读写操作应该放在一起，不要穿插  
2. 如果某个样式是通过重排得到的，那么最好缓存一下，避免下一次用到时，浏览器又要重排  
3. 通过改变`css`或者`csstext`属性一次性的改变样式
```
// bad
var left = 10
var top = 10
el.style.left = left + "px"
el.style.top = top + "px"

// good
el.className += " theclassname"

// good
el.style.cssText += "; left: " + left + "px; top: " + top + "px;"
```
4. 尽量使用离线DOM，而不是真是的网页DOM，来改变元素样式：  
如：操作Document
Fragment对象，完成后把这个对象加入DOM。再比如：使用`cloneNode()`方法，在可伶的节电上进行操作，然后再用克隆的节点代替原始节点  
5. 先将元素设为`display:
   none`(需要一次重排重绘)，然后对这个节点进行100次操作，再恢复显示(一次重排重绘)，结果就是2次抵一次  
6. position属性为`absolute`或`fixed`的元素，重排的开销会比较小，因为不用考虑其他元素的影响  
7. 只在必要时，才将元素的`display`属性设为可见，`visibility:
   hidden`的元素只影响重绘  
8. 使用虚拟的DOM脚本库，如React(什么叫虚拟的DOM脚本库?)
9. 使用`window.requestAnimationFrame()`、`window.requestIdleCallback()`这两个方法调节重新渲染
