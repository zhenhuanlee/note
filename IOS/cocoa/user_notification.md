# [iOS通知](https://onevcat.com/2016/08/notification/)  
#### UserNotification框架解析
##### 基本流程  
申请和注册  -->  创建和发起  -->  展示和处理  
需要向用户请求权限，然后发送通知，如果应用处于后台或者没有运行的话，就通过(弹窗，横幅，或者通知中心)进行显示。如果应用已经在运行，则可以自行决定要不要显示这个通知，还可以给点击通知赋予打开应用以外的功能  

##### 权限申请  
###### 通用权限  
- iOS 8之前，本地推送(`UILocalNotification`)和远程推送(Remote Notification)是区别对待的，应用只需要在进行远程推送时获取用户的同意。  
- iOS 8则无论是本地还是远程推送，都需要申请权限  
- iOS 10里进一步消除了本地通知和推送通知的区别，向用户申请通知权限非常简单  
```swift
// 需要导入UserNotifications库
import UserNotifications

UNUserNotificationCenter.current().requestAuthorization(options: [.alert, .sound, .badge]) {
  granted, error in
  if granted {
    // 用户允许进行通知
  }
}
```

##### 远程推送
如果要通过服务器发送远程消息的话，还需要多一个获取用户token的操作。你的服务器可以使用这个token向Apple Push Notification的服务器提交请求，然后APNs通过token识别设备和应用，将通知推给客户  

提交token请求和获取token的回调是现在"唯二"不在新框架中的API。  
```swift
// 向APNs请求token
UIApplication.shared.registerForRemoteNotifications()

// AppDelegate.swift
func application(_ application:UIApplication, didRegisterForRemoteNotificationsWithDeviceToken deviceToken:Data) {
  let tokenString = deviceToken.hexString
  print("Get Push Token: \(tokenString)\")
}
```
获得的`deviceToken`是一个`Data`类型，一般会将其转换为一个字符串  

##### 权限设置  
用户可以在设置中修改应用的通知权限，除了打开和关闭全部通知权限外，用户也可以限制应用只能进行某种形式的通知形式，如只允许横幅二部允许弹窗及通知中心等。一般来说，不应该干涉用户的选择，但是，可以对当前用户进行的设置检查  
```swift
UNUserNotificationCenter.current().getNotificationSettings {
  settings in
  print(settings.authorizationStatus) // .authorized | .denied | .notDetermined
  print(setting.badgeSetting) // .enabled | .disabled | .notSupported
  // etc...
}
```

#### 发送通知  
`UserNotifications`中对通知进行了统一，我们通过通知的内容(`UNNotificationContent`)，发送的时机(`UNNotificationTrigger`)以及一个发送通知的`String`类型的标识符，来生成一个`UNNotificationRequest`类型的发送请求。最后，将这个请求添加到`UNUserNotificationCenter.current()`中，就可以等待通知到达了  
```swift
// 1. 创建通知内容
let content = UNMutableNotificationContent()
content.title = "Time Interval Notification"
content.body = "My first notification"

// 2. 创建发送触发
let trigger = UNTimerIntervalNotificationTrigger(timeInterval: 5, repeats: false)

// 3. 发送请求标识符
let requestIdentifier = "com.onevcat.usernotification.myFirstNotification"

// 4. 创建一个发送请求
let request = UNNotificationRequest(identifier: requestIdentifier, content: content, trigger: trigger)

// 将请求添加到发送中心
UNUserNotificationCenter.current().add(request) { error in
  if error == nil {
    print("Time interval Notification scheduled: \(requestIdentifier)\")
  }
}
```

- 通知不仅仅支持简单的一行文字，还可以添加`title`和`subtitle`，来用粗体的形式强调通知的目的，对于远程推送，iOS 10值钱一般只含有消息的推送payload是这样的  
```swift
{
  "aps": {
    "alert": "Test",
    "sound": "default",
    "badge": 1
  }
}
// 加入title和subtitle
{
  "aps: {
    "alert": {
      "title": "title",
      "subtitle": "subtitle",
      "body": "body"
    },
    "sound": "default",
    "badge": 1
  }
}
```

- 触发器  
触发器只是针对本地而言的，远程推送的通知会立刻显示  
1. `UNTimeIntervalNotificationTrigger` 在一定时间后触发
2. `UNCalendarNotificationTrigger` 在某月某日某时触发  
3. `UNLocationNotificationTrigger` 在用户进入或离开某个区域时触发  
