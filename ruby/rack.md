# Unserstanding Rack Apps And Middleware
## What's Rack
Rack是ruby的`Net::HTTP`包的封装  
如何开发一个Racky应用？首先得有一个响应`call`方法的对象，取environment
hash然后返回一个带response code, headers,
body的HTTP数组。启动服务可以使用`Rack::Handler:WEBrick`或者将代码放在config.rb文件中，然后使用`rackup config.rb`  

## How Rack Workds
下面有一些应该知道的方法，这些方法都可以在config.ru中直接调用  
### run 
`run`将一个application(响应`call`方法的对象)作为参数  
> 将application作为参数？是block的意思么？
```ruby
run Proc.new{ |env| ['200', {'Content-Tpye' => 'text/html'}, ['get rack\'d']] }
```

### map
匹配字符串制定的路由，匹配成功方法块中的代码会被执行  

### use
制定Rack使用的中间件  

## The Environment Hash
Rack应用需要一个Environment Hash，这个Hash会包含以下内容  
- REQUEST_METHOD: 请求的HTTP方法，必须  
- PATH_INFO: 请求的URL，应用根的相对路径  
- QUERY_STRING: '?'后面的所有字符  
- SERVER_NAME and SERVER_PORT: 服务的地址和端口  
- rack.version: rack的版本  
- rack.url_scheme: http还是https  
- rack.input: 一个像`IO`的对象，包含了原生的HTTP POST数据  
- rack.errors: 一个响应`puts`,`write`,`flush`的对象  
- rack.session: 一个存储请求session的键值对  
- rack.logger: 一个对象可以记录请求，实现了`info`,`debug`,`warn`,`error`和`fatal`方法  

许多基于Rack的框架都将env
hash打包进了`Rack::Request`对象，这个对象提供了许多方便的方法，`request_method`,`quert_string`,`session`和`logger`，这个对象也让你可以检查参数，HTTP schema，是否使用ssl  

## The Response
一个Rack服务的返回必须包含三部分：status，headers，body，像request一样，`Rack::Response`对象也提供了很多语法糖`write`,`set_cookie`,`finish`...

### Status
200 400啥的  

### Headers
一个响应`each`的键值对对象，可以在此处设置`Content-Type`,`Content-Lenght`

### Body
服务器对请求的返回  

## What is Middleware
Rack如此好用的原因之一就是添加中间件非常的方便  
中间件位于客户端和服务器之间，处理请求和响应  

## Using Middleware in a Rack App
如何添加一个中间件到Rack程序：只需要告诉Rack取使用它就可以了  
可以使用多个中间件，它们会改变请求或者响应在到达下一个模块之前，这些中间件模块叫做中间件堆  

### WARDEN
展示如果使用在一个项目中使用`Warden`,Warden必须跟随session类的中间件，所以也会使用`Rack::Session::cookie`  

1. 将`Warden`加入Gemfile  
2. 将`Warden`加入到config.ru  
    ```ruby
    require 'warden'

    use Rack::Session::Cookie, secret: "MY_SECRET"
    failure_app = Proc.new { |env| ['401', {'Content-Type' => 'text/html'},
    ['UNAUTHORIZED']] }

    use Warden::Manager do |manager|
      manager.default_strategies :password, :basic
      manager.failure_app = failure_app
    end

    run Proc.new { |env| ['200', {'Content-Type' => 'text/html'}, ['get'rack\'d']] }
    ```
    还有其他方法定义middleware
stack，可以使用`Rack::Builder`去打包多个moddleware和apps在一个大的application  
    ```ruby
    failure_app = Proc.new { |env| ['401', {'Content-Type' => 'text/html'}, ['UNAUTHORIZED']] }

    app = Rack::Builder.new do
      use Rack::Session::Cookie, secret: 'MY_SECRET'

      use Warden::Manager do |manager|
        manager.default_strategies :password, :basic
        manager.failure_app = failure_app
      end
    end

    run app
    ```

### Rack Basic Auth
一个很有用的中间件是`Rack::Auth::Basic`，它可以通过HTTP basic
Authentication保护任何的Rack app。这是一个轻量的迟早有用的  
比如，Ryan Bates使用它来保护Resque服务  
```ruby
use Rack::Auth::Basic 'Restricted Area' do |username, password|
  [username, password] == ['admin', 'abc123']
end
```

## Using Middleware in Rails
### How Rails use Rack
你可能注意到每个Rails app的根目录下面都有一个config.rb文件
```ruby
# This file is used by Rack-based servers to start the application.

require ::File.expand_path('../config/environment', __FILE__)
run Rails.application
```
引用了`config/environment`文件，继续加载了`config/application.rb`文件  
`config/application.rb`文件加载了`config/boot.rb`，`config/boot.rb`require了所有的`rails`，加载了`environment`(text, development, production)，也定义了命名空间的版本，像下面这样  
```ruby
module MyApplication
  class Application < Rails::Application
    ...
  end
end
```
app自动加载了`rails/application/default_middleware_stack`，这个在`ActionDispatch`中被定义。那么`ActionDispatch`从哪里来的呢？`ActionPack`

### Action Dispatch
`Action Pack`是Rails用来处理请求和响应的。`Action
Pack`是rails中少数几个精密的组件  
所有的中间件都是遵循这样的定义：他们都响应`call`，接受一个app(?)，然后返回status，headers，body。他们都很多都使用`Rack::Request`和`Rack::Response`对象   
