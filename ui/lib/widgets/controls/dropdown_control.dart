import 'package:flutter/material.dart';

import 'package:ui/theme/app_colors.dart';

class DropdownControl extends StatelessWidget {
  final String label;
  final String value;
  final List<String> items;
  final ValueChanged<String?> onChanged;

  const DropdownControl({
    super.key,
    required this.label,
    required this.value,
    required this.items,
    required this.onChanged,
  });

  @override
  Widget build(BuildContext context) {
    return Row(
      mainAxisAlignment: MainAxisAlignment.spaceBetween,
      children: [
        SizedBox(
          width: 110,
          child: Text(
            label,
            style: const TextStyle(color: Colors.white70, fontSize: 12),
          ),
        ),
        Expanded(
          child: DropdownButton<String>(
            padding: const EdgeInsets.all(5.0),
            value: value,
            isDense: true,
            isExpanded: true,
            iconEnabledColor: AppColors.accent,
            dropdownColor: const Color(0xFF2C2C2C),
            style: const TextStyle(color: Colors.white, fontSize: 12),
            underline: Container(height: 1, color: Colors.white24),
            items: items
                .map((i) => DropdownMenuItem(value: i, child: Text(i)))
                .toList(),
            onChanged: onChanged,
          ),
        ),
      ],
    );
  }
}
