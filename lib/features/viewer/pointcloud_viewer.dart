// features/viewer/pointcloud_viewer.dart
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../sensor/sensor_providers.dart';
import '../streaming/streaming_providers.dart';
import '../streaming/streaming_controller.dart';

class PointCloudViewer extends ConsumerStatefulWidget {
  const PointCloudViewer({super.key});

  @override
  ConsumerState<PointCloudViewer> createState() => _PointCloudViewerState();
}

class _PointCloudViewerState extends ConsumerState<PointCloudViewer> {
  String? _subscribedId;

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
      return const Center(child: Text('Select a sensor'));
    }

    final frameAsync = ref.watch(sensorFrameStreamProvider(selectedId));

    return frameAsync.when(
      data: (frame) {
        // TODO: frame.payload -> point cloud parse & render
        return Center(
          child: Text(
            'PointCloud frame from $selectedId (${frame.payload.length} bytes)',
          ),
        );
      },
      loading: () => const Center(child: Text('Waiting for data...')),
      error: (e, _) => Center(child: Text('Stream error: $e')),
    );
  }
}
