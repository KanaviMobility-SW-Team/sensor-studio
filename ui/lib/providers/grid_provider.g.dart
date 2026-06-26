// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'grid_provider.dart';

// **************************************************************************
// RiverpodGenerator
// **************************************************************************

// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint, type=warning

@ProviderFor(GridSettingsNotifier)
final gridSettingsProvider = GridSettingsNotifierProvider._();

final class GridSettingsNotifierProvider
    extends $NotifierProvider<GridSettingsNotifier, GridSettings> {
  GridSettingsNotifierProvider._()
    : super(
        from: null,
        argument: null,
        retry: null,
        name: r'gridSettingsProvider',
        isAutoDispose: true,
        dependencies: null,
        $allTransitiveDependencies: null,
      );

  @override
  String debugGetCreateSourceHash() => _$gridSettingsNotifierHash();

  @$internal
  @override
  GridSettingsNotifier create() => GridSettingsNotifier();

  /// {@macro riverpod.override_with_value}
  Override overrideWithValue(GridSettings value) {
    return $ProviderOverride(
      origin: this,
      providerOverride: $SyncValueProvider<GridSettings>(value),
    );
  }
}

String _$gridSettingsNotifierHash() =>
    r'bc158252df409487a183bd3fd405b1c91b9779da';

abstract class _$GridSettingsNotifier extends $Notifier<GridSettings> {
  GridSettings build();
  @$mustCallSuper
  @override
  void runBuild() {
    final ref = this.ref as $Ref<GridSettings, GridSettings>;
    final element =
        ref.element
            as $ClassProviderElement<
              AnyNotifier<GridSettings, GridSettings>,
              GridSettings,
              Object?,
              Object?
            >;
    element.handleCreate(ref, build);
  }
}
