# Sensor Studio UI

`Sensor Studio UI`는 Sensor Studio Core와 연동되는 Flutter 기반 데스크톱 UI 애플리케이션입니다.

Core에서 WebSocket으로 전달되는 실시간 Point Cloud 데이터를 수신하고, OpenGL 기반 3D 뷰어를 통해 시각화합니다. 또한 센서 표시 상태, 렌더링 옵션, 대시보드 레이아웃, 시스템 로그 등을 UI에서 제어할 수 있습니다.

## 주요 기능

* 실시간 Point Cloud 시각화
* Sensor Studio Core와 WebSocket 연결
* Foxglove 호환 채널 advertise 및 subscribe / unsubscribe 처리
* Binary Point Cloud payload 파싱
* `point_glass_opengl` 기반 OpenGL 3D 렌더링
* 센서 설정 사이드바

  * 표시 / 숨김
  * 포인트 크기
  * 투명도
  * 컬러 범위
* Grid / Axis / Label 표시 제어
* 하단 시스템 로그 콘솔
* 사이드바 및 하단 콘솔 레이아웃 제어
* JSON 기반 UI 설정 저장 및 복원

  * 대시보드 레이아웃 상태
  * Grid 설정
  * 마지막으로 사용한 WebSocket 주소

## 요구 사항

* Flutter SDK
* Linux 또는 Windows 데스크톱 환경

Linux 데스크톱 빌드를 사용하는 경우 Flutter의 Linux desktop 지원이 활성화되어 있어야 합니다.

```bash
flutter config --enable-linux-desktop
```

## 실행 방법

의존성 설치:

```bash
flutter pub get
```

앱 실행:

```bash
flutter run
```

## 빌드 방법

### Linux x86_64

```bash
./build_linux_x86_64.sh
```

### Windows x86_64

```cmd
build_windows_x86_64.cmd
```

빌드 결과물은 `dist/` 디렉터리에 생성됩니다.

예시:

```text
dist/sensor-studio-ui-v0.1.0-linux-x86_64/
dist/sensor-studio-ui-v0.1.0-linux-x86_64.tar.gz
dist/sensor-studio-ui-v0.1.0-windows-x86_64/
dist/sensor-studio-ui-v0.1.0-windows-x86_64.zip
```

## 버전 관리

UI 버전은 `pubspec.yaml`에서 관리합니다.

```yaml
version: 0.1.0+1
```

릴리스 산출물 이름에는 Flutter build number를 제외한 semantic version만 사용합니다.

```text
0.1.0+1 -> v0.1.0
```