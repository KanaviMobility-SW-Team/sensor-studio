// features/streaming/ws_protocol.dart
import 'dart:convert';
import 'dart:typed_data';
import '../../shared/models/frame.dart';

class WsProtocol {
  // 예시: [type(1)][idLen(1)][idBytes][payload...]
  static Frame decode(Uint8List bytes) {
    final typeByte = bytes[0];
    final idLen = bytes[1];
    final sensorId = idLen > 0
        ? utf8.decode(bytes.sublist(2, 2 + idLen))
        : null;
    final payload = Uint8List.fromList(bytes.sublist(2 + idLen));

    final type =
        FrameType.values[typeByte.clamp(0, FrameType.values.length - 1)];
    return Frame(type: type, sensorId: sensorId, payload: payload);
  }

  static Uint8List encodeJson(Map<String, dynamic> cmd) {
    return Uint8List.fromList(utf8.encode(jsonEncode(cmd)));
  }
}
