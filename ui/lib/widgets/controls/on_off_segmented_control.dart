import 'package:flutter/material.dart';

import 'package:ui/theme/app_colors.dart';

class OnOffSegmentedControl extends StatefulWidget {
  final String label;
  final bool value;
  final ValueChanged<bool> onChanged;

  const OnOffSegmentedControl({
    super.key,
    required this.label,
    required this.value,
    required this.onChanged,
  });

  @override
  State<OnOffSegmentedControl> createState() => _OnOffSegmentedControlState();
}

class _OnOffSegmentedControlState extends State<OnOffSegmentedControl> {
  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.only(top: 2.0, bottom: 2.0),
      child: Row(
        mainAxisAlignment: MainAxisAlignment.spaceBetween,
        children: [
          SizedBox(
            width: 110,
            child: Text(
              widget.label,
              style: const TextStyle(color: Colors.white70, fontSize: 13),
            ),
          ),
          Expanded(
            child: Row(
              mainAxisAlignment: MainAxisAlignment.center,
              children: [
                Expanded(
                  child: Material(
                    shape: RoundedRectangleBorder(
                      borderRadius: BorderRadius.circular(5),
                    ),
                    color: widget.value
                        ? AppColors.accent.withAlpha(100)
                        : Colors.transparent,
                    child: InkWell(
                      borderRadius: BorderRadius.circular(5),
                      mouseCursor: SystemMouseCursors.click,
                      hoverColor: AppColors.accent.withAlpha(100),
                      splashColor: AppColors.accent.withAlpha(100),
                      highlightColor: AppColors.accent.withAlpha(50),
                      onTap: () {
                        widget.onChanged(true);
                      },
                      child: Padding(
                        padding: const EdgeInsets.all(5.0),
                        child: SizedBox(
                          width: 50,
                          child: Text(
                            'On',
                            textAlign: TextAlign.center,
                            style: const TextStyle(
                              color: Colors.white,
                              fontSize: 13,
                            ),
                          ),
                        ),
                      ),
                    ),
                  ),
                ),
                const SizedBox(width: 3),
                Expanded(
                  child: Material(
                    shape: RoundedRectangleBorder(
                      borderRadius: BorderRadius.circular(5),
                    ),
                    color: !widget.value
                        ? AppColors.accent.withAlpha(100)
                        : Colors.transparent,
                    child: InkWell(
                      borderRadius: BorderRadius.circular(5),
                      mouseCursor: SystemMouseCursors.click,
                      hoverColor: AppColors.accent.withAlpha(100),
                      splashColor: AppColors.accent.withAlpha(100),
                      highlightColor: AppColors.accent.withAlpha(50),
                      onTap: () {
                        widget.onChanged(false);
                      },
                      child: Padding(
                        padding: const EdgeInsets.all(5.0),
                        child: Text(
                          'Off',
                          textAlign: TextAlign.center,
                          style: const TextStyle(
                            color: Colors.white,
                            fontSize: 13,
                          ),
                        ),
                      ),
                    ),
                  ),
                ),
              ],
            ),
          ),
        ],
      ),
    );
  }
}
