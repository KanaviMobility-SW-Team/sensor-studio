import 'package:flutter/material.dart';

import 'package:flutter_riverpod/flutter_riverpod.dart';

import 'package:sensor_studio/features/sensor/sensor_providers.dart';

class SensorSettingPanel extends ConsumerStatefulWidget {
  const SensorSettingPanel({super.key});

  @override
  ConsumerState<SensorSettingPanel> createState() => _SensorSettingPanelState();
}

class _SensorSettingPanelState extends ConsumerState<SensorSettingPanel> {
  final minControllers = List.generate(4, (i) => TextEditingController());
  final maxControllers = List.generate(4, (i) => TextEditingController());
  ProviderSubscription<SensorFilter>? _filterSub;

  @override
  void initState() {
    super.initState();
    final sensorFilter = ref.read(sensorFilterProvider);

    // 컨트롤러 초기값 설정
    for (int i = 0; i < 4; i++) {
      minControllers[i].text = sensorFilter.getMin(i).toString();
      maxControllers[i].text = sensorFilter.getMax(i).toString();
    }

    // // Provider 값이 바뀔 때마다 컨트롤러 값 동기화
    // ref.listen<SensorFilter>(sensorFilterProvider, (prev, next) {
    //   setState(() {
    //     for (int i = 0; i < 4; i++) {
    //       minControllers[i].text = next.getMin(i).toString();
    //       maxControllers[i].text = next.getMax(i).toString();
    //     }
    //   });
    // });
  }

  @override
  void didChangeDependencies() {
    super.didChangeDependencies();
    _filterSub?.close();
    _filterSub = ref.listenManual<SensorFilter>(sensorFilterProvider, (
      prev,
      next,
    ) {
      setState(() {
        for (int i = 0; i < 4; i++) {
          minControllers[i].text = next.intensityMin[i].toString();
          maxControllers[i].text = next.intensityMax[i].toString();
        }
      });
    });
  }

  @override
  void dispose() {
    _filterSub?.close();
    for (final c in minControllers) {
      c.dispose();
    }
    for (final c in maxControllers) {
      c.dispose();
    }
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return Container(
      color: Colors.blueGrey[900]?.withAlpha(50),
      child: Column(
        children: [
          SizedBox(
            child: Padding(
              padding: EdgeInsets.all(12),
              child: Row(
                children: [
                  SizedBox(width: 30),
                  Text(
                    'Sensor Settings',
                    style: TextStyle(fontSize: 14),
                    textAlign: TextAlign.left,
                  ),
                  const Spacer(),
                ],
              ),
            ),
          ),
          Expanded(
            child: ListView.builder(
              itemCount: 4,
              itemBuilder: (context, i) => Padding(
                padding: const EdgeInsets.symmetric(
                  horizontal: 24,
                  vertical: 8,
                ),
                child: Row(
                  children: [
                    Text('Echo $i', style: TextStyle(color: Colors.white)),
                    SizedBox(width: 16),
                    SizedBox(
                      width: 100,
                      child: TextField(
                        controller: minControllers[i],
                        keyboardType: TextInputType.number,
                        decoration: InputDecoration(
                          labelText: 'Min',
                          labelStyle: TextStyle(color: Colors.white70),
                          filled: true,
                          fillColor: Colors.white10,
                        ),
                        style: TextStyle(color: Colors.white),
                      ),
                    ),
                    SizedBox(width: 16),
                    SizedBox(
                      width: 100,
                      child: TextField(
                        controller: maxControllers[i],
                        keyboardType: TextInputType.number,
                        decoration: InputDecoration(
                          labelText: 'Max',
                          labelStyle: TextStyle(color: Colors.white70),
                          filled: true,
                          fillColor: Colors.white10,
                        ),
                        style: TextStyle(color: Colors.white),
                      ),
                    ),
                  ],
                ),
              ),
            ),
          ),
          SizedBox(
            height: 100,
            child: Row(
              children: [
                const Spacer(),
                Padding(
                  padding: EdgeInsets.symmetric(horizontal: 20),
                  child: ElevatedButton(
                    onPressed: () {
                      final newMin = minControllers
                          .map((c) => int.tryParse(c.text) ?? 0)
                          .toList();
                      final newMax = maxControllers
                          .map((c) => int.tryParse(c.text) ?? 0xFFFFFFFF)
                          .toList();
                      ref
                          .read(sensorFilterProvider.notifier)
                          .state = SensorFilter(
                        intensityMin: newMin,
                        intensityMax: newMax,
                      );
                      FocusScope.of(context).unfocus();
                    },
                    child: Text('확인'),
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
