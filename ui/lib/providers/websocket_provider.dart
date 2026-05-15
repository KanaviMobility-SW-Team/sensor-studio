import 'package:riverpod_annotation/riverpod_annotation.dart';
import 'package:web_socket_channel/web_socket_channel.dart';
import 'package:logging/logging.dart';

part 'websocket_provider.g.dart';

enum ConnectionStatus { disconnected, connecting, connected, error }

class WebSocketState {
  final ConnectionStatus status;

  WebSocketState({required this.status});

  WebSocketState copyWith({ConnectionStatus? status}) {
    return WebSocketState(status: status ?? this.status);
  }
}

@riverpod
class WebSocketManager extends _$WebSocketManager {
  WebSocketChannel? _channel;
  final _log = Logger('WebSocketManager');

  @override
  WebSocketState build() {
    return WebSocketState(status: ConnectionStatus.disconnected);
  }

  void connect(String url) {
    if (state.status == ConnectionStatus.connected) return;

    state = state.copyWith(status: ConnectionStatus.connecting);
    _log.info('Connecting to $url...');

    try {
      _channel = WebSocketChannel.connect(Uri.parse(url));

      _channel!.stream.listen(
        (message) {
          if (state.status != ConnectionStatus.connected) {
            state = state.copyWith(status: ConnectionStatus.connected);
            _log.info('WebSocket Connected Successfully!');
          }
        },
        onDone: () {
          state = state.copyWith(status: ConnectionStatus.disconnected);
          _log.info('WebSocket connection closed.');
        },
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

  void disconnect() {
    _log.info('Disconnecting by user request...');
    _channel?.sink.close();
    _channel = null;
    state = state.copyWith(status: ConnectionStatus.disconnected);
  }
}
