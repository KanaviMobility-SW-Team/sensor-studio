// features/streaming/streaming_controller.dart
import 'dart:async';
import 'dart:convert';
import 'dart:typed_data';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:sensor_studio/features/sensor/sensor_providers.dart';

import '../../shared/models/frame.dart';
import 'ws_client.dart';
import 'ws_protocol.dart';

enum ConnectionStateX { disconnected, connecting, connected, error }

class StreamingController {
  final WsClient _client;
  final Ref _ref;

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
          print("Binary data received");
          final frame = WsProtocol.decode(data);
        } else {
          print("Unknown data type: ${data.runtimeType}");
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

  Future<void> dispose() async {
    await _sub?.cancel();
    for (final c in _sensorStreams.values) {
      await c.close();
    }
    await _conn.close();
    await _client.close();
  }
}
