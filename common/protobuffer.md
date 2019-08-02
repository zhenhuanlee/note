# [protobuffer](https://langzi989.github.io/2017/05/26/protobuffer%E5%AD%A6%E4%B9%A0%E6%80%BB%E7%BB%93/)
> 序列化和反序列化的一种数据描述语言(可类比json)，是二进制的，体积小，解析快  

#### 语法  
```
Package example

message person{
  required string name = 1;
  required int32 id = 2;
  optional string email = 3;

  enum PhoneType{
    mobile = 1;
    home = 2;
    work = 3;
  }

  message PhoneNumber{
    required string number = 1;
    optional PhoneType type = 2;
  }

  repeated PhoneNumber phone = 4;
}
```

#### 规则  
protobuffer中字段规则包含以下三种：  
- required: 实例中必须包含的字段  
- optional: 实例中可以选择性包含的字段，若实例没有指定，则为默认，若没有设置该字段的默认值，其值是该类型的默认值。如string则默认为""，bool则默认为false，整数默认为0  
- repeated: 可以有多个值得字段，类似于vector，可以存储此类型的多个值  
> 由于一些历史原因，几本书之类型的repeated字段并没有被尽可能的高效编码，在新的代码中，应该使用特殊选项\[packed=true\]来保证高效的编码  
> 一般情况小慎用required字段，当此字段一定是必须的时候才使用  

##### repeated使用实例  
```c++
message Person {
  required int32 age = 1;
  required string name = 2;
}

message Family {
  repeated Person person = 1;
}
```  
```
int main(int argc, char* argv[])
{
  GOOGLE_PROTOBUF_VERIFY_VERSION;

  Family family;
  Person* person;

  // 添加一个家庭成员，John  
  person = family.add_person();
  person->set_age(25);
  person->set_name('John')

  // 添加一个家庭成员，Lucy  
  person = family.add_person();  
  person->set_age(23);  
  person->set_name("Lucy");  

  // 添加一个家庭成员，Tony  
  person = family.add_person();  
  person->set_age(2);  
  person->set_name("Tony");  

  // 显示所有家庭成员  
  int size = family.person_size();  


  cout << "这个家庭有 " << size << " 个成员，如下：" << endl;  

  for(int i=0; i<size; i++)  
  {  
      Person psn = family.person(i);  
      cout << i+1 << ". " << psn.name() << ", 年龄 " << psn.age() << endl;  
  }  

  getchar();  
  return 0;  
}
```

