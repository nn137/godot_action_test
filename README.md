# godot_action_test


- https://github.com/firebelley/godot-export
- https://github.com/abarichello/godot-ci


# CI CD Basic

- Needs an `export_presets.cfg` file
- Templates must be present in `~/.local/share/godot/export_templates/${GODOT_VERSION}.stable` folder
  - `export_templates` folder
- Need to use `--headless` and `--export-release` or `--export-debug` flags

# CI CD GDExtension

> TODO, Install rust or C++

> TODO, Compile GDExtension library before exporting Godot game
