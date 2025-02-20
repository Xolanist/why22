# Mimblewimble 과 Why 에 대한 소개

*다른 언어로 되어있는 문서를 읽으려면: [English](../intro.md), [Español](intro_ES.md), [Русский](intro.ru.md), [日本語](intro.jp.md), [简体中文](intro.zh-cn.md).*

MimbleWimlbe은 블록체인 포맷이면서 프로토콜 입니다.
Mimblewimble은 암호학적 기반에 의해서 극대화된 좋은 확장성, 프라이버시, 그리고 대체가능성을 제공합니다. 이러한 특성은 지금 현존하는 모든 블록체인 구현체에 존재하는 문제점들을 처리합니다.

Why 은 Mimble Wimble 블록체인을 구현한 오픈소스 프로젝트 입니다. 또한 완전한 블록체인와 크립토 커런시의 배포에 필요한 갭을 채워줍니다.
Why 프로젝트의 주요 목적과 특성들은 아래 설명을 참고하십시오.

* 프라이버시가 기본으로 제공됩니다. 이 기능은 필요에 따라서 선택적으로 정보를 공개 할 수 없도록 해서 완전한 대체가능성을 할 수 있게 합니다.
* 주로 유저의 규모와 최소한의 트랜잭션 수의 규모로 (100byte 미만의 kernel(transaction)) 다른 블록체인들과 비교하면 많은 저장공간을 절약할 수 있습니다.
* Mimble Wimble 은 수십년 동안 테스트하고 사용되었던 강력한 암호기술인 ECC만 사용합니다.
* 간단한 디자인은 감사와 유지보수를 시간이 지나도 수월하게 만듭니다.
* 커뮤니티가 주도하며, 채굴 탈중앙화가 권장됩니다.

## 모두의 혀를 묶자.
이 문서는 블록체인에 대해 어느정도 이해가 있고 암호학에 대한 기본적인 이해가 있는 독자들을 대상으로 합니다. 이것을 염두에 두고 우리는 Mimblewimble의 기술적인 발전과 어떻게 Why에 적용되었는지 관해 설명 할 것입니다.
저희는 이 문서가 대부분의 기술적인 성격을 가진 독자들을 이해시킬 수 있길 바랍니다. 우리의 목적은 독자가 Why에 대해 흥미를 느끼게 하고 어떤 방식으로든 Why에 기여할 수 있게 이끄는 것입니다.
이러한 목적을 이루기 위해, 우리는 Mimblewimble 의 구현체인 Why을 이해하는데 필요한 주요 컨셉들에 대해서 소개할것입니다.

 우선 Why이 어디에서 부터 기초로 하고 있는지에 대해 이해하기 위해서 타원 곡선 암호 (ECC)의 몇몇 속성들에 대한 간단한 설명으로 시작하겠습니다. 그 다음, Mimblewimble 블록체인의 트랜잭션과 블록에 한 모든 요소들을 설명하겠습니다.

### 타원곡선에 대한 조그마한 조각들
ECC의 너무 복잡한 사항을 캐지 않고 어떻게 mimble wimble 이 어떻게 작동하는지에 대해 이해하는데 필요한 요소들만 리뷰할 것입니다. 이런 가정들을 좀 더 알고싶은 독자들은 [이 링크](http://andrea.corbellini.name/2015/05/17/elliptic-curve-cryptography-a-gentle-introduction/)를 참고하세요.

암호학에서의 타원 곡선이란 우리가 _C_ 라고 부르는 단순히 아주 큰 좌표의 집합입니다.
이 좌표들은 정수들로 (인티저, 또는 스칼라 ) 더하고 빼고 곱할 수 있습니다.

주어진 정수 _K_ 에 스칼라 곱셈을 한다면 우리는 곡선 _c_ 위에 있는 좌표 K*H를 계산 할 수 있습니다.
또 달리 주어진 정수 _j_ 에 우리는`k*H + j*H` 와 같은 `(k+j)*H`를 계산 할 수 있습니다.

타원곡선 위에서의 덧셈과 정수 곱셈은 제시된 수의 순서에 관계없이 결과가 동일하다는 성질과 덧셈과 곱셈의 계산 순서와 관계없이 동일한 결과가 나온다는 성질을 가지고 있습니다.

    (k+j)*H = k*H + j*H

ECC 안에서 우리가 매우 큰 숫자인 _k_ 를 프라이빗 키로 가정할 때 `k*H` 는 해당하는 퍼블릭 키로 해당되어 집니다. 누군가 공개키인 `k*H`의 값을 알더라도 _k_ 를 추론해 내는것은 불가능에 가깝습니다. ( 달리 얘기하자면, 곱셉은 쉬우나 곡선 좌표에 의한 "나눗셈"은 정말 어렵습니다.  )

_k_ 와 _j_ 둘다 비밀키인 이전 공식 `(k+j)*H = k*H + j*H` 는 두개의 비밀키를 더해서 얻은 한 개의 공개키 (`(k+j)*H`) 와 각각 두개의 비밀키에 공개키를 더한것과 같습니다. Bitcoin blockchain에서도 HD 지갑은 이 원칙에 의존하고 있습니다. Mimblewimble 과 Why의 구현또한 마찬가지 입니다.

### Mimblewimble 함께 거래하기
트랜잭션의 구조는 Mimblewimble의 강력한 프라이버시와 비밀이 유지된다라고 하는 중요한 규칙을 나타냅니다.

Mimblewimble 트랜잭션의 확인은 두가지 기본적인 성격을 전제로 합니다.

* **제로섬의 검증:** 결과값에서 입력값을 뺸 합은 항상 0과 같습니다. 이것은 실제 전송되는 코인의 양을 드러내지 않고도 트랜잭션ㅇ이 새로운 코인을 만들지 않았다는 것을 증명합니다.
* **비밀키의 소유:** 다른 많은 크립토 커런시 들처럼 , 트랜잭션의 소유권은 ECC 비밀키에 의해 보장됩니다. 그러나 어떤 실체가 이런 비밀키들을 소유하고 있다고 증명하는것이 직접적으로 트랜잭션에 사인한다고해서 얻어지는 것은 아닙니다.

다음 섹션들에서는 잔고, 소유권, 거스름돈과 증명들의 상세들이 어떻게 저 두가지 기본적인 성질에 의해서 얻어지는지 알아보겠습니다.

#### 잔고
위에서 언급한 ECC의 특성들을 기반으로 해서 트랜잭션안의 가치들을 보기 어렵게 할 수 있습니다.
만약 _v_ 가 트랜잭션 입력값이거나 출력값이고 _H_ 가 타원곡선이라면 , 단순히 _v_ 대신 `v*H`를 끼워 넣을 수 있습니다.

이것은 ECC를 사용하기 때문에 작동하는 것입니다. 우리는 출력값의 합이 입력값의 합과 같다는 것을 여전히 확인할 수 있습니다.

    v1 + v2 = v3  =>  v1*H + v2*H = v3*H

이 특성을 모든 트랜잭션에 확인하는것은 프로토콜이 트랜잭션은 돈을 난데없이 만들지 않는다는 것을 실제 돈이 얼마나 있는지 알지 않아도 검증할 수 있게 합니다.
그러나 사용가능한 한정된 숫자가 있고 그 숫자 중 하나를 사용해서 당신의 트랜잭션이 얼마만큼의 코인을 가졌는지 추측 할 수 있습니다. 더해서, v1을 알고 ( 예시로 사용된 이전의 트랜잭션에서 온 값 ) 그에따른 `v1*H`의 결과를 알면 블록체인 전체에 걸쳐서 v1 값이 있는 모든 출력값들이 드러나게 됩니다.

이러한 이유로 두번째 타원곡선인 _G_ 를 제시합니다. ( 실제로 _G_ 는 _H_ 의 그룹과 같은 곡선에 있으며 단지 다른 좌표를 생성해 냅니다.) 그리고 비밀키 _r_ 은 *blinding factor* 로 사용됩니다.

그렇다면 트랜잭션 안의 입력값과 출력값은 다음과 같이 표현됩니다.

    r*G + v*H

여기서

* _r_ 은 비밀키이고 blinding factor 로 사용됩니다. _G_ 는 타원 곡선 이고 `r*G`는 _G_ 안에 있는 _r_ 의 공개키 입니다.
* _v_ 는 출력값이거나 입력값이고 _H_ 는 다른 타원곡선입니다.타원곡선의 근본적인 특성을 이용했기 때문에 _v_ 와 _r_ 은 추측될 수 없습니다.  `r*G + v*H`를 _Pedersen Commitment_ 라고 부릅니다.

예를 들어 , ( 전송료는 무시하고) 두개의 입력값과 한개의 출력값으로 트랜잭션을 만들기 원한다고 가정해봅시다.

* vi1 과 v2 는 출력값
* vo3는 출력값 이라면

그렇다면

    vi1 + vi2 = vo3

입니다.

각각의 입력값에 대해서 blining factor 로 비밀키를 만들고 각각의 값을 각각의 이전의 공식에 있던 Pederson Commitment로 교체한다고 하면 다음과 같습니다.

    (ri1*G + vi1*H) + (ri2*G + vi2*H) = (ro3*G + vo3*H)

결과로 다음과 같습니다.

    ri1 + ri2 = ro3

이것이 Mimblewimble의 첫번째 특징입니다. 트랜잭션을 검증하는 산술적인 연산은 아무런 값을 알지 못해도 가능합니다.

이 아이디어는 Greg Maxwell 의 [Confidential Transactions](https://elementsproject.org/features/confidential-transactions/investigation) 에서 유래했습니다. Confidential transaction은 Adam back의 비트코인에 동형암호를 적용하자는 제안에서 비롯되었습니다.

#### 소유권

이전의 섹션에서 트랜잭션의 값을 보기 어렵게 하는 Blinding factor로서 비밀키를 소개했습니다. Mimblewimble 의 두번째 통찰은 비밀키가 어떤 값의 소유권을 증명하는데 사용할 수 있다는 것입니다.

Alice는 당신에게 3 코인을 보내면서 그 양을 가렸고, 당신은 28을 당신의 blinding factor로 선택했습니다. ( 실제로 blinding factor는 비밀키로 정말 무진장 큰 숫자 입니다.)

블록체인 어딘가에 다음과 같은 출력값이 나타나 있고 당신에 의해서만 소비될 수 있습니다.

    X = 28*G + 3*H

_X_ 는 덧셈의 결과이면서 모두에게 다 보여집니다. 3은 당신과 Alice만 알고 있고 28은 당신만이 알고 있습니다.

다시 3코인을 보내기 위해선, 프로토콜은 어떻게든 28을 알고 있어야 됩니다. 어떻게 이것이 작동하는지 보기 위해서, 당신이 캐롤에게 같은 3코인을 보내고 싶어한다고 합시다. 그렇다면 당신은 아래와 같은 간단한 트랜잭션을 작성해야 합니다.

    Xi => Y

여기서 _Xi_는 _X_ 출력을 사용하는 입력이고 Y는 Carol의 출력입니다.
당신의 비밀키인 28을 모르고서는 트랜잭션과 잔액을 만들 수 있는 방법이 없습니다.

실제로 캐롤이 이 트랜잭션의 잔액을 위해선 그녀는 받는 값과 당신의 비밀키를 알아야 합니다.

그러므로

    Y - Xi = (28*G + 3*H) - (28*G + 3*H) = 0*G + 0*H

입니다.

모든계산이 0으로 되었는지 확인함으로써, 새로운 돈이 만들어지지 않았다는 것을 확인할 수 있습니다.
오 잠시만요! 당신은 지금 캐롤의 출력값에 비밀키가 있다는것을 알았습니다. ( 이런경우에는 당신의 잔액이 나간것과 동일 해야 합니다.) 그리고 당신은 캐롤로 부터 돈을 훔칠수 있습니다. 이걸 해결하기위해서 캐롤은 그녀가 선택한 비밀키를 사용합니다.

캐롤이 113을 비밀키로 선택했다면 블록체인 안에서는 아래와 같이 마무리 됩니다.

    Y - Xi = (113*G + 3*H) - (28*G + 3*H) = 85*G + 0*H

모든 blinding factor합계 결과로 타원곡선 _G_ 위에서 트랜잭션은 _초과값_ (85) 을 가지게 되고 트랜잭션의 합은 더이상 0 이 아닙니다.

그러나 `85*G` 은 비밀키 85 와 함께 타원곡선 _C_ 에서 유효한 공개키이기 때문에 모든 x와 y는 `y = 0`가 `x*G + y*H`일때 곡선 _G_ 에서 유효한 공개키입니다.

그러므로 모든 프로토콜은 (`Y - Xi`) 가 _G_위에서 유효한 공개키인지 ,거래당사자들이 비밀키를 알고있는지 ( 캐롤과의 트랜잭션에서는 85) 를 검증해야 할 필요가 있습니다. 가장 간단하게 검증하는 방법은 Signature가 초과값과 함께 만들어졌다는 것을 요구한 다음 아래와 같은것을 인증하는 겁니다.

* 거래하는 당사자들은 모두 비밀키를 알고 있고
* 트랜잭션의 입력값을 뺀 출력값들의 합은 0입니다. ( 왜냐하면 비밀키와 매칭된 유효한 공개키만 Signature 를 체크할 것이기 때문입니다. )

모든 트랜잭션에 포함된 이 Signature 는 덧붙여진 어떤 데이터와 함께(채굴 수수료와 같은 데이터) _transaction kernel_ 이라고 부르고 모든 Validator 에 의해 체크됩니다.

#### 몇몇 더 좋은 점들

이 섹션은 트랜잭션을 만들때 잔돈이 어떻게 보여지고 범위 증명(range proofs)의 요구사항에 대해서 모든 값이 음수가 아닌지에 대해서 좀 더 자세하게 설명하려고 합니다. 이러한 개념들 역시 Mimblewimble 과 Why 에 대한 이해가 당연히 필요합니다. 만약 당신이 조급하다면 [이 링크를 참고하세요.](#putting-it-all-together).

##### 잔돈에 대해서

캐롤에게 2개의 코인을 보내고 3개를 앨리스에게서 받는다고 해봅시다.이렇게 하려면 당신은 남은 1개의 코인을 잔돈으로 당신에게 돌려줘야 합니다. 이때, 다른 비밀키를 blinding factor 로 만들어서 (12라고 합시다.) 출력값을 보호해야 합니다. 캐롤은 이전에 썻던 그녀의 비밀키를 씁니다.

    잔돈의 출력값 :     12*G + 1*H
    캐롤의 출력값 :    113*G + 2*H

블록체인 안에서의 결과는 예전과 매우 흡사합니다. 그리고 Signature 은 초과되는 값과 함께 다시 만들어질겁니다. 이 예시에서는 97이라고 합시다.

    (12*G + 1*H) + (113*G + 2*H) - (28*G + 3*H) = 97*G + 0*H

##### Range Proofs

위의 모든 계산에서 트랜잭션의 값들은 항상 양의(+)값입니다. 음의 값은 모든 트랜잭션마다 새로운 돈을 만들수 있다는 것이므로 매우 문제점이 될겁니다.

예를 들어 입력값이 3이고 출력값이 5과 -3인 트랜잭션을 만들수 있으며 이것은 이전 섹션의 정의에 따라 잘 구성된 트랜잭션입니다. 적절한 좌표 `x.H`가 다른 좌표처럼 곡선위에 있어서 _x_가 음수이더라도 찾기가 쉽지 않습니다.

이 문제점을 해결하기 위해서, Mimblewimble 은 Range proofs 라는 다른 암호학 개념을 사용합니다. ( 이 또한 Confidential Transaction 에서 유래했습니다.)
Range proof 란 숫자를 밝히지 않고 어떤 숫자가 주어진 범위안에 있는지 증명하는 것입니다.
Range proof 에 대해서 자세히 설명하지 않을것이지만은, 그래도 어떤 `r.G + v.H` 의 결과가 _v_ 가 0보다 크고 오버플로우가 일어나지 않는다는 것을 증명할 수 있습니다. 또한 위의 예에서 유효한 Range proof 를 만들기 위해서 트랜잭션을 만들고 Signing 할때 사용된 초과값인 113과 28 두 값이 알려지는것은 중요합니다. 그 이유에 대해선 [range proof paper](https://eprint.iacr.org/2017/1066.pdf) 안에 Range proof에 대해 좀더 자세한 설명이 있습니다.

#### 모든것을 함깨 놓고 이해하기

MimbleWimlbe 트랜잭션은 다음을 포함합니다.

* 이전의 출력값들이 참조하고 사용한 입력값의 셋트들
* 새로운 출력값들은 다음을 포함합니다.
  * 곡선위에서 `r.G + v.H` 로 합해 지는 값 과 blinding factor (그냥 새로운 비밀 키).
  * v 가 음수가 아님을 보여주는 Range proof.
* 분명히 명시된 트랜잭션 수수료
* 수수료가 더해진 모든 출력밧에서 입력값을 뺸 초과 blinding 값이 계산되고 그것이 비밀키로 사용된 Signature.

### 블록들과 체인 state에 대해서

위에서 Mimblewimble 트랜잭션이 유요한 블록체인에 필요한 속성을 유지하면서 어떻게 강한 익명성을 보장하는지 설명했습니다.예를 들면 트랜잭션이 더이상 코인을 만들지 않으면서 비밀키를 통해 소유권을 증명하지 않는 방법들 같은것 말이죠.

추가적으로 _cut-through_ 라는 개념이 Mimblewimble 블록 포멧에 사용 됩니다. 이로 인해 Mimblewimble 체인은 아래와 같은 장점을 얻습니다.

* 대부분의 트랜잭션 데이터는 보안을 희생하지 않고서도 시간이 지나면 없어 질 수 있으므로 엄청나게 좋은 확장성을 얻게 됩니다.
* 트랜잭션 데이터를 섞고 없애서 익명성을 추가로 획득합니다.
* 새로운 노드가 네트웍에서 동기화를 이룰때 매우 효과적입니다.

#### 트랜잭션 합치기

트랜잭션은 다음와 같은것들로 이뤄져 있다는걸 상기해봅시다.

* 이전의 출력값들이 참조하고 사용한 입력값의 셋트들
* 새로운 출력값의 세트들 ( Pederson commitment)
* kernal execess와 (kernel 초과값이 공개키로 사용된) 트랜잭션 Signature로 이뤄진 트랜잭션 Kernel.

sign 된 트랜잭션과 Signature 은 _transaction kernel_ 에 포함됩니다.
Signature 공개키로서 트랜잭션의 합이 0임을 증명하는 _kernel excess_ 를 이용해서 생성됩니다.

    (42*G + 1*H) + (99*G + 2*H) - (113*G + 3*H) = 28*G + 0*H

이번 예시에서 공개키는 `28*G` 입니다.

다음은 어떠한 유효한 트랜잭션에서도 참이라고 말 할 수 있습니다. (단순함을 위해 수수료는 무시합니다. )

    출력값의 합 - 입력값의 합 = kernel_excess

블록이 입력값과 출력값의 합 그리고 트랜잭션 kernel들의 집합이면 블록도 마찬가지라고 할 수 있습니다. 트랜잭션의 출력값을 더할 수 있고 입력값의 합을 뺀다음 그 결과인 Perderson commitment 와 kernal excess와 비교합니다.

    출력값의 합 - 입력값의 합 = kernel_excess의 합

약간 단순화 시켜서 ( 트랜잭션 수수료를 무시하고) 우리는 MimbleWimbl block 이 Mimblewimble 트랜잭션들로 다뤄진다고 말 할 수 있습니다.

##### Kernel 오프셋들

위에 설명했던겉 처럼 Mimblewimble 블록과 트랜잭션에 조그마한 문제가 있습니다. 그것은 블록에 있는 구성 트랜잭션을 재구성하는것이 가능합다는 겁니다.(그리고 어떤 사소한 경우에도요).
이것은 분명히 프라이버시에는 좋지 않습니다. 이걸 "subset" 문제 라고 합니다.
"Subset" 문제란 주어진 입력값들, 출력값들과 트랜잭션 kernel들의 Subset 들이 재조합되어서 유효한 트랜잭션을 다시 만든다는 것입니다.

예를 들어 다음과 같이 두 트랜잭션이 있다고 해봅시다.

    (in1, in2) -> (out1), (kern1)
    (in3) -> (out2), (kern2)

다음과 같은 블록에 합칠 수 있을겁니다. ( 아니면 트랜잭션을 합쳐도 됩니다.)

    (in1, in2, in3) -> (out1, out2), (kern1, kern2)

(합계가 0일경우 ) 트랜잭션들 중 하나를 복구하기 위해서 가능한 모든 순열 조합을 조합해보는것은 쉽습니다.

    (in1, in2) -> (out1), (kern1)

또한 남은 트랜잭션이 다른 유효한 트랜잭션을 만드는데 사용되기도 합니다.

    (in3) -> (out2), (kern2)

이런것을 완화 시키기 위해 _kernel offset_ 이라는 것을 모든 트랜잭션 kernel 에 포함시킵니다.  실행 값이 0이라는 것을 증명하기 위해 kernel excess 에 더해져야 하는 blinding factor (비밀키)입니다.

    출력값의 합 - 입력값의 합 = kernel_excess + kernel 오프셋(offset)

블록 안에서 트랜잭션을 합칠때, _single_ 통합 오프셋(offset)을 블록 헤더에 저장합니다.
그래서 single 오프셋으로 인해 개별 트랜잭션 kernel offset 을 개별로 분리할 수 없고 트랜잭션 들은 더이상 재구성 될 수 없습니다.

    출력값의 합 - 입력값의 합 = kernel_excess의 합 + kernel_offset

키 `k`를 트랜잭션 구성 중에 `k1+k2` 안에 나누어서 넣었습니다. 트랜잭션 커널인`(k1+k2)*G` 에 대해 excess 인 `k1*G`와 오프셋(offset) 인 `k2`를 보여주고 이전처럼 `k1*G`로 트랜잭션에 sign 합니다.

블록을 만드는 동안 블록안의 모든 트랜잭션을 커버하기 위한 한개의 통합 `k` 오프셋을 만들기 위해 `k2`오프셋을 간단히 합할 수 있습니다. `k2`오프셋은 어떤 개별 트랜잭션이 복구되지 못하도록 합니다.

#### 컷 스루 (Cut-through)

블록들은 채굴자들이 여러 트랜잭션들을 하나의 세트에 넣고 체인에 더할수 있게 합니다.
다음 블록은 3개의 트랜잭션을 포함하고 있습니다. 오직 입력과 출력만을 보여줍니다.
소비한 출력값은 입력값을 참고 합니다. 출력값은 소문자 x로 표시된 이전 블록을 포함합니다.

    I1(x1) --- O1
            |- O2

    I2(x2) --- O3
    I3(O2) -|

    I4(O3) --- O4
            |- O5

다음과 같은 두가지 성질을 알려드리자면:
* 이 블록 안에서는 어떤 출력값은 포함된 입력값을 바로 사용합니다.(I3는 02를 소비하고 I4는 03을 소비합니다.)
* 실제로 각 트랜잭션의 구조는 문제가 아닙니다. 모든 트랜잭션들의 개개의 합계가 0이듯이 모든 트랜잭션의 입력값과 출력값이 0이여야만 합니다.

트랜잭션과 비슷하게 블록에서 체크해야 되는 것은 _transaction kernels_ 에서 비롯되는 소유권의 증명과 coinbase 에서 증가하는 코인 외 모든 블록이 돈의 공급을 추가하지 않았다는 것입니다.
매칭된 값은 전체의 값을 상쇄하므로 매치되는 입력값과 출력값은 없앨 수 있고 다음과 같이 좀 더 작은 블록이 됩니다.

    I1(x1) | O1
    I2(x2) | O4
           | O5

모든 트랜잭션 구조는 다 제거되었고 입력값과 출력값의 순서는 더이상 중요하지 않습니다.
그러나 블록에서 입력값을 뺸 모든 출력값의 합은 여전히 0임을 보증합니다.

블록은 아래와 간단히 말하자면 아래를 포함합니다.

* 블록헤더
* 컷 스루 이후 남은 입력값의 리스트
* 컷 스루 이후 남은 출력값의 리스트
* 모든 블록을 커버하기 위한 단일 kernel offset
* 트랜잭션 kernel들은 각 트랜잭션에 아래와 같은 것들을 포함합니다.
  * The public key `r*G` obtained from the summation of all the commitments.
  * 모든 커밋들의 합을 포함한 공개키 `r*G`
  * 초과값 (excess value) 을 이용해 생성된 Signature
  * 채굴 수수료

이런 방법으로 구조가 만들어진다면 MimbleWimblw 블록은 엄청나게 좋은 프라이버시를 보장할 수 있습니다.

* 중간 트랜잭션 ( 컷 스루 트랜잭션) 은 트랜잭션 kernel에서만 표시될것 입니다.
* 모든 출력값은 똑같이 보일것입니다. 출력값은 다른 것과 구분하기 블가능한 아주 큰 숫자일겁니다. 만약 하나를 다른 출력값에서 제외하려면 모든 출력값을 제외해야 합니다.
* 모든 트랜잭션 구조는 지워지고 출력값이 각 입력값과 매치된다고 말하기엔 불가능 해집니다.

그리고 아직까진 모든게 유효합니다!

#### 모두 컷 스루( Cut-through ) 하기

이전 예시의 블록으로 돌아가서 출력값인 x1,과 x2는 I1과 I2에 대해서 사용되고 이것은 반드시 이전 블록체인 안에서 나타나야 합니다. 이 블록이 추가된 이후에 I1과 I2 과 출력값들은 전체 합계에 영향을 주지 않으므로 모든 체인에서 지워 질 수 있습니다.

일반화 하자면, 헤더를 제외하고 어떤 시점에서든 체인의 스테이트는 다음과 같은 정보로 요약될 수 있습니다.


1. 체인안에서 채굴에 의해서 만들어진 코인의 총량
1. 쓰지 않은 출력값의 모든 세트
1. 각 트랜잭션의 트랜잭션 kernel

첫번째 정보는 Genesis 블록으로부터의 거리인 블록 높이를 가지고 유추 될 수 있습니다. 그리고 쓰지 않는 출력값과 트랜잭션 kernel은 매우 작습니다. 이것에는 아래와 같이 2가지 중요한 결과를 가지고 있습니다.

* Mimblewimble 블록체인에 있는 노드가 유지해야 되는 스테이트가 매우 작습니다.(비트코인 사이즈의 Blockchain의 경우 수 기가 바이트이고 잠재젹으로 수백 메가바이트까지 최적화 될 수 있습니다.)
* 새로운 노드가 Mimblewimble 체인에 가입히면, 전달해야 하는 정보의 양이 매우 적습니다.

덧붙여서 출력값을 더하거나 제거하더라도 쓰지 않는 출력값의 모든 세트를 조작할 순 없습니다. 그렇게 하면 트랜잭션 kernel 내의 모든 blinding factor의 합과 출력값 내의 blinding factor의 합이 달라집니다.

### 결론 내리기

이 문서에서는 Mimblewimble 블록체인 안의 기본적인 원리에 대해서 다루었습니다.
타원 곡선 암호의 다른 성질을 사용해서 알아보기 어려우나 적절하게 입증될 수 있는 트랜잭션을 만들수 있습니다. 블록에 이러한 성질들을 일반화 시키면 큰 용량의 블록체인 데이터를 없앨 수 있고 새로운 피어들에게 높은 확장성과 빠른 동기화를 가능하게 할 수 있습니다.
