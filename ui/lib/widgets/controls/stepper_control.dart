import 'package:flutter/material.dart';

import 'package:ui/theme/app_colors.dart';

class StepperControl extends StatefulWidget {
  final String label;
  final int value;
  final int step;
  final int min;
  final int max;
  final ValueChanged<int> onChanged;

  const StepperControl({
    super.key,
    required this.label,
    required this.value,
    required this.step,
    required this.min,
    required this.max,
    required this.onChanged,
  });

  @override
  State<StepperControl> createState() => _StepperControlState();
}

class _StepperControlState extends State<StepperControl> {
  bool _isHovered = false;
  bool _isLeftButtonHovered = false;
  bool _isRightButtonHovered = false;

  @override
  Widget build(BuildContext context) {
    return Row(
      mainAxisAlignment: MainAxisAlignment.spaceBetween,
      children: [
        SizedBox(
          width: 110,
          child: Text(
            widget.label,
            style: const TextStyle(color: Colors.white70, fontSize: 12),
          ),
        ),
        Expanded(
          child: MouseRegion(
            onEnter: (_) => setState(() => _isHovered = true),
            onExit: (_) => setState(() => _isHovered = false),
            child: Container(
              padding: const EdgeInsets.all(5.0),
              decoration: BoxDecoration(
                color: _isHovered
                    ? Colors.white.withAlpha(20)
                    : Colors.transparent,
                borderRadius: BorderRadius.circular(6),
              ),
              child: Row(
                mainAxisAlignment: MainAxisAlignment.center,
                children: [
                  IgnorePointer(
                    ignoring: !_isHovered,
                    child: AnimatedOpacity(
                      opacity: _isHovered ? 1.0 : 0.0,
                      duration: const Duration(milliseconds: 150),
                      child: Container(
                        decoration: BoxDecoration(
                          color: _isLeftButtonHovered
                              ? AppColors.accent.withAlpha(100)
                              : Colors.transparent,
                          borderRadius: BorderRadius.circular(9),
                        ),
                        child: InkWell(
                          onHover: (hovering) {
                            setState(() {
                              _isLeftButtonHovered = hovering;
                            });
                          },
                          onTap: widget.value > widget.min
                              ? () => widget.onChanged(
                                  (widget.value - widget.step).clamp(
                                    widget.min,
                                    widget.max,
                                  ),
                                )
                              : null,
                          child: widget.value > widget.min
                              ? const Padding(
                                  padding: EdgeInsets.all(2.0),
                                  child: Icon(
                                    Icons.chevron_left,
                                    size: 18,
                                    color: Colors.white70,
                                  ),
                                )
                              : const SizedBox(width: 20, height: 20),
                        ),
                      ),
                    ),
                  ),
                  SizedBox(
                    width: 56,
                    child: Text(
                      '${widget.value}',
                      textAlign: TextAlign.center,
                      style: const TextStyle(color: Colors.white, fontSize: 13),
                    ),
                  ),
                  IgnorePointer(
                    ignoring: !_isHovered,
                    child: AnimatedOpacity(
                      opacity: _isHovered ? 1.0 : 0.0,
                      duration: const Duration(milliseconds: 150),
                      child: Container(
                        decoration: BoxDecoration(
                          color: _isRightButtonHovered
                              ? AppColors.accent.withAlpha(100)
                              : Colors.transparent,
                          borderRadius: BorderRadius.circular(9),
                        ),
                        child: InkWell(
                          onHover: (hovering) {
                            setState(() {
                              _isRightButtonHovered = hovering;
                            });
                          },
                          onTap: widget.value < widget.max
                              ? () => widget.onChanged(
                                  (widget.value + widget.step).clamp(
                                    widget.min,
                                    widget.max,
                                  ),
                                )
                              : null,
                          child: widget.value < widget.max
                              ? const Padding(
                                  padding: EdgeInsets.all(2.0),
                                  child: Icon(
                                    Icons.chevron_right,
                                    size: 18,
                                    color: Colors.white70,
                                  ),
                                )
                              : const SizedBox(width: 20, height: 20),
                        ),
                      ),
                    ),
                  ),
                ],
              ),
            ),
          ),
        ),
      ],
    );
  }
}

class DoubleStepperControl extends StatefulWidget {
  final String label;
  final double value;
  final double step;
  final double min;
  final double max;
  final ValueChanged<double> onChanged;

  const DoubleStepperControl({
    super.key,
    required this.label,
    required this.value,
    required this.step,
    required this.min,
    required this.max,
    required this.onChanged,
  });

  @override
  State<DoubleStepperControl> createState() => _DoubleStepperControlState();
}

class _DoubleStepperControlState extends State<DoubleStepperControl> {
  bool _isHovered = false;
  bool _isLeftButtonHovered = false;
  bool _isRightButtonHovered = false;

  double _round(double val) => (val / widget.step).round() * widget.step;

  @override
  Widget build(BuildContext context) {
    return Row(
      mainAxisAlignment: MainAxisAlignment.spaceBetween,
      children: [
        SizedBox(
          width: 110,
          child: Text(
            widget.label,
            style: const TextStyle(color: Colors.white70, fontSize: 12),
          ),
        ),
        Expanded(
          child: MouseRegion(
            onEnter: (_) => setState(() => _isHovered = true),
            onExit: (_) => setState(() => _isHovered = false),
            child: Container(
              padding: const EdgeInsets.all(5.0),
              decoration: BoxDecoration(
                color: _isHovered
                    ? Colors.white.withAlpha(20)
                    : Colors.transparent,
                borderRadius: BorderRadius.circular(6),
              ),
              child: Row(
                mainAxisAlignment: MainAxisAlignment.center,
                children: [
                  IgnorePointer(
                    ignoring: !_isHovered,
                    child: AnimatedOpacity(
                      opacity: _isHovered ? 1.0 : 0.0,
                      duration: const Duration(milliseconds: 150),
                      child: Container(
                        decoration: BoxDecoration(
                          color: _isLeftButtonHovered
                              ? AppColors.accent.withAlpha(100)
                              : Colors.transparent,
                          borderRadius: BorderRadius.circular(9),
                        ),
                        child: InkWell(
                          onHover: (hovering) {
                            setState(() {
                              _isLeftButtonHovered = hovering;
                            });
                          },
                          onTap: widget.value > widget.min
                              ? () => widget.onChanged(
                                  _round(
                                    widget.value - widget.step,
                                  ).clamp(widget.min, widget.max),
                                )
                              : null,
                          child: widget.value > widget.min
                              ? const Padding(
                                  padding: EdgeInsets.all(2.0),
                                  child: Icon(
                                    Icons.chevron_left,
                                    size: 18,
                                    color: Colors.white70,
                                  ),
                                )
                              : const SizedBox(width: 20, height: 20),
                        ),
                      ),
                    ),
                  ),
                  SizedBox(
                    width: 56,
                    child: Text(
                      widget.value.toStringAsFixed(widget.step < 1.0 ? 1 : 0),
                      textAlign: TextAlign.center,
                      style: const TextStyle(color: Colors.white, fontSize: 13),
                    ),
                  ),
                  IgnorePointer(
                    ignoring: !_isHovered,
                    child: AnimatedOpacity(
                      opacity: _isHovered ? 1.0 : 0.0,
                      duration: const Duration(milliseconds: 150),
                      child: Container(
                        decoration: BoxDecoration(
                          color: _isRightButtonHovered
                              ? AppColors.accent.withAlpha(100)
                              : Colors.transparent,
                          borderRadius: BorderRadius.circular(9),
                        ),
                        child: InkWell(
                          onHover: (hovering) {
                            setState(() {
                              _isRightButtonHovered = hovering;
                            });
                          },
                          onTap: widget.value < widget.max
                              ? () => widget.onChanged(
                                  _round(
                                    widget.value + widget.step,
                                  ).clamp(widget.min, widget.max),
                                )
                              : null,
                          child: widget.value < widget.max
                              ? const Padding(
                                  padding: EdgeInsets.all(2.0),
                                  child: Icon(
                                    Icons.chevron_right,
                                    size: 18,
                                    color: Colors.white70,
                                  ),
                                )
                              : const SizedBox(width: 20, height: 20),
                        ),
                      ),
                    ),
                  ),
                ],
              ),
            ),
          ),
        ),
      ],
    );
  }
}
