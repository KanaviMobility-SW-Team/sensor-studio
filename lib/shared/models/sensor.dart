// shared/models/sensor.dart
class Sensor {
  final String id;
  final String name;
  final String type; // "lidar" / "camera"
  const Sensor({required this.id, required this.name, required this.type});
}
