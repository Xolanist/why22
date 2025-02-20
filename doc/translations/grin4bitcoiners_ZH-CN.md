
# Why/Mimblewimble 致比特币持有者

*阅读其他语言版本：  [English](../why4bitcoiners.md), [Korean](why4bitcoiners_KR.md).*

## 隐私和可互换性

Why 链上交易有三个隐私特性：

1. 没有交易地址；
2. 没有交易金额；
3. 两笔交易，一笔输入是另一笔输出，可以在一个区块中混合成为一笔交易，清除所有中间信息。

前两个特性意味着所有交易不可识别。只要直接进行交易，所有输入、输出数据都显示为随机数（专业术语就是“随机曲线点”）。

另外，单个区块中可以添加更多交易。Why 区块内数据就像是一笔大交易，原始交易所有输入和输出数据都被删除。

## 扩展性

如上所述，由于 Mimblewimble 交易和区块格式，交易可以合并，一笔输出可以从另一笔的输入直接花费。例如，甲给乙转账，乙之后转账给丙。这一系列交易中可以去除乙的数据，乙的交易数据绝对不会在链上显示。

详细来说，区块间多数交易输出迟早都会变成另笔交易的输入。因此，*所有输出花费都可以安全删除*。而且假设 Why 交易量与比特币相当，仅需几个 GB 或更少容量即可存储、下载和验证完整区块。

这就意味着 Why 区块链可扩展用户数量，而不是交易数量。目前有个问题是：每笔交易需要保留小量数据（“内核”，大约 100 字节）。但开发团队在努力优化。

## 脚本


或许你听说过 Mimblewimble 协议不支持脚本 (Script)。某种程度上这是事实。但利用密码学方法，许多需要脚本的比特币合约在 Why 上可以使用椭圆曲线密码学 (Elliptic Curve Cryptography) 实现。迄今为止已知的方法有：

* 多签交易
* 原子交换
* 时间锁定交易和输出
* 闪电网络

## 发行率

比特币出块时间为 10 分钟，初始每个块奖励 50 btc，每四年减半，直到 2100 万比特币全部挖出。Why 的货币发行率为线性增长，也就是说不会降低。目前为每 60 秒出块，每个区块奖励 60 Why。这种发行方也有效，因为 1）稀释率逐渐为零；2）每年丢失或销毁的币数量也不小。

## 常见问答集

### 什么？没有地址？

没有地址。Why 交易中所有输出是单独数据，与之前的输出不共享数据。不用已知地址发送货币，取而代之的是交互式交易，两个（或更多）钱包间彼此交换数据。这种交互方式不需要双方保持同时在线。实际上，有很多方式可以在两个程序间私密安全地进行交互。也可以使用电子邮件或 Signal（或“信鸽”）来进行此种交互通讯。

### 如果删除交易信息，是不是就能欺骗并造新币？

不可以，这就是 Mimblewimble 协议和 Why 的出众之处。机密交易是一种[同态加密](https://en.wikipedia.org/wiki/Homomorphic_encryption)形式。Why 不用公开交易金额，即可验证交易输入总额等于交易输出与交易费总和。也就是说，比较挖矿产生的货币总量与现有货币总量，Why 节点来检查货币总量是否准确。

### 如果监听交易中继，我是不是就能在交易核销前分析出持币人信息？

你可以分析出哪笔交易的输出花费，但也仅限这么多信息。所有输入与输出都是随机数据，因此你没法识别钱是不是被转走，是不是还是同一个人持有，哪笔输出是真正的转账，及哪笔是找零等等。Why 交易完成*没有可识别信息*。

另外，Why 利用[蒲公英中继](dandelion/dandelion.md)技术来隐藏交易 IP 地址和客户端，并允许汇集交易，从而提高匿名性。

### 出现量子计算机怎么办？

每笔 Why 的输出交易中，也包含哈希数据。这些数据抗量子计算。如果出现量子计算机，我们可以安全引入其他验证方式，来保护现有货币系统不会遭到黑客攻击。

### 所有魔法是怎么实现的？

详情请参阅[技术简介](intro.md)。
