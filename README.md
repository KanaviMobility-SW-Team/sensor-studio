# Sensor Studio

Sensor Studio는 다양한 센서 장비로부터 데이터를 수집하고,
이를 공통 데이터 구조로 변환하여 실시간으로 시각화할 수 있도록
구성된 센서 데이터 스트리밍 및 분석 플랫폼이다.

현재는 LiDAR 센서를 중심으로 개발되고 있으며,
여러 종류의 LiDAR 장비를 통합적으로 관리하고
실시간 PointCloud 데이터를 클라이언트에 스트리밍하는 것을 목표로 한다.

---

## 주요 구성

### Core

Core는 센서 데이터 네트워킹과 시스템 안정성을 전담하는 백엔드 서버(Rust)이다.

주요 역할:

- 독립적 태스크(Task Loop)를 통한 개별 센서 인스턴스 격리 및 자체 복구 로직 수행
- UDP 등 네트워크 레이어를 통한 데이터 수신 및 버퍼 최적화(Zero-copy 지향)
- C-FFI 기반 Engine 플러그인 연동을 통한 PointCloud 데이터 파싱 위임
- Foxglove 프로토콜 기반 단일 WebSocket 다중화(Multiplexing)
- REST API를 배제하고 JSON-RPC 기반 동적 제어(Extension API) 기능 통합

---

### UI

UI는 복잡한 비즈니스 로직을 배제하고 시각화와 상호작용에 집중하는 Flutter 기반 프론트엔드이다.

주요 역할:

- 수신된 PointCloud 바이너리 데이터의 실시간 3D 시각화 (데이터 파싱 및 가공 배제)
- 단일 Foxglove WebSocket 채널을 통한 스트리밍 및 시스템 제어 통합
- 다중 센서 구독 시 동일 3D 좌표 스페이스 내 병합 시각화(Point Fusion) 지원
- Core에서 수신한 제어 메타데이터(Extension API) 기반 런타임 동적 UI 렌더링

---

## 문서

프로젝트 설계 문서는 `docs` 디렉터리에서 확인할 수 있다.

- [Software Requirements Specification (SRS)](docs/SRS.md)
- [Software Design Specification (SDS)](docs/SDS.md)
- [Software Design Description (SDD)](docs/SDD.md)

---

## 저장소 구조

 | Directory | Description |
 | --- | --- |
 | docs | Sensor Studio 요구사항 및 설계/구현 문서 (SRS, SDS, SDD) |
 | core | 모듈 격리 및 Foxglove 멀티플렉싱을 전담하는 Rust 기반 핵심 백엔드 시스템 |
 | sensor-studio-engines | 센서 모델별 프로토콜 파싱을 담당하는 C-FFI 기반 Engine 플러그인 |
 | ui | 런타임 동적 제어 컴포넌트 및 실시간 3D 렌더링에 집중하는 Flutter 클라이언트 |

---

## Author

- **Team:** Kanavi Mobility Sensor Fusion Lab S/W Team
- **Author:** 송윤수 (immsong@icloud.com)
- **Contact:** tsw_sensor@kanavi-auto.com
- **Repository:** [KanaviMobility-SW-Team/sensor-studio](https://github.com/KanaviMobility-SW-Team/sensor-studio)

---

## License

이 프로젝트의 소스 코드는 참고 및 학습 목적으로만 공개됩니다.
소스 코드를 열람하거나 다운로드할 수 있으나, **Kanavi Mobility의 사전 서면 동의 없이 사용, 복제, 수정, 병합, 배포 및 상업적 이용을 하는 것은 엄격히 금지됩니다.**

자세한 내용은 [LICENSE](LICENSE) 파일을 참고해 주시기 바랍니다.