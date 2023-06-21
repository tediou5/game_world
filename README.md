# game_world

author: tedious

data: 2023-6-15

version: 0.0.0

## Road Map

- [x] 单机方案设计
- [x] 分布式方案设计
  - [x] 负载均衡
  - [x] 高可用
  - [x] 观测性
- [x] Raft元数据和集群构建:v0.1
  - [x] 动态移除故障节点v0.1.1
- [ ] 用户集群:v0.4
  - [ ] API:v0.2
  - [x] 用户请求日志:v0.3
  - [ ] 主逻辑完成:v0.4
- [ ] 计算集群:v1.0
  - [ ] API:v0.6
  - [x] Slot手动迁移:v0.7
  - [ ] 主逻辑完成:v1.0

### Todo List

- [ ] user集群Conhash分配user.
- [ ] 集群快照动态更新.
- [ ] slot保证连续以减小网络交互.
- [ ] slot集群动态负载.
- [ ] 计算节点主从切换.
- [ ] actor-pre-core架构优化单体性能.
- [ ] 根据slot计算query时仅外围slot需要重新计算, 否则必在范围内. 可优化单体性能.
- [ ] 每个step内的多个query可以合为一个, 降低服务器压力.
- [ ] 为ComputeRequest实现系列From方法, 问就是更rustly.
