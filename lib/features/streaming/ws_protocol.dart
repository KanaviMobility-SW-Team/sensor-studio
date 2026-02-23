// features/streaming/ws_protocol.dart
import 'dart:convert';
import 'dart:typed_data';
import '../../shared/models/frame.dart';

class WsProtocol {
  static Frame decodeLidarFrame(Uint8List bytes) {
    // print("Decoding LiDAR frame, byte length: ${bytes.length}");
    // print(bytes.sublist(0, 20));

    // 헤더 파싱
    final opcode = bytes[0];
    final subscriptionId = ByteData.view(
      bytes.buffer,
    ).getUint32(1, Endian.little);
    final timestamp = ByteData.view(bytes.buffer).getUint64(5, Endian.little);

    // print(
    //   'Header: opcode=$opcode, subscriptionId=$subscriptionId, timestamp=$timestamp',
    // );

    // Payload(JSON) 추출
    final jsonStart = 13;
    final jsonString = utf8.decode(bytes.sublist(jsonStart));
    final json = jsonDecode(jsonString) as Map<String, dynamic>;

    // print(jsonString.substring(0, 100));

    // 기존 포인트 파싱 코드
    final frameId = json['frame_id'] as String? ?? 'unknown';
    final pointSize = json['point_stride'] as int;

    final dataList = json['data'] as List<dynamic>;
    final numPoints = dataList.length ~/ pointSize;

    final data = Uint8List.fromList(dataList.cast<int>());
    final byteData = ByteData.view(data.buffer);

    List<LidarPoint> points = [];
    for (int i = 0; i < numPoints; i++) {
      final pointOffset = i * pointSize;
      final x = byteData.getFloat32(pointOffset, Endian.little);
      final y = byteData.getFloat32(pointOffset + 4, Endian.little);
      final z = byteData.getFloat32(pointOffset + 8, Endian.little);
      final intensity = byteData.getUint32(pointOffset + 12, Endian.little);
      final echo = data[pointOffset + 16];
      points.add(
        LidarPoint(x: x, y: y, z: z, intensity: intensity, echo: echo),
      );
    }

    // print("Decoded ${points.length} points");
    return Frame.lidar(sensorId: frameId, payload: bytes, points: points);
  }

  static Uint8List encodeJson(Map<String, dynamic> cmd) {
    return Uint8List.fromList(utf8.encode(jsonEncode(cmd)));
  }
}
