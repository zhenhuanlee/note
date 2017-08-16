### [Swarm mode key concepts](https://docs.docker.com/engine/swarm/key-concepts/)

#### What is a swarm
swarm是一个部署服务的Docker engines或nodes的集群，Docker的命令行或者API包含了管理swarm nodes的命令  
如果以swarm模式运行Docker，你可以把服务协调的管理起来  

#### What is node
node是swarm中众多Docker engine中的一个实例，也可以把它当做一个docker节点，可以在一个物理机或云服务器上运行一个或多个nodes，但是在生产环境中，Docker nodes分布在多个物理和云主机上  
部署程序到一个swarm中，你需要din定义一个manager node。这个node分发工作单元(tasks)去worker nodes  
Manager nodes也提供管理desired state的swarm集群管理方法，Manager nodes只选择一个leader领导orchestration tasks  
`Worker nodes`接收执行`manager nodes`分发的tasks。默认的，`manager nodes`也向worker nodes一样运行服务，但是可以通过设置，使得它只管理节点。每个worker node上都会运行一个代理，用来报告分配给它的tasks。worker node会通知manager node关于所分配tasks的当前状态，所以manager 可以管理每一个worker的desired state  

#### Services and tasks
`service`的定义是：manger或worker nodes上的执行的tasks。它是swarm系统的中心结构也是用户和swarm交互的primary root  
当创建了一个service，你就指定了所使用的container image和在容器中锁执行的命令  
在replicated services模型中，在desired state下，swarm manager基于规模给nodes之间的复制的tasks分配了一个特殊的数字  
对于global services，swarm为集群中所有可用的node运行了一个task  
一个`task`搭载了一个Docker container和container中运行的命令。它是swarm中的原子单元。Manager nodes向worker node分配tasks基于service scale中锁复制的数字。一旦task被分配到一个节点，它就不会被分配到其他的node  
