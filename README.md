# FrandHome
Actix 와 Yew 를 이용하여 Rust 로 개발하는 풀스택 웹 프로젝트
개발 진행중...

## 목적
- 서버와 클라이언트 사이의 소켓 통신을 이용한 웹앱을 개발하다 보면 
실수하기 쉽고 가독성 떨어지는 보일러 플레이트 코드들이 양산됩니다.
이는 여러 가지 잠재적 보안 위험과 디버깅하기 어려운 버그들을 발생시킬 수 있습니다.
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
    - (구현중) process(packet) 을 호출하여 미리 지정된 fn(node, message, packet) 을 실행하고 packet 으로부터 일어날 값의 변화들을 여러 개의 packet 으로 받을 수 있습니다.
* __Message__ 는 Model 의 각 field 에 해당하는 값을 가질 수 있는 enum 입니다.
    - __Packet__ 으로 직렬화, 역직렬화 될 수 있습니다.
    - __Message__ 를 match 하여 각 이벤트를 필터하고 전달된 값을 추출할 수 있습니다.
* __Packet__ 은 Model 의 각 field 에 해당하는 유일한 key 값을 헤더로, 같이 전달될 state 를 페이로드로 가집니다.
    - [u8] 로 직렬화, 역직렬화 될 수 있습니다.
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
