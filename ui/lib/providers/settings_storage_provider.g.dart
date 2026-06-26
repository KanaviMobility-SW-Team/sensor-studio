// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'settings_storage_provider.dart';

// **************************************************************************
// RiverpodGenerator
// **************************************************************************

// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint, type=warning

@ProviderFor(SettingsFileStorageNotifier)
final settingsFileStorageProvider = SettingsFileStorageNotifierProvider._();

final class SettingsFileStorageNotifierProvider
    extends
        $NotifierProvider<SettingsFileStorageNotifier, SettingsFileStorage> {
  SettingsFileStorageNotifierProvider._()
    : super(
        from: null,
        argument: null,
        retry: null,
        name: r'settingsFileStorageProvider',
        isAutoDispose: false,
        dependencies: null,
        $allTransitiveDependencies: null,
      );

  @override
  String debugGetCreateSourceHash() => _$settingsFileStorageNotifierHash();

  @$internal
  @override
  SettingsFileStorageNotifier create() => SettingsFileStorageNotifier();

  /// {@macro riverpod.override_with_value}
  Override overrideWithValue(SettingsFileStorage value) {
    return $ProviderOverride(
      origin: this,
      providerOverride: $SyncValueProvider<SettingsFileStorage>(value),
    );
  }
}

String _$settingsFileStorageNotifierHash() =>
    r'0e4e6bdd59a17373d90b17f9384a5c3335acfd53';

abstract class _$SettingsFileStorageNotifier
    extends $Notifier<SettingsFileStorage> {
  SettingsFileStorage build();
  @$mustCallSuper
  @override
  void runBuild() {
    final ref = this.ref as $Ref<SettingsFileStorage, SettingsFileStorage>;
    final element =
        ref.element
            as $ClassProviderElement<
              AnyNotifier<SettingsFileStorage, SettingsFileStorage>,
              SettingsFileStorage,
              Object?,
              Object?
            >;
    element.handleCreate(ref, build);
  }
}
