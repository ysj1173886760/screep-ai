# TODO

## RCL自动升级

RCL升级有几个阶段：
* 刚开始只有一个Spawn
* Storage造好
* Terminal造好
* 最终成型

看了一下overmind的做法，会根据当前的rcl，overlord会做不同的决策。比如mine，会根据当前是否有container/link/construction site，做不同的动作。

overmind是怎么保证creep不会乱的？
* overlord在初始化的时候，根据当前的rcl来选择miner的配置，并发送请求给spawn
* creep会有不同的role，在生成之后，会作为idle creep出现，overlord会找idle creep中需要的角色，将其加入到自己的控制下。将自己的名字写入到creep的memory中，这样可以通过creep的memory确定当前creep是否有overlord控制，以及是那个overlord在控制
* 上面这个可以在spawn的时候加一个option，在spawn的时候初始化好这个creep的overlord即可
* 怎么去重，overmind貌似没有去重，就是每个tick每个overlord都发出请求，然后spawn根据优先级去生成creep。所以这个tick没有生成就下个tick再去请求。所以处理spawn的逻辑要后于处理overlord的逻辑

