import 'package:riverpod_annotation/riverpod_annotation.dart';

part 'sensor_provider.g.dart';

class SensorConfig {
  final String name;
  final String displayName;
  final bool isVisible;
  final double pointSize;
  final double opacity;
  final String colorField;
  final String colorMap;
  final Map<String, (double, double)> valueRange;

  SensorConfig({
    required this.name,
    this.displayName = "",
    this.isVisible = false,
    this.pointSize = 1.5,
    this.opacity = 0.6,
    this.colorField = 'intensity',
    this.colorMap = 'viridis',
    this.valueRange = const {
      'intensity': (0.0, 0.0),
      'distance': (0.0, 0.0),
      'z': (0.0, 0.0),
    },
  });

  factory SensorConfig.fromTopic(
    String topic, {
    bool isVisible = false,
    double pointSize = 1.5,
    double opacity = 0.6,
    String colorField = 'intensity',
    String colorMap = 'viridis',
  }) {
    var splitString = topic.split('/');
    var displayName = splitString.isNotEmpty ? splitString.last : topic;
    if (displayName == "raw") {
      displayName = splitString[splitString.length - 2];
    }

    return SensorConfig(
      name: topic,
      displayName: displayName,
      isVisible: isVisible,
      pointSize: pointSize,
      opacity: opacity,
      colorField: colorField,
      colorMap: colorMap,
    );
  }

  SensorConfig copyWith({
    String? name,
    bool? isVisible,
    double? pointSize,
    double? opacity,
    String? colorField,
    String? colorMap,
    Map<String, (double, double)>? valueRange,
  }) {
    return SensorConfig(
      name: name ?? this.name,
      displayName: displayName,
      isVisible: isVisible ?? this.isVisible,
      pointSize: pointSize ?? this.pointSize,
      opacity: opacity ?? this.opacity,
      colorField: colorField ?? this.colorField,
      colorMap: colorMap ?? this.colorMap,
      valueRange: valueRange ?? this.valueRange,
    );
  }
}

@riverpod
class SensorList extends _$SensorList {
  @override
  List<SensorConfig> build() {
    return [];
  }

  void syncSensors(List<String> advertisedTopics) {
    final currentSensors = state;
    final List<SensorConfig> updatedList = [];

    for (final topic in advertisedTopics) {
      // raw 토픽(binary point cloud 데이터)이 아닌 경우 무시
      if (!topic.endsWith('/raw')) continue;

      final existing = currentSensors.where((s) => s.name == topic).firstOrNull;

      if (existing != null) {
        // 이미 리스트에 있는 센서(토픽)라면 기존 설정(투명도, 크기 등)을 그대로 유지
        updatedList.add(existing);
      } else {
        // 새로 발견된 토픽이라면 기본 설정으로 추가 (기본적으로 시각화 Off 상태로 추가)
        updatedList.add(SensorConfig.fromTopic(topic));
      }
    }

    state = updatedList;
  }

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

  void updateValueRange(String name, String field, double min, double max) {
    _updateSensor(name, (s) {
      final newValueRange = Map<String, (double, double)>.from(s.valueRange);
      newValueRange[field] = (min, max);
      return s.copyWith(valueRange: newValueRange);
    });
  }
}
