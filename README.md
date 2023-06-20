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
  - [ ] 动态移除故障节点v0.1.1
- [ ] 用户集群:v0.4
  - [ ] API:v0.2
  - [ ] 用户请求日志:v0.3
  - [ ] 主逻辑完成:v0.4
- [ ] 计算集群:v1.0
  - [ ] API:v0.6
  - [ ] Slot手动迁移:v0.7
  - [ ] 主逻辑完成:v1.0

### Features

- [ ] 集群快照动态更新.
- [ ] slot保证连续以减小网络交互.
- [ ] slot集群动态负载.
- [ ] 计算节点主从切换.
