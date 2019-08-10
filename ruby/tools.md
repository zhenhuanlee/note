### Benchmark 测试性能
```ruby
require 'benchmark'

Benchmark.bm do |x|
  x.report('hello: ') { 100.times { prints 'world' } }
end
```
结果会显示用户CPU时间，系统CPU时间，总和时间，和真实时间
