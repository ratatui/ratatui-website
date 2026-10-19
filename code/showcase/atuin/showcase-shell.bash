# Sourced by the recording shell; all Atuin state is local to the demo.
export ATUIN_HOME="$PWD/target/showcase/atuin"
export ATUIN_CONFIG_DIR="$ATUIN_HOME"
export PS1='\[\e[1;32m\]❯ \[\e[0m\]'
export ATUIN_SESSION=019a0000000070008000000000000000
_showcase_search() {
    READLINE_LINE=$(atuin search --interactive -- "$READLINE_LINE" 2> /dev/tty)
    READLINE_POINT=${#READLINE_LINE}
}
bind -x '"\C-r": _showcase_search'
clear
