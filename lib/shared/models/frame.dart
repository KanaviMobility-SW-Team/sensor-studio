// shared/models/frame.dart
import 'dart:typed_data';

enum FrameType { pointCloud, sensorList, ack, error }

class Frame {
  final FrameType type;
  final String? sensorId;
  final Uint8List payload;
  Frame({required this.type, required this.payload, this.sensorId});
}