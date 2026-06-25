// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'loading_overlay_provider.dart';

// **************************************************************************
// RiverpodGenerator
// **************************************************************************

// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint, type=warning

@ProviderFor(LoadingOverlay)
final loadingOverlayProvider = LoadingOverlayProvider._();

final class LoadingOverlayProvider
    extends $NotifierProvider<LoadingOverlay, bool> {
  LoadingOverlayProvider._()
    : super(
        from: null,
        argument: null,
        retry: null,
        name: r'loadingOverlayProvider',
        isAutoDispose: true,
        dependencies: null,
        $allTransitiveDependencies: null,
      );

  @override
  String debugGetCreateSourceHash() => _$loadingOverlayHash();

  @$internal
  @override
  LoadingOverlay create() => LoadingOverlay();

  /// {@macro riverpod.override_with_value}
  Override overrideWithValue(bool value) {
    return $ProviderOverride(
      origin: this,
      providerOverride: $SyncValueProvider<bool>(value),
    );
  }
}

String _$loadingOverlayHash() => r'72cbdb93a5b17cb8d95c728588b9d30236cfeeab';

abstract class _$LoadingOverlay extends $Notifier<bool> {
  bool build();
  @$mustCallSuper
  @override
  void runBuild() {
    final ref = this.ref as $Ref<bool, bool>;
    final element =
        ref.element
            as $ClassProviderElement<
              AnyNotifier<bool, bool>,
              bool,
              Object?,
              Object?
            >;
    element.handleCreate(ref, build);
  }
}
