# Sensor Studio

## Software Design Description (SDD)

**Version:** 1.0  
**Last Updated:** 2026-04-13

---

# 1. Introduction

## 1.1 Purpose
본 문서는 Sensor Studio 시스템의 실제 구동 원리와 데이터, 제어 흐름 등을 정의하는 상세 설계 명세서(SDD)이다. SDS에서 정의한 시스템 구조를 기반으로, Rust `tokio` 런타임 위에서의 동시성 처리, 메모리 할당 전략, C-FFI 확장 로딩, Foxglove WebSocket 프로토콜 변환 등 구체적 구현(Implementation Details)을 모두 명시한다.

---

# 2. System Modules & Directory Structure 

Core (Rust 기반 백엔드) 시스템은 기능적 응집도를 높이고 불필요한 의존증가를 막기 위해 철저히 모듈화되어 있다.

```text
core/src/
 ├ main.rs          # 애플리케이션 진입점 및 Tokio 런타임스핀
 ├ config/          # `runtime.toml` 파싱 및 시스템 Configuration 바인딩
 ├ types/           # `PointCloudFrame`, `PointField` 등 공용 데이터 구조체 (ROS 호환)
 ├ transport/       # `UdpTransport`, 소켓 바인딩 및 Multicast 등 네트워크 인입 계층
 ├ engine/          # `Engine` Trait 정의. (Byte -> PointCloudFrame 추상화)
 ├ instance/        # `Instance` 객체와 상태머신, 격리된 Task Loop 로직 구현
 ├ runtime/         # FFI(`adapter.rs`, `ffi.rs`) 및 `EngineExtensionRegistry` (확장 제어)
 └ stream/          # Foxglove WebSocket 멀티플렉싱, `ChannelRegistry`, `Broadcast` 처리
```

---

# 3. Component Design & Inter-operation

## 3.1 Bootstrap & Resource Allocation (`main.rs`)

시스템 구동 시 다음 순서에 따라 자원을 할당하고 스레드를 분배한다.
1. **Config 파싱:** CLI 인자 및 `runtime.toml`을 파싱해 `InstanceConfig` 정보들을 얻는다.
2. **Logger 시스템 초기화:** `env_logger` 기반의 정규화된 로깅 시스템을 초기화하여 Core 런타임뿐 아니라 C-FFI 플러그인 경계를 넘어 통합된 로그 수집 환경을 구성한다.
3. **Global Channel 생성:** 단일 연결망을 위한 `tokio::sync::broadcast::channel<WebSocketMessage>(capacity)`를 생성한다.
4. **Extension / Channel Registry 생성:** UI와의 통신 메타데이터를 관리하기 위해 `ChannelRegistry`(스트리밍 Topic 목적)와 `EngineExtensionRegistry`(제어 목적)를 초기화한다. 
5. **JSON-FFI 주입:** 기존 파일 경로 기반의 설정을 탈피하고, 파싱된 센서 및 설정 정보를 포함한 단일 JSON 페이로드(`config_json`)를 생성하여 Engine FFI 초기화 함수에 주입한다.
6. **Task 분배 (Zero Blast Radius):**
   - **`WebSocketServer::serve`:** 단 1개의 태스크에서 수신 커넥션 및 송신을 관리한다.
   - **`Instance Loop` 생성:** `runtime_config.instances` 개수만큼 반복하며 `tokio::spawn`을 이용해 독립적인 Task를 각각 분리(Isolate)하여 실행시킨다.

## 3.2 Transport Layer (`transport/udp.rs`)
최상단 네트워크 레이어로서 센서 장비의 패킷을 받아 Engine에 전달한다.
- **Multicast 및 소켓 설정:** UDP Socket 생성 시 `SO_REUSEADDR` 설정과 Non-blocking 제어를 수행하며, 설정에 의해 IGMP 멀티캐스트(`join_multicast_v4`) 참여를 동적으로 지원한다.
- **Buffer Management:** `recv_buffer: Vec<u8>`을 멤버 변수로 고정 할당하여 소켓 `recv_from` 과정에서 매 프레임별 메모리 할당(Allocation)을 방지한다.
- **Zero-copy 지향:** 수신된 유효 길이 버퍼는 `bytes::Bytes::copy_from_slice`로 복제하여 Reference Counting 기반 구조체인 `Bytes` 형태로 변환, 소유권 이동 시의 부하를 없앤다.

## 3.3 The Engine Trait & FFI Adapter (`engine/mod.rs`, `runtime/ffi.rs`)
센서 프로토콜을 파싱하는 외부 공유 라이브러리를 바인딩한다.
- **Trait Definition:** 
  ```rust
  pub trait Engine: Send {
      fn id(&self) -> &str;
      fn process(&mut self, chunk: Bytes, sender_addr: SocketAddr) -> Vec<PointCloudFrame>;
  }
  ```
  단발성, 무상태 형태로 Byte Chunk와 패킷 송신지 정보(`sender_addr`)를 받아 다중 센서 Multiplexing 처리를 거친 후 PointCloud 패킷 리스트로 변환한다.
- **Synchronization (Control):** `SharedEngineExtension`는 `Arc<Mutex<FfiEngineAdapter>>` 구조로 래핑되어 있다. `WebSocketServer`를 통한 클라이언트 제어(JSON-RPC API 호출)가 들어오면 Mutex Lock을 취해 FFI(C 인터페이스) 함수를 안전하게 호출한다.

## 3.4 Isolated Instance Loop (`instance/mod.rs`)
`Instance`는 1개의 Transport와 1개의 Engine을 캡슐화한 구조체다.
- **State Machine:** `Created` ➔ `Running` <➔ `Error` (장애 발생 시)
- **Execution Flow (`run_once`):** 
  Transport에서 `read_chunk` (Async) $\rightarrow$ `chunk`를 `Engine.process`에 주입 (Sync) $\rightarrow$ `Vec<PointCloudFrame>` 반환
- **Exponential Backoff:** 
  - 특정 Instance Loop가 `Error`를 반환하면 루프를 종료하지 않고 지수 백오프(`backoff = (backoff * 2).min(MAX_BACKOFF)`)를 통해 Task 내부에서 100ms ~ 최고 5초 대기 후 자체 복구를 시도한다.

## 3.5 Foxglove Protocol Stream multiplexer (`stream/websocket/`)
UI 개발과의 완벽한 경계를 위한 통신 인터페이스 계층.
- **Data Serialization:** `PointCloudFrame` 내부의 `fields`(Ros-Type PointField DataType) 및 바이너리 Payload(`Vec<u8>`)를 Foxglove WebSocket 표준 프로토콜(v1) 바이너리 메시지로 인코딩한다.
- **RPC Extension Routing:** 
  JSON 스펙으로 들어오는 메타데이터(UI 컴포넌트 정보, 센서 조작) 송수신은 `ExtensionRegistry`를 거쳐 파서 엔진 플러그인(FFI)으로 전달된다.
- **OOM Defense (Drop-Lagged):** 
  `tokio::sync::broadcast`의 성질을 이용해 UI 클라이언트 웹뷰가 렌더링에 병목이 걸려 송신 큐가 포화 상태(Bound capacity 초과)에 도달하면 가장 오래된(oldest) 메시지를 Drop(Lagged 에러 처리)하여 서버 메모리 가용성을 항시 보장한다.

---

# 4. Data Structures

시스템 내부 전역에서 사용되는 중심 데이터 구조 (`types/pointcloud.rs`)

```rust
pub struct PointCloudFrame {
    pub timestamp_ns: u64,
    pub frame_id: String,
    pub width: u32,
    pub height: u32,
    pub point_step: u32,
    pub row_step: u32,
    pub fields: Vec<PointField>, // x, y, z, intensity 등의 메모리 오프셋/타입 
    pub is_dense: bool,
    pub data: Vec<u8>,           // Raw 파싱된 Point 배열 데이터
}
```
위 오프셋 기반의 ROS 호환 자료구조를 통해, Flutter UI 클라이언트는 별도의 센서 프로토콜 이해 없이 Three.js 나 Canvas 등 3D 시각화 계층에서 `data` 버퍼를 Off-set으로 끊어서 그대로 렌더링에 투입할 수 있다.