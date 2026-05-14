import 'package:riverpod_annotation/riverpod_annotation.dart';
import 'package:web_socket_channel/web_socket_channel.dart';

part 'websocket_provider.g.dart';

// 연결 상태를 나타내는 Enum
enum ConnectionStatus { disconnected, connecting, connected, error }

class WebSocketState {
  final ConnectionStatus status;
  final List<String> logs;

  WebSocketState({required this.status, required this.logs});

  WebSocketState copyWith({ConnectionStatus? status, List<String>? logs}) {
    return WebSocketState(
      status: status ?? this.status,
      logs: logs ?? this.logs,
    );
  }
}

// 상태 관리 클래스 (WebSocket Manager)
@riverpod
class WebSocketManager extends _$WebSocketManager {
  WebSocketChannel? _channel;

  @override
  WebSocketState build() {
    // 초기 상태 세팅
    return WebSocketState(
      status: ConnectionStatus.disconnected,
      logs: ['[INFO] WebSocket Manager Initialized.'],
    );
  }

  // 내부 로그 추가용 헬퍼 함수
  void _addLog(String message) {
    final time = DateTime.now().toIso8601String().substring(11, 19);
    // 기존 로그 배열을 복사하고 새로운 로그를 추가하여 상태 업데이트
    state = state.copyWith(logs: [...state.logs, '[$time] $message']);
  }

  // Core 엔진 웹소켓 서버에 연결
  void connect(String url) {
    if (state.status == ConnectionStatus.connected) return;

    state = state.copyWith(status: ConnectionStatus.connecting);
    _addLog('Connecting to $url...');

    try {
      _channel = WebSocketChannel.connect(Uri.parse(url));

      // 스트림 구독 (데이터 수신 대기)
      _channel!.stream.listen(
        (message) {
          if (state.status != ConnectionStatus.connected) {
            state = state.copyWith(status: ConnectionStatus.connected);
            _addLog('WebSocket Connected Successfully!');
          }
          // 추후 여기서 Foxglove 바이너리/JSON 메시지를 파싱하여 센서 데이터로 분배합니다.
          // _addLog('Received: ${message.toString().substring(0, 20)}...'); // 너무 많은 로그 방지를 위해 주석 처리
        },
        onDone: () {
          state = state.copyWith(status: ConnectionStatus.disconnected);
          _addLog('WebSocket connection closed.');
        },
        onError: (error) {
          state = state.copyWith(status: ConnectionStatus.error);
          _addLog('WebSocket Error: $error');
        },
      );
    } catch (e) {
      state = state.copyWith(status: ConnectionStatus.error);
      _addLog('Connection Exception: $e');
    }
  }

  // 수동으로 연결 종료
  void disconnect() {
    _addLog('Disconnecting by user request...');
    _channel?.sink.close();
    _channel = null;
    state = state.copyWith(status: ConnectionStatus.disconnected);
  }
}
