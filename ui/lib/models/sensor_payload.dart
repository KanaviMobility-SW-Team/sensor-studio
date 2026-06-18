import 'dart:typed_data';

/// 백그라운드 Isolate로 센서 데이터를 넘기기 위한 DTO(Data Transfer Object)
class SensorPayload {
  final Float32List buf;
  final int stride;
  final int? xIdx;
  final int? yIdx;
  final int? zIdx;
  final String? colorField;
  final int? colorFieldIdx;

  SensorPayload({
    required this.buf,
    required this.stride,
    required this.xIdx,
    required this.yIdx,
    required this.zIdx,
    required this.colorField,
    required this.colorFieldIdx,
  });
}
