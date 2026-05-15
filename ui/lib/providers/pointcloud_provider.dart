import 'dart:typed_data';

import 'package:flutter/foundation.dart';

import 'package:riverpod_annotation/riverpod_annotation.dart';
import 'package:vector_math/vector_math.dart' as vm;

part 'pointcloud_provider.g.dart';

List<PointData> _parsePointCloudInBackground(Map<String, dynamic> payload) {
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
    return [];
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

  return points;
}

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

  // 💡 비동기(async) 처리로 변경하여 백그라운드 연산을 기다리도록 합니다.
  Future<void> processPayload(
    String topic,
    Map<String, dynamic> payload,
  ) async {
    // compute()를 통해 무거운 파싱 함수와 데이터를 백그라운드 스레드에서 처리
    // 메인 화면(UI)은 파싱이 끝날 때까지 멈추지 않고 부드럽게 유지
    final parsedPoints = await compute(_parsePointCloudInBackground, payload);

    if (parsedPoints.isNotEmpty) {
      // 연산이 완료된 완성품(parsedPoints)만 전달받아 UI 상태를 업데이트합니다.
      state = {...state, topic: parsedPoints};
    }
  }
}
