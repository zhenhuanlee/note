# Iterm的小技巧
## SSH的时候改变背景颜色
```shell
set_iterm_bgcolor(){
  local R=$1
  local G=$2
  local B=$3

  # set tab color
  echo -ne "\033]6;1;bg;red;brightness;$R\a"
  echo -ne "\033]6;1;bg;green;brightness;$G\a"
  echo -ne "\033]6;1;bg;blue;brightness;$B\a"

  # set bg color
  /usr/bin/osascript <<EOF
tell application "iTerm"
  tell the current window
    tell the current session
      set background color to {$(($R*65535/255)), $(($G*65535/255)), $(($B*65535/255))}
    end tell
  end tell
end tell
EOF
}

iterm-reset() {
  set_iterm_bgcolor 0 0 0
  echo -ne "\033]6;1;bg;*;default\a"
}

# Change the color of the tab when using SSH
# reset the color after the connection closes
color-ssh() {
  if [[ -n "$ITERM_SESSION_ID" ]]; then
    trap "iterm-reset" INT EXIT
    if [[ "$*" =~ "jude@115" ]]; then
      set_iterm_bgcolor 0 139 139
    elif [[ "$*" =~ "fizz@45" ]]; then
      set_iterm_bgcolor 139 69 19
    else
      set_iterm_bgcolor 139 69 19
    fi
  fi
  ssh $*
}
compdef _ssh color-ssh=ssh

alias ssh=color-ssh
```
