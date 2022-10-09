## 声明式的有限元预处理

做有限元的时候, 你觉得是像`系纽扣`呢, 还是像`拉拉链`?

拉链拉到后面, 前面开了, 怎么办呢?

## 不用🥺

软件目前仅支持LS-DYNA

windows下我只会用WSL(linux子系统)

需要Rust工具链

## 要用🙇

[配置环境](https://github.com/AndyJado/chitto/blob/d73ffdf4cd03fbbbcca38c32b4784939c41a4d32/terminal-stf/src/oh-WSL.md)

```sh
git clone https://github.com/AndyJado/duckbubble
cd duckbubble
cargo install --path duckbubble
```

[一个不太友好的视频介绍](https://www.bilibili.com/video/BV1QR4y1R76t/)

---

#### Goals:

- 降低实际生产中有限元计算所需的时间成本

- 返回程序错误,而不是计算的错误结果,保证计算安全

- 支持主流商业软件

- 人的意图与商业软件之间的一位门童

#### NonGoals:

- 不帮你建模, 但在你建模之前提供指导

- 不帮你计算, 但帮你生成计算文件

---

# License

GPL-2.0
