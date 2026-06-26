// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'websocket_provider.dart';

// **************************************************************************
// RiverpodGenerator
// **************************************************************************

// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint, type=warning

@ProviderFor(WebSocketManager)
final webSocketManagerProvider = WebSocketManagerProvider._();

final class WebSocketManagerProvider
    extends $NotifierProvider<WebSocketManager, WebSocketState> {
  WebSocketManagerProvider._()
    : super(
        from: null,
        argument: null,
        retry: null,
        name: r'webSocketManagerProvider',
        isAutoDispose: true,
        dependencies: null,
        $allTransitiveDependencies: null,
      );

  @override
  String debugGetCreateSourceHash() => _$webSocketManagerHash();

  @$internal
  @override
  WebSocketManager create() => WebSocketManager();

  /// {@macro riverpod.override_with_value}
  Override overrideWithValue(WebSocketState value) {
    return $ProviderOverride(
      origin: this,
      providerOverride: $SyncValueProvider<WebSocketState>(value),
    );
  }
}

String _$webSocketManagerHash() => r'b4cad9360b67ad1324ec3c0e138fc1b78c89acd1';

abstract class _$WebSocketManager extends $Notifier<WebSocketState> {
  WebSocketState build();
  @$mustCallSuper
  @override
  void runBuild() {
    final ref = this.ref as $Ref<WebSocketState, WebSocketState>;
    final element =
        ref.element
            as $ClassProviderElement<
              AnyNotifier<WebSocketState, WebSocketState>,
              WebSocketState,
              Object?,
              Object?
            >;
    element.handleCreate(ref, build);
  }
}
