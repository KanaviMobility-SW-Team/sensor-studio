// features/sensor/widgets/sensor_list_item.dart
import 'package:flutter/material.dart';
import '../../../shared/models/sensor.dart';

class SensorListItem extends StatelessWidget {
  final Sensor sensor;
  final bool selected;
  final VoidCallback onTap;

  const SensorListItem({
    super.key,
    required this.sensor,
    required this.selected,
    required this.onTap,
  });

  @override
  Widget build(BuildContext context) {
    return ListTile(
      selected: selected,
      title: Text(sensor.name),
      subtitle: Text(sensor.type),
      onTap: onTap,
    );
  }
}