// features/streaming/streaming_providers.dart
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../../shared/models/frame.dart';
import 'ws_client.dart';
import 'streaming_controller.dart';

final wsClientProvider = Provider((ref) => WsClient());

final streamingControllerProvider = Provider((ref) {
  final c = StreamingController(ref.watch(wsClientProvider), ref);
  ref.onDispose(() => c.dispose());
  return c;
});

final connectionStateProvider = StreamProvider((ref) {
  return ref.watch(streamingControllerProvider).connState;
});

final sensorFrameStreamProvider = StreamProvider.family<Frame, String>((
  ref,
  sensorId,
) {
  return ref.watch(streamingControllerProvider).streamForSensor(sensorId);
});
