# 修剪区块链数据

*阅读其它语言版本: [English](../pruning.md), [Korean](pruning_KR.md).*

Mimblewimble 的主要吸引力之一是其理论空间效率。确实，一个受信任或预先验证的完整区块链状态仅需要未花费的交易输出，它可以非常小。

why 的区块链包括以下类型的数据（我们假设对 Mimblewimble 协议有事先了解）：

1. 交易输出，其中每个输出包括:
    1. 一个 Pedersen 承诺（33 个字节）。
    1. 范围证明（目前超过 5KB）。
1. 交易输入，仅作为输出的引用（32 字节）。
1. 交易“证明”，每笔交易包括：
    1. 交易的超额承诺总和（33 个字节）。
    1. 超额签名生成（平均 71 字节）。
1. 一个块头包括 Merkle trees 和工作量证明（约 250 个字节）。

假设有一个有一百万个区块区块链，一千万笔交易（平均 2 个输入，2.5 个输出）和 100,000 个未花费的输出，我们得到以下近似大小以的一条完整的链（无修剪（no pruning），无核销（no cut-through））：

* 128GB的 交易数据（输入和输出）。
* 1 GB 的交易证明数据。
* 250MB 的块头。
* 总链大小约为 130GB。
* 核销后的总链大小为 1.8GB（但包含块头）。
* UTXO 大小为 520MB。
* 总链大小，不包含 4GB 的范围证明。
* UTXO 大小，不包含 3.3MB 的范围证明。

我们注意到，在所有数据中，一旦对链进行了充分验证，对于节点正常运行，仅严格要求一组 UTXO 承诺即可。

在某些情况下可以修剪数据：

* 完全验证的节点可能会删除一些已经验证过的数据以释放空间。
* 部分验证的节点（类似于 SPV）可能并不会对接收或保留所有数据感兴趣。
* 当一个新节点加入网络时，即使最终是要成为一个完全验证的节点，它也可能暂时充当部分验证的节点，使其可以更快地使用。

## 验证完全修剪后的状态

修剪需要删除尽可能多的数据，但同时保留完整的 Mimblewimble-style 验证的所有保证。
为了保持正在修剪的节点状态的正常运行，这是必需的，而且在第一次快速同步时（仅将最小量的数据发送到新节点）。

完整验证链状态要求：

* 所有内核签名均根据其公共密钥进行验证。
* 所有 UTXO 承诺的总和，减去供给即为有效的公共密钥（可用于对空字符串进行签名）。
* 所有内核发布密钥的总和等于所有 UTXO 承诺的总和减去供应。
* UTXO PMMR 的根哈希，范围证明的 PMMR 和内核的 MMR 将块头与有效的工作量证明链相匹配。
* 所有范围证明均有效。

另外，尽管不必验证整个链状态，但为了能够接受和验证新块，还需要其他数据：

*输出功能，使所有 UTXO 都需要完整的输出数据。

至少，需要以下数据：

* 块头链。
* 所有内核，按包含在链中的顺序。这也允许重建内核 MMR。
* 所有未花费的输出。
* UTXO MMR 和 范围证明 MMR（以了解修剪后的数据的哈希值）。

请注意，可以通过仅验证一个由验证节点随机选择的范围证明的子集来进行进一步修剪。
