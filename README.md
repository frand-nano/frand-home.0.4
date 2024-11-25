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
- proc macro __node__ 를 derive 한 구조체를 기술하면
그에 맞는 구조의 __Node__ 와 __Message__ 가 생성됩니다.
__Node__ 는 클라이언트에서 Yew Component 로 전달되어 이벤트를 처리하는 View Model 로 사용되거나
서버에서 서버와 클라이언트의 상태를 보관하고 클라이언트로 보낼 __Message__ 를 생성하는 역할을 합니다.
- Yew Component 에 __Node__ 를 전달하고 value() 로 값을 꺼내고 emit() 으로 값 변경 메시지를 보낼 수 있습니다.
- handle_message() 에서 __Message__ 를 받아 match 하여 서버의 상태를 변경하고 클라이언트에 메시지를 보낼 수 있습니다.


## 구성
* frand-home
    - frand-node 작동을 확인하기 위한 테스트베드 상태. 간단한 숫자 더하기가 구현되어 있습니다.
* frand-web
    - 웹 개발을 위한 기초 코드. Actix 를 위한 ServerSocket 과 Yew 를 위한 ClientSocket 이 구현되어 있습니다.
* frand-node
    - 서버와 클라이언트 간의 메시지 통신을 추상화하기 위한 Node 가 구현되어 있습니다.
    - 각 필드에 emit() 을 호출함으로써 값을 변경하기 위한 Payload 를 생성하고 통신에 직접 사용하거나
        Message enum 으로 변환하여 rust 의 match 기능을 활용한 값 변경 전파로 이벤트를 제어할 수 있습니다.

## 기능
- proc macro 를 이용한 wss 통신용 State/Node/Message 생성
- wss 프로토콜을 통한 server-client 간 상태 관리 통신
- Docker Image 생성
