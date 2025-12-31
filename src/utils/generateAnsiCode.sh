#!/bin/bash

set -euo pipefail

PROJECT_ROOT=$(git rev-parse --show-toplevel)
INPUT_FILE="${PROJECT_ROOT}/code/tutorials/quickstart-ratatui/src/main.rs"
OUTPUT_FILE="${PROJECT_ROOT}/src/assets/code-example.ts"
THEME="night-owlish"
TARGET_WIDTH=64 #Line width excluding padding

bat_highlight() {
  bat --theme="$THEME" --color=always --decorations=never "$INPUT_FILE"
}

# Pad lines to fixed line width of 66 with background color
pad_lines_with_bg() {
  while IFS= read -r line; do
    original="$line"
    # Strip ANSI codes for width calculation
    stripped=$(echo "$line" | sed 's/\x1b\[[0-9;]*m//g')
    # Get display width
    line_len=$(echo -n "$stripped" | wc -L)
    
    if [ "$line_len" -lt "$TARGET_WIDTH" ]; then
      padding=$((TARGET_WIDTH - line_len))
      pad_str=$(printf "%${padding}s" "")
      echo "${original}\\x1b[48;2;1;22;39m${pad_str}\\x1b[0m"
    else
      echo "${original}\\x1b[0m"
    fi
  done
}

add_blank_line() {
  local position="$1"
  local pattern="$2"
  local blank_line='\x1b[48;2;1;22;39m                                                                \x1b[0m'
  
  case "$position" in
    before)
      sed "/${pattern}/i\\${blank_line}"
      ;;
    after)
      sed "/${pattern}/a\\${blank_line}"
      ;;
    around)
      sed "/${pattern}/{i\\${blank_line}
a\\${blank_line}
}"
      ;;
  esac
}

apply_blank_line_rules() {
  local input="$1"
  local output="$input"
  
  # Array of position:pattern pairs
  local rules=(
    "before:use.*::"
    "after:fn.*main"
    "after:let.*block"
    "before:frame.*render"
    "around:std.*thread.*sleep"
    "after:Ok"
  )
  
  for rule in "${rules[@]}"; do
    local position="${rule%%:*}"
    local pattern="${rule#*:}"
    output=$(echo "$output" | add_blank_line "$position" "$pattern")
  done
  
  echo "$output"
}

add_prefix_to_brackets() {
  sed 's/\[/\\x1b\[/g'
}

add_background_to_spaces() {
  sed 's/ /\\x1b[48;2;1;22;39m&/g'
}

add_background_to_codes() {
  sed 's/\\x1b\[38;2/\\x1b[48;2;1;22;39m\\x1b[38;2/g'
}

add_blank_lines() {
  # Add top blank lines with background color \x1b[48;2;1;22;39m 
  sed '1i\\x1b[48;2;1;22;39m                                                                \x1b[0m' | \
  # Add bottom blank line with background color \x1b[48;2;1;22;39m 
  sed '$a\\x1b[48;2;1;22;39m                                                                \x1b[0m' | \
  # Add background color \x1b[48;2;1;22;39m to all remaining blank lines
  sed 's/^$/\\x1b[48;2;1;22;39m                                                                \\x1b[0m/'
}

add_side_margins() {
  sed '/./ s/^/\\x1b[48;2;1;22;39m \\x1b[0m/'
}

build_ansi_code() {
  {
    echo -n 'export const ANSI_CODE = `'
    local content=$(bat_highlight | pad_lines_with_bg)
    content=$(apply_blank_line_rules "$content")
    echo "$content" \
      | add_prefix_to_brackets \
      | add_background_to_spaces \
      | add_background_to_codes \
      | add_blank_lines \
      | add_side_margins
    echo '`;'
  } > "$OUTPUT_FILE"
}

main() {
  build_ansi_code
}

main

