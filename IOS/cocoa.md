# COCOA

## [UITableView基本使用](http://www.jianshu.com/p/284fd7d8c9e9)
#### tableVIew 层次结构
![tableViewLevle](http://upload-images.jianshu.io/upload_images/831339-f16a76237b05408d.png?imageMogr2/auto-orient/strip%7CimageView2/2/w/1240)

#### cell
![cellStruct](http://upload-images.jianshu.io/upload_images/831339-5d0e0b5e642525aa.png?imageMogr2/auto-orient/strip%7CimageView2/2/w/1240)

- contentView下默认有三个子视图  
  - 2个`UILabel`(`textLabel`、`detailTextLabel`)  
  - 1个`UIImageView`(`imageView`)  

- UITableViewCellStyle属性  
  - 用于决定使用`contentView`的那些子视图，以及这些子视图在contentView中的位置  
  ![UITableViewCellStyle](http://upload-images.jianshu.io/upload_images/831339-0493410b9c85bc3d.png?imageMogr2/auto-orient/strip%7CimageView2/2/w/1240)

- cell重用原理  
  当滚动列表时，部分`UITableViewCell`会移出窗口，  
  `UITableView`会将窗口外的`UITableViewCell`放入一个对象池中，等待重用   
  当`UITableView`要求`dataSource`返回`UITableViewCell`时，  
  `dataSource`会先查看这个对象池，如果池中有未使用的`UITableViewCell`，  
  `dataSource`会用新的数据配置这个`UITableViewCell`，然后返回给`UITableView`，重新显示到窗口中，从而避免创建新对象  

- 不同类型的Cell重用  
  - 解决方案：制定不同类型对应的重用标识来区分  
    `UITableViewCell`有个`NSString *reuseIdentifier`属性  
    可以在初始化`UITableViewCell`的时候传入一个特定的字符串标识来设置`reuseIdentifier`(一般用`UITableViewCell`的类名)  
    当`UITableView`要求dataSource返回`UITableViewCell`时  
    先通过一个字符串标识到对象池中查找对应类型的`UITableViewCell`对象，  
    如果有就重用，如果没有，就传入这个字符串标识来初始化一个`UItableVIewCell`对象  

- 注册cell
```objective-c
[self.tableView registerClass:[TagCell class] forCellReuseIdentifier:@"tgID"]
```

- 快速常见Cell: xib / storyboard
```objective-c
- (instancetype)initWithTableView:(UITableView *)tableView{
  static NSString *identifier = @"tgCell";
  TgCell *cell = [tableViewdequeueReuseableCellWithIdentifier:identifier];
  if (!cell) {
    // 1. xib 缓存池里没有通过 loadNibNamed...方法加载重写创建
    //    cell = [[NSBundle mainBundle] localNibNamed:@"TgCell" owner:nil options:nil] lastObject];
    // 2. storyboard   缓存池中没有通过 initWithStyle...方法加载重写创建  
    cell = [[TgCell alloc] initWithStyle:UITableViewCellStyleDefaultreuseIdentifier:identifier];
  }
  return cell;
}
```

  - iOS8，自动计算Cell高度  
  ```objective-c
  // 告诉tableView的真实高度是自动计算的，根据你的约束来计算
  self.tableView.estimateRowHeight = 44
  // 告诉tableView所有cell的估计行高
  self.tableView.estimatedRowHeight = 44
  // 返回估计高度，告诉tableView显示的时候，先根据估算高度得到整个tableView高，而不必知道每个cell的高度，从而达到高度方法的懒加载调用
  ```

#### 常见属性  
```objective-c
// UITableView的两种样式
UITableViewStylePlain / UITableViewStyleGround

self.tableView.backgroundColor = [UIColor purpleColor]

// 设置索引条内部文字颜色
self.tableView.backgroundColor = [UIColor colorWithRed:1, green:1 blue:1 alpha:1];

// 设置索引条背景颜色
self.tableView.sectionIndexBackgroundColor = [UIColor colorWithRed:0, green:0 blue:0 alpha:1];

etc...
```
