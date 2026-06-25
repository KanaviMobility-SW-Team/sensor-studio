import 'dart:async';
import 'dart:convert';
import 'dart:typed_data';

import 'package:logging/logging.dart';
import 'package:riverpod_annotation/riverpod_annotation.dart';
import 'package:web_socket_channel/web_socket_channel.dart';

import 'package:ui/providers/sensor_provider.dart';
import 'package:ui/providers/pointcloud_provider.dart';

part 'websocket_provider.g.dart';

enum ConnectionStatus { disconnected, connecting, connected, error }

class FoxgloveChannel {
  final int id;
  final String topic;
  final String encoding;
  final String schemaName;

  FoxgloveChannel({
    required this.id,
    required this.topic,
    required this.encoding,
    required this.schemaName,
  });

  factory FoxgloveChannel.fromJson(Map<String, dynamic> json) {
    return FoxgloveChannel(
      id: json['id'] as int,
      topic: json['topic'] as String,
      encoding: json['encoding'] as String,
      schemaName: json['schemaName'] as String,
    );
  }
}

class WebSocketState {
  final ConnectionStatus status;
  final Map<String, FoxgloveChannel> availableChannels;
  final Map<String, int> activeSubscriptions; // topic -> subscription_id

  WebSocketState({
    required this.status,
    this.availableChannels = const {},
    this.activeSubscriptions = const {},
  });

  WebSocketState copyWith({
    ConnectionStatus? status,
    Map<String, FoxgloveChannel>? availableChannels,
    Map<String, int>? activeSubscriptions,
  }) {
    return WebSocketState(
      status: status ?? this.status,
      availableChannels: availableChannels ?? this.availableChannels,
      activeSubscriptions: activeSubscriptions ?? this.activeSubscriptions,
    );
  }
}

@riverpod
class WebSocketManager extends _$WebSocketManager {
  WebSocketChannel? _channel;
  final _log = Logger('WebSocketManager');
  int _nextSubscriptionId = 1;

  @override
  WebSocketState build() {
    return WebSocketState(status: ConnectionStatus.disconnected);
  }

  // WebSocket 서버에 연결하는 함수
  Future<void> connect(String url) async {
    if (state.status == ConnectionStatus.connected) return;

    state = state.copyWith(status: ConnectionStatus.connecting);
    _log.info('Connecting to $url...');

    try {
      _channel = WebSocketChannel.connect(Uri.parse(url));

      await _channel!.ready.timeout(
        const Duration(seconds: 3),
        onTimeout: () {
          _channel!.sink.close();
          throw TimeoutException('WebSocket connection timeout');
        },
      );

      _channel!.stream.listen(
        (message) => _handleMessage(message),
        onDone: () => _handleDisconnect(),
        onError: (error) {
          state = state.copyWith(status: ConnectionStatus.error);
          _log.severe('WebSocket Error: $error');
        },
      );
    } catch (e) {
      state = state.copyWith(status: ConnectionStatus.error);
      _log.severe('Connection Exception: $e');
    }
  }

  // 메시지 분류 및 처리 로직
  void _handleMessage(dynamic message) {
    if (state.status != ConnectionStatus.connected) {
      state = state.copyWith(status: ConnectionStatus.connected);
      _log.info('WebSocket Connected Successfully!');
    }

    if (message is String) {
      try {
        final data = jsonDecode(message);
        final op = data['op'];

        if (op == 'serverInfo') {
          _log.info('Server Info: ${data['name']}');
        } else if (op == 'advertise') {
          _handleAdvertise(data['channels'] as List);
        }
      } catch (e) {
        _log.warning('Failed to parse JSON control message: $e');
      }
    } else if (message is Uint8List || message is List<int>) {
      final bytes = message is Uint8List
          ? message
          : Uint8List.fromList(message as List<int>);
      if (bytes.isEmpty) return;

      final op = bytes[0];

      // Opcode 0x01: Message Data
      if (op == 0x01) {
        final byteData = ByteData.sublistView(bytes);

        // 구조: [0x01] + [SubID(4)] + [Timestamp(8)] + [Payload]
        final subId = byteData.getUint32(1, Endian.little);
        final payloadBytes = Uint8List.sublistView(bytes, 13);

        final topic = state.activeSubscriptions.entries
            .where((e) => e.value == subId)
            .map((e) => e.key)
            .firstOrNull;

        if (topic == null) return;

        if (topic.endsWith('/raw')) {
          // 커스텀 바이너리 포맷 — JSON 디코딩 없이 bytes 직접 전달
          ref
              .read(pointCloudDataProvider.notifier)
              .processBinaryPayload(topic, payloadBytes);
        } else {
          try {
            final payloadStr = utf8.decode(payloadBytes);
            final payloadJson = jsonDecode(payloadStr);
            ref
                .read(pointCloudDataProvider.notifier)
                .processPayload(topic, payloadJson);
          } catch (e) {
            _log.warning('Failed to parse binary payload: $e');
          }
        }
      }
    }
  }

  // advertise 메시지에서 채널 목록 파악
  void _handleAdvertise(List<dynamic> channelsList) {
    final newChannels = Map<String, FoxgloveChannel>.from(
      state.availableChannels,
    );

    for (var ch in channelsList) {
      final channel = FoxgloveChannel.fromJson(ch as Map<String, dynamic>);
      // 토픽 이름(예: lidar_roof)을 키로 저장
      newChannels[channel.topic] = channel;
      _log.info(
        'Advertised Topic: ${channel.topic} [ID: ${channel.id}, Schema: ${channel.schemaName}]',
      );
    }

    state = state.copyWith(availableChannels: newChannels);

    // 센서 리스트에 토픽들 동기화
    ref
        .read(sensorListProvider.notifier)
        .syncSensors(newChannels.keys.toList());
  }

  // 사이드바에서 센서를 켰을 때 호출할 구독 함수
  void toggleSubscription(String topic, bool subscribe) {
    if (_channel == null || state.status != ConnectionStatus.connected) {
      _log.warning('Cannot subscribe to $topic: WebSocket disconnected.');
      return;
    }

    final channelInfo = state.availableChannels[topic];
    if (channelInfo == null) {
      _log.warning(
        'Cannot subscribe to $topic: Topic not advertised by Core yet.',
      );
      return;
    }

    if (subscribe) {
      // 이미 구독 중이면 무시
      if (state.activeSubscriptions.containsKey(topic)) return;

      final subId = _nextSubscriptionId++;
      final command = jsonEncode({
        "op": "subscribe",
        "subscriptions": [
          {"id": subId, "channelId": channelInfo.id},
        ],
      });

      _channel!.sink.add(command);
      _log.info(
        'Sent subscribe command for $topic (SubID: $subId, ChID: ${channelInfo.id})',
      );

      final newSubs = Map<String, int>.from(state.activeSubscriptions)
        ..[topic] = subId;
      state = state.copyWith(activeSubscriptions: newSubs);
    } else {
      // 구독 해제 로직
      final subId = state.activeSubscriptions[topic];
      if (subId == null) return;

      final command = jsonEncode({
        "op": "unsubscribe",
        "subscriptionIds": [subId],
      });

      _channel!.sink.add(command);
      _log.info('Sent unsubscribe command for $topic (SubID: $subId)');

      final newSubs = Map<String, int>.from(state.activeSubscriptions)
        ..remove(topic);
      state = state.copyWith(activeSubscriptions: newSubs);
    }
  }

  void _handleDisconnect() {
    state = state.copyWith(
      status: ConnectionStatus.disconnected,
      availableChannels: {},
      activeSubscriptions: {},
    );
    _log.info('WebSocket connection closed.');
    _channel = null;

    // 센서 리스트 초기화
    ref.read(sensorListProvider.notifier).syncSensors([]);
  }

  void disconnect() {
    _log.info('Disconnecting by user request...');
    _channel?.sink.close();
    _handleDisconnect();
  }
}
