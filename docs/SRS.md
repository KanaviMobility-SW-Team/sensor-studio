# Sensor Studio

## Software Requirements Specification (SRS)

**Version:** 1.1\
**Last Updated:** 2026-04-06

---

# 1. Introduction

## 1.1 Purpose

본 문서는 Sensor Studio 플랫폼의 전체 소프트웨어 요구사항을 정의한다.

본 플랫폼은 고성능 센서(LiDAR 등) 네트워크의 실시간 스트리밍 및 제어를 위한 통합 인프라를 제공한다. 엄격한 역할 분리에 따라 코어(Core)는 네트워크 입출력과 시스템 안정성을 전담하고, 외부 엔진(Engine 플러그인, C-FFI)은 센서 프로토콜 파싱만을 전담하여 공통 PointCloud 구조로 변환한다. 최종적으로 단일 WebSocket 채널(Foxglove 프로토콜)을 통해 UI 클라이언트에서 3D 시각화 및 동적 제어를 수행하는 것을 목표로 한다.

## 1.2 Definitions

| Term | Description |
| --- | --- |
| Core | 센서 데이터 네트워킹(수신/스트리밍), 스레드 및 메모리 자원 관리, WebSocket 다중화를 전담하는 백엔드 서버 |
| Engine | (C-FFI) 순수하게 센서 바이트 데이터를 입력받아 `PointCloudFrame`으로 변환하고 제어 메타데이터를 제공하는 상태를 저장하지 않는(Stateless) 공유 라이브러리 |
| UI | 실시간 센서 데이터를 렌더링하고 동적 제어 인터페이스를 제공하는 Flutter 기반 프론트엔드 (데이터 파싱 배제) |
| Instance | 하나의 물리적 센서 장비와 1:1로 매핑되는 완전히 격리된 실행 단위 |
| Foxglove Protocol | 바이너리 스트리밍과 JSON-RPC 제어를 단일 채널에서 혼합 전송(Multiplexing)하는 통신 규격 (`foxglove.websocket.v1`) |
| Channel | 멀티 센서 환경에서 각 센서 데이터(Topic, Schema)를 구분하는 구독 단위 |
| runtime.toml | 시스템 초기화 시 준수되어야 하는 엄격한 구조의 설정 파일 |

---

# 2. System Overview

Sensor Studio는 분산 처리 및 고가용성 철학에 기반하여 설계되었다. 중앙 집중적 관리자(Manager) 아키텍처를 배제하고, 각 센서 인스턴스가 완전히 캡슐화된 태스크로 동작한다.

- **Core의 철저한 경계:** Core는 네트워크(UDP/WS)와 인스턴스의 라이프사이클만 관리한다. 데이터가 어떤 의미를 가지는지는 관여하지 않으며, 오직 `Engine`만이 데이터의 해석과 모델 특화 기능을 담당한다.
- **WebSocket의 이중 역할:** 클라이언트와의 연결은 Foxglove 프로토콜 기반의 단일 WebSocket으로 구성되며, **대용량 PointCloud 바이너리 스트리밍**과 **상태/제어용 JSON-RPC 메시지**를 단일 연결로 혼합 전송(Multiplexing)하여 처리한다.
- **UI의 역할 범위 축소:** UI는 데이터 가공 책임을 가지지 않으며, Core가 제공하는 정규화된 데이터의 쾌적한 렌더링 및 동적 Extension API 호스팅에 집중한다.

---

# 3. Functional Requirements: Core System

## 3.1 Strict Configuration & Isolated Instance

- **FR-CORE-001 엄격한 설정 검증:** `runtime.toml`은 시스템의 엄밀한 기준으로 취급된다. 실행 시 초기 구동 단계에서 엄격한 사전 검증을 수행하며, 형식이 맞지 않거나 필수 값이 누락된 경우 즉시 시스템 기동을 중단한다. CLI 인자를 통한 오버라이드를 지원한다.
- **FR-CORE-002 독립적 장애 격리:** 중앙 `InstanceManager` 없이, 각 센서 설정은 독립된 비동기 태스크로 생성된다. 한 센서 인스턴스의 오류나 크래시는 다른 센서에 어떠한 영향도 주지 않아야 한다(장애 전파 차단).
- **FR-CORE-003 Instance Self-Recovery:** 파싱 오류 및 네트워크 장애 발생 시, 각 인스턴스는 시스템 전체 개입 없이 내부적으로 점진적인 대기 시간 증가를 거치며 자체 복구를 시도한다.

## 3.2 Core/Engine Boundary & Stability

- **FR-CORE-010 스레드 점유 방지:** 외부 C-FFI 기반 엔진의 연산 지연이 코어 시스템의 비동기 처리에 병목을 주지 않도록, 스레드 혼잡 현상을 원천 차단하는 구조를 적용한다.
- **FR-CORE-011 데이터 수신 레이어 최적화:** 초당 수만 건의 UDP 패킷 스트리밍을 견디기 위해, 서버 자원 할당을 최소화하는 버퍼 재사용(Buffer Reuse) 구조를 필수적으로 적용해야 한다.
- **FR-CORE-012 Pure Data Transformation:** Engine은 상태를 가지지 않는 순수 데이터 변환 모듈처럼 동작해야 하며, 오프셋 메타데이터만으로 파싱을 수행하여 `PointCloudFrame`을 Core로 반환한다.
- **FR-CORE-013 Multi-Sensor Multiplexing Support:** Core 시스템은 단일 포트로 들어오는 다중 센서의 UDP 패킷을 구별하기 위해, Engine에게 수신된 패킷의 송신지 IP(`sender_addr`) 정보를 전달해야 한다.
- **FR-CORE-014 Dynamic Configuration Injection:** Core는 센서 장비의 구체적인 설정 스키마 및 옵션을 알 필요 없이, 파싱된 범용 JSON 페이로드(`config_json`)를 Engine에 주입하여 동적으로 초기화한다. 이를 통해 센서별 옵션 파라미터가 추가되거나 변경되더라도 Core 시스템의 코드 수정 및 재배포가 발생하지 않아야 한다.

## 3.3 Dual-Role WebSocket Multiplexing

- **FR-CORE-020 Unified Protocol Layer:** 별도의 HTTP REST API 없이 단일 WebSocket 세션을 통해 모든 통신을 통합한다.
- **FR-CORE-021 메모리 고갈 방지 (자원 한계 보호):** 클라이언트의 렌더링 지연으로 인한 서버의 메모리 고갈(Out Of Memory)을 방어하기 위해 제한된 크기의 대기열(Bounded Queue)을 사용하며, 대기열 포화 시 가장 오래된 프레임을 우선 폐기한다. 
- **FR-CORE-022 Dynamic Registry & Extension API:** Foxglove 구조 내에서 `ChannelRegistry`를 통한 토픽 구독을 지원하고, Engine에서 제공하는 Extension API (제어/설정 파라미터 메타데이터) 목록을 JSON-RPC를 통해 클라이언트로 동적 프로비저닝한다.

---

# 4. Functional Requirements: UI System (Flutter)

UI는 복잡한 비즈니스 로직을 배제하고 시각화와 상호작용에만 집중한다.

## 4.1 Connection & Dynamic Registry

- **FR-UI-001:** WS Address 입력 후 단일 커넥션을 맺고 연결 상태를 실시간으로 모니터링한다. 끊김 발생 시 자동 재접속(Auto-reconnect) 메커니즘을 수행한다.
- **FR-UI-002:** 접속 성공 직후 수신되는 채널(Channel) 메타데이터 리스트를 해석하여 구독 가능한 센서 토픽 목록을 동적으로 화면에 표시한다.

## 4.2 Data Visualization (3D View)

- **FR-UI-010:** Core로부터 실시간 수신되는 `PointCloudFrame` 바이너리 페이로드를 즉시 디코딩하여 고속 3D 렌더링(Canvas/GL)을 수행한다. (UI 내 추가 조작 및 데이터 가공 배제)
- **FR-UI-011:** 마우스/터치 이벤트를 이용한 3D 카메라 뷰 제어(Pan, Zoom, Tilt) 기능을 제공한다.
- **FR-UI-012:** 멀티 센서 구독 시 동일한 3D 좌표 스페이스 내에 병합 시각화(Point Fusion)를 지원한다.

## 4.3 Dynamic Control Panel (Extension API)

- **FR-UI-020 Metadata-driven UI:** 하드코딩된 UI를 지양하고, 특정 인스턴스(센서)에서 RPC로 전달받은 Extension API(`list_extension_apis`) 메타데이터를 기반으로 제어 폼(버튼, 입력 필드 등)을 런타임에 동적으로 렌더링한다.
- **FR-UI-021:** 동적 생성된 UI를 통한 사용자 입력을 JSON-RPC 규격의 `call_extension_api` 요청으로 감싸 단일 WebSocket 채널로 전송하고 결과를 피드백한다.

---

# 5. Non-Functional Requirements

- **NFR-001 (운영 안정성 및 메모리 고갈 방지):** 서버 시스템은 극한의 네트워크 부하나 불안정한 클라이언트 접속과 마주하더라도 일정한 메모리 점유율을 한계치 내로 유지하여 전체 시스템 크래시를 방지해야 한다.
- **NFR-002 (독립적 장애 격리):** 외부 Engine 플러그인 내부에서 심각한 오류가 발생하더라도, 해당 센서 처리 과정만 독립적으로 재시작되며 타 센서 및 Core 메인 시스템에는 장애가 전파되지 않아야 한다.
- **NFR-003 (유연한 확장성):** Core 시스템과 UI 시스템 코드의 재배포나 수정 없이, 특정 센서 전용의 컴파일된 라이브러리(`.so`/`.dll`)만을 동적으로 로딩하여 신규 모델을 즉시 지원할 수 있어야 한다.

---

# 6. Revision History

| Version | Date | Description |
| --- | --- | --- |
| 1.0 | 2026-03-03 | Initial Version (Core 위주의 초안) |
| 1.1 | 2026-04-06 | 아키텍처 원칙(Core/Engine 경계, 인스턴스 격리, 단일 WS 멀티플렉싱, OOM 방어) 및 UI 역할 정의 요구사항 반영, M/O 구분 제거 |