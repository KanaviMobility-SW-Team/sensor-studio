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
  bool _isOnButtonHovered = false;
  bool _isOffButtonHovered = false;

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
                  child: Container(
                    decoration: BoxDecoration(
                      color: _isOnButtonHovered
                          ? AppColors.accent.withAlpha(100)
                          : widget.value
                          ? AppColors.accent.withAlpha(100)
                          : Colors.transparent,
                      borderRadius: BorderRadius.circular(5),
                    ),
                    child: InkWell(
                      onHover: (hovering) {
                        setState(() {
                          _isOnButtonHovered = hovering;
                        });
                      },
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
                  child: Container(
                    decoration: BoxDecoration(
                      color: _isOffButtonHovered
                          ? AppColors.accent.withAlpha(100)
                          : !widget.value
                          ? AppColors.accent.withAlpha(100)
                          : Colors.transparent,
                      borderRadius: BorderRadius.circular(5),
                    ),
                    child: InkWell(
                      onHover: (hovering) {
                        setState(() {
                          _isOffButtonHovered = hovering;
                        });
                      },
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
