#!/usr/bin/env bash
set -euo pipefail

readonly UPSTREAM_TAG="ratatui-v0.30.2"
readonly UPSTREAM_COMMIT="e665c36cb14752a61cd777fbd06dbef8474f2add"

if [[ $# -ne 1 ]]; then
  echo "usage: $0 /path/to/ratatui-${UPSTREAM_COMMIT}" >&2
  exit 2
fi

readonly upstream_root="$(cd "$1" && pwd)"
readonly destination_root="$(cd "$(dirname "$0")" && pwd)"
readonly examples_root="${destination_root}/examples"

sync_file() {
  local source_path="$1"
  local destination_path="$2"
  local source="${upstream_root}/${source_path}"
  local destination="${examples_root}/${destination_path}"

  if [[ ! -f "${source}" ]]; then
    echo "missing upstream source: ${source_path}" >&2
    exit 1
  fi

  mkdir -p "$(dirname "${destination}")"
  cp "${source}" "${destination}"
  printf 'synced %-32s <- %s\n' "${destination_path}" "${source_path}"
}

apply_user_input_link_override() {
  local destination="${examples_root}/user_input.rs"
  local upstream_link="https://github.com/rhysd/tui-textarea"
  local maintained_link="https://github.com/ratatui/ratatui-textarea"

  if ! rg --fixed-strings --quiet "${upstream_link}" "${destination}"; then
    echo "user_input.rs no longer contains the expected upstream textarea link" >&2
    exit 1
  fi

  perl -pi -e "s{\\Q${upstream_link}\\E}{${maintained_link}}g" "${destination}"
  printf 'applied website override            %s\n' "user_input.rs textarea link"
}

# App packages are flattened under stable website compatibility names.
sync_file "examples/apps/advanced-widget-impl/src/main.rs" "widget_impl.rs"
sync_file "examples/apps/async-github/src/main.rs" "async.rs"
sync_file "examples/apps/calendar-explorer/src/main.rs" "calendar-explorer/main.rs"
sync_file "examples/apps/canvas/src/main.rs" "canvas-app/main.rs"
sync_file "examples/apps/chart/src/main.rs" "chart-app/main.rs"
sync_file "examples/apps/color-explorer/src/main.rs" "colors.rs"
sync_file "examples/apps/colors-rgb/src/main.rs" "colors_rgb.rs"
sync_file "examples/apps/constraint-explorer/src/main.rs" "constraint-explorer.rs"
sync_file "examples/apps/constraints/src/main.rs" "constraints.rs"
sync_file "examples/apps/custom-widget/src/main.rs" "custom_widget.rs"
sync_file "examples/apps/flex/src/main.rs" "flex.rs"
sync_file "examples/apps/gauge/src/main.rs" "gauge-app/main.rs"
sync_file "examples/apps/hello-world/src/main.rs" "hello_world.rs"
sync_file "examples/apps/hyperlink/src/main.rs" "hyperlink.rs"
sync_file "examples/apps/inline/src/main.rs" "inline.rs"
sync_file "examples/apps/minimal/src/main.rs" "minimal.rs"
sync_file "examples/apps/modifiers/src/main.rs" "modifiers.rs"
sync_file "examples/apps/panic/src/main.rs" "panic.rs"
sync_file "examples/apps/popup/src/main.rs" "popup.rs"
sync_file "examples/apps/scrollbar/src/main.rs" "scrollbar-app/main.rs"
sync_file "examples/apps/table/src/main.rs" "table-app/main.rs"
sync_file "examples/apps/todo-list/src/main.rs" "todo-list/main.rs"
sync_file "examples/apps/tracing/src/main.rs" "tracing.rs"
sync_file "examples/apps/user-input/src/main.rs" "user_input.rs"
apply_user_input_link_override

sync_file "examples/apps/demo/src/app.rs" "demo/app.rs"
sync_file "examples/apps/demo/src/crossterm.rs" "demo/crossterm.rs"
sync_file "examples/apps/demo/src/main.rs" "demo/main.rs"
sync_file "examples/apps/demo/src/termina.rs" "demo/termina.rs"
sync_file "examples/apps/demo/src/termion.rs" "demo/termion.rs"
sync_file "examples/apps/demo/src/termwiz.rs" "demo/termwiz.rs"
sync_file "examples/apps/demo/src/ui.rs" "demo/ui.rs"

sync_file "examples/apps/demo2/src/app.rs" "demo2/app.rs"
sync_file "examples/apps/demo2/src/colors.rs" "demo2/colors.rs"
sync_file "examples/apps/demo2/src/destroy.rs" "demo2/destroy.rs"
sync_file "examples/apps/demo2/src/main.rs" "demo2/main.rs"
sync_file "examples/apps/demo2/src/tabs.rs" "demo2/tabs.rs"
sync_file "examples/apps/demo2/src/tabs/about.rs" "demo2/tabs/about.rs"
sync_file "examples/apps/demo2/src/tabs/email.rs" "demo2/tabs/email.rs"
sync_file "examples/apps/demo2/src/tabs/recipe.rs" "demo2/tabs/recipe.rs"
sync_file "examples/apps/demo2/src/tabs/traceroute.rs" "demo2/tabs/traceroute.rs"
sync_file "examples/apps/demo2/src/tabs/weather.rs" "demo2/tabs/weather.rs"
sync_file "examples/apps/demo2/src/theme.rs" "demo2/theme.rs"

# Widget examples already have the flattened shape. Two historical names remain compatible.
sync_file "ratatui-widgets/examples/barchart-grouped.rs" "barchart-grouped.rs"
sync_file "ratatui-widgets/examples/barchart.rs" "barchart.rs"
sync_file "ratatui-widgets/examples/block.rs" "block.rs"
sync_file "ratatui-widgets/examples/calendar.rs" "calendar.rs"
sync_file "ratatui-widgets/examples/canvas.rs" "canvas.rs"
sync_file "ratatui-widgets/examples/chart.rs" "chart.rs"
sync_file "ratatui-widgets/examples/collapsed-borders.rs" "collapsed-borders.rs"
sync_file "ratatui-widgets/examples/gauge.rs" "gauge.rs"
sync_file "ratatui-widgets/examples/line-gauge.rs" "line_gauge.rs"
sync_file "ratatui-widgets/examples/list.rs" "list.rs"
sync_file "ratatui-widgets/examples/logo.rs" "ratatui-logo.rs"
sync_file "ratatui-widgets/examples/paragraph.rs" "paragraph.rs"
sync_file "ratatui-widgets/examples/scrollbar.rs" "scrollbar.rs"
sync_file "ratatui-widgets/examples/shadow.rs" "shadow.rs"
sync_file "ratatui-widgets/examples/sparkline.rs" "sparkline.rs"
sync_file "ratatui-widgets/examples/table.rs" "table.rs"
sync_file "ratatui-widgets/examples/tabs.rs" "tabs.rs"

printf '\nSynced from %s (%s).\n' "${UPSTREAM_TAG}" "${UPSTREAM_COMMIT}"
printf 'Preserved website-only files: README.md, docsrs.rs, layout.rs\n'
