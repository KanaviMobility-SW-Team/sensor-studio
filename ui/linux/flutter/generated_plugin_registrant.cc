//
//  Generated file. Do not edit.
//

// clang-format off

#include "generated_plugin_registrant.h"

#include <point_glass_opengl/point_glass_opengl_plugin.h>

void fl_register_plugins(FlPluginRegistry* registry) {
  g_autoptr(FlPluginRegistrar) point_glass_opengl_registrar =
      fl_plugin_registry_get_registrar_for_plugin(registry, "PointGlassOpenglPlugin");
  point_glass_opengl_plugin_register_with_registrar(point_glass_opengl_registrar);
}
