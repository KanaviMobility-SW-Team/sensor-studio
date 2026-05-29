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

String _$webSocketManagerHash() => r'4505f1532a59b33b893f989f493b86b3c807de3e';

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
