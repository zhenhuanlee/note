# CoreData
## 保存数据的方式
- 用户偏好设置  
- 归档  
- sqlite  
- CoreData(sqllite的封装，ORM)  

### CoreData的使用步骤  
1. 创建模型文件（相当于一个dataBase）    
2. 添加实体 （相当于一个表）  
3. 创建实体类 （相当于一个模型类）  
4. 生成上下文（关联数据库） 关联模型文件生成数据库  
5. 保存对象到数据库  
6. 从数据库获取对象  
7. 更新数据  
8. 删除数据  

#### [实例](https://www.raywenderlich.com/145809/getting-started-core-data-tutorial)
[代码](https://github.com/pumpkin2011/CoreData)  
```objective-c
// 通过AppDelegate来获取当前的NSManagedObject
AppDelegate *appDelegate = (AppDelegate * )[[UIApplication sharedApplication] delegate];
NSManagedObjectContext *managedContext = [[appDelegate persistentContainer] viewContext];
NSEntityDescription *entity = [NSEntityDescription entityForName:@"Person" inManagedObjectContext:managedContext];
NSManagedObject *person = [[NSManagedObject alloc] initWithEntity:entity insertIntoManagedObjectContext:managedContext];
[person setValue:name forKey:@"name"];
```
