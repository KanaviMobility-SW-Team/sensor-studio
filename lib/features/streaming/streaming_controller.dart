// features/streaming/streaming_controller.dart
import 'dart:async';
import 'dart:convert';
import 'dart:typed_data';

import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_multiplatform_logger/flutter_multiplatform_logger.dart';

import 'package:sensor_studio/features/sensor/sensor_providers.dart';

import '../../shared/models/frame.dart';
import 'ws_client.dart';
import 'ws_protocol.dart';

enum ConnectionStateX { disconnected, connecting, connected, error }

class StreamingController {
  final WsClient _client;
  final Ref _ref;
  final _logger = Logger("StreamingController");

  StreamSubscription? _sub;

  final _conn = StreamController<ConnectionStateX>.broadcast();
  Stream<ConnectionStateX> get connState => _conn.stream;

  final Map<String, StreamController<Frame>> _sensorStreams = {};

  StreamingController(this._client, this._ref);

  Stream<Frame> streamForSensor(String sensorId) {
    return _sensorStreams
        .putIfAbsent(sensorId, () => StreamController<Frame>.broadcast())
        .stream;
  }

  Future<void> connect(Uri uri) async {
    _conn.add(ConnectionStateX.connecting);

    try {
      await _client.connect(uri);
      _conn.add(ConnectionStateX.connected);

      _sub?.cancel();
      _sub = _client.incoming.listen((data) {
        if (data is String) {
          final jsonMap = jsonDecode(data);
          _ref
              .read(sensorListNotifierProvider.notifier)
              .updateFromServer(jsonMap);
        } else if (data is Uint8List || data is List<int>) {
          final frame = WsProtocol.decodeLidarFrame(data);
          for (final controller in _sensorStreams.values) {
            controller.add(frame);
          }
        } else {
          _logger.severe("Unknown data type: ${data.runtimeType}");
        }
      }, onError: (_) => _conn.add(ConnectionStateX.error));
    } catch (_) {
      _conn.add(ConnectionStateX.error);
    }
  }

  void subscribe(String sensorId) {
    _client.sendString(
      jsonEncode({
        "op": "subscribe",
        "subscriptions": [
          {"id": 0, "channelId": int.tryParse(sensorId) ?? 0},
        ],
      }),
    );
  }

  void unsubscribe(String sensorId) {
    _client.sendString(
      jsonEncode({
        "op": "unsubscribe",
        "subscriptionIds": [0],
      }),
    );
  }

  void sendMessage(String message) {
    _client.sendString(message);
  }

  Future<void> dispose() async {
    await _sub?.cancel();
    for (final c in _sensorStreams.values) {
      await c.close();
    }
    await _conn.close();
    await _client.close();
  }
}
