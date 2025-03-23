## 安装插件 (https://www.nuget.org/packages/MySql.Data)  
`dotnet add package MySql.Data --version 9.2.0`

## [migration](http://stackoverflow.com/questions/45382536/how-to-enable-migrations-in-visual-studio-for-mac)
- dotnet tool install --global dotnet-ef  
- add tools to path  
- run command  
`dotnet ef migrations add initial`  
`dotnet ef database update`
