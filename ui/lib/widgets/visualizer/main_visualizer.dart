import 'package:flutter/material.dart';

import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:point_glass/point_glass.dart';
import 'package:ui/providers/pointcloud_provider.dart';

import '../../providers/sensor_provider.dart';

class MainVisualizer extends ConsumerStatefulWidget {
  const MainVisualizer({super.key});

  @override
  ConsumerState<MainVisualizer> createState() => _MainVisualizerState();
}

class _MainVisualizerState extends ConsumerState<MainVisualizer> {
  late final ValueNotifier<ViewContext> _viewContext;

  @override
  void initState() {
    super.initState();
    _viewContext = ValueNotifier(
      ViewContext(
        model: ModelTransform(),
        camera: PinholeCamera(cameraZ: 20),
        proj: PinholeProjection(focalPx: 800, near: 1, far: 20000),
        canvasCenter: const Offset(0, 0),
      ),
    );
  }

  @override
  void dispose() {
    _viewContext.dispose();
    super.dispose();
  }

  Color _mapValueToColor(
    double value,
    String colorMapType, {
    double min = 0,
    double max = 255,
  }) {
    // 값을 0.0 ~ 1.0 사이로 정규화
    final normalized = ((value - min) / (max - min)).clamp(0.0, 1.0);

    if (colorMapType == 'rainbow') {
      final hue = (1.0 - normalized) * 240;
      return HSVColor.fromAHSV(1.0, hue, 1.0, 1.0).toColor();
    }

    if (colorMapType == 'turbo') {
      // Turbo colormap 근사 공식
      const r = [0.18995, 0.5, 0.8, 1.0, 0.9, 0.5];
      const g = [0.07176, 0.5, 0.9, 0.8, 0.3, 0.1];
      const b = [0.23217, 0.9, 0.5, 0.1, 0.05, 0.0];

      final idx = (normalized * (r.length - 1));
      final lo = idx.floor().clamp(0, r.length - 2);
      final t = idx - lo;

      return Color.fromARGB(
        255,
        ((r[lo] + t * (r[lo + 1] - r[lo])) * 255).round(),
        ((g[lo] + t * (g[lo + 1] - g[lo])) * 255).round(),
        ((b[lo] + t * (b[lo + 1] - b[lo])) * 255).round(),
      );
    }

    return Colors.white;
  }

  @override
  Widget build(BuildContext context) {
    final sensors = ref.watch(sensorListProvider);

    final pointCloudData = ref.watch(pointCloudDataProvider);

    final pointsGroup = sensors.where((s) => s.isVisible).map((sensor) {
      final realPoints = pointCloudData[sensor.name] ?? <PointData>[];

      Color baseColor = Colors.white70;

      final pointList = realPoints.map((p) {
        Color pointColor = baseColor;
        if (sensor.colorField == 'intensity') {
          pointColor = _mapValueToColor(
            p.attributes[sensor.colorField] ?? 0.0,
            sensor.colorMap,
            min: 0,
            max: 12000,
          );
        } else if (sensor.colorField == 'z') {
          pointColor = _mapValueToColor(
            p.position.z,
            sensor.colorMap,
            min: -10.0,
            max: 10.0,
          );
        }

        return PointGlassPoint(
          point: p.position,
          strokeWidth: sensor.pointSize,
          alpha: (sensor.opacity * 255).toInt(),
          color: pointColor,
        );
      }).toList();

      return PointGlassPoints(enable: true, points: pointList);
    }).toList();

    return Expanded(
      child: Container(
        color: const Color(0xFF121212),
        child: ClipRect(
          child: PointGlassViewer(
            viewContext: _viewContext,
            mode: PointGlassViewerMode.rotate,
            grid: PointGlassGrid(
              enable: true,
              gridSize: 20,
              gridStep: 1,
              enableLabel: true,
              labelStyle: TextStyle(color: Colors.white.withAlpha(150)),
            ),
            axis: PointGlassAxis(enable: true, axisLength: 2.0),
            pointsGroup: pointsGroup,
          ),
        ),
      ),
    );
  }
}
