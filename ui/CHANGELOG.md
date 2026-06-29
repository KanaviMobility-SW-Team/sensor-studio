# Changelog

All notable changes to the `sensor-studio-ui` project will be documented in this file.

## [v0.1.0] - 2026-06-29

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