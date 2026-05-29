import 'dart:math';

import 'package:flutter/material.dart';

import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:point_glass/point_glass.dart';

import 'package:ui/providers/grid_provider.dart';
import 'package:ui/providers/pointcloud_provider.dart';
import 'package:ui/providers/view_context_provider.dart';
import 'package:ui/providers/websocket_provider.dart';
import 'package:ui/providers/sensor_provider.dart';
import 'package:ui/theme/app_colors.dart';
import 'package:ui/widgets/controls/controls.dart';

class SensorSidebar extends ConsumerStatefulWidget {
  const SensorSidebar({super.key});

  @override
  ConsumerState<SensorSidebar> createState() => _SensorSidebarState();
}

class _SensorSidebarState extends ConsumerState<SensorSidebar> {
  int _sensorExpansionCount = 0;

  @override
  void initState() {
    super.initState();
    WidgetsBinding.instance.addPostFrameCallback((_) {
      final sensors = ref.read(sensorListProvider);
      setState(() {
        _sensorExpansionCount = sensors.length;
      });
    });
  }

  @override
  Widget build(BuildContext context) {
    final sensors = ref.watch(sensorListProvider);
    final wsNotifier = ref.read(webSocketManagerProvider.notifier);
    final gridSettings = ref.watch(gridSettingsProvider);
    final gridNotifier = ref.read(gridSettingsProvider.notifier);
    final viewContext = ref.watch(viewContextProvider);

    return Container(
      width: 300,
      color: const Color(0xFF1E1E1E),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Padding(
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
              itemCount: sensors.length + 1,
              itemBuilder: (context, index) {
                if (index == sensors.length) {
                  return ExpansionTile(
                    collapsedIconColor: Colors.white54,
                    iconColor: AppColors.accent,
                    leading: InkWell(
                      onTap: () {
                        gridNotifier.updateShowGrid(!gridSettings.showGrid);
                      },
                      child: Icon(
                        Icons.grid_4x4,
                        color: gridSettings.showGrid
                            ? AppColors.accent
                            : Colors.white54,
                      ),
                    ),
                    title: Text(
                      "GRID",
                      style: const TextStyle(color: Colors.white, fontSize: 15),
                    ),
                    childrenPadding: const EdgeInsets.symmetric(
                      horizontal: 16.0,
                      vertical: 8.0,
                    ),
                    children: [
                      StepperControl(
                        label: 'Grid Size',
                        value: gridSettings.gridSize.toInt(),
                        step: 10,
                        min: 10,
                        max: 2000,
                        onChanged: (val) {
                          gridNotifier.updateGridSize(val.toDouble());
                          viewContext.value = viewContext.value.copyWith(
                            proj: PinholeProjection(
                              focalPx: 800 - (val - 30) * (500 / 120),
                              near: viewContext.value.proj.near,
                              far: viewContext.value.proj.far,
                            ),
                          );
                        },
                      ),
                      StepperControl(
                        label: 'Grid Step',
                        value: gridSettings.gridStep.toInt(),
                        step: 1,
                        min: 1,
                        max: 200,
                        onChanged: (val) {
                          gridNotifier.updateGridStep(val.toDouble());
                        },
                      ),
                    ],
                  );
                } else {
                  final sensor = sensors[index];
                  final notifier = ref.read(sensorListProvider.notifier);

                  return ExpansionTile(
                    onExpansionChanged: (value) {
                      setState(() {
                        if (value) {
                          _sensorExpansionCount++;
                        } else {
                          _sensorExpansionCount--;
                        }

                        if (_sensorExpansionCount < 0) {
                          _sensorExpansionCount = 0;
                        }
                      });
                    },
                    collapsedIconColor: Colors.white54,
                    iconColor: AppColors.accent,
                    leading: InkWell(
                      onTap: () {
                        notifier.toggleVisibility(
                          sensor.name,
                          !sensor.isVisible,
                        );
                        wsNotifier.toggleSubscription(
                          sensor.name,
                          !sensor.isVisible,
                        );
                      },
                      child: Icon(
                        sensor.isVisible
                            ? Icons.visibility
                            : Icons.visibility_off,
                        color: sensor.isVisible
                            ? AppColors.accent
                            : Colors.white54,
                      ),
                    ),
                    title: Text(
                      sensor.displayName,
                      style: const TextStyle(color: Colors.white, fontSize: 15),
                    ),
                    childrenPadding: const EdgeInsets.symmetric(
                      horizontal: 16.0,
                      vertical: 8.0,
                    ),
                    children: [
                      DoubleStepperControl(
                        label: 'Point Size',
                        value: sensor.pointSize,
                        step: 0.5,
                        min: 0.5,
                        max: 10.0,
                        onChanged: (val) =>
                            notifier.updatePointSize(sensor.name, val),
                      ),
                      DoubleStepperControl(
                        label: 'Opacity',
                        value: sensor.opacity,
                        step: 0.1,
                        min: 0.0,
                        max: 1.0,
                        onChanged: (val) =>
                            notifier.updateOpacity(sensor.name, val),
                      ),
                      DropdownControl(
                        label: 'Color Field',
                        value: sensor.colorField,
                        items: const ['intensity', 'distance'],
                        onChanged: (val) {
                          if (val != null) {
                            notifier.updateColorField(sensor.name, val);
                          }
                        },
                      ),
                      DropdownControl(
                        label: 'Color Map',
                        value: sensor.colorMap,
                        items: const ['turbo', 'rainbow'],
                        onChanged: (val) {
                          if (val != null) {
                            notifier.updateColorMap(sensor.name, val);
                          }
                        },
                      ),
                      NumberInputControl(
                        label: 'Color Min',
                        value: sensor.colorMin,
                        onChanged: (val) => notifier.updateColorRange(
                          sensor.name,
                          val,
                          sensor.colorMax,
                        ),
                        onAuto: () {
                          final (min, _) = _autoColorRange(
                            sensor.name,
                            sensor.colorField,
                          );

                          if (min.isInfinite) {
                            return;
                          }

                          notifier.updateColorRange(
                            sensor.name,
                            min,
                            sensor.colorMax,
                          );
                        },
                      ),
                      NumberInputControl(
                        label: 'Color Max',
                        value: sensor.colorMax,
                        onChanged: (val) => notifier.updateColorRange(
                          sensor.name,
                          sensor.colorMin,
                          val,
                        ),
                        onAuto: () {
                          final (_, max) = _autoColorRange(
                            sensor.name,
                            sensor.colorField,
                          );

                          if (max.isInfinite) {
                            return;
                          }

                          notifier.updateColorRange(
                            sensor.name,
                            sensor.colorMin,
                            max,
                          );
                        },
                      ),
                    ],
                  );
                }
              },
            ),
          ),
        ],
      ),
    );
  }

  (double, double) _autoColorRange(String sensorName, String colorField) {
    double min = double.infinity;
    double max = double.negativeInfinity;

    final pcbuf = ref.read(pointCloudDataProvider)[sensorName];
    if (pcbuf == null || pcbuf.buf.isEmpty) {
      return (min, max);
    }

    if (pcbuf.stride <= 0) {
      return (min, max);
    }

    if (colorField == "distance") {
      final xIdx = pcbuf.fields['x'];
      final yIdx = pcbuf.fields['y'];
      final zIdx = pcbuf.fields['z'];
      if (xIdx == null || yIdx == null || zIdx == null) {
        return (min, max);
      }

      final numPoints = pcbuf.buf.length ~/ pcbuf.stride;
      for (int i = 0; i < numPoints; i++) {
        final base = i * pcbuf.stride;
        final x = pcbuf.buf[base + xIdx];
        final y = pcbuf.buf[base + yIdx];
        final z = pcbuf.buf[base + zIdx];

        final distance = sqrt(x * x + y * y + z * z);
        if (distance < min) min = distance;
        if (distance > max) max = distance;
      }
    } else {
      final fieldIdx = pcbuf.fields[colorField];
      if (fieldIdx == null) {
        return (min, max);
      }

      for (int i = fieldIdx; i < pcbuf.buf.length; i += pcbuf.stride) {
        final v = pcbuf.buf[i];
        if (v < min) min = v;
        if (v > max) max = v;
      }
    }

    if (min.isInfinite || max.isInfinite) {
      return (min, max);
    }

    return (min, max);
  }
}
