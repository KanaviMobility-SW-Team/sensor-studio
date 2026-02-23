import 'package:flutter/material.dart';

double _normalize(num value, num min, num max) {
  if (max == min) return 0.0; // 중요
  return ((value - min) / (max - min)).clamp(0.0, 1.0).toDouble();
}

Color rainbowColor(num value, num min, num max) {
  final t = _normalize(value, min, max);

  final hue = 240.0 * (1.0 - t); // 파랑→빨강
  final hsl = HSLColor.fromAHSL(
    1.0, // alpha
    hue,
    1.0, // saturation
    0.5, // lightness (0.5가 가장 선명)
  );

  return hsl.toColor();
}

Color turboColor(num value, num min, num max) {
  final t = _normalize(value, min, max);

  final r =
      34.61 +
      t *
          (1172.33 +
              t *
                  (-10793.56 +
                      t * (33300.12 + t * (-38394.49 + t * 14825.05))));

  final g =
      23.31 +
      t *
          (557.33 +
              t * (1225.33 + t * (-3574.96 + t * (1073.77 + t * -71.18))));

  final b =
      27.2 +
      t *
          (3211.1 +
              t * (-15327.97 + t * (27814.0 + t * (-22569.18 + t * 6838.66))));

  int clamp(double v) => v.clamp(0, 255).toInt();

  return Color.fromARGB(255, clamp(r), clamp(g), clamp(b));
}

Color distanceColorA(num distance, num min, num max) {
  final t = _normalize(distance, min, max);

  final hue = 240.0 * t; // red → blue

  return HSLColor.fromAHSL(1.0, hue, 1.0, 0.5).toColor();
}

Color distanceColorB(num distance, num min, num max) {
  final t = _normalize(distance, min, max);

  final hue = 240.0 * (1.0 - t); // 반전

  return HSLColor.fromAHSL(1.0, hue, 1.0, 0.5).toColor();
}
