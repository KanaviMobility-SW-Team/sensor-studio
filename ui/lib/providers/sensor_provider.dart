import 'package:riverpod_annotation/riverpod_annotation.dart';

part 'sensor_provider.g.dart';

class SensorConfig {
  final String name;
  final bool isVisible;
  final double pointSize;
  final double opacity;
  final String colorField;
  final String colorMap;

  SensorConfig({
    required this.name,
    this.isVisible = true,
    this.pointSize = 2.0,
    this.opacity = 1.0,
    this.colorField = 'intensity',
    this.colorMap = 'turbo',
  });

  // 상태 업데이트를 위한 copyWith 메서드
  SensorConfig copyWith({
    String? name,
    bool? isVisible,
    double? pointSize,
    double? opacity,
    String? colorField,
    String? colorMap,
  }) {
    return SensorConfig(
      name: name ?? this.name,
      isVisible: isVisible ?? this.isVisible,
      pointSize: pointSize ?? this.pointSize,
      opacity: opacity ?? this.opacity,
      colorField: colorField ?? this.colorField,
      colorMap: colorMap ?? this.colorMap,
    );
  }
}

@riverpod
class SensorList extends _$SensorList {
  @override
  List<SensorConfig> build() {
    // 앱 실행 시 최초로 세팅되는 센서 리스트 (초기 상태)
    return [
      SensorConfig(name: 'lidar_roof'),
      SensorConfig(name: 'lidar_bumper'),
      SensorConfig(name: 'radar_front'),
    ];
  }

  // --- 상태 변경 메서드들 ---
  void _updateSensor(String name, SensorConfig Function(SensorConfig) updater) {
    state = [
      for (final sensor in state)
        if (sensor.name == name) updater(sensor) else sensor,
    ];
  }

  void toggleVisibility(String name, bool isVisible) {
    _updateSensor(name, (s) => s.copyWith(isVisible: isVisible));
  }

  void updatePointSize(String name, double size) {
    _updateSensor(name, (s) => s.copyWith(pointSize: size));
  }

  void updateOpacity(String name, double opacity) {
    _updateSensor(name, (s) => s.copyWith(opacity: opacity));
  }

  void updateColorField(String name, String field) {
    _updateSensor(name, (s) => s.copyWith(colorField: field));
  }

  void updateColorMap(String name, String map) {
    _updateSensor(name, (s) => s.copyWith(colorMap: map));
  }
}
