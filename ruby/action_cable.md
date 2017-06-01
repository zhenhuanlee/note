# Action Cable
## Introduction
Action
Cable将WebSocket与Rails的其余部分无缝集成了。它可以让你像写其他东西那样写实时功能，并保持高性能和可扩展。这是一个提供JS框架和服务端Ruby框架的全栈提供者，可以访问所有的ORM框架的model  

## What is Pub/Sub
Pub/Sub（Publish-Subscribe），指的是一个消息队列范例，senders将消息发送到抽象的一类接收者，而不是某个特定的接受者。Action Cable用这个方法来实现一对多通信  

## Server-Side Components
### Connections
`Connection`是客户-服务器的基础，服务器每接收到一个WebSocket，一个`connection`对象就会被实例化。所有`channel subscripitions`(频道订阅)都是在`connection`的基础上创建的。  
`Connection`本身不处理任何的除身份认证和授权之外的逻辑。  
WebSocket连接的客户端叫做`connection`的consumer  
`Connection`是`ApplicationCable::Connection`的实例，在这个类中，可以授权`connection`和建立连接

#### Connection Setup
```ruby
# app/channels/application_cable/connection.rb
module ApplicationCable
  class Connection < ActionCable::Connection::Base
    identified_by :current_user

    def connect
      self.current_user = find_verified_user
    end

    private
      def find_verified_user
        if current_user = User.find_by(id: cookies.signed[:user_id])
          current_user
        else
          reject_unauthorized_connection
        end
      end
  end
end
```
这里的`identified_by`是一个可以用来查找特定`connection`的标识符。注意：在声明连接标识符的同事，在基于连接创建的频道实例上，会自动创建同名`delegate`(委托)  

这个例子基于你已经在其他地方处理已用户验证，而且成功设置了用户ID的cookie  
当一个新的`connection`尝试连接时，这个cookie会自动传送给`connection`实例，然后使用这个去设置`current_user`，并通过这个用户标识这个`connection`，也可以通过这个用户检索所有打开的`connection`  

### Channels
一个`channel`封装了一个逻辑单元，就像一个MVC的控制器。默认的，Rails会创建一个`Application::Channel`父类来封装`channel`之间的共有的逻辑  

#### Parent Channel Setup
```ruby
# app/channels/application_cable/channel.rb
module ApplicationCable
  class Channel < ActionCable::Channel::Base
  end
end
```
然后就创建自己的`channel`类  
```ruby
# app/channels/chat_channel.rb
class ChatChannel < ApplicationCable::Channel
end
 
# app/channels/appearance_channel.rb
class AppearanceChannel < ApplicationCable::Channel
end
```
一个用户就可以订阅了  

#### Subscriptions
用户订阅`channels`，这个`connection`就叫做一次订阅。发送信息等下会被`routed`到这些基于标识符频道的订阅者 
```ruby
# app/channels/chat_channel.rb
class ChatChannel < ApplicationCable::Channel
  # Called when the consumer has successfully
  # become a subscriber of this channel.
  def subscribed
  end
end
```


## Client-Side Components
### Connections
用户需要一个`connection`的实例，这个可以通过下面的JS来创建(Rails自动创建)  

#### Connect Consumer
```ruby
// app/assets/javascripts/cable.js
//= require action_cable
//= require_self
//= require_tree ./channels
 
(function() {
  this.App || (this.App = {});
 
  App.cable = ActionCable.createConsumer();
}).call(this);
```
这个会通过`/cable`来让用户和服务器创建连接。在你指定了至少一个订阅之前，连接不会被建立  

#### Subscriber
用户在订阅了一个`channle`后就成了订阅者  
```ruby
# app/assets/javascripts/cable/subscriptions/chat.coffee
App.cable.subscriptions.create { channel: "ChatChannel", room: "Best Room" }
 
# app/assets/javascripts/cable/subscriptions/appearance.coffee
App.cable.subscriptions.create { channel: "AppearanceChannel" }
```
一个订阅者可以同时订阅多个频道，例如：一个用户可以同时订阅多个聊天室  
```ruby
App.cable.subscriptions.create { channel: "ChatChannel", room: "1st Room" }
App.cable.subscriptions.create { channel: "ChatChannel", room: "2nd Room" }
```

## Cleint-Server Interactions
### Streams
频道通过`stream`来将内容广播给订阅者  
```ruby
# app/channels/chat_channel.rb
class ChatChannel < Application::Channel
  def subscribed
    stream_from "Chat_#{params[:room]}"
  end
end
```
如果有一个流是关联model的，那么这个广播流可以通过model和channel来生成，下面的例子会订阅一个`comments:Z2lkOi8vVGVzdEFwcC9Qb3N0LzE`这样的广播  
```ruby
class CommentsChannel < ApplicationCable::Channel
  def subscribed
    post = Post.find(params[:id])
    stream_for post
  end
end
```
然后就可以向下面这样向频道广播了  
```ruby
CommentsChannel.broadcast_to(@post, @comment)
```

### Broadcasting
`broadcasting`是一个发送者向订阅者发送流的连接，每个`channel`可以发送0~n个`broadcasting`  
`broadcastings`是依赖实践的在线队列，如果一个用户没有使用流(订阅频道)，他们将不收到这个广播，即使他们之后进行了连接  
`broadcasts`在别的地方被调用  
```ruby
WebNotificationChannel.broadcast_to(
  current_user,
  title: 'New things!',
  body: 'All the news fit to print'
)
```
这个`WebNotificationsChannel.broadcast_to`调用会推送一条消息给当前的订阅适配器(默认的，生产环境是redis，其他是async)的发布订阅队列，每个用户的广播名都是独一无二的，如一个ID是1的用户，广播名是`web_notifications:1`  
通过调用`received`这个回调，`channel`会使用流把到`web_notifications:1`的信息直接发送给客户端  

### Subscriptions
当一个用户订阅了`channel`，他就成了一个订阅者。这个`connection`叫做一个订阅。收到的消息会被路由到这些基于用户标识区分的订阅  
```coffee 
# app/assets/javascripts/cable/subscriptions/chat.coffee
# Assumes you've already requested the right to send web 
notifications
App.cable.subscriptions.create { channel: "ChatChannel", room: "Best Room" },
  received: (data) ->

  appendLine: (data) ->
    html = @createLine(data)
    $("[data-chat-room='Best Room']").append(html)

  createLine: (data) ->
    """
    <article class="chat-line">
      <span class="speaker">#{data["sent_by"]}</span>
      <span class="body">#{data["body"]}</span>
    </article>
    """
```

### Passing Parameters to Channels
可以从客户端传参到服务端通过创建一个订阅
```ruby
# app/channels/chat_channel.rb
class ChatChannel < ApplicationCable::Channel
  def subscribed
    stream_from "chat_#{params[:room]}"
  end
end
```
