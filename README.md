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

Core는 센서 데이터 수집 및 처리를 담당하는 핵심 시스템이다.

주요 역할:

- 다수의 센서 인스턴스 관리
- UDP, TCP, USB, Serial 등 다양한 인터페이스를 통한 데이터 수신
- 센서 모델별 Engine 플러그인을 통한 데이터 파싱
- 공통 데이터 구조로 변환
- WebSocket 기반 실시간 데이터 스트리밍
- REST API 기반 설정 및 상태 관리

---

### UI

UI는 센서 데이터를 시각화하는 클라이언트 애플리케이션이다.

주요 역할:

- 실시간 데이터 시각화 (PointCloud 등)
- Core와 WebSocket을 통한 데이터 수신
- 센서 상태 및 정보 표시
- 장비 모니터링 및 설정 인터페이스 제공

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
 | docs | 프로젝트 설계 문서 |
 | core | 센서 데이터 처리 시스템 |
 | ui | 센서 시각화 클라이언트 |