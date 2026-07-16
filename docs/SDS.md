# Sensor Studio

## Software Design Specification (SDS)

**Version:** 1.2  
**Last Updated:** 2026-07-16

---

# 1. Introduction

## 1.1 Purpose

본 문서는 Sensor Studio 시스템의 소프트웨어 설계를 정의한다.

SRS에서 정의한 요구사항을 기반으로 시스템의 전체 아키텍처, 주요 구성 요소의 책임, 데이터 및 제어 흐름, 컴포넌트 간 인터페이스를 설명한다. 본 문서는 구현에 앞서 구성요소 간 경계를 명확히 하고, 센서 모델과 통신 방식의 확장 시 일관된 설계를 유지하기 위한 기준으로 사용한다.

## 1.2 Scope

본 문서는 다음 내용을 포함한다.

- 시스템 전체 아키텍처
- Core, Engine, Transport, Streaming Layer 및 UI의 책임
- Instance 실행 구조
- 센서 데이터 및 장비 제어 흐름
- WebSocket 스트리밍·제어 인터페이스
- Engine 플러그인 확장 구조

구체적인 자료구조와 처리 절차는 Software Design Description(SDD)에서 정의한다.

## 1.3 Definitions

| Term | Description |
|---|---|
| Core | 설정, Instance 실행, 채널 관리 및 WebSocket 서비스를 담당하는 백엔드 |
| Engine | 센서별 패킷 조립, 프로토콜 해석 및 장비 제어를 담당하는 플러그인 |
| Instance | 하나의 Transport와 Engine 실행 객체를 결합한 처리 단위 |
| Transport | 센서 또는 장비와 Raw byte 데이터를 송수신하는 계층 |
| Stream | Engine이 생성한 PointCloud를 WebSocket 메시지로 전달하는 계층 |
| Channel Registry | UI가 구독할 수 있는 데이터 채널의 메타데이터를 관리하는 구성요소 |
| Extension Registry | Instance와 Engine 실행 객체를 연결하여 확장 API 요청을 라우팅하는 구성요소 |

---

# 2. System Architecture

Sensor Studio는 공통 실행 환경과 센서별 프로토콜 구현을 분리한 모듈형 아키텍처를 사용한다.

시스템은 다음 구성요소로 이루어진다.

- Sensor Device
- Transport Layer
- Instance Runtime
- Engine Plugin
- Channel and Streaming Layer
- Flutter UI

## 2.1 High-Level Architecture

```text
Sensor Device
    ↓
UDP / USB Transport
    ↓
Instance Runtime
    ↓
Engine Frame Assembly and Parsing
    ↓
PointCloudFrame
    ↓
Channel Registry and WebSocket Streaming
    ↓
Flutter UI
    ↓
PointCloud Decode and 3D Rendering
```

장비 제어가 필요한 경우 Engine은 Transport Request를 생성하며, Instance는 이를 Transport로 전달하고 응답을 다시 Engine에 입력한다.

```text
Engine Transport Request
    ↓
Instance Runtime
    ↓
UDP / USB Transaction
    ↓
Device Response
    ↓
Engine Processing
```

---

# 3. Core Architecture

Core는 설정을 기반으로 공통 실행 환경을 구성하고, Transport·Engine·Stream 간의 처리 흐름을 조정한다. 센서 고유 프로토콜 해석은 Engine이 담당하며 Core는 해당 의미를 직접 해석하지 않는다.

```text
Core
 ├ Independent Task Loops (Instance)
 ├ Transport Layer
 ├ Engine Extension Registry
 ├ Data Pipeline
 └ WebSocket Server (Stream and Control)
```

## 3.1 Independent Task Loops (Instance)

`runtime.toml`에 정의된 각 Instance는 독립 비동기 실행 단위로 생성된다.

각 Instance는 다음 요소를 포함한다.

- Instance ID
- Transport 객체
- Engine 실행 객체
- PointCloud 전달 채널
- 실행 상태

Instance는 Transport에서 데이터를 읽어 Engine에 전달하고, Engine이 반환한 PointCloud를 Streaming Layer로 전송한다. Engine이 장비 송신 요청을 제공한 경우 요청을 Transport로 전달하고 필요한 응답을 다시 Engine에 입력한다.

복구 가능한 Transport 오류가 발생하면 Instance는 점진적으로 증가하는 대기 시간을 적용한 뒤 처리를 재시도한다. 시스템 종료 시에는 Engine이 제공하는 종료 요청을 처리한 후 Transport와 Engine 자원을 해제한다.

## 3.2 Transport Layer

Transport Layer는 센서 장비와 Raw byte 데이터를 송수신하며 센서 프로토콜을 해석하지 않는다.

현재 지원 인터페이스는 다음과 같다.

- **UDP:** 지정 주소 및 포트 바인드, 송신지 주소 식별, Multicast 참여, 수신 버퍼 재사용
- **USB:** VID/PID 기반 장치 선택, Interface claim, Bulk IN/OUT 전송, Control Transfer 전송

Transport는 일반 수신과 요청 기반 송수신을 공통 인터페이스로 제공한다. 요청 응답 방식은 응답 없음, 1회 응답, 지정 횟수 응답, timeout까지 응답 수신 방식으로 구분한다.

TCP 및 Serial은 Transport 확장 지점으로 정의하되 현재 제품 범위에는 포함하지 않는다.

## 3.3 Engine Extension Registry

Engine Extension Registry는 Instance ID와 해당 Engine 실행 객체의 연결 관계를 관리한다.

주요 역할은 다음과 같다.

- WebSocket 제어 요청의 대상 Instance 확인
- Engine Extension API 목록 조회 요청 전달
- Engine Extension API 호출 요청 전달
- Engine 호출 결과 또는 오류 응답 반환

Extension Registry는 센서별 API 파라미터의 의미를 해석하지 않으며, 요청을 대상 Engine으로 안전하게 라우팅하는 역할을 수행한다.

## 3.4 Data Pipeline

센서 데이터 처리 흐름은 다음과 같다.

```text
Raw Transport Data
    ↓
Engine Packet Accumulation
    ↓
Frame Assembly
    ↓
Sensor Protocol Parsing
    ↓
PointCloudFrame
    ↓
Channel Matching and Encoding
```

Engine은 수신 데이터를 누적하여 완전한 센서 프레임을 구성할 수 있으며 하나의 입력에서 0개 이상의 PointCloudFrame을 생성할 수 있다. PointCloudFrame은 필드 이름, 자료형, offset, point stride 및 원시 Point 배열을 포함한다.

Core는 Channel 설정의 source 식별자와 Engine이 생성한 frame 식별자를 비교하여 대상 채널을 결정한다.

## 3.5 WebSocket Server (Stream & Control)

WebSocket Server는 PointCloud 스트리밍과 Engine 제어 인터페이스를 하나의 연결로 제공한다.

주요 기능은 다음과 같다.

- `foxglove.websocket.v1` subprotocol 협상
- Server 정보 전송
- Channel 메타데이터 광고
- 구독 및 구독 해제 처리
- Foxglove Message Data 형식의 PointCloud 전송
- Engine API 목록 조회 및 API 호출 메시지 처리

PointCloud는 JSON 채널과 Binary 채널로 제공할 수 있다. Binary 채널은 대용량 Point 데이터를 효율적으로 전달하기 위해 필드 메타데이터와 원시 Point 배열을 결합한 프로젝트 전용 payload를 사용한다.

클라이언트별로 subscription별 최신 대기 프레임을 관리하며, 동일 subscription의 새로운 프레임이 도착하면 이전 대기 프레임을 교체한다.

## 3.6 Extension API (Control)

Extension API는 별도 REST API를 사용하지 않고 WebSocket 연결 안에서 프로젝트 전용 JSON 메시지로 제공한다.

지원 기능은 다음과 같다.

- Engine API 목록 조회
- Instance 및 API 이름을 지정한 Engine API 호출
- API 실행 결과 반환
- 대상 Instance 또는 API 오류 반환

제어 메시지는 `op` 필드를 기준으로 구분한다. 본 인터페이스는 JSON-RPC 2.0이 아니라 Sensor Studio 전용 제어 프로토콜이다.

현재 Core는 Extension API 전송 기능을 제공한다. Flutter UI의 현 제품 범위는 PointCloud 시각화이며, Engine API 메타데이터 기반의 동적 제어 화면은 향후 확장 대상으로 둔다.

---

# 4. Engine Architecture

Engine은 센서 모델별 데이터 처리 및 장비 제어 기능을 담당한다. 각 Instance는 독립 Engine 실행 객체를 생성하며 Engine은 패킷 조립 상태, 센서별 캐시 및 제어 상태를 보유할 수 있다.

## 4.1 Engine Interface

Engine은 다음 기능을 제공한다.

- Engine 식별자 및 버전 제공
- JSON 설정을 이용한 실행 객체 생성
- Raw byte 및 출처 정보 처리
- PointCloudFrame 생성 및 반환
- 장비 송신용 Transport Request 생성
- 종료 시 필요한 Transport Request 제공
- Extension API 목록 및 호출 처리
- Engine 로그를 Core 로깅 시스템으로 전달

입력은 Raw byte와 출처 정보이며, 출력은 PointCloudFrame 및 선택적 Transport Request이다.

## 4.2 Engine Plugin Model

Engine은 플랫폼별 동적 라이브러리 형태로 제공한다.

```text
Core
 ├ Sensor Model A Engine
 ├ Sensor Model B Engine
 └ Sensor Model C Engine
```

모든 Engine은 정의된 FFI 함수와 메모리 소유권 규칙을 준수해야 한다. 신규 센서 모델은 새로운 Engine 라이브러리와 실행 설정을 추가하는 방식으로 확장한다.

---

# 5. Configuration Handling

Core는 `runtime.toml`을 통해 다음 항목을 구성한다.

- WebSocket 서버 주소 및 포트
- 전역 데이터 전달 용량
- Instance ID
- Engine ID 및 동적 라이브러리 위치
- 센서별 설정
- UDP 또는 USB Transport 설정
- PointCloud Channel 설정

Core는 파싱된 Instance 및 센서 설정을 JSON으로 변환하여 Engine 생성 시 전달한다. Engine은 센서별 설정을 검증하고 내부 동작에 적용한다.

서버 주소와 포트는 실행 인자로 재정의할 수 있다. 설정 파일 파싱 또는 필수 Engine 생성에 실패하면 Core는 시작을 중단한다.

---

# 6. Data Flow

```text
Sensor Device
      ↓
Transport Layer
      ↓
Raw Byte and Source Information
      ↓
Engine Frame Assembly and Parsing
      ↓
PointCloudFrame
      ↓
Channel Encoding
      ↓
WebSocket Streaming
      ↓
UI Decode and Rendering
```

UI는 수신한 Binary PointCloud를 별도 Isolate에서 실수 배열로 변환하고, 활성화된 여러 채널의 최신 데이터를 동일 3차원 화면에 중첩 표시한다.

---

# 7. Interface Design

## 7.1 Streaming Interface

Core와 UI는 `foxglove.websocket.v1` 기반 WebSocket을 사용한다.

연결 직후 Core는 서버 정보와 Channel 목록을 제공한다. UI는 subscription ID와 channel ID를 이용하여 구독 또는 구독 해제를 요청한다. Core는 각 데이터 메시지를 다음 정보와 함께 전송한다.

- Message Data opcode
- Subscription ID
- Timestamp
- PointCloud payload

Binary PointCloud payload는 point stride, field metadata 및 raw point data로 구성한다. 모든 숫자 필드는 별도 명시가 없는 경우 little-endian을 사용한다.

## 7.2 Control Interface

Engine 제어는 WebSocket 텍스트 메시지를 사용한다.

주요 명령은 다음과 같다.

- Engine API 목록 조회
- Engine API 호출

요청은 `op`, `instanceId`, `apiName`, `input` 등의 필드를 사용하며 응답은 결과 또는 오류 정보를 포함한다. Core는 파라미터 내용을 변경하지 않고 대상 Engine으로 전달한다.

---

# 8. Extensibility

Sensor Studio는 다음 확장 지점을 제공한다.

- FFI 계약을 준수하는 신규 Engine 추가
- 공통 Transport 인터페이스를 구현하는 신규 Transport 추가
- 새로운 PointCloud 필드 및 Channel 추가
- Engine Extension API를 사용하는 UI 제어 기능 추가
- 새로운 렌더링 표현 또는 시각화 옵션 추가

확장 시 기존 Core와 UI 간 공통 PointCloud 및 WebSocket 인터페이스의 호환성을 유지해야 한다.

---

# 9. Revision History

| Version | Date | Description |
|---|---|---|
| 1.0 | 2026-03-05 | Initial Version |
| 1.1 | 2026-04-13 | 단일 WebSocket 기반 스트리밍 및 제어 구조 반영 |
| 1.2 | 2026-07-16 | 기존 장·절 번호를 유지하면서 UDP/USB Transport, 상태 기반 Engine, Transport Request, Binary PointCloud, 최신 프레임 전송 및 UI 처리 구조를 현행화 |
