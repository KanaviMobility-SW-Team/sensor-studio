import 'package:flutter/material.dart';

class AppTheme {
  static ThemeData dark() {
    const background = Color(0xFF0E141B);
    const surface = Color(0xFF161E27);
    const textPrimary = Color(0xFFE5E7EB);

    final colorScheme = ColorScheme.fromSeed(
      seedColor: const Color(0xFF2DD4BF), // accent (cyan/teal)
      brightness: Brightness.dark,
      background: background,
      surface: surface,
    );

    return ThemeData(
      useMaterial3: true,
      colorScheme: colorScheme,
      scaffoldBackgroundColor: background,
      appBarTheme: const AppBarTheme(
        backgroundColor: background,
        foregroundColor: textPrimary,
        elevation: 0,
      ),
      dividerTheme: const DividerThemeData(
        thickness: 1,
      ),
      listTileTheme: const ListTileThemeData(
        iconColor: textPrimary,
        textColor: textPrimary,
      ),
    );
  }
}