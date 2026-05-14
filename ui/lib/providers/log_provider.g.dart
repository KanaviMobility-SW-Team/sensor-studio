// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'log_provider.dart';

// **************************************************************************
// RiverpodGenerator
// **************************************************************************

// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint, type=warning

@ProviderFor(SystemLogs)
final systemLogsProvider = SystemLogsProvider._();

final class SystemLogsProvider
    extends $NotifierProvider<SystemLogs, List<String>> {
  SystemLogsProvider._()
    : super(
        from: null,
        argument: null,
        retry: null,
        name: r'systemLogsProvider',
        isAutoDispose: true,
        dependencies: null,
        $allTransitiveDependencies: null,
      );

  @override
  String debugGetCreateSourceHash() => _$systemLogsHash();

  @$internal
  @override
  SystemLogs create() => SystemLogs();

  /// {@macro riverpod.override_with_value}
  Override overrideWithValue(List<String> value) {
    return $ProviderOverride(
      origin: this,
      providerOverride: $SyncValueProvider<List<String>>(value),
    );
  }
}

String _$systemLogsHash() => r'10cf3193b1b757fe326bae1a429c6e530c13c5fc';

abstract class _$SystemLogs extends $Notifier<List<String>> {
  List<String> build();
  @$mustCallSuper
  @override
  void runBuild() {
    final ref = this.ref as $Ref<List<String>, List<String>>;
    final element =
        ref.element
            as $ClassProviderElement<
              AnyNotifier<List<String>, List<String>>,
              List<String>,
              Object?,
              Object?
            >;
    element.handleCreate(ref, build);
  }
}
