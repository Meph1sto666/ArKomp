# ArKomp

A (not yet so lightweight) plugin-based server to render an overlay for L2Ds.

## Contents

<!--toc:start-->
- [ArKomp](#arkomp)
  - [Contents](#contents)
  - [Getting started](#getting-started)
  - [Security](#security)
  - [Commands](#commands)
    - [LoadPlugin](#loadplugin)
    - [UnloadPlugin](#unloadplugin)
    - [SpawnOperator](#spawnoperator)
    - [RetreatOperator](#retreatoperator)
    - [OperatorInfo](#operatorinfo)
  - [Events](#events)
  - [Window rules](#window-rules)
    - [Hyprland](#hyprland)
  - [Contributing / Development](#contributing-development)
    - [Roadmap / TODOs](#roadmap-todos)
<!--toc:end-->

> !Windows is not yet tested, expect possibly weird UI behaviour.

Please note that this project does not implement any rendering logic, but only the overlay (as "canvas") and merely serves as a server managing renderers.
Plugins with character render logics can be [found here](https://github.com/meph1sto666/arkomp-plugins)

## Getting started

Clone the repo and run the server in release mode.

```sh
git clone https://github.com/Meph1sto666/ArKomp.git
cd ArKomp
cargo run --release & disown
```

As this only hosts the server and opens the plugins need to be built as well.

```sh
cd plugins/
git clone https://github.com/meph1sto666/arkomp-plugins.git
git checkout dev
cargo build --release -p <plugin name> && cp target/release/*.so ../
cd -
```

A small example script to spawn a cute kitty!

```sh
#!/bin/bash
WS_URL="ws://127.0.0.1:2887"
COMMANDS=(
  '{"command": "LoadPlugin", "name": "operator", "path": "plugins/liboperator.so"}'

  '{"command": "SpawnOperator", "name": "char_391_rosmon_sale#16", "plugin": "operator"}'
  '{"command": "ScheduleEvent", "SetPosition": {"to": "char_391_rosmon_sale#16", "position": [1810, 1060]}}'
  '{"command": "ScheduleEvent", "SetAnimation": {"to": "char_391_rosmon_sale#16", "ani": "Sit"}}'
  '{"command": "ScheduleEvent", "Resize": {"to": "char_391_rosmon_sale#16", "scale": 0.9}}'
  '{"command": "ScheduleEvent", "SetFacingDirection": {"to": "char_391_rosmon_sale#16", "direction": "left"}}'
)

send_command() {
  local command="$1"
  echo "$command" | websocat "$WS_URL"
}

for cmd in "${COMMANDS[@]}"; do
  echo "$cmd"
  send_command "$cmd"
done
```

## Security

Only load plugins from trusted sources; inspect them if possible.
Run the application as unprivileged user and **not** as root. (unless you know very well what you're doing)

## Commands

Commands are sent via a web socket default port `2887` and handled by the program.

### LoadPlugin

Load a plugin file and register it. Currently only Operator plugins are supported.

```json
{
  "command": "LoadPlugin",
  "name": "operator", // name for the internal registry
  "path": "plugins/liboperator.so" // relative to arkomp root or absolute
}
```

### UnloadPlugin

Unload a plugin file and deregister it.

```json
{
  "command": "UnloadPlugin",
  "name": "operator", // name of the loaded plugin
}
```

### SpawnOperator

```json
{
  "command": "SpawnOperator",
  "name": "character_name", // name for the internal registry
  "plugin": "operator" // plugin to use for loading
}
```

### RetreatOperator

```json
{
  "command": "RetreatOperator",
  "name": "<name of registry entry>" // must match name from SpawnOperator
}
```

### OperatorInfo

```json
{
  "command": "OperatorInfo",
  "name": "<name of registry entry>" // must match name from SpawnOperator
}
```

## Events

Events can be both an event action taken by the plugin as well as a command for the plugin.

An event can be scheduled via the command `ScheduleEvent` like this

```json
{
  "command": "ScheduleEvent",
  "<event name>": {
    // event arguments
  }
}
```

|   Variant          | Payload | Description |
|--------------------|---------|-------------|
| SetPosition        | {"to": String, "position": [f32, f32]}         | Set the position of an OP |
| SetSkin            | {"to": String, "skin": String}                 | Change the skin of an OP |
| SetAnimation       | {"to": String, "ani": String}                  | Change the animation of "to" to "ani" |
| MoveTo             | {"to": String, "position": [f32, f32]}          | Walk to a position. |
| Resize             | {"to": String, "scale": f32}                   | Adjust the scale of an operator |
| SetFacingDirection | {"to": String, "direction": String} | Set the OP facing "left"/"right" |
| CustomEvent | {"from": String, "to": String, "payload": String} | Custom event, payload may be json|

## Window rules

### Hyprland

To use the overlay on Hyprland I recommend setting a window-rule similar to this one.

```conf
windowrule {
  name = arkomp
  float = on
  no_blur = on
  no_focus = on
  no_shadow = on
  border_size = 0
  size = (monitor_w*1) (monitor_h*1)
  pin = on
  xray = 0
  opacity = 1 override 1 override
  match:title = ^([Aa]rkomp.*)$
}
```

## Contributing / Development

There's still a lot of work to do, I plan to switch to iced probably (as egui has its limitation on wayland mouse passthrough etc.) and rewrite a bunch, will see about that sooner or later.

If you open a PR please do so on the dev branch.

### Roadmap / TODOs

- [ ] plugin hot-reloading
- [ ] multi screen support
- [ ] allow for region based mouse capture
- [ ] performance
  - [ ] caching
