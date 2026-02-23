// features/streaming/ws_client.dart
import 'dart:async';
import 'dart:typed_data';
import 'package:web_socket_channel/web_socket_channel.dart';
import 'package:flutter_multiplatform_logger/flutter_multiplatform_logger.dart';

class WsClient {
  WebSocketChannel? _ch;
  final _incoming = StreamController<dynamic>.broadcast();
  final _logger = Logger('WsClient');

  Stream<dynamic> get incoming => _incoming.stream;

  Future<void> connect(Uri uri) async {
    _ch = WebSocketChannel.connect(uri);
    _ch!.stream.listen(
      (data) {
        if (data is String) {
          _logger.info('WsClient received text: $data');
        }

        _incoming.add(data);
      },
      onError: _incoming.addError,
      cancelOnError: true,
    );
  }

  void send(Uint8List bytes) {
    _logger.info('WsClient sending binary: ${bytes.length} bytes');
    _ch?.sink.add(bytes);
  }

  void sendString(String msg) {
    _logger.info('WsClient sending string: ${msg}');
    _ch?.sink.add(msg);
  }

  Future<void> close() async => _ch?.sink.close();
}
