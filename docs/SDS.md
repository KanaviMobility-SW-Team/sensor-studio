# Sensor Studio

## Software Design Specification (SDS)

Version: 1.0  
Last Updated: 2026-03-05

---

# 1. Introduction

## 1.1 Purpose

본 문서는 Sensor Studio 시스템의 소프트웨어 설계를 정의한다.

SRS에서 정의된 요구사항을 기반으로 시스템의 전체 아키텍처,
주요 구성 요소, 데이터 흐름, 그리고 컴포넌트 간 인터페이스를
설명하는 것을 목적으로 한다.

본 문서는 시스템 구현에 앞서 전체 설계 구조를 명확히 정의하기
위한 기준 문서로 사용된다.

## 1.2 Scope

본 문서는 다음 내용을 포함한다.

- 시스템 전체 아키텍처
- 주요 컴포넌트 구조
- Core 내부 모듈 구성
- Engine 플러그인 구조
- 데이터 처리 흐름
- Core와 UI 간 인터페이스

구현 세부사항은 Software Design Description(SDD) 문서에서
별도로 정의한다.

## 1.3 Definitions

| Term | Description |
|---|---|
| Core | 센서 데이터 수집 및 처리 시스템 |
| Engine | 센서 모델별 데이터 파싱 모듈 |
| Instance | 하나의 센서 연결 단위 |
| Transport | 센서 데이터 수신 계층 |
| Stream | 처리된 데이터를 UI로 전달하는 계층 |

---

# 2. System Architecture

Sensor Studio는 센서 데이터 수집, 처리, 그리고 시각화를 위한
모듈형 아키텍처를 기반으로 구성된다.

시스템은 크게 다음 구성 요소로 이루어진다.

- Sensor Device
- Core
- Engine
- Streaming Layer
- UI

## 2.1 High-Level Architecture

```
Sensor Device
    ↓
Transport Layer
    ↓
Core Processing
    ↓
Engine Frame Assembly
    ↓
Engine Parsing
    ↓
PointCloudFrame
    ↓
Streaming Layer
    ↓
UI Client
```
---

# 3. Core Architecture

Core는 센서 데이터 처리의 중심 시스템이며,
센서 데이터 수신, 파싱, 변환, 스트리밍을 담당한다.

Core는 다음과 같은 주요 모듈로 구성된다.

```
Core
 ├ Independent Task Loops (Instance)
 ├ Transport Layer
 ├ Engine Extension Registry
 ├ Data Pipeline
 └ WebSocket Server (Foxglove)
```

## 3.1 Independent Task Loops (Instance)

중앙 관리자(Manager) 없이, `runtime.toml`에 정의된 각각의 센서 설정은 완전히 독립된 비동기 태스크(Task Loop)로 생성된다.

주요 특징:

- 센서 인스턴스 간 완벽한 운영 격리 (독립적 장애 격리)
- 개별 인스턴스 상태 및 생명주기 자체 관리
- 네트워크/파싱 장애 발생 시 점진적 대기 시간 증가를 통한 자체 재시도/복구 수행

각 Instance는 다음 정보를 포함한다.

- Sensor ID
- Transport 정보
- Engine 타입
- 연결 상태

## 3.2 Transport Layer

Transport Layer는 센서 장비로부터 데이터를 수신하는 계층이다.

지원 인터페이스:

- UDP
- TCP
- USB
- Serial

Transport Layer의 주요 역할:

-   센서 데이터 수신
-   Raw byte stream 전달
-   입력 버퍼 관리

Transport Layer는 프로토콜을 해석하지 않는다. 수신된 데이터는 그대로
Engine으로 전달된다.

## 3.3 Engine Extension Registry

Engine Extension Registry는 센서 모델별 데이터 파싱을 담당하는 Engine의 
동적 제어 기능 (Extension API) 목록을 관리하고 인스턴스에 매핑한다.

각 Engine은 특정 센서 프로토콜을 해석하는 역할을 수행한다.

주요 역할:

- C-FFI 기반 Engine 매핑 및 할당
- Extension API(제어 폼 메타데이터) 전달 및 관리

## 3.4 Data Pipeline

Data Pipeline은 센서로부터 수신된 데이터를 처리하는 내부 흐름을 정의한다.

데이터 처리 흐름:

```
Raw Byte Stream  
    ↓  
Engine Frame Assembly
    ↓  
Engine Parsing  
    ↓  
PointCloudFrame
```

Frame Assembly 및 프로토콜 해석은 Engine 내부에서 수행된다.

Engine은 Raw 데이터를 분석하여 공통 데이터 구조인
PointCloudFrame으로 변환한다.

## 3.5 WebSocket Server (Stream & Control)

WebSocket Server는 센서 데이터 스트리밍과 시스템 제어 인터페이스를 
단일 채널로 융합하여 UI 클라이언트로 제공한다.

전송 방식:

- Foxglove Protocol 기반 단일 WebSocket 전송 체계 (Multiplexing)

주요 기능:

- PointCloudFrame 등 실시간 3D 데이터 전송 (Binary)
- ChannelRegistry를 통한 구독 및 채널 메타데이터 관리
- 다중 클라이언트 연결 및 메모리 고갈 방지(Bounded Queue) 적용

## 3.6 Extension API (Control)

센서 설정 변경 및 제어를 위해 별도의 REST API를 두지 않고, 
WebSocket 채널 내에 동기화되는 JSON-RPC 규격을 통해 Extension API를 제공한다.

지원 기능:

- 장비별로 호스팅된 Extension UI 메타데이터(list_extension_apis) 제공
- 클라이언트로부터 수신된 설정 변경/제어 요청(call_extension_api)의 라우팅
- 응답 피드백 및 상태 동기화

---

# 4. Engine Architecture

Engine은 센서 모델별 데이터 파싱을 담당하는 모듈이다.

각 Engine은 특정 센서 프로토콜을 해석하여
공통 데이터 구조로 변환하는 역할을 수행한다.

## 4.1 Engine Interface

Engine은 다음과 같은 역할을 수행한다.

-   Raw byte stream 처리
-   Frame Assembly 수행
-   프로토콜 파싱
-   PointCloudFrame 생성
-   장비별 Configuration 처리

입력: `Raw Byte Stream`

출력: `PointCloudFrame`

## 4.2 Engine Plugin Model

Engine은 플러그인 형태로 구성되며,
센서 모델별로 독립적으로 구현된다.

```
Engine
 ├ Kanavi R2 Engine
 ├ Kanavi R4 Engine
 └ Kanavi S16 Engine
```

이 구조를 통해 새로운 센서를 쉽게 확장할 수 있다.

---

# 5. Configuration Handling

각 센서 장비는 서로 다른 설정 구조를 가질 수 있다.

Sensor Studio에서는 Configuration Handling을 Engine에 위임하는
구조를 사용한다.

Core의 역할:

-   WebSocket을 통한 JSON-RPC 제어 명령 수신
-   설정 적용 결과 클라이언트로 반환
-   단일 WebSocket 채널 기반으로 통신 스펙 통합

Engine의 역할:

-   제어 폼(UI 요소) 구성을 위한 메타데이터 및 스키마 정의
-   설정 값 검증
-   장비 설정 적용 및 상태 유지

Core는 Engine 설정의 의미를 해석하지 않는다.

---

# 6. Data Flow

```
Sensor Device
      ↓
Transport Layer
      ↓
Raw Byte Stream
      ↓
Engine Frame Assembly
      ↓
Engine Parsing
      ↓
PointCloudFrame
      ↓
WebSocket Streaming
      ↓
UI Client
```

---

# 7. Interface Design

## 7.1 Streaming Interface

Core는 Foxglove 프로토콜 기반 단일 WebSocket을 사용하여 UI 클라이언트와 통신한다.

UI 클라이언트는 접속 시 서버가 제공하는 `Channel` 메타데이터 리스트를 수신하고,
원하는 대상 토픽을 구독하여 실시간 PointCloudFrame 데이터를 바이너리로 수신할 수 있다.

## 7.2 Control Interface

센서 제어 및 설정 기능은 별도의 HTTP REST API 없이 단일 WebSocket 세션을 활용한다.

시스템 제어 및 설정 기능은 `Extension API` 메커니즘을 통해 JSON-RPC 형태로 제공된다.
Core는 공통 통신 브릿지만 담당하며, 구체적인 파라미터 구조나 제어용 UI 메타데이터는
각 Engine 내부 단위에서 동적으로 만들어져 런타임에 클라이언트로 전달된다.

---

# 8. Extensibility

Sensor Studio는 새로운 센서 장비를 쉽게 지원할 수 있도록
확장 가능한 구조로 설계된다.

확장 방식:

- Engine 추가
- Transport 인터페이스 추가
- 데이터 처리 모듈 확장

---

# 9. Revision History

| Version | Date | Description |
|---|---|---|
| 1.0 | 2026-03-05 | Initial Version |
| 1.1 | 2026-04-13 | 요구사항 명세서(SRS)에 맞추어 REST API 내용 삭제, 단일 WebSocket/JSON-RPC 혼합 체계 등 반영 |
