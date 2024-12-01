# FrandHome
Actix 와 Yew 를 이용하여 Rust 로 개발하는 풀스택 웹 프로젝트


## 목적
- 서버와 클라이언트 사이의 소켓 통신을 이용한 웹앱을 개발하다 보면 
실수하기 쉽고 가독성 떨어지는 보일러 플레이트 코드들이 양산됩니다.
이 프로젝트는 Rust Macro 를 활용한 __Node__ 로 위와 같은 문제를 줄이고
높은 생산성과 가독성을 달성하는 프로젝트 기반을 만드는 것을 목적으로 합니다.


## 구조
* attribute proc macro __node__ 를 사용하여 구조체 형태로 Model Hierarchy 를 정의할 수 있습니다.
* 정의한 구조에 따라 __State__, __Node__, __Message__ 가 생성됩니다.
* __State__ 는 상태를 소유하는 단순 구조체입니다.
* __Node__ 는 Model 에 관련된 다양한 기능을 제공합니다.
    - emit(state) 을 호출하여 state 를 필드의 값으로 설정하는 동작에 해당하는 __Packet__ 을 생성하고, 설정된 클로저를 callback 할 수 있습니다.
    - emit_packet(packet) 을 호출하여 설정된 클로저를 callback 할 수 있습니다.
    - apply(state) 를 호출하여 state 를 필드의 값으로 설정할 수 있습니다.
    - apply_packet(packet) 을 호출하여 packet 이 지정하는 필드에 packet 이 가진 state 를 값으로 설정할 수 있습니다.
* __Message__ 는 Model 의 각 field 에 해당하는 값을 가질 수 있는 enum 입니다.
    - __Packet__ 으로부터 생성될 수 있습니다.
    - __Message__ 를 match 하여 각 이벤트를 필터하고 전달된 값을 추출할 수 있습니다.
* __Packet__ 은 Model 의 각 field 에 해당하는 유일한 key 값을 헤더로, 같이 전달될 state 를 페이로드로 가집니다.
    - [u8] 로 직렬화, 역직렬화 될 수 있습니다.
* __Container__ 는 __Node__ 를 소유하며 Node.emit() 으로부터 발생된 __Packet__ 들을 처리합니다
    - process(packet) 을 호출하여 FnMut(node, message, packet) 클로저를 packet 과 packet 으로부터 발생될 새로운 packet 들에 대하여 수행할 수 있습니다.
* 위의 구성들을 활용하여 Model 에 새로운 값을 넣었을 때 일어날 변화들을 여러 개의 __Packet__ 들로 추출할 수 있고 그 __Packet__ 들을 원하는 곳으로 전달하고 다양한 작업에 응용할 수 있습니다.


## 구성
* frand-home
    - frand-node 작동을 확인하기 위한 테스트베드 상태. 간단한 숫자 더하기가 구현되어 있습니다.
* frand-web
    - 웹 개발을 위한 기초 코드. Actix 를 위한 ServerSocket 과 Yew 를 위한 ClientSocket 이 구현되어 있습니다.
* frand-node
    - 서버와 클라이언트 간의 메시지 통신을 추상화하기 위한 Node 가 구현되어 있습니다.
    - 각 필드에 emit() 을 호출함으로써 값을 변경하기 위한 Packet 를 생성하고 통신에 직접 사용하거나
        Message enum 으로 변환하여 rust 의 match 기능을 활용한 값 변경 전파로 이벤트를 제어할 수 있습니다.


## 기능
- proc macro 를 이용한 wss 통신용 State/Node/Message 생성
- wss 프로토콜을 통한 server-client 간 상태 관리 통신
- Docker Image 생성


## 구현 예정
- 압축된 Packet Key
- IString 등의 경량 타입 지원
- Vec 등의 Collection Node 지원
- Option 등의 Enum Node 지원
- 유연하고 편리한 사용을 위한 Component 구현
- Actix, Yew, eframe 등과 간편하게 연동되는 App 구현


## 예시

- node 매크로를 사용해 __Node__ 구조를 작성합니다  

```rust
#[node]
#[derive(Properties)]
pub struct Shared {
    pub sum1: NumberSum,
    pub sum2: NumberSum,
    pub sum3: NumberSum,
}

#[node]
#[derive(yew::Properties)]
pub struct NumberSum {
    pub a: i32,
    pub b: i32,
    pub sum: i32,
}
```

- a 와 b 를 더해 sum 에 emit 하는 함수를 포함하였습니다.  
  async 동작을 확인하기 위해 200ms 지연하는 코드를 넣어 비싼 연산을 시뮬레이션합니다.  

```rust
#[cfg(not(target_arch = "wasm32"))]
impl NumberSum {
    pub fn emit_expensive_sum(&self) {
        use tokio::time::sleep;
        use std::time::Duration;

        let (av, bv) = (*self.a, *self.b);
        self.sum.emit_future(async move {
            sleep(Duration::from_millis(200)).await;
            av + bv
        });
    }
}
```

- __Node__ 를 브라우저에 출력하기 위한 View 를 작성합니다.  

```rust
#[function_component]
pub fn NumberSumView(node: &NumberSum) -> Html {
    log::debug!("NumberSum::view");
    let a = node.a.clone();
    let b = node.b.clone();
    let sum = node.sum.clone();

    html! {
        <span> { format!("{a} + {b} : {sum}") } </span>
    }
}
```

- UI 기능을 추가하기 위해 View와 View Model 들을 작성합니다.  

```rust
#[derive(Properties, Clone, PartialEq)]
pub struct NumberInc {
    pub name: &'static str,
    pub number: Node<i32>,
}

#[function_component]
pub fn NumberIncView(node: &NumberInc) -> Html {
    let name = node.name;
    let number = node.number.clone();
    let number_value = *node.number;
    let inc = move |_| {
        number.emit(number_value + 1)
    };

    html! {
        <button onclick = {inc}>
            { format!("inc {name}: {number_value}") }
        </button>
    }
}

#[function_component]
pub fn NumberSumIncView(node: &NumberSum) -> Html {
    html! {
        <div>
            <NumberSumView ..node.clone() />
            <NumberIncView ..NumberInc{ name:"a", number: node.a.clone() } />
            <NumberIncView ..NumberInc{ name:"b", number: node.b.clone() } />
        </div>
    }
}

#[function_component]
pub fn SharedView(node: &Shared) -> Html {
    html! {
        <div>
            {"Shared"}
            <NumberSumIncView ..node.sum1.clone() />
            <NumberSumIncView ..node.sum2.clone() />
            <NumberSumView ..node.sum3.clone() />
        </div>
    }
}
```

- backend 에서 수행할 제어 코드를 작성합니다.  
  sum1의 a 또는 b 값이 변하면 emit_expensive_sum() 을 호출하여 sum 값을 설정합니다.  
  sum1.sum 또는 sum2.sum 값이 변하면 sum3의 a 또는 b 값을 설정합니다.  
  각 값들은 emit_expensive_sum() 에 의해 일정 시간 이후 변경 사항이 전파됩니다.  

```rust
pub async fn process(&mut self, packet: Packet) {
    self.processor.process(packet, |node, _packet, message| {
        use RootMessage::*;
        use NumberSumMessage::*;
        match message {
            shared(message) => {
                use SharedMessage::*;
                match message {
                    sum1(a(_) | b(_)) => node.shared.sum1.emit_expensive_sum(),
                    sum1(sum(s)) => node.shared.sum3.a.emit(s),

                    sum2(a(_) | b(_)) => node.shared.sum2.emit_expensive_sum(),
                    sum2(sum(s)) => node.shared.sum3.b.emit(s),

                    sum3(a(_) | b(_)) => node.shared.sum3.emit_expensive_sum(),
                    _ => {},
                }
            },
            personal(message) => { /* 생략 */ },
            _ => {},
        }
    }).await;
}
```