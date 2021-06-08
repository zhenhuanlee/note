# 一些常用的方法
```swift
// 裁剪一个圆
clipShape(Circle())

// overlay 加一层
// stroke 掏空中心, 白色，边框粗10
// 经典用法，给一个圆加一层圆形，然后加阴影
overlay(Circle().stroke(Color.white, lineWidth: 10)).shadow(radius: 10)

// 图片变大小
.imageScale()

// forEach 和 List 差不多
List([...]) { item in }  // 适用于一个元素
List {  // 多个元素，主要作用是划线
  Toggle(){}
  ForEach([...]) { item in }  
}
```

# 如何在 swiftui 中自定义元素，比如兼容uikit
```swift
struct MyView: UIViewRepresentabable {
  func makeUiView(context: Context) -> XXView {  }
  func UpdateView()...
}
```

# ObservableObject 用法
```swift
// 继承 ObservableObject
// @Published 表示，这个变量变化会引起视图刷新
final class UserData: ObservableObject {
    @Published var userLandMarks = landmarks
}
```

# animation
```swift
Button(action: {
    withAnimation() { // 表示这个变化有动画效果
        self.isOn.toggle()
    }
}){
    Image(systemName: "delete.right.fill")
        .scaleEffect(isOn ? 4 : 2)
        .animation(nil)    // 表示之前的属性变化不会有动画效果
        .rotationEffect(.degrees(isOn ? 90 : 0))
}
```

#### 动画2
```swift
Image(systemName: "person")
    .scaleEffect(5)
    .offset(x: 0, y: 100)
    .transition(.xx)

extension AnyTransition {
    static var xx: AnyTransition {
        .asymmetric(   // 进 出 采用不同的动画，很棒
            insertion: AnyTransition.move(edge: .trailing).combined(with: .opacity),
            removal: AnyTransition.scale.combined(with: .opacity)
        )
    }
}
```

#### 动画3
```swift
Image(systemName: "sun.max")
    .resizable()
    .frame(width: 100, height: isOn ? 200 : 100)
    .padding(.top, 200)
    .animation()

var yy: Animation {
    Animation.spring()
      .speed(5)  // 速度
      .delay(2)) // 延迟，很有用，可以让一组动画形成波浪形效果，很酷
}
```

## 当发现边框不靠边的时候，试试  
```swift
listRowInset()
// 给 List 中的元素设置，甚至可以是 Image
// 貌似子元素会继承父元素的 这个属性，所以最外层设置就可以
```

## Environment 环境变量
```swift
@Environment(\.editMode) var mode
// 系统提供，可以用于判断是否在编辑状态
// 还有其他好多，比如 colorSchema


// 常用的环境变量 
// 黑色模式
View().environment(\.colorScheme, .dark)
```