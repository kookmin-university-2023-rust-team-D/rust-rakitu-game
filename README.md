# bevy-lakitu-game
team D's final project

## 프로젝트 소개
bevy를 이용한 마리오와 김수한무의 1대1 대결을 만든 코드입니다.


## Getting Started

We recommend checking out [The Bevy Book](https://bevyengine.org/learn/book/introduction) for a full tutorial.

```sh
# Switch to the correct version (latest release, default is main development branch)
git clone https://github.com/kookmin-university-2023-rust-team-D/rust-rakitu-game.git
cd rust-rakitu-game
```

## 혼자 플레이 해보기
로컬에서 서버, 클라이언트 역할을 하는 터미널을 만들어야 합니다.
터미널 3개를 준비해주시고 각각에 맞는 코드를 실행합니다.

## p2p로 다른 컴퓨터와 경쟁해보기
matchbox_server를 열어줄 컴퓨터와 각각의 클라이언트는 이 서버의 ip address를 알아야합니다.


## 프로그램 실행
```sh
#server: 
cargo install matchbox_server
```          
위 코드로 matchbox_server를 설치해주시고
```sh
matchbox_server
```
위 코드를 통해 서버를 열어주세요. 서버가 열어진 상태에서 클라이언트를 실행시키면 됩니다.

--------

```sh
# clients: player1, player2 각각 다른 터미널에서 아래 코드를 실행
cargo run
```

## Dependencies
+ bevy, bevy_ggrs, matcbox


