// features/sensor/sensor_providers.dart
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../../shared/models/sensor.dart';

class SensorListNotifier extends StateNotifier<List<Sensor>> {
  SensorListNotifier() : super(const []);

  void updateFromServer(Map<String, dynamic> json) {
    final channels = (json['channels'] as List<dynamic>?) ?? [];
    final parsed = channels
        .map((c) {
          final ch = Map<String, dynamic>.from(c as Map);
          return Sensor(
            id: (ch['id'] ?? '').toString(),
            name: (ch['metadata']?['model'] ?? ch['topic']) as String,
            type: ((ch['metadata']?['type'] as String?) ?? ''),
          );
        })
        .toList(growable: false);
    if (parsed.isNotEmpty) state = parsed;
  }
}

final sensorListNotifierProvider =
    StateNotifierProvider<SensorListNotifier, List<Sensor>>(
      (ref) => SensorListNotifier(),
    );

final selectedSensorIdProvider = StateProvider<String?>((ref) => null);

class SensorFilter {
  final List<int> intensityMin;
  final List<int> intensityMax;

  SensorFilter({List<int>? intensityMin, List<int>? intensityMax})
    : intensityMin = intensityMin ?? [0, 0, 0, 0],
      intensityMax =
          intensityMax ?? [4294967295, 4294967295, 4294967295, 4294967295];

  int getMin(int echo) => intensityMin[echo];
  int getMax(int echo) => intensityMax[echo];
}

final sensorFilterProvider = StateProvider<SensorFilter>(
  (ref) => SensorFilter(),
);
