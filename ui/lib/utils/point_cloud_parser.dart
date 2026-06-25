import 'dart:typed_data';
import 'dart:math';

import 'package:ui/models/sensor_payload.dart';

/// Isolate(백그라운드)에서 실행될 순수 파싱 함수
/// GPU 셰이더 처리를 위해 [X, Y, Z, Value] 4개의 Float로 압축합니다.
Float32List buildGLPointCloudData(List<SensorPayload> payloads) {
  int totalPoints = 0;
  for (final p in payloads) {
    if (p.stride > 0) totalPoints += p.buf.length ~/ p.stride;
  }

  // 점 1개당 4개의 float 데이터 (X, Y, Z, Value)
  final glData = Float32List(totalPoints * 4);
  int offset = 0;

  for (final p in payloads) {
    if (p.xIdx == null || p.yIdx == null || p.zIdx == null || p.stride <= 0) {
      continue;
    }

    final numPoints = p.buf.length ~/ p.stride;

    for (int i = 0; i < numPoints; i++) {
      final base = i * p.stride;
      final x = p.buf[base + p.xIdx!];
      final y = p.buf[base + p.yIdx!];
      final z = p.buf[base + p.zIdx!];
      final distance = sqrt(x * x + y * y + z * z);

      // 색상의 기준이 될 값 (Z축 높이 / 거리 또는 Intensity)
      double value = distance; // 기본값은 거리
      if (p.colorFieldIdx != null) {
        value = p.buf[base + p.colorFieldIdx!];
      } else {
        if (p.colorField == 'z') {
          value = z; // Z축 높이로 색상 결정
        }
      }

      glData[offset++] = x;
      glData[offset++] = y;
      glData[offset++] = z;
      glData[offset++] = value;
    }
  }

  return glData;
}
