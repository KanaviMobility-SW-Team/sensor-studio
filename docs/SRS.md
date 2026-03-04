# LiDAR Multi-Engine Streaming Core

## Software Requirements Specification (SRS)

**Version:** 1.0\
**Last Updated:** 2026-03-03

---

# 1. Introduction

## 1.1 Purpose

본 문서는 LiDAR Multi-Engine Streaming Core 시스템의 소프트웨어
요구사항을 정의한다.

본 시스템은 다양한 통신 인터페이스(UDP, TCP, USB, Serial 등)를 통해
연결된 LiDAR 장비들로부터 데이터를 수집하고, 공통 PointCloud 구조로
변환하여 UI에 실시간 스트리밍하는 것을 목표로 한다.

## 1.2 Requirement Conventions

### 1.2.1 Requirement Level

본 문서에서 요구사항의 중요도는 다음과 같이 구분한다.

- Mandatory (M): 반드시 구현되어야 하는 요구사항
- Optional (O): 구현 선택 사항

## 1.3 Definitions

  | Term             | Description |
  | ---              | --- |
  | Core             | 중앙 처리 시스템 |
  | Engine           | LiDAR 모델별 Framing/Parsing 플러그인 |
  | Lidar Instance   | 하나의 물리적 LiDAR 연결 단위 |
  | RawChunk         | Transport에서 수신한 원시 바이트 데이터 |
  | PointCloudFrame  | 공통 구조로 정규화된 포인트클라우드 데이터 |
  
---

# 2. System Overview

본 시스템은 다음과 같은 기능을 수행한다.

-   다중 LiDAR 인스턴스 관리
-   Engine 플러그인 기반 Framing/Parsing 구조
-   공통 PointCloud 데이터 모델 제공
-   WebSocket 기반 실시간 스트리밍
-   REST API 기반 설정 및 상태 관리

본 시스템은 LiDAR 하드웨어, 펌웨어, 네트워크 인프라 및 UI 렌더링 성능을 직접적으로 책임지지 않는다.

---

# 3. Functional Requirements

## 3.1 LiDAR Instance Management

-   FR-LIDAR-001 (M): 다수 LiDAR 인스턴스 동시 관리
-   FR-LIDAR-002 (M): 각 인스턴스 고유 ID 보유
-   FR-LIDAR-003 (M): UDP/TCP/USB/Serial 등 다양한 입력 포트 지원
-   FR-LIDAR-004 (M): 설정 파일 기반 인스턴스 생성
-   FR-LIDAR-005 (M): 자동 재연결 지원

## 3.2 Data Processing

-   FR-DATA-001 (M): Raw byte 데이터 수신
-   FR-DATA-002 (M): Raw 데이터는 지정 Engine으로 전달
-   FR-DATA-003 (M): Engine은 PointCloudFrame 변환 수행

## 3.3 Streaming

-   FR-STREAM-001 (M): WebSocket 바이너리 스트리밍 제공
-   FR-STREAM-002 (M): `/pointcloud/{lidar_id}` 토픽 구독 지원
-   FR-STREAM-003 (M): 다중 클라이언트 동시 접속 지원

## 3.4 Control & Configuration

-   FR-CTRL-001 (M): LiDAR 목록 조회 지원
-   FR-CTRL-002 (O): LiDAR별 설정 조회 지원
-   FR-CTRL-003 (O): LiDAR별 설정 변경 지원
-   FR-CTRL-004 (M): LiDAR 상태 조회 지원
-   FR-CTRL-005 (M): Engine 목록 조회 지원

---

# 4. Non-Functional Requirements

- NFR-001: 시스템은 실시간 데이터 스트리밍을 지원해야 한다.
- NFR-002: 시스템은 병목 상황에서 지연 누적을 방지해야 한다(프레임 드롭 허용).
- NFR-003: 시스템은 입력/연결 오류에 대해 자동 복구(reconnect 등)를 지원해야 한다.
- NFR-004: Engine 오류가 Core 전체 중단으로 이어지지 않도록 설계되어야 한다.
- NFR-005: 시스템은 멀티 LiDAR 및 신규 Engine 추가에 대해 확장 가능해야 한다.
- NFR-006: 시스템은 24/7 운영을 고려한 안정성(메모리 누수, 장기 동작)을 유지해야 한다.

---

# 5. Data Model

## 5.1 Transport Input

LiDAR 장비로부터 수신되는 데이터는
장비별 프로토콜에 따른 raw byte stream이다.

## 5.2 PointCloudFrame

UI로 스트리밍되는 포인트클라우드 데이터는
ROS `sensor_msgs/PointCloud2` 메시지 구조와
구조적으로 호환되는 스키마를 따라야 한다.

---

# 6. Revision History

| Version | Date       | Description     |
| ---     | ---        | ---             |
| 1.0     | 2026-03-03 | Initial Version |