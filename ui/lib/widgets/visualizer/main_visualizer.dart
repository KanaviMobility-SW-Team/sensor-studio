import 'package:flutter/material.dart';

import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:point_glass/point_glass.dart';

import 'package:ui/providers/grid_provider.dart';
import 'package:ui/providers/pointcloud_provider.dart';
import 'package:ui/providers/view_context_provider.dart';

import '../../providers/sensor_provider.dart';

class MainVisualizer extends ConsumerStatefulWidget {
  const MainVisualizer({super.key});

  @override
  ConsumerState<MainVisualizer> createState() => _MainVisualizerState();
}

class _MainVisualizerState extends ConsumerState<MainVisualizer> {
  List<PointGlassRawPoints> _rawPointsGroup = const [];

  @override
  void initState() {
    super.initState();
  }

  @override
  void dispose() {
    super.dispose();
  }

  void _scheduleRender() {
    final data = ref.read(pointCloudDataProvider);
    final visibleSensors = ref
        .read(sensorListProvider)
        .where((s) => s.isVisible)
        .toList();

    final rawGroup = visibleSensors
        .map((sensor) {
          final pcbuf = data[sensor.name];
          if (pcbuf == null || pcbuf.buf.isEmpty) return null;

          final colorMap = sensor.colorMap == 'turbo'
              ? ColorMap.turbo
              : ColorMap.rainbow;
          final isDistanceField = sensor.colorField == 'distance';

          return PointGlassRawPoints(
            enable: true,
            buf: pcbuf.buf,
            stride: pcbuf.stride,
            fields: pcbuf.fields,
            colorMap: colorMap,
            colorField: sensor.colorField,
            colorMin: 0.0,
            colorMax: isDistanceField ? 10.0 : 12000.0,
            strokeWidth: sensor.pointSize,
            alpha: (sensor.opacity * 255).toInt(),
          );
        })
        .whereType<PointGlassRawPoints>()
        .toList();

    setState(() => _rawPointsGroup = rawGroup);
  }

  @override
  Widget build(BuildContext context) {
    final viewContext = ref.read(viewContextProvider);
    final gridSettings = ref.watch(gridSettingsProvider);

    // 데이터 또는 센서 설정이 바뀌면 백그라운드에서 색상 재계산
    ref.listen(pointCloudDataProvider, (_, __) => _scheduleRender());
    ref.listen(sensorListProvider, (_, __) => _scheduleRender());

    return Expanded(
      child: Container(
        color: const Color(0xFF121212),
        child: ClipRect(
          child: PointGlassViewer(
            viewContext: viewContext,
            mode: PointGlassViewerMode.rotate,
            grid: PointGlassGrid(
              enable: gridSettings.showGrid,
              gridSize: gridSettings.gridSize,
              gridStep: gridSettings.gridStep,
              enableLabel: true,
              labelStyle: TextStyle(color: Colors.white.withAlpha(120)),
            ),
            axis: PointGlassAxis(enable: true, axisLength: 1.0),
            rawPointsGroup: _rawPointsGroup,
          ),
        ),
      ),
    );
  }
}
