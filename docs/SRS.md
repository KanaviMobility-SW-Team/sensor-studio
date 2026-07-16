# Sensor Studio

## Software Requirements Specification (SRS)

**Version:** 1.2  
**Last Updated:** 2026-07-16

---

# 1. Introduction

## 1.1 Purpose

본 문서는 Sensor Studio 플랫폼의 소프트웨어 요구사항을 정의한다.

Sensor Studio는 LiDAR 및 ToF 센서와 같은 3차원 센서 데이터를 수집하고, 센서 모델별 프로토콜을 공통 PointCloud 형식으로 변환하여 실시간으로 시각화하는 통합 소프트웨어이다. 시스템은 센서 통신과 실행 관리를 담당하는 Core, 센서별 프로토콜 처리를 담당하는 Engine 플러그인, 실시간 데이터를 표시하는 UI로 구성한다.

본 문서는 현재 제품 범위에서 제공하는 기능과 품질 요구사항을 정의하며, 구현 세부사항은 SDS 및 SDD에서 기술한다.

## 1.2 Definitions

| Term | Description |
| --- | --- |
| Core | 센서 통신, Engine 실행, 채널 관리, WebSocket 스트리밍 및 시스템 종료 절차를 담당하는 백엔드 소프트웨어 |
| Engine | 센서별 패킷 조립, 프로토콜 해석, PointCloud 생성 및 장비 제어 요청 생성을 담당하는 동적 라이브러리 |
| UI | Core에 연결하여 센서 채널을 구독하고 PointCloud를 3차원으로 표시하는 Flutter 기반 사용자 인터페이스 |
| Instance | 하나의 Transport와 하나의 Engine 실행 객체를 결합한 독립 처리 단위 |
| Transport | 센서 또는 장비와 Raw byte 데이터를 송수신하는 통신 계층 |
| Channel | WebSocket에서 데이터 종류와 센서 출처를 식별하기 위한 구독 단위 |
| PointCloudFrame | 좌표 필드, 부가 필드, 시간 정보 및 원시 Point 데이터를 포함하는 공통 PointCloud 자료구조 |
| Extension API | Engine이 제공하는 장비별 조회·제어 기능을 WebSocket을 통해 호출하기 위한 확장 인터페이스 |
| runtime.toml | Core의 서버, Instance, Engine, Transport 및 Channel 구성을 정의하는 실행 설정 파일 |

---

# 2. System Overview

Sensor Studio는 센서 모델별 처리 로직과 공통 실행 환경을 분리하는 플러그인 기반 구조를 적용한다.

- **Core:** 설정을 읽고 Instance를 생성하며, UDP 또는 USB를 통해 데이터를 수신한다. Engine이 생성한 PointCloud를 채널로 변환하여 WebSocket으로 전송한다.
- **Engine:** 센서별 패킷 조립 및 프로토콜 해석을 수행한다. 필요한 경우 장비로 송신할 Transport Request를 생성하고, Extension API를 제공한다.
- **UI:** Core의 채널 정보를 수신하고 사용자가 선택한 PointCloud 채널을 구독한다. 수신 데이터를 공통 렌더링 형식으로 변환하여 3차원 화면에 표시한다.

Core와 UI는 단일 WebSocket 연결을 사용한다. 채널 광고 및 구독, PointCloud 데이터 전송, Engine 확장 기능 호출을 동일한 연결에서 처리한다. PointCloud 전송은 Foxglove WebSocket v1의 채널 및 Message Data 구조를 사용하고, Engine 확장 기능은 프로젝트 전용 JSON 제어 메시지를 사용한다.

현재 제품 범위에서 지원하는 센서 Transport는 UDP와 USB이다. TCP 및 Serial Transport, USB 자동 재탐색, UI 자동 재접속, UI의 동적 Engine 제어 폼 생성은 향후 확장 범위로 관리한다.

---

# 3. Functional Requirements: Core System

## 3.1 Strict Configuration & Isolated Instance

- **FR-CORE-001 엄격한 설정 검증:** Core는 시작 시 `runtime.toml`을 읽고 필수 설정, Instance 식별자 중복, Engine 라이브러리 경로 및 Transport 기본 조건을 검증해야 한다. 설정 파일을 읽을 수 없거나 필수 초기화에 실패한 경우 오류 원인을 기록하고 시작을 중단해야 한다. 서버 주소와 포트는 CLI 인자로 재정의할 수 있어야 한다.

- **FR-CORE-002 독립 실행 단위 구성:** Core는 설정에 정의된 각 Instance를 독립 비동기 실행 단위로 생성해야 한다. 각 Instance는 고유 식별자, Transport, Engine 실행 객체 및 데이터 전송 채널을 보유해야 한다. 복구 가능한 I/O 오류는 해당 Instance 실행 흐름에서 처리하며 다른 Instance의 정상 처리를 직접 중단시키지 않아야 한다.

- **FR-CORE-003 Instance 오류 처리 및 종료:** Instance에서 복구 가능한 Transport 오류가 발생한 경우 점진적으로 증가하는 대기 시간을 적용하여 재시도해야 한다. 시스템 종료 신호가 발생하면 신규 데이터 처리를 중지하고, Engine이 제공하는 종료 요청을 Transport로 전달한 후 자원을 해제해야 한다.

## 3.2 Core/Engine Boundary & Stability

- **FR-CORE-010 Blocking 작업 격리:** USB 및 외부 Engine 호출과 같이 blocking 가능성이 있는 작업은 비동기 네트워크 처리에 미치는 영향을 줄일 수 있도록 별도의 blocking 실행 구간에서 수행해야 한다. 동일 Engine 객체에 대한 패킷 처리와 제어 호출은 동기화되어야 한다.

- **FR-CORE-011 데이터 수신 최적화:** UDP Transport는 설정된 크기의 수신 버퍼를 재사용해야 하며, 송신지 주소와 실제 수신 길이를 함께 전달해야 한다. USB Transport는 설정된 VID, PID, Interface 및 Endpoint를 사용하여 장치를 초기화하고 Bulk 수신을 지원해야 한다.

- **FR-CORE-012 상태 기반 Engine 처리:** Engine은 Instance별 실행 상태를 보유할 수 있어야 하며, 분할 수신된 패킷의 조립, 센서 프로토콜 해석, 내부 캐시 및 장비 제어 절차를 처리할 수 있어야 한다. 처리 결과는 하나 이상의 `PointCloudFrame`과 장비 송신 요청으로 제공할 수 있어야 한다.

- **FR-CORE-013 다중 송신지 식별:** UDP로 수신한 데이터는 송신지 주소와 함께 Engine에 전달되어야 한다. Engine은 필요한 경우 동일 UDP 포트로 수신되는 여러 센서의 데이터를 송신지 기준으로 구분할 수 있어야 한다. USB 입력은 Instance를 식별할 수 있는 고정 출처 정보를 사용해야 한다.

- **FR-CORE-014 동적 설정 주입:** Core는 Instance, 센서 및 Engine별 설정을 범용 JSON 형식으로 구성하여 Engine 생성 시 전달해야 한다. Core는 센서별 설정 필드의 의미를 직접 해석하지 않아야 한다.

## 3.3 Dual-Role WebSocket Multiplexing

- **FR-CORE-020 통합 WebSocket 인터페이스:** Core는 설정된 주소와 포트에서 WebSocket 서비스를 제공해야 하며, `/ws` 경로와 `foxglove.websocket.v1` subprotocol을 지원해야 한다. 채널 광고, 구독, 데이터 전송 및 Engine 확장 제어는 동일 WebSocket 연결에서 처리해야 한다.

- **FR-CORE-021 최신 데이터 우선 전송:** Core는 전역 데이터 전달 채널의 용량을 제한해야 한다. 각 클라이언트에 대해서는 subscription별로 아직 전송되지 않은 최신 프레임을 유지하고, 동일 subscription의 새 프레임이 도착하면 이전 대기 프레임을 대체해야 한다. 이를 통해 느린 클라이언트가 과거 PointCloud를 무제한 누적하지 않도록 해야 한다.

- **FR-CORE-022 Channel Registry & Extension API:** Core는 설정된 PointCloud 채널을 Channel Registry에 등록하고 WebSocket 연결 시 채널 메타데이터를 광고해야 한다. 각 PointCloud 채널은 JSON 형식과 고속 전송용 Binary 형식을 제공할 수 있어야 한다. 또한 Instance별 Engine Extension API 목록 조회 및 API 호출 요청을 프로젝트 전용 JSON 제어 메시지로 라우팅해야 한다.

---

# 4. Functional Requirements: UI System (Flutter)

UI는 Core가 제공하는 공통 PointCloud 인터페이스를 이용하여 센서 데이터를 시각화하고 사용자 설정을 관리한다. UI는 센서 고유 패킷 프로토콜을 직접 해석하지 않는다.

## 4.1 Connection & Dynamic Registry

- **FR-UI-001:** 사용자는 WebSocket 서버 주소를 입력하고 연결 또는 연결 해제를 수행할 수 있어야 한다. UI는 연결 중, 연결됨, 연결 해제 및 오류 상태를 표시해야 하며 마지막으로 입력한 주소를 저장해야 한다. 현재 제품 범위에서는 사용자의 명시적 연결 동작을 기준으로 하며 자동 재접속은 포함하지 않는다.

- **FR-UI-002:** UI는 서버가 광고한 Channel 메타데이터를 해석하여 구독 가능한 PointCloud 채널을 센서 목록으로 표시해야 한다. 사용자가 센서 표시 여부를 변경하면 해당 Binary 채널을 구독하거나 구독 해제해야 한다.

## 4.2 Data Visualization (3D View)

- **FR-UI-010:** UI는 Foxglove Message Data frame과 PointCloud Binary payload를 디코딩해야 한다. Point 데이터 변환은 UI 응답성을 유지할 수 있도록 별도 Isolate에서 수행해야 하며, 결과는 GPU 렌더링에 적합한 연속형 실수 배열로 생성해야 한다.

- **FR-UI-011:** UI는 마우스를 이용한 회전, 이동, 확대·축소 등 3차원 카메라 조작을 제공해야 한다. Grid, Axis, Label 및 마우스 좌표 표시 여부를 사용자가 설정할 수 있어야 한다.

- **FR-UI-012:** 여러 센서 채널이 활성화된 경우 UI는 각 채널에서 수신한 최신 PointCloud를 동일한 3차원 공간에 함께 표시해야 한다. 본 기능은 좌표 데이터의 중첩 표시이며, 시간 동기화·정합 추정·객체 융합 알고리즘은 포함하지 않는다.

## 4.3 Dynamic Control Panel (Extension API)

- **FR-UI-020 Extension API 연계 범위:** Core는 Engine API 목록 조회 및 API 호출 인터페이스를 제공해야 한다. 현재 UI 제품 범위는 PointCloud 연결·구독·시각화 기능으로 한정하며, Engine 메타데이터를 이용한 동적 제어 폼 생성은 포함하지 않는다.

- **FR-UI-021 향후 확장 호환성:** 향후 동적 제어 기능을 추가할 때에는 Core가 제공하는 동일 WebSocket 연결과 프로젝트 전용 Engine 제어 메시지를 사용해야 하며, 센서별 제어 파라미터를 UI 코드에 고정하지 않는 구조를 적용해야 한다.

---

# 5. Non-Functional Requirements

- **NFR-001 운영 안정성 및 메모리 관리:** PointCloud 스트리밍 경로는 제한된 전달 용량과 최신 프레임 대체 정책을 적용해야 한다. 수신 버퍼와 연속형 데이터 배열을 활용하여 반복 할당과 불필요한 객체 생성을 줄여야 한다.

- **NFR-002 장애 처리 범위:** 복구 가능한 Transport 오류와 Engine이 정상적으로 반환한 오류는 Instance 단위로 처리해야 한다. Engine은 Core 프로세스 내부에서 동적 라이브러리로 실행되므로 프로세스 중단을 유발하는 네이티브 오류는 Instance 단위 격리 범위에 포함하지 않는다.

- **NFR-003 유연한 확장성:** 신규 센서 모델은 Core와 정의된 FFI 계약을 준수하는 Engine 동적 라이브러리 및 설정 추가를 통해 지원할 수 있어야 한다. 신규 Transport는 공통 Transport 인터페이스를 구현하는 방식으로 확장할 수 있어야 한다.

---

# 6. Revision History

| Version | Date | Description |
| --- | --- | --- |
| 1.0 | 2026-03-03 | Initial Version (Core 위주의 초안) |
| 1.1 | 2026-04-06 | Core/Engine 경계, Instance 격리, 단일 WebSocket 및 UI 역할 요구사항 반영 |
| 1.2 | 2026-07-16 | 기존 문서 구조와 요구사항 번호를 유지하면서 UDP/USB Transport, 상태 기반 Engine, Binary PointCloud, 최신 프레임 전송, UI 적용 범위 및 종료 절차를 현행화 |
