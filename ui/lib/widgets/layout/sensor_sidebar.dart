import 'package:flutter/material.dart';

import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../providers/sensor_provider.dart';

class SensorSidebar extends ConsumerWidget {
  const SensorSidebar({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    // 프로바이더의 상태를 구독 (데이터가 바뀌면 자동 렌더링)
    final sensors = ref.watch(sensorListProvider);

    return Container(
      width: 300,
      color: const Color(0xFF1E1E1E),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          const Padding(
            padding: EdgeInsets.all(16.0),
            child: Text(
              'SENSOR LIST',
              style: TextStyle(
                color: Colors.white70,
                fontSize: 14,
                fontWeight: FontWeight.bold,
              ),
            ),
          ),
          Expanded(
            child: ListView.builder(
              itemCount: sensors.length,
              itemBuilder: (context, index) {
                final sensor = sensors[index];
                final notifier = ref.read(sensorListProvider.notifier);

                return ExpansionTile(
                  collapsedIconColor: Colors.white54,
                  iconColor: Colors.blueAccent,
                  leading: InkWell(
                    onTap: () {
                      notifier.toggleVisibility(sensor.name, !sensor.isVisible);
                    },
                    child: Icon(
                      sensor.isVisible
                          ? Icons.visibility
                          : Icons.visibility_off,
                      color: sensor.isVisible
                          ? Colors.blueAccent
                          : Colors.white54,
                    ),
                  ),
                  title: Text(
                    sensor.name,
                    style: const TextStyle(color: Colors.white, fontSize: 15),
                  ),
                  childrenPadding: const EdgeInsets.symmetric(
                    horizontal: 16.0,
                    vertical: 8.0,
                  ),
                  children: [
                    _buildSlider(
                      'Point Size',
                      sensor.pointSize,
                      0.1,
                      10.0,
                      (val) => notifier.updatePointSize(sensor.name, val),
                    ),
                    _buildSlider(
                      'Opacity',
                      sensor.opacity,
                      0.0,
                      1.0,
                      (val) => notifier.updateOpacity(sensor.name, val),
                    ),
                    _buildDropdown(
                      'Color Field',
                      sensor.colorField,
                      ['intensity', 'z', 'ring'],
                      (val) {
                        if (val != null) {
                          notifier.updateColorField(sensor.name, val);
                        }
                      },
                    ),
                    _buildDropdown(
                      'Color Map',
                      sensor.colorMap,
                      ['turbo', 'rainbow', 'viridis'],
                      (val) {
                        if (val != null) {
                          notifier.updateColorMap(sensor.name, val);
                        }
                      },
                    ),
                  ],
                );
              },
            ),
          ),
        ],
      ),
    );
  }

  // UI 헬퍼 함수들
  Widget _buildSlider(
    String label,
    double value,
    double min,
    double max,
    ValueChanged<double> onChanged,
  ) {
    return Padding(
      padding: const EdgeInsets.only(bottom: 8.0),
      child: Row(
        children: [
          SizedBox(
            width: 80,
            child: Text(
              label,
              style: const TextStyle(color: Colors.white70, fontSize: 12),
            ),
          ),
          Expanded(
            child: Slider(
              value: value,
              min: min,
              max: max,
              activeColor: Colors.blueAccent,
              onChanged: onChanged,
            ),
          ),
          SizedBox(
            width: 30,
            child: Text(
              value.toStringAsFixed(1),
              style: const TextStyle(color: Colors.white54, fontSize: 12),
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildDropdown(
    String label,
    String value,
    List<String> items,
    ValueChanged<String?> onChanged,
  ) {
    return Padding(
      padding: const EdgeInsets.only(bottom: 8.0),
      child: Row(
        mainAxisAlignment: MainAxisAlignment.spaceBetween,
        children: [
          SizedBox(
            width: 110,
            child: Text(
              label,
              style: const TextStyle(color: Colors.white70, fontSize: 12),
            ),
          ),
          Expanded(
            child: DropdownButton<String>(
              value: value,
              isExpanded: true,
              dropdownColor: const Color(0xFF2C2C2C),
              style: const TextStyle(color: Colors.white, fontSize: 12),
              underline: Container(height: 1, color: Colors.white24),
              items: items
                  .map((i) => DropdownMenuItem(value: i, child: Text(i)))
                  .toList(),
              onChanged: onChanged,
            ),
          ),
        ],
      ),
    );
  }
}
