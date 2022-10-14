## 声明式的有限元预处理

做有限元的时候, 你觉得是像`系纽扣`呢, 还是像`拉拉链`呢?

拉链拉到后面, 前面开了, 怎么办?

## 我用得上吗?

如果你使用LS-DYNA, 那么你就用得上, 据我所知, 还没有别的软件做这个事情.

[walk through_`2min video`](https://www.bilibili.com/video/BV1QR4y1R76t/)

## 不用🥺

软件目前仅支持LS-DYNA

仅测试了windows下配合WSL(LINUX子系统)使用

安装需要Rust工具链

还没写文档

## 要用🙇

[配置环境](https://github.com/AndyJado/chitto/blob/d73ffdf4cd03fbbbcca38c32b4784939c41a4d32/terminal-stf/src/oh-WSL.md)

配置好之后

回到终端

国内的话

```sh
git clone https://gitee.com/AndyJado/duckbubble.git
cd duckbubble
cargo install --path=.
```

---

#### Goals:

- 降低实际生产中有限元计算所需的时间成本

- 返回程序错误,而不是计算的错误结果,保证计算安全

- 支持主流商业软件

- 人的意图与商业软件之间的一位门童

- 开箱即用

#### NonGoals:

- 不帮你建模, 但在你建模之前提供指导

- 不帮你计算, 但帮你生成计算文件

---

# License

GPL-2.0
