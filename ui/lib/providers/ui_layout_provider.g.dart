// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'ui_layout_provider.dart';

// **************************************************************************
// RiverpodGenerator
// **************************************************************************

// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint, type=warning

@ProviderFor(UILayout)
final uILayoutProvider = UILayoutProvider._();

final class UILayoutProvider
    extends $NotifierProvider<UILayout, UILayoutState> {
  UILayoutProvider._()
    : super(
        from: null,
        argument: null,
        retry: null,
        name: r'uILayoutProvider',
        isAutoDispose: true,
        dependencies: null,
        $allTransitiveDependencies: null,
      );

  @override
  String debugGetCreateSourceHash() => _$uILayoutHash();

  @$internal
  @override
  UILayout create() => UILayout();

  /// {@macro riverpod.override_with_value}
  Override overrideWithValue(UILayoutState value) {
    return $ProviderOverride(
      origin: this,
      providerOverride: $SyncValueProvider<UILayoutState>(value),
    );
  }
}

String _$uILayoutHash() => r'773c49db5b720eda9d526e352cba7dd61f5b50c3';

abstract class _$UILayout extends $Notifier<UILayoutState> {
  UILayoutState build();
  @$mustCallSuper
  @override
  void runBuild() {
    final ref = this.ref as $Ref<UILayoutState, UILayoutState>;
    final element =
        ref.element
            as $ClassProviderElement<
              AnyNotifier<UILayoutState, UILayoutState>,
              UILayoutState,
              Object?,
              Object?
            >;
    element.handleCreate(ref, build);
  }
}
