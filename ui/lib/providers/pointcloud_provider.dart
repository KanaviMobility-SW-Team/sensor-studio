import 'dart:typed_data';

import 'package:riverpod_annotation/riverpod_annotation.dart';
import 'package:vector_math/vector_math.dart' as vm;

part 'pointcloud_provider.g.dart';

class PointData {
  final vm.Vector3 position;
  final Map<String, double> attributes;

  PointData(this.position, this.attributes);
}

@riverpod
class PointCloudData extends _$PointCloudData {
  @override
  Map<String, List<PointData>> build() {
    return {};
  }

  void processPayload(String topic, Map<String, dynamic> payload) {
    final pointStride = payload['point_stride'] as int;
    final rawData = payload['data'] as List<dynamic>;
    final fields = payload['fields'] as List<dynamic>;
    final fieldReaders = <String, double Function(ByteData, int)>{};

    for (var field in fields) {
      final name = field['name'] as String;
      final offset = field['offset'] as int;
      final type = field['type'] as int;

      // Foxglove 데이터 타입
      switch (type) {
        case 2: // UINT8
          fieldReaders[name] = (data, baseOffset) =>
              data.getUint8(baseOffset + offset).toDouble();
          break;
        case 4: // UINT16
          fieldReaders[name] = (data, baseOffset) =>
              data.getUint16(baseOffset + offset, Endian.little).toDouble();
          break;
        case 7: // FLOAT32
          fieldReaders[name] = (data, baseOffset) =>
              data.getFloat32(baseOffset + offset, Endian.little);
          break;
        case 8: // FLOAT64
          fieldReaders[name] = (data, baseOffset) =>
              data.getFloat64(baseOffset + offset, Endian.little);
          break;
        default:
          // 지원하지 않는 타입은 무시하거나 0.0 처리
          fieldReaders[name] = (data, baseOffset) => 0.0;
      }
    }

    // 최소한의 xyz 위치 데이터가 없으면 렌더링 불가이므로 종료
    if (!fieldReaders.containsKey('x') ||
        !fieldReaders.containsKey('y') ||
        !fieldReaders.containsKey('z')) {
      return;
    }

    final bytes = Uint8List.fromList(rawData.cast<int>());
    final byteData = ByteData.sublistView(bytes);

    final numPoints = bytes.length ~/ pointStride;
    final List<PointData> points = [];

    for (int i = 0; i < numPoints; i++) {
      final baseOffset = i * pointStride;

      // 필수 위치값 추출
      final x = fieldReaders['x']!(byteData, baseOffset);
      final y = fieldReaders['y']!(byteData, baseOffset);
      final z = fieldReaders['z']!(byteData, baseOffset);

      // 위치(xyz)를 제외한 나머지 모든 동적 속성값 추출 (intensity 등)
      final attributes = <String, double>{};
      for (var entry in fieldReaders.entries) {
        final fieldName = entry.key;
        if (fieldName != 'x' && fieldName != 'y' && fieldName != 'z') {
          attributes[fieldName] = entry.value(byteData, baseOffset);
        }
      }

      points.add(PointData(vm.Vector3(x, y, z), attributes));
    }

    state = {...state, topic: points};
  }
}
