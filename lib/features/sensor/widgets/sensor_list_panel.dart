// features/sensor/widgets/sensor_list_panel.dart
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../sensor_providers.dart';
import 'sensor_list_item.dart';

class SensorListPanel extends ConsumerWidget {
  const SensorListPanel({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final sensors = ref.watch(sensorListNotifierProvider);
    final selectedId = ref.watch(selectedSensorIdProvider);

    return ListView(
      children: [
        const Padding(
          padding: EdgeInsets.all(12),
          child: Text('Sensors', style: TextStyle(fontSize: 14)),
        ),
        for (final s in sensors)
          SensorListItem(
            sensor: s,
            selected: selectedId == s.id,
            onTap: () => ref.read(selectedSensorIdProvider.notifier).state = s.id,
          ),
      ],
    );
  }
}