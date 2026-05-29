import 'dart:convert';

import 'package:flutter/foundation.dart';

import 'package:riverpod_annotation/riverpod_annotation.dart';

part 'pointcloud_provider.g.dart';

typedef PointCloudBuffer = ({
  Float32List buf,
  Map<String, int> fields,
  int stride,
});

PointCloudBuffer _parsePointCloudBinaryInBackground(Uint8List bytes) {
  final byteData = ByteData.sublistView(bytes);
  int cursor = 0;

  final pointStride = byteData.getUint32(cursor, Endian.little);
  cursor += 4;

  final numFields = byteData.getUint8(cursor);
  cursor += 1;

  final fieldReaders = <String, double Function(ByteData, int)>{};

  for (int i = 0; i < numFields; i++) {
    final nameLen = byteData.getUint8(cursor);
    cursor += 1;

    final name = utf8.decode(bytes.sublist(cursor, cursor + nameLen));
    cursor += nameLen;

    final fieldOffset = byteData.getUint16(cursor, Endian.little);
    cursor += 2;

    final type = byteData.getUint8(cursor);
    cursor += 1;

    final fo = fieldOffset;
    switch (type) {
      case 1: // UINT8
        fieldReaders[name] = (data, base) =>
            data.getUint8(base + fo).toDouble();
        break;
      case 2: // INT8
        fieldReaders[name] = (data, base) => data.getInt8(base + fo).toDouble();
        break;
      case 3: // UINT16
        fieldReaders[name] = (data, base) =>
            data.getUint16(base + fo, Endian.little).toDouble();
        break;
      case 4: // INT16
        fieldReaders[name] = (data, base) =>
            data.getInt16(base + fo, Endian.little).toDouble();
        break;
      case 5: // INT32
        fieldReaders[name] = (data, base) =>
            data.getInt32(base + fo, Endian.little).toDouble();
        break;
      case 6: // UINT32
        fieldReaders[name] = (data, base) =>
            data.getUint32(base + fo, Endian.little).toDouble();
        break;
      case 7: // FLOAT32
        fieldReaders[name] = (data, base) =>
            data.getFloat32(base + fo, Endian.little);
        break;
      case 8: // FLOAT64
        fieldReaders[name] = (data, base) =>
            data.getFloat64(base + fo, Endian.little);
        break;
      default:
        fieldReaders[name] = (data, base) => 0.0;
    }
  }

  if (!fieldReaders.containsKey('x') ||
      !fieldReaders.containsKey('y') ||
      !fieldReaders.containsKey('z')) {
    return (buf: Float32List(0), fields: {}, stride: 0);
  }

  final pointByteData = ByteData.sublistView(bytes, cursor);
  final numPoints = (bytes.length - cursor) ~/ pointStride;

  final allFields = fieldReaders.keys.toList()
    ..sort((a, b) {
      const p = {'x': 0, 'y': 1, 'z': 2};
      return (p[a] ?? 99).compareTo(p[b] ?? 99);
    });
  final stride = allFields.length;
  final fieldIndex = {for (int i = 0; i < stride; i++) allFields[i]: i};
  final buf = Float32List(numPoints * stride);

  for (int i = 0; i < numPoints; i++) {
    final base = i * pointStride;
    for (int j = 0; j < stride; j++) {
      buf[i * stride + j] = fieldReaders[allFields[j]]!(pointByteData, base);
    }
  }

  return (buf: buf, fields: fieldIndex, stride: stride);
}

PointCloudBuffer _parsePointCloudInBackground(Map<String, dynamic> payload) {
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
    return (buf: Float32List(0), fields: {}, stride: 0);
  }

  final bytes = Uint8List.fromList(rawData.cast<int>());
  final pointByteData = ByteData.sublistView(bytes);
  final numPoints = bytes.length ~/ pointStride;

  final allFields = fieldReaders.keys.toList()
    ..sort((a, b) {
      const p = {'x': 0, 'y': 1, 'z': 2};
      return (p[a] ?? 99).compareTo(p[b] ?? 99);
    });
  final stride = allFields.length;
  final fieldIndex = {for (int i = 0; i < stride; i++) allFields[i]: i};
  final buf = Float32List(numPoints * stride);

  for (int i = 0; i < numPoints; i++) {
    final base = i * pointStride;
    for (int j = 0; j < stride; j++) {
      buf[i * stride + j] = fieldReaders[allFields[j]]!(pointByteData, base);
    }
  }

  return (buf: buf, fields: fieldIndex, stride: stride);
}

@riverpod
class PointCloudData extends _$PointCloudData {
  // 토픽별 처리 중 여부 및 대기 중인 최신 페이로드
  final _isProcessing = <String, bool>{};
  final _pendingPayloads = <String, Map<String, dynamic>>{};
  final _pendingBinaryPayloads = <String, Uint8List>{};

  @override
  Map<String, PointCloudBuffer> build() => {};

  Future<void> processPayload(
    String topic,
    Map<String, dynamic> payload,
  ) async {
    // 이미 처리 중이면 최신 프레임만 보관하고 중간 프레임은 드롭
    if (_isProcessing[topic] == true) {
      _pendingPayloads[topic] = payload;
      return;
    }

    _isProcessing[topic] = true;
    try {
      final parsedPoints = await compute(_parsePointCloudInBackground, payload);
      if (parsedPoints.buf.isNotEmpty) {
        state = {...state, topic: parsedPoints};
      }
    } finally {
      _isProcessing[topic] = false;
      // 처리 완료 후 대기 중인 최신 프레임이 있으면 즉시 처리
      final pending = _pendingPayloads.remove(topic);
      if (pending != null) {
        processPayload(topic, pending);
      }
    }
  }

  Future<void> processBinaryPayload(String topic, Uint8List bytes) async {
    // 이미 처리 중이면 최신 프레임만 보관하고 중간 프레임은 드롭
    if (_isProcessing[topic] == true) {
      _pendingBinaryPayloads[topic] = bytes;
      return;
    }

    _isProcessing[topic] = true;
    try {
      final parsedPoints = await compute(
        _parsePointCloudBinaryInBackground,
        bytes,
      );
      if (parsedPoints.buf.isNotEmpty) {
        state = {...state, topic: parsedPoints};
      }
    } finally {
      _isProcessing[topic] = false;
      // 처리 완료 후 대기 중인 최신 프레임이 있으면 즉시 처리
      final pending = _pendingBinaryPayloads.remove(topic);
      if (pending != null) {
        processBinaryPayload(topic, pending);
      }
    }
  }
}
