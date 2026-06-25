import 'package:flutter/material.dart';
import 'package:flutter/foundation.dart';

import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:point_glass_opengl/point_glass_opengl.dart';

import 'package:ui/providers/grid_provider.dart';
import 'package:ui/providers/pointcloud_provider.dart';
import 'package:ui/providers/sensor_provider.dart';
import 'package:ui/models/sensor_payload.dart';
import 'package:ui/utils/point_cloud_parser.dart';

class MainVisualizer extends ConsumerStatefulWidget {
  const MainVisualizer({super.key});

  @override
  ConsumerState<MainVisualizer> createState() => _MainVisualizerState();
}

class _MainVisualizerState extends ConsumerState<MainVisualizer> {
  final PointGlassOpenGLController _glController = PointGlassOpenGLController();
  bool _isProcessing = false; // 파싱 중복 실행 방지 플래그

  @override
  void initState() {
    super.initState();
  }

  @override
  void dispose() {
    super.dispose();
  }

  Future<void> _scheduleRender() async {
    // 이미 다른 데이터를 파싱 중이라면 스킵 (병목 방지)
    if (_isProcessing) return;

    final data = ref.read(pointCloudDataProvider);
    final visibleSensors = ref
        .read(sensorListProvider)
        .where((s) => s.isVisible)
        .toList();

    List<SensorPayload> payloadsForIsolate = [];

    bool isSetPointCloudDisplayParams = false;
    double alpha = 1.0;
    double pointSize = 1.0;
    double valueMax = 0.0;
    double valueMin = 0.0;
    PointGlassOpenGLPointsColorMode colorMode =
        PointGlassOpenGLPointsColorMode.viridis;

    for (final sensor in visibleSensors) {
      if (!isSetPointCloudDisplayParams) {
        alpha = sensor.opacity;
        pointSize = sensor.pointSize;
        valueMax = sensor.valueRange[sensor.colorField]?.$2 ?? 0.0;
        valueMin = sensor.valueRange[sensor.colorField]?.$1 ?? 0.0;
        colorMode = PointGlassOpenGLPointsColorMode.values.firstWhere(
          (e) => e.name == sensor.colorMap,
          orElse: () => PointGlassOpenGLPointsColorMode.viridis,
        );
        isSetPointCloudDisplayParams = true;
      }

      final pcbuf = data[sensor.name];
      if (pcbuf == null || pcbuf.buf.isEmpty) continue;

      payloadsForIsolate.add(
        SensorPayload(
          buf: pcbuf.buf,
          stride: pcbuf.stride,
          xIdx: pcbuf.fields['x'],
          yIdx: pcbuf.fields['y'],
          zIdx: pcbuf.fields['z'],
          colorField: sensor.colorField,
          colorFieldIdx: pcbuf.fields[sensor.colorField],
        ),
      );
    }

    if (payloadsForIsolate.isEmpty) {
      if (mounted) {
        _glController.setPoints(
          Float32List.fromList([
            // dummy point
            0.0, 0.0, 0.0, 0.0,
          ]),
        );
      }

      return;
    }

    _isProcessing = true;

    // 백그라운드 스레드에서 GPU용 Float32List로 변환
    final rawFloatData = await compute(
      buildGLPointCloudData,
      payloadsForIsolate,
    );

    if (mounted) {
      _glController.setPoints(rawFloatData);
      _glController.setPointCloudDisplayParams(
        alpha,
        pointSize,
        valueMin,
        valueMax,
        colorMode,
      );
      _glController.render();
    }
    _isProcessing = false;
  }

  @override
  Widget build(BuildContext context) {
    final gridSettings = ref.watch(gridSettingsProvider);

    // 데이터 또는 센서 설정이 바뀌면 백그라운드에서 색상 재계산
    ref.listen(pointCloudDataProvider, (_, _) => _scheduleRender());
    ref.listen(sensorListProvider, (_, _) => _scheduleRender());

    return Expanded(
      child: Container(
        color: const Color(0xFF121212),
        child: ClipRect(
          child: PointGlassOpenGLViewer(
            grid: PointGlassOpenGLGrid(
              enable: gridSettings.showGrid,
              gridSize: gridSettings.gridSize,
              gridStep: gridSettings.gridStep,
              enableLabel: gridSettings.showGrid
                  ? gridSettings.showGridLabels
                  : false,
            ),
            axis: PointGlassOpenGLAxis(
              enable: gridSettings.showGrid ? gridSettings.showGridAxis : false,
              length: gridSettings.gridStep,
              labelEnable: gridSettings.showGrid
                  ? gridSettings.showGridAxis
                  : false,
            ),
            controller: _glController,
          ),
        ),
      ),
    );
  }
}
