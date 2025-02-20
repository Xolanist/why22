Mimblewimble 和 Why 简介
=====================================

*阅读其它语言版本: [English](../intro.md), [Español](intro_ES.md), [Nederlands](intro_NL.md), [Русский](intro_RU.md), [日本語](intro_JP.md), [Deutsch](intro_DE.md), [Portuguese](intro_PT-BR.md), [Korean](intro_KR.md).*

Mimblewimble是一个区块链格式和协议，依托于健壮的加密原语，提供非常好的可扩展性、隐私和可替代性。它解决了当前几乎所有实现的区块链（与现实需求之间）差距。Mimblewimble 的白皮书在[本项目的WiKi](https://github.com/mimblewimble/docs/wiki/A-Brief-History-of-MinbleWimble-White-Paper)中可以找到，WiKi是开放的。

Why是一个实现Mimblewimble区块链的开源软件项目，并填补了（Mimblewimble协议所缺失的）实现一个完整的区块链和加密货币必需的一些东西。

Why 项目的主要目的和特性如下:

* 隐私保护的缺省特性。 这使它具备了完全可替代性，且保留了按需选择性披露信息的能力。
* 区块大小与交易量相适配，历史交易仅保留约100字节的交易核（_transaction kernel_）, 相比其它区块链节省了大量空间。
* 强大且经过验证的密码学。 Mimblewimble只采用椭圆曲线密码，该密码技术已经过了数十年的试用和测试。
* 简单的设计使得日后的代码审查和维护变得容易。
* 社区驱动。采用一种抗拒ASIC的挖矿算法(Cuckoo Cycle算法)，借此来鼓励去中心化的挖矿。

# Tongue Tying for Everyone

**备注**：Mimblewimble 出自《哈利波特》中的一句咒语，详见：[Tongue-Tying Curse](http://harrypotter.wikia.com/wiki/Tongue-Tying_Curse)，这个标题的涵义应该是希望所有读到这篇介绍的人都可以来为这个开放社区做点贡献，真心希望如此。

本文针对的读者是已经了解过区块链并了解一些基本的密码学知识的人群。我们尝试解释Mimblewimble的技术构建，以及它如何应用于Why。我们希望这篇介绍能够浅显易懂，我们的目的是鼓励您对Why产生兴趣，并加入Why的开放社区，以任何您可能的方式对其做出贡献。

为了实现这个目标，我们将介绍一个主要概念：Why是一个Mimblewimble实现。我们将从椭圆曲线密码（ECC）的简短描述开始，这是Why的重要基础。然后描述Mimblewimble区块链交易和区块的所有关键要素。


## 椭圆曲线简介

我们首先简要介绍一下椭圆曲线密码学（后面简称为：ECC），只是简单说明一下理解Mimblewimble如何工作所必需了解的ECC属性，这里并不深入研究和讨论ECC。对于想要更多一点了解ECC的读者，可以参考这个介绍：
[了解更多](http://andrea.corbellini.name/2015/05/17/elliptic-curve-cryptography-a-gentle-introduction/).

用于密码学目的的椭圆曲线只是一大组我们称之为 _C_ 的点。这些点可以被加、减或乘以整数（也称为标量）。 给定一个整数 _k_ 并使用标量乘法运算，我们可以计算`k * H`，这也是曲线 _C_ 上的一个点。
给定另一个整数 _j_，我们也可以计算`（k + j）* H`，它等于`k * H + j * H`。 椭圆曲线上的加法和标量乘法运算保持加法和乘法的交换律和结合律：

    (k+j)*H = k*H + j*H

在ECC中，如果我们选择一个非常大的数字 _k_ 作为私钥，则`k * H`被作为相应的公钥。 即使人们知道公钥`k * H`的值，推导 _k_ 几乎不可能（或者换句话说，椭圆曲线点的乘法计算是微不足道的，然而曲线点的“除法”计算却极其困难。参见：[椭圆曲线密码学](https://zh.wikipedia.org/wiki/椭圆曲线密码学)。

先前的公式`（k + j）* H = k * H + j * H`中， _k_ 和 _j_ 都是私钥，演示了从两个私钥的加和获取公钥`（k + j）* H`，等价于每个私钥的对应公钥加和（`k * H + j * H`）。在比特币区块链中，[分层确定性钱包(HD Wallets/BIP32)](https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki)严重依赖于这个原则。 Mimblewimble和Why也是如此。

## Mimblewimble 交易

交易结构的设计显示了Mimblewimble的一个关键原则：强大的隐私性和保密性。

Mimblewimble的交易确认依赖于两个基本属性:

* **0和验证。** 输出总和减去输入总是等于零，证明交易没有凭空创造新的资金，而且**不会显示实际金额**。
* **拥有私钥即拥有交易输出的所有权。** 像大多数其他加密货币一样，交易输出通过拥有ECC私钥来保证其所有权。 然而，在Mimblewimble中，证明一个所有者拥有这些私钥并不是通过直接签署交易来实现的。

下面介绍账户余额、所有权、变更和证明，并借此说明上面的这两个基本属性是如何得以实现的。

### 等式平衡

基于上面描述的ECC的属性，可以在交易数据中掩盖实际交易值。

如果 _v_ 是交易输入或输出的值，而 _H_ 是椭圆曲线，我们可以简单地在交易中嵌入`v * H`而不是 _v_。 这是因为使用ECC操作，我们仍然可以验证交易的输出总和等于输入总和：

    v1 + v2 = v3  =>  v1*H + v2*H = v3*H

验证每笔交易的这个属性允许协议验证交易不会凭空创造出金钱，而无需了解实际的交易值是多少。但是，可用数值是有限的，攻击者可以尝试每一个可能的数值来猜测你的交易值。 另外，知道v1（来自
上面的交易示例）和`v1 * H`，就等于在整个区块链中揭露了等于v1的交易。 出于这些原因，我们引入了第二个椭圆曲线 _G_（实际上 _G_ 只是与 _H_ 相同的曲线组上的另一个发生器点）和私钥 _r_ 用作*致盲因子*。

交易中的输入或输出值可以表示为：

    r*G + v*H

其中：

* _r_ 是一个私钥，用作致盲因子， _G_ 是一个椭圆曲线点，他们的乘积 `r*G` 是 _r_ 在 _G_ 上的公钥。
* _v_ 是输入或输出值，_H_ 是另一个椭圆曲线点。

无论是 _v_ 还是 _r_ 都不能被推导出来，从而利用了椭圆曲线密码学的基本属性。 `r * G + v * H`被称为 _Pedersen Commitment_ 。

作为一个例子，我们假设我们想用两个输入和一个输出创建一笔交易。 我们有（忽略费用）：

* vi1 和 vi2 作为输入值
* vo3 作为输出值

满足：

    vi1 + vi2 = vo3

为每个输入值生成一个私钥作为致盲因子，将上面的等式替换每个值为他们各自的 Pedersen Commitments，我们获得：

    (ri1*G + vi1*H) + (ri2*G + vi2*H) = (ro3*G + vo3*H)

并且要求:

    ri1 + ri2 = ro3

这是Mimblewimble的第一个支柱：验证交易的算术运算可以在完全不知道任何实际交易值的情况下完成。

补充最后一点说明，这个想法实际上派生自Greg Maxwell的[机密交易](https://elementsproject.org/features/confidential-transactions/investigation)，机密交易本身是从Adam Back提出的用于比特币的同态值提议中发展而来。

### 所有权

在前面的章节中，我们介绍了一个私钥作为致盲因子来掩盖实际交易值。Mimblewimble的第二个见解就是这个私钥可以用来证明值的所有权。

Alice 给你发了3个币并且隐藏了这个数字，你选择了28作为你的致盲因子（请注意，在实践中，致盲因子是一个私钥，是一个非常大的数字）。 区块链上的某处显示以下交易输出，并只能由你来用（做交易输入）：

    X = 28*G + 3*H

_X_, 上述加法的输出值，是对所有人可见的。 但是值3只有你和 Alice 知道，而28就只有你自己知道了。

为了再次转移这3个币，协议要求（交易者）以某种方式知道28。
为了演示这是如何工作的，假设你想将这3个相同的币转移给Carol。
您需要构建一个简单的交易，以便：

    Xi => Y

其中 _Xi_ 是一个输入，它花掉你之前得到的输出值 _X_ ，而 Y 是 Carol 的输出。如果不知道你的私钥28，就没有办法建立这笔交易。的确，如果Carol要平衡这个交易，她既需要知道发送的值，也需要知道你的私钥，
以便：

    Y - Xi = (28*G + 3*H) - (28*G + 3*H) = 0*G + 0*H

通过检查一切已被清零，我们可以再次确认没有创造新的金钱。

等等！ 停一下！ 现在你知道了 Carol的输出中的私钥（在上面的情况下，它必须与你的相同，为了让等式两边平衡），所以你可以把钱从Carol那里偷回来！

为了解决这个问题，我们允许Carol增加她选择的另一个值。 113，最后在区块链上的结果变成了：

    Y - Xi = (113*G + 3*H) - (28*G + 3*H) = 85*G + 0*H

现在交易不会再归零了，我们在_G_上有一个 _excess value_（85），这是所有致盲因子总和的结果。 但是因为`85 * G`是椭圆曲线 _C_ 上的有效公钥，85，
对于任何x和y，只有`y = 0`是 _G_ 上的`x * G + y * H`有效公钥。

因此，协议需要验证的其实就是：（`Y - Xi`）是_G_上的一个有效公钥，以及交易者知道私钥（我们与Carol的交易中的85）。最简单的方法就是要求使用excess value（85）进行签名，然后验证：

* 交易者知道这个交易输出的私钥
* 交易输出的和，减去输入，加起来等于0

这个关联到每笔交易的签名，附加一些额外数据（比如交易费），被称为交易核（_transaction kernel_）。

### 一些更深入的细节

本节阐述创建交易，通过讨论交易的找零机制和范围证明的要求以便所有值都被证明为非负。 这些都不是了解Mimblewimble和Why的必需内容，所以如果你想快速了解，随时可以直接跳过本节内容，直接到[Putting It All Together](#transaction-conclusion).

#### 找零

在上面的例子中，你必须分享你的私人密钥（致盲因子）给Carol。 一般来说，即使私钥永远不会被重用，这也不是一个十分可取的方法。 实际上，这不是问题，因为交易包括找零输出。

比方说，你只想从你收到的来自Alice的3个币里送出2个币给Carol。你简单地生成另一个私钥（比如12）作为一个致盲因子来保护你的找零输出，并告诉Carol 你正在发送2个币给她。Carol像以前一样使用自己的私钥：

    Your change output:  12*G + 1*H
    Carol's output:      113*G + 2*H

最终，链中发生的交易基本上就是上述这种过程。签名使用_excess value_，例如这个例子当中就是97。

    (12*G + 1*H) + (113*G + 2*H) - (28*G + 3*H) = 97*G + 0*H


#### 范围证明（Range Proofs）

在所有上述计算中，我们都依赖交易值始终为正值。如果可能的话，引入负值将是非常有问题的，由于可以在每笔交易中凭空捏造新的金钱。

例如，可以创建一个输入为2并且输出为5和-3的交易，并且依照前面章节中的定义仍然可以获得平衡的事务。 这是不容易被检测到的，因为即使x是负数，ECDSA曲线上的对应点x.H看起来也是任何值。

为了解决这个问题，Mimblewimble利用了另一个加密概念（也来自机密交易），称为范围证明：一个数字落在给定范围内的证明，而不会泄露数字。
我们不会详细说明范围证明，您只需要知道，对于任何`r.G + v.H`，我们都可以创建一个证明，证明 _v_ 大于零且不会溢出。

同样重要的是要注意，为了从上面的示例中创建有效的范围证明，必须知道在创建和签署excess value时使用的值113和28。 其原因以及范围证明的更详细描述在[range proof paper](https://eprint.iacr.org/2017/1066.pdf)中进一步详述。

<a name="transaction-conclusion"></a>
### 小结

Mimblewimble交易包括以下内容：

* 一组输入，参考和花费一组以前的输出。
* 一组新的输出包括：
  * 一个值和一个致盲因子（它只是一个新的私钥）在曲线上相乘并相加为r.G + v.H.
  * 范围证明显示v是非负的。
* 明确的交易费用。
* 一个签名，通过采取excess value（所有输出加费用之和减去输入）并将其用作私钥来计算。

## 区块状态和链状态

我们已经在上面解释了Mimblewimble交易如何在保持有效区块链所需的属性的同时提供强大的匿名性保证，即交易不会凭空捏造出货币，并且通过私钥建立所有权证明。

Mimblewimble区块格式通过引入一个附加概念来构建：核销（_cut-through_）。 有了这个补充，一个Mimblewimble链可获得：

* 极大的可扩展性，因为绝大部分交易数据主体可以随时间消除，而不会影响安全性。
* 通过混合和删除交易数据进一步匿名。
* 新节点能够非常高效地与网络其余部分同步。


### 交易聚合（_Transaction Aggregation_）

回顾一下一笔交易的组成：
* 一系列交易输入，用来引用并花掉一系列以前的交易输出
* 一系列新的交易输出（Pedersen commitments）
* 一个交易核，包含：
	* kernel excess，用来确保等式平衡
	* 交易签名（采用kernel excess作为签名公钥）

例如：

    (42*G + 1*H) + (99*G + 2*H) - (113*G + 3*H) = 28*G + 0*H

这个例子中使用的签名公钥是 `28*G`。

任何一笔交易必须满足以下条件： （为了描述简便，这里忽略掉交易费部分）

	sum(outputs) - sum(inputs) = kernel_excess

这个条件同样适用于区块，因为区块只是一系列聚合的交易输入、交易输出和交易核。我们可以把所有的交易输出加起来，减去所有的交易输入，将结果与所有交易核中的kernel excess之和做比较：

	sum(outputs) - sum(inputs) = sum(kernel_excess)

简单来说，（依然忽略交易费部分）我们可以认为，对Mimblewimble区块的处理方法和对Mimblewimble交易的处理方法是严格一致的。

#### 交易核偏移因子（Kernel Offsets）

上面描述的Mimblewimble区块和交易设计有一个小问题，有可能从一个区块中的数据来重建交易（即找出一笔或几笔完整的交易，分辨哪一笔交易输入对应哪一笔交易输出）。这个对于隐私而言当然是不好的事情。这个问题也被称为子集问题（"subset" problem） - 给定一系列交易输入、交易输出和交易核，有可能能够从中分辨出一个子集来重新拼出对应的完整的交易（很像拼图游戏）。

例如，假如有下面的两笔交易：

	(in1, in2) -> (out1), (kern1)
	(in3) -> (out2), (kern2)

我们能够聚合它们并构建下面的区块（或一笔聚合交易（_aggregate transaction_））：

	(in1, in2, in3) -> (out1, out2), (kern1, kern2)

很容易利用等式平衡关系用穷举法试验所有可能的组合，从而找出原始的交易关系：

	(in1, in2) -> (out1), (kern1)

只要找出了一笔交易，那么剩下的当然也是符合等式平衡关系的，于是很容易就拼凑出另一笔交易：

	(in3) -> (out2), (kern2)

为了大幅降低这个拼凑的可能性，从而缓解这个问题的不利影响，我们设计一个交易核偏移因子（_kernel offset_）给每一个交易核。 这也是一个致盲因子（或者说一个私钥），它需要加到kernel excess当中用于验证等式平衡关系：

	sum(outputs) - sum(inputs) = kernel_excess + kernel_offset

当我们聚合这些交易到区块的时候，我们在区块头中存储一个（且仅一个）聚合偏移因子（aggregate offset）（即所有交易核偏移因子的总和）。这样一来，因为我们一个区块只有一个偏移因子，再也不可能将其分拆对应到每一笔交易的交易核偏移因子了，从而也就不可能再从区块中拼凑出任何一笔交易了。

	sum(outputs) - sum(inputs) = sum(kernel_excess) + kernel_offset

具体的实现方法就是，在创建交易时将 `k` 分割成 `k1+k2`。 对于交易核 `(k1+k2)*G`，我们在交易核中发布出去的是 `k1*G` （称之为：the excess），以及 `k2`（称为：the offset），并跟以前一样使用 `k1*G` 作为公钥来对交易进行签名。
在矿工构建区块的时候，我们对打包的所有交易的`k2`（the offset）求和，以生成一个单个的聚合值（aggregate `k2` offset）用于该区块所打包的所有交易。一旦区块打包完成并发布和被链所接受，其原始的对应每笔交易的`k2` （the offset）即成为不可恢复的。

### 核销（_Cut-through_）

区块让矿工将多个交易组合成一个单个集合添加到链中。 在下面的区块表示中，包含3个交易，我们只显示交易的输入和输出。 输入关联其花费的输出。 前一个区块中包含的输出标记为小写字母x。

    I1(x1) --- O1
            |- O2

    I2(x2) --- O3
    I3(O2) -|

    I4(O3) --- O4
            |- O5

我们注意到以下两个属性：

* 在这个区块内，一些输出直接被包含的输入消耗（I3花费O2并且I4花费O3）。
* 每笔交易的结构并不重要。 由于所有的单个交易均归于零，因此所有交易输入和输出的总和也必须为零。

与单个交易类似，所有需要在一个区块中进行检查的是所有权已经被证实（来自交易内核 _transaction kernels_），并且整个区块没有增加任何货币供应（除了coinbase所允许的之外）。
因此，匹配输入和输出可以被消除，因为它们对总和的贡献被抵消了。 这导致了以下更紧凑的块：

    I1(x1) | O1
    I2(x2) | O4
           | O5

请注意，所有的交易结构已被消除，输入和输出的顺序已不再成问题。 但是，该块中所有输出的总和减去输入，仍然保证为零。

一个块的建立来自：

* 块头。
* 核销（_cut-through_） 后剩余的输入列表。
* 核销（_cut-through_） 后剩余的输出列表。
* 每个交易的交易核(transaction kernels)包含：
  * 从所有commitments总和中获得的公钥`r * G`。
  * 使用excess value生成的签名。
  * 挖矿费用 (fee)。

当区块以这种方式构建时，Mimblewimble区块提供了非常好的隐私保证：

* 更多的交易可能已经完成，但不会显式出现（在区块中）。
* 所有的输出看起来都是一样的：只是一些非常大的数字，不可能相互区分。 如果有人想排除某些输出，他们将不得不排除所有输出。
* 所有的交易结构已被删除，使得区分哪个输出与哪个输入匹配成为不可能任务。

然而，区块仍然可验证！

### 尽可能多地核销（_Cut-through_）

回到前面的示例块，I1和I2花费的输出x1和x2必须先前出现在区块链中。因此，在添加此区块后，这些输出以及I1和I2也可以从整体链中移除，因为它们不会影响整体总和。

总而言之，我们得出结论：任何时间点的链状态（不包括区块头）都可以通过这些信息来概括：

1. 链中采矿产生的代币总量。
1. 未使用的交易输出(即UTXO)的完整集合。
1. 每笔交易的交易内核。

第一条信息可以使用块高度（与起始块的距离）推导出来。未使用的输出和交易内核都非常紧凑。这有两个重要的后果：

* Mimblewimble区块链中给定的节点需要维护的状态非常小（对于比特币大小的区块链，几个G字节大小的数量级，可能优化到几百兆字节）。
* 当新节点加入构建Mimblewimble链的网络时，需要传输的信息量也非常小。

另外，未使用的交易输出(即UTXO)组成的完整集是不可篡改的，即使只是想去添加或删除一些交易输出。这样做会导致交易内核中所有致盲因因子的总和与输出中致盲因素的总和不同。

## 结论

在本文中，我们介绍了基于Mimblewimble区块链的基本原则。 通过使用椭圆曲线密码的附加属性，我们能够构建完全不透明但仍可以正确验证的交易。
通过应用这些属性，我们可以消除大量区块链数据，从而实现新对等点的大规模部署和快速同步。
