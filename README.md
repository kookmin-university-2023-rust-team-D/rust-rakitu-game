# bevy-lakitu-game
team D's final project

## 프로젝트 소개
bevy를 이용한 마리오와 김수한무의 1대1 대결을 만든 코드입니다.


## Getting Started


```sh
git clone https://github.com/kookmin-university-2023-rust-team-D/rust-rakitu-game.git
cd rust-rakitu-game
```

## 혼자 플레이 해보기
로컬에서 서버, 클라이언트 역할을 하는 터미널을 만들어야 합니다.
터미널 3개를 준비해주시고 각각에 맞는 코드를 실행합니다.
혼자서 플레이할 때는 코드 수정이 필요없습니다.

## p2p로 다른 컴퓨터와 경쟁해보기
rakitu game은 아직 완전한 프로그램이 아니라서 p2p를 이용하기 위해서는 클라이언트가 직접 코드를 수정해야 합니다...(개선시킬 예정입니다)
p2p.rs 파일에서 아래와 같은 함수를 볼 수 있는데

<img width="509" alt="Screenshot 2023-05-04 at 1 09 21 PM" src="https://user-images.githubusercontent.com/68311908/236110780-8ecf711c-9c59-4809-86a5-459bc7b8a13a.png">

```python3
let room_url = "ws://127.0.0.1:3536/room";
# {127.0.0.1}이면 로컬에서 실행 가능하고
# 이 부분을 matchbox_server를 연 서버의 ip address로 바꾸면 p2p가 가능해집니다.
```

모든 클라이언트가 서버에 연결되어야 조작이 가능하도록 구현되었습니다. 그 전까지 안움직여도 당황하지마세요! 


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

## 게임 실행 화면
<img width="1270" alt="Screenshot 2023-05-04 at 1 17 01 PM" src="https://user-images.githubusercontent.com/68311908/236111537-70fa00fd-98b7-4dab-9351-aadd420907ad.png">


