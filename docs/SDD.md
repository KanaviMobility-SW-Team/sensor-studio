# Sensor Studio

## Software Design Description (SDD)

**Version:** 1.1  
**Last Updated:** 2026-07-16

---

# 1. Introduction

## 1.1 Purpose

본 문서는 Sensor Studio 시스템의 구체적인 동작 방식과 컴포넌트 간 처리 절차를 정의하는 상세 설계 명세서이다. SDS에서 정의한 아키텍처를 기반으로 Core 초기화, Transport 송수신, Engine 플러그인 연계, Instance 실행, WebSocket 스트리밍 및 UI 데이터 처리 방식을 기술한다.

본 문서의 자료구조와 인터페이스는 구현 언어에 종속된 소스 파일 목록보다 컴포넌트의 책임과 상호운용 규칙을 중심으로 정의한다.

---

# 2. System Modules & Directory Structure

Sensor Studio는 다음 논리 모듈로 구성한다.

```text
Sensor Studio
 ├ Core
 │  ├ Bootstrap and Configuration
 │  ├ Transport
 │  ├ Engine Adapter and FFI
 │  ├ Instance Runtime
 │  ├ Channel Registry
 │  └ WebSocket Stream and Control
 └ UI
    ├ WebSocket Connection
    ├ Channel and Sensor State
    ├ PointCloud Decode
    ├ 3D Visualization
    ├ Layout and Display Settings
    └ Application Logging
```

각 모듈의 책임은 다음과 같다.

| Module | Responsibility |
|---|---|
| Bootstrap and Configuration | 실행 인자와 설정 파일을 읽고 공통 서비스와 Instance를 초기화한다. |
| Transport | UDP 또는 USB를 통해 Raw byte를 송수신한다. |
| Engine Adapter and FFI | Engine 동적 라이브러리를 로드하고 데이터·제어 호출 및 메모리 해제를 중개한다. |
| Instance Runtime | Transport와 Engine 사이의 수신, 요청·응답 및 종료 흐름을 실행한다. |
| Channel Registry | PointCloud의 논리 채널과 WebSocket channel ID를 관리한다. |
| WebSocket Stream and Control | 채널 광고, 구독, 데이터 전송 및 Engine 확장 호출을 처리한다. |
| WebSocket Connection | 서버 연결 상태와 송수신을 관리한다. |
| PointCloud Decode | Binary 또는 JSON PointCloud를 공통 실수 배열로 변환한다. |
| 3D Visualization | 여러 채널의 최신 PointCloud를 하나의 3차원 화면에 표시한다. |

---

# 3. Component Design & Inter-operation

## 3.1 Bootstrap & Resource Allocation

시스템은 다음 순서로 초기화한다.

1. **실행 인자 처리:** 설정 파일 위치, 서버 주소 및 포트 재정의 값을 읽는다.
2. **설정 로드 및 검증:** `runtime.toml`을 구조체로 변환하고 Instance ID 중복, Engine 경로 및 기본 Transport 조건을 검증한다.
3. **로깅 초기화:** 구조화된 tracing 기반 콘솔 로그와 일별 파일 로그를 구성한다. Engine 로그 callback도 동일 로깅 체계로 연결한다.
4. **전역 데이터 채널 생성:** Instance가 생성한 PointCloud를 WebSocket 계층으로 전달하기 위한 제한 용량 broadcast channel을 생성한다.
5. **Registry 생성:** Channel Registry와 Engine Extension Registry를 구성한다.
6. **Engine 및 Transport 생성:** Instance별 Engine 동적 라이브러리를 로드하고 JSON 설정으로 Engine 객체를 생성한 뒤 UDP 또는 USB Transport를 초기화한다.
7. **비동기 실행:** WebSocket 서버와 각 Instance 실행 루프를 독립 task로 시작한다.
8. **종료 처리:** 종료 신호 수신 시 cancellation을 전파하고 Instance별 shutdown request를 처리한 후 task와 자원을 정리한다.

설정 파일을 읽을 수 없거나 필수 Engine 또는 Transport를 생성하지 못하면 초기화를 실패로 처리한다.

## 3.2 Transport Layer

Transport는 다음 공통 동작을 제공한다.

- `read_chunk`: 장비에서 Raw byte와 출처 정보를 읽는다.
- `transact_chunk`: Engine이 생성한 요청을 장비에 송신하고 설정된 방식으로 응답을 수집한다.
- `kind`: Transport 종류를 반환한다.

### UDP 처리

UDP Transport는 시작 시 수신용 byte buffer를 한 번 생성하여 반복 사용한다. 수신 결과는 실제 수신 길이만큼 독립 byte buffer로 전달하고 송신지 `SocketAddr`을 함께 보존한다.

Multicast 주소가 설정된 경우 지정 interface를 이용하여 multicast group에 참여한다. UDP transaction은 요청의 목적지 주소로 데이터를 송신하고 응답 방식에 따라 0회, 1회, 지정 횟수 또는 timeout까지 데이터를 수신한다.

### USB 처리

USB Transport는 VID와 PID로 장치를 찾고 지정 interface를 claim한다. Bulk IN endpoint는 일반 수신 및 응답 수신에 사용하며 Bulk OUT endpoint는 장비 요청 전송에 사용한다.

USB Control Transfer 요청은 request type, request, value, index 및 data length로 구성되는 setup 정보를 사용한다. 전송 데이터 길이는 setup 정보와 일치해야 한다.

USB는 네트워크 주소가 없으므로 Instance 및 장치 출처를 표현하기 위한 고정 출처 값을 Engine에 전달한다.

## 3.3 The Engine Trait & FFI Adapter

Engine 실행 객체는 Instance별로 생성되며 내부 상태를 보유할 수 있다. Core와 Engine의 논리 인터페이스는 다음 기능을 포함한다.

```text
Engine
 ├ id / version
 ├ process(raw bytes, source)
 ├ pop point cloud frame
 ├ pop transport request
 ├ pop shutdown request
 ├ list extension APIs
 └ call extension API
```

`process`는 입력 byte를 Engine 내부 버퍼 또는 상태에 반영한 뒤 새로 완성된 PointCloudFrame과 Transport Request의 존재 여부를 갱신한다. 하나의 입력에서 프레임이 생성되지 않을 수도 있고 여러 프레임이 생성될 수도 있다.

FFI Adapter는 다음 책임을 가진다.

- 플랫폼별 동적 라이브러리 로드
- 필수 함수 존재 확인
- Engine 생성 및 파괴
- Raw packet 전달
- Engine이 할당한 frame·API buffer·request buffer 복사 및 해제
- Engine 로그 callback 등록
- 선택적 Transport Request 및 shutdown request 기능 연결

동일 Engine 객체는 패킷 처리와 WebSocket 제어 요청에서 함께 사용될 수 있으므로 상호배제 방식으로 접근한다. Blocking 가능성이 있는 FFI 호출은 비동기 worker가 장시간 직접 점유되지 않도록 blocking 실행 구간에서 수행한다.

## 3.4 Isolated Instance Loop

Instance는 Transport, Engine 및 PointCloud 전달 채널을 캡슐화한다.

실행 상태는 다음과 같다.

```text
Created → Running ↔ Error → Stopped
```

한 번의 처리 주기는 다음 순서로 수행한다.

1. Engine에 대기 중인 Transport Request가 있는지 확인한다.
2. 요청이 있으면 Transport로 송신하고 응답을 수신한다.
3. 각 응답을 Engine에 다시 입력한다.
4. 요청이 없으면 Transport의 일반 입력을 수신한다.
5. 수신 byte와 출처 정보를 Engine에 전달한다.
6. Engine이 생성한 모든 PointCloudFrame을 전역 데이터 채널로 전달한다.

복구 가능한 I/O 오류가 발생하면 Instance 상태를 Error로 변경하고 100ms에서 시작하여 최대 5초까지 증가하는 대기 시간을 적용한다. 다음 처리에 성공하면 대기 시간은 초기값으로 복원한다.

종료 시에는 Engine에서 shutdown request를 순차적으로 가져와 Transport로 전송하고, 필요한 응답을 처리한 뒤 실행 루프를 종료한다.

## 3.5 Foxglove Protocol Stream multiplexer

WebSocket 서버는 `/ws` endpoint에서 `foxglove.websocket.v1` subprotocol을 사용한다.

연결 직후 서버는 다음 텍스트 메시지를 전송한다.

- `serverInfo`: 서버 이름과 지원 기능
- `advertise`: 구독 가능한 channel 목록

클라이언트는 `subscribe`와 `unsubscribe` 메시지를 이용해 subscription을 관리한다.

PointCloud 데이터는 Foxglove Message Data binary frame으로 전송한다.

```text
Byte 0       : opcode = 0x01
Byte 1..4    : subscription ID, little-endian u32
Byte 5..12   : timestamp, little-endian u64
Byte 13..N   : encoded PointCloud payload
```

각 설정 channel은 필요에 따라 JSON 채널과 Binary 채널을 제공한다. Binary payload는 다음 순서로 구성한다.

```text
point_stride : u32
field_count  : u8
fields[]
  name_length : u8
  name        : UTF-8 bytes
  offset      : u16
  datatype    : u8
point_data    : raw bytes
```

WebSocket 연결은 구독 정보와 subscription별 최신 대기 프레임을 개별적으로 관리한다. 데이터 수신 속도가 송신 속도보다 빠른 경우 동일 subscription의 이전 대기 프레임은 최신 프레임으로 교체한다.

Engine 확장 제어는 같은 WebSocket의 텍스트 메시지로 처리한다. 주요 `op`는 API 목록 조회와 API 호출이며 요청은 Instance ID, API 이름 및 입력 JSON을 포함한다. 응답은 API 목록, 실행 결과 또는 오류 형태로 반환한다.

### UI Client Processing

UI는 serverInfo와 advertise 메시지를 처리하여 연결 상태와 센서 채널 목록을 구성한다. 사용자가 센서를 활성화하면 해당 Binary channel을 구독한다.

Binary Message Data frame에서 subscription ID, timestamp 및 PointCloud payload를 분리하고, PointCloud decoding은 별도 Isolate에서 수행한다. 각 point에서 `x`, `y`, `z` 및 색상 기준 값을 추출하여 다음 렌더링 배열을 생성한다.

```text
[x, y, z, value, x, y, z, value, ...]
```

여러 센서가 활성화된 경우 센서별 최신 배열을 병합하여 OpenGL 기반 뷰어에 전달한다. Grid, Axis, Label, 좌표 표시, 레이아웃 및 WebSocket 주소는 사용자 설정으로 관리한다.

---

# 4. Data Structures

시스템의 중심 자료구조는 PointCloudFrame이다.

```text
PointCloudFrame
 ├ timestamp_ns : u64
 ├ frame_id     : String
 ├ width        : u32
 ├ height       : u32
 ├ point_step   : u32
 ├ row_step     : u32
 ├ fields       : PointField[]
 ├ is_dense     : bool
 └ data         : byte[]
```

PointField는 다음 정보를 가진다.

```text
PointField
 ├ name      : String
 ├ offset    : u32
 ├ datatype  : PointFieldDataType
 └ count     : u32
```

지원 자료형은 signed·unsigned 8/16/32-bit 정수와 32/64-bit 실수이다. Point 데이터는 field offset과 point step을 이용하여 해석한다.

장비 송신 요청은 다음 정보로 구성한다.

```text
TransportRequest
 ├ destination
 ├ payload
 ├ response mode
 └ write mode
```

Response mode는 다음과 같다.

- 응답 없음
- 1회 응답
- 지정 횟수 응답
- timeout까지 응답

Write mode는 다음과 같다.

- Bulk 또는 일반 Datagram 전송
- USB Control Transfer

Channel 정보는 channel ID, topic, encoding, schema name 및 schema definition을 포함한다. WebSocket subscription은 클라이언트가 부여한 subscription ID와 channel ID를 연결한다.

UI의 PointCloud 상태는 topic별 최신 frame과 렌더링용 실수 배열을 보유한다. 센서 표시 설정은 표시 여부, point size, opacity, color map 및 color range를 포함한다.

---

## Revision History

| Version | Date | Description |
|---|---|---|
| 1.0 | 2026-04-13 | Sensor Studio 상세 설계 문서 최초 작성 |
| 1.1 | 2026-07-16 | 통신 계층, Engine 연계, 데이터 스트리밍 및 UI 처리 절차 현행화 |