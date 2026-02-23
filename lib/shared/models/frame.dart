// shared/models/frame.dart
import 'dart:typed_data';

enum FrameType { pointCloud, sensorList, ack, error }

class Frame {
  final FrameType? type;
  final String? sensorId;
  final Uint8List payload;

  // LiDAR point cloud data
  final List<LidarPoint>? points;

  Frame({this.type, this.sensorId, required this.payload, this.points});

  factory Frame.lidar({
    required String sensorId,
    required Uint8List payload,
    required List<LidarPoint> points,
  }) {
    return Frame(
      type: FrameType.pointCloud,
      sensorId: sensorId,
      payload: payload,
      points: points,
    );
  }
}

class LidarPoint {
  final double x;
  final double y;
  final double z;
  final int intensity;
  final int echo;

  LidarPoint({
    required this.x,
    required this.y,
    required this.z,
    required this.intensity,
    required this.echo,
  });
}
