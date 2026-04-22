# Changelog

All notable changes to the `sensor-studio-core` project will be documented in this file.

## [v0.1.0] - 2026-04-22

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
