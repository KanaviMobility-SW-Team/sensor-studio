// features/viewer/pointcloud_viewer.dart
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:point_glass/point_glass.dart';
import 'package:sensor_studio/shared/models/frame.dart';
import '../sensor/sensor_providers.dart';
import '../streaming/streaming_providers.dart';
import '../streaming/streaming_controller.dart';
import 'package:vector_math/vector_math.dart' as vm;
import 'package:sensor_studio/features/sensor/sensor_providers.dart';
import 'package:sensor_studio/shared/utils/color_utils.dart';

enum ColorMode { rainbow, turbo, distanceA, distanceB }

class PointCloudViewer extends ConsumerStatefulWidget {
  const PointCloudViewer({super.key});

  @override
  ConsumerState<PointCloudViewer> createState() => _PointCloudViewerState();
}

class _PointCloudViewerState extends ConsumerState<PointCloudViewer> {
  String? _subscribedId;

  ValueNotifier<ViewContext> viewContext = ValueNotifier(
    ViewContext(
      model: ModelTransform(),
      camera: PinholeCamera(cameraZ: 10),
      proj: PinholeProjection(focalPx: 800, near: 1, far: 20000),
      canvasCenter: Offset(0, 0),
    ),
  );

  List<PointGlassPoints> pointsGroup = [
    PointGlassPoints(enable: true, points: []),
    PointGlassPoints(enable: true, points: []),
    PointGlassPoints(enable: true, points: []),
    PointGlassPoints(enable: true, points: []),
  ];

  List<int> intensityMin = [0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF];
  List<int> intensityMax = [0, 0, 0, 0];

  List<int> filteredIntensityMin = [
    0xFFFFFFFF,
    0xFFFFFFFF,
    0xFFFFFFFF,
    0xFFFFFFFF,
  ];
  List<int> filteredIntensityMax = [0, 0, 0, 0];

  double gridSize = 20;
  double gridStep = 1;

  double axisLength = 0.5;
  bool axisOnOff = true;

  PointGlassViewerMode viewMode = PointGlassViewerMode.rotate;

  ColorMode colorMode = ColorMode.distanceA;

  @override
  void initState() {
    super.initState();
    // connect once
    Future.microtask(() async {
      final controller = ref.read(streamingControllerProvider);
      await controller.connect(Uri.parse('ws://localhost:18293'));
    });
  }

  @override
  Widget build(BuildContext context) {
    final selectedId = ref.watch(selectedSensorIdProvider);
    final filter = ref.watch(sensorFilterProvider);

    // selection 변경 시 subscribe 갱신
    if (selectedId != null && selectedId != _subscribedId) {
      final controller = ref.read(streamingControllerProvider);
      if (_subscribedId != null) {
        controller.unsubscribe(_subscribedId!);
      }

      controller.subscribe(selectedId);
      _subscribedId = selectedId;
    }

    if (selectedId == null) {
      return Stack(
        children: [
          ClipRect(
            child: PointGlassViewer(
              viewContext: viewContext,
              mode: viewMode,
              grid: PointGlassGrid(
                enable: true,
                gridSize: gridSize,
                gridStep: gridStep,
                enableLabel: true,
                labelStyle: TextStyle(color: Colors.white.withAlpha(150)),
              ),
              axis: PointGlassAxis(enable: axisOnOff, axisLength: axisLength),
              polygons: [],
              annualSectors: [],
              pointsGroup: pointsGroup,
            ),
          ),
          const Center(
            child: Text(
              'Please select a sensor.',
              style: TextStyle(color: Colors.white, fontSize: 18),
            ),
          ),
        ],
      );
    }

    final frameAsync = ref.watch(sensorFrameStreamProvider(selectedId));

    return Stack(
      children: [
        frameAsync.when(
          data: (frame) {
            for (final controller in pointsGroup) {
              controller.points.clear();
            }

            frame.points?.asMap().forEach((_, p) {
              if (p.echo >= pointsGroup.length) {
                return;
              }

              int minIntensity = filter.intensityMin[p.echo];
              int maxIntensity = filter.intensityMax[p.echo];
              int intensity = p.intensity;

              if (intensityMin[p.echo] > intensity) {
                intensityMin[p.echo] = intensity;
              }

              if (intensityMax[p.echo] < intensity) {
                intensityMax[p.echo] = intensity;
              }

              if (intensity <= minIntensity || intensity >= maxIntensity) {
                return;
              }

              if (filteredIntensityMin[p.echo] > intensity) {
                filteredIntensityMin[p.echo] = intensity;
              }

              if (filteredIntensityMax[p.echo] < intensity) {
                filteredIntensityMax[p.echo] = intensity;
              }

              double distance = vm.Vector3(p.x, p.y, p.z).length;

              pointsGroup[p.echo].points.add(
                PointGlassPoint(
                  enable: true,
                  color: colorMode == ColorMode.rainbow
                      ? rainbowColor(intensity, minIntensity, maxIntensity)
                      : colorMode == ColorMode.turbo
                      ? turboColor(intensity, minIntensity, maxIntensity)
                      : colorMode == ColorMode.distanceA
                      ? distanceColorA(distance, 0, 30)
                      : distanceColorB(distance, 0, 30),
                  alpha: 200,
                  strokeWidth: 1,
                  point: vm.Vector3(p.x, p.y, p.z),
                ),
              );
            });

            return PointGlassViewer(
              viewContext: viewContext,
              mode: viewMode,
              grid: PointGlassGrid(
                enable: true,
                gridSize: gridSize,
                gridStep: gridStep,
                enableLabel: true,
                labelStyle: TextStyle(color: Colors.white.withAlpha(150)),
              ),
              axis: PointGlassAxis(enable: axisOnOff, axisLength: axisLength),
              polygons: [],
              annualSectors: [],
              pointsGroup: pointsGroup,
            );
          },
          loading: () => const Center(child: Text('Waiting for data...')),
          error: (e, _) => Center(child: Text('Stream error: $e')),
        ),
        Column(
          children: [
            const Spacer(),
            Row(
              children: [
                IconButton(
                  onPressed: () {
                    setState(() {
                      colorMode = ColorMode.rainbow;
                    });
                  },
                  icon: Icon(
                    Icons.gradient,
                    color: Colors.white.withAlpha(100),
                  ),
                ),
                IconButton(
                  onPressed: () {
                    setState(() {
                      colorMode = ColorMode.turbo;
                    });
                  },
                  icon: Icon(Icons.bolt, color: Colors.white.withAlpha(100)),
                ),
                IconButton(
                  onPressed: () {
                    setState(() {
                      colorMode = ColorMode.distanceA;
                    });
                  },
                  icon: Icon(
                    Icons.straighten,
                    color: Colors.red.withAlpha(100),
                  ),
                ),
                IconButton(
                  onPressed: () {
                    setState(() {
                      colorMode = ColorMode.distanceB;
                    });
                  },
                  icon: Icon(
                    Icons.straighten,
                    color: Colors.blue.withAlpha(100),
                  ),
                ),
                const Spacer(),
                IconButton(
                  onPressed: () {
                    final controller = ref.read(streamingControllerProvider);
                    controller.sendMessage('unsetFilter');
                    ref
                        .read(sensorFilterProvider.notifier)
                        .state = SensorFilter(
                      intensityMin: [0, 0, 0, 0],
                      intensityMax: [
                        0xFFFFFFFF,
                        0xFFFFFFFF,
                        0xFFFFFFFF,
                        0xFFFFFFFF,
                      ],
                    );
                  },
                  icon: Icon(
                    Icons.auto_fix_off,
                    color: Colors.white.withAlpha(100),
                  ),
                ),
                IconButton(
                  onPressed: () {
                    final controller = ref.read(streamingControllerProvider);
                    controller.sendMessage('setFilter');
                    ref
                        .read(sensorFilterProvider.notifier)
                        .state = SensorFilter(
                      intensityMin: intensityMin,
                      intensityMax: intensityMax,
                    );
                  },
                  icon: Icon(
                    Icons.auto_fix_high,
                    color: Colors.white.withAlpha(100),
                  ),
                ),
                IconButton(
                  onPressed: () {
                    final controller = ref.read(streamingControllerProvider);
                    controller.sendMessage('setFilter');
                    ref
                        .read(sensorFilterProvider.notifier)
                        .state = SensorFilter(
                      intensityMin: [50000, 45000, 40000, 0],
                      intensityMax: [330000, 330000, 260000, 0],
                    );
                  },
                  icon: Icon(Icons.tune, color: Colors.white.withAlpha(100)),
                ),
              ],
            ),
          ],
        ),
      ],
    );
  }
}
