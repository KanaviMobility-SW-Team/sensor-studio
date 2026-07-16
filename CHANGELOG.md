# Changelog

All notable changes to the `sensor-studio` project will be documented in this file.

## [v0.4.0] - 2026-07-16

Sensor Studio Core와 UI의 버전 체계를 통합한 첫 번째 제품 릴리스입니다.

### Features

* **USB Control Transfer 지원**: Engine이 Transport Request별로 `Bulk` 또는 `Control` 전송 방식을 지정할 수 있도록 확장하고, USB Control Setup Packet을 기반으로 Control Transfer를 수행하도록 구현.
* **마우스 좌표 표시 설정 추가**: 3D 뷰어의 마우스 좌표 오버레이 표시 여부를 제어할 수 있는 토글 버튼을 추가하고, 설정값이 애플리케이션 재실행 후에도 유지되도록 구현.

### Improvements

* **Core/UI 버전 체계 통합**: 기존에 개별적으로 관리하던 Core와 UI 버전을 Sensor Studio 제품 버전으로 통합하여 릴리스 관리 체계를 단순화.
* **Shutdown Request 처리 구조 개선**: 단일 Shutdown Payload 방식에서 `TransportRequest` 기반의 다중 요청 처리 방식으로 변경하고, 종료 요청에서도 USB Bulk 및 Control 전송 방식을 사용할 수 있도록 개선.
* **OpenGL 뷰어 기능 확장**: 마우스 좌표 표시 기능을 지원하는 `point_glass_opengl` v0.2.0으로 의존성을 업데이트.

### Docs

* **루트 CHANGELOG 추가**: 기존에 Core와 UI에서 개별 관리하던 변경 이력을 Sensor Studio 제품 단위로 관리하도록 루트 `CHANGELOG.md`를 추가.
* **프로젝트 문서 현행화**: 현재 시스템 구성과 동작에 맞게 SRS, SDS 및 SDD 문서를 정비.

---

## [ui-v0.1.0] - 2026-06-29

### Initial Release

`sensor-studio-ui`의 첫 번째 릴리스입니다.  
Core 엔진에서 전달되는 실시간 Point Cloud 데이터를 WebSocket으로 수신하고, 3D 뷰어를 통해 시각화하기 위한 Flutter 기반 UI 애플리케이션을 제공합니다.

### Features

* **Dashboard UI 구현**: 상단 헤더, 좌측 센서 설정 패널, 메인 3D 뷰포트, 하단 로그 콘솔로 구성된 대시보드 화면을 추가.
* **WebSocket 연동 지원**: Core 엔진과 WebSocket으로 연결하고, 연결 상태를 UI에서 실시간으로 확인할 수 있도록 구현.
* **실시간 Point Cloud 시각화**: 수신된 Point Cloud 데이터를 3D 뷰어에 렌더링하는 시각화 파이프라인을 추가.
* **Foxglove 채널 연동**: advertise 메시지 기반으로 토픽을 동기화하고, 센서별 subscribe / unsubscribe 흐름을 지원.
* **Binary Point Cloud 처리**: 대용량 Point Cloud 데이터를 효율적으로 처리하기 위한 binary payload 파싱 경로를 구현.
* **OpenGL 기반 뷰어 적용**: `point_glass_opengl` 기반 렌더링 파이프라인을 적용하여 3D 시각화 성능과 확장성을 개선.
* **센서 설정 패널 추가**: 센서별 visibility, point size, opacity, color range 등을 UI에서 조절할 수 있도록 지원.
* **Grid / Axis / Label 제어**: 3D 뷰어의 grid, axis, label 표시 옵션을 제어할 수 있도록 추가.
* **System Log Console 추가**: 앱 내부 로그를 하단 콘솔에서 확인하고 초기화할 수 있는 로그 UI를 제공.
* **UI Layout 제어 추가**: 좌측 사이드바와 하단 콘솔을 열고 닫을 수 있는 layout control 기능을 추가.
* **사용자 설정 저장 / 복원**: grid 설정, dashboard layout 상태, WebSocket 주소 등을 JSON 기반 설정 파일로 저장하고 앱 재실행 시 복원.

### Improvements

* **Rendering Pipeline 최적화**: Point 객체 생성 대신 `Float32List` 기반 버퍼 처리 구조를 적용하여 렌더링 데이터 처리 비용을 줄임.
* **High-frequency Update 안정화**: 고주파 Point Cloud 스트림에서 frame dropping 및 background compute 처리를 적용하여 UI 응답성을 개선.
* **Interaction Flow 개선**: loading overlay, WebSocket 주소 입력 dialog, on/off segmented control 등을 추가하여 조작 흐름을 정리.
* **상태 관리 구조 개선**: Riverpod 기반 provider 구조를 적용하여 센서 상태, WebSocket 상태, grid 설정, layout 상태를 분리 관리.

---

## [core-v0.3.0] - 2026-06-26

### Features

* **Binary Point Cloud Streaming 지원**: `ChannelEncoder`에 Binary 모드를 추가하여 기존 JSON 기반 스트림과 함께 Binary 기반 포인트 클라우드 스트리밍을 지원하도록 확장.
* **JSON/Binary Dual Channel 지원**: 채널 자동 생성 구조를 통해 동일한 포인트 클라우드 데이터를 JSON/Binary 두 가지 형식으로 구독 및 송신할 수 있도록 개선.

### Improvements

* **WebSocket Outbound 전송 구조 개선**: 클라이언트별 outbound queue 방식 대신 subscription별 최신 프레임만 유지하는 state 기반 전송 구조로 변경.
* **실시간 스트리밍 안정성 개선**: 센서 데이터는 `Notify` 기반으로 최신 상태만 flush 하도록 정리하여 느린 클라이언트나 고주파 데이터 입력 상황에서도 최신 프레임 중심으로 동작하도록 개선.
* **불필요한 Payload Encoding 최소화**: subscription 매칭 이후 실제 전송이 필요한 경우에만 JSON/Binary 인코딩을 수행하도록 변경하여 불필요한 인코딩 오버헤드 감소.

---

## [core-v0.2.0] - 2026-05-14

### Features
* **USB Transport 구현**: `rusb` 기반의 USB Bulk Read/Write 기능을 추가하여 Wide-30m ToF 등 USB 기반 장치 지원 확장.
* **Graceful Shutdown 및 CancellationToken 도입**: 시스템 종료 시 인스턴스 및 엔진의 안전한 종료(Join)를 보장하는 프로세스 구현.
* **Transport Request/Response 인터페이스 확장**: FFI 경로를 통한 엔진-코어 간 양방향 통신(Polling) 및 Shutdown Payload 전송 기능 추가.
* **USB 설정 모델링**: vendor/product ID 및 endpoint 설정을 위한 전용 구조체 정의 및 16진수 문자열 파싱 로직 반영.

### Improvements
* **Transport 계층 추상화**: 인스턴스 실행 경로에서 특정 구체 타입 대신 공통 `Transport` Trait 객체를 사용하도록 구조 일반화.
* **비동기 워커 블로킹 방지**: USB I/O 등 Blocking 작업 구간을 `block_in_place`로 격리하여 런타임 스케줄러 안정성 확보.
* **ID 체계 통합**: UDP/USB 구분 없이 `instance_id`를 기반으로 한 단일 Transport 식별 및 관리 체계로 정리.
* **설계 구조 개선**: Runtime Factory가 설정에 따라 UDP/USB Transport를 동적으로 생성하여 주입하도록 로직 고도화.

### Fixes
* **런타임 루프 안정성 강화**: 단일 사이클 내 처리 가능한 Transport 요청 수 제한을 통해 잠재적인 무한 루프 및 점유 방지.
* **로그 노이즈 제거 및 포맷 최적화**: 불필요한 로그 출력 제거 및 USB 소스 ID의 16진수 표기 표준화.
* **설정 검증 로직 보강**: 미구현된 USB 설정 항목에 대해 초기화 단계에서 명확한 실패(Fail-fast)를 반환하도록 수정.

---

## [core-v0.1.0] - 2026-04-22

### Initial Release
Sensor Studio Core의 첫 번째 릴리스입니다. Rust 기반의 고성능 멀티 센서 데이터 처리 및 외부 엔진 플러그인(FFI), 그리고 Foxglove 호환 실시간 데이터 스트리밍을 지원합니다.

### Features
* **비동기 런타임 및 파이프라인 구축**: `tokio` 기반으로 UDP 데이터 수신부, 엔진 처리, 스트리밍 퍼블리셔 파이프라인 통합.
* **동적 환경 설정**: `runtime.toml` 또는 JSON 페이로드 기반으로 여러 인스턴스, 엔진 채널, 포트 매핑을 동적으로 할당하고 검증.
* **멀티캐스트 UDP Transport**: 네트워크 인터페이스 자동 탐색 및 UDP 멀티캐스트 그룹 조인, 송신자 주소 (sender IP/Port) 트래킹 기능 구현.
* **C-FFI 엔진 플러그인 지원**: `.so` 등 외부 엔진 라이브러리의 동적 로딩, 버전 읽기, 확장 API 호출(Extension API) 지원.
* **엔진 로거 통합**: FFI 콜백 함수를 통해 외부 엔진의 내부 로그 이벤트를 Core 런타임의 `tracing` 로그로 통합하여 출력하도록 지원.
* **Foxglove 호환 실시간 스트림**: Foxglove WebSocket 프로토콜 핸드셰이크, 채널 메타데이터 Advertise, PointCloud 프레임 포맷 변환 및 실시간 브로드캐스트 지원.
* **웹소켓 제어 경로**: 웹소켓을 통해 외부 엔진의 특화된 확장 API(Extension API)를 직접 호출 및 조회할 수 있는 기능 추가.

### Improvements
* **메모리 할당 최적화**: UDP 수신부에서 매번 발생하는 `vec!` 할당을 제거하고 고정 수신 버퍼를 재사용하도록 성능 개선.
* **구조화된 로깅**: `tracing` 크레이트를 도입하여 콘솔 출력 로그 노이즈 최소화 및 일별 롤링 방식의 파일 로그 구성.
* **비동기 블로킹 방지**: FFI 엔진 어댑터 호출 구간을 `block_in_place`로 감싸서 비동기 런타임 워커가 블로킹되는 영향을 상쇄.

### Fixes
* **OOM 방지**: 처리 속도가 느린 웹소켓 클라이언트 접속 시 메모리 고갈 방지를 위해 큐 크기 제한을 두고 가장 오래된 데이터부터 Drop 하도록 예외 처리.
* **에러 복구**: 런타임 엔진 루프 동작 중 오류 발생 시 즉시 종료하지 않고 지수 백오프(Backoff) 알고리즘을 사용한 재시도 및 복구 추가.
* **필드 타입 매핑 수정**: Foxglove 로 송신하는 পয়인트 클라우드 타입 구조에서 일부 필드가 알려지지 않은 타입(UNKNOWN)으로 표시되는 버그 수정.

### Build & Docs
* **Docker Multi-Arch Build**: Linux x86_64, aarch64 환경에 대응하기 위해 컨테이너 기반 릴리스 바이너리 빌드 스크립트(`build.sh`) 환경 구성 완료.
* **Architecture Docs**: 소프트웨어 요구사항(SRS), 시스템 설계(SDS), 상세 설계(SDD) 등 주요 아키텍처 문서를 현재 동시성 모델 기반에 맞게 최신화.
