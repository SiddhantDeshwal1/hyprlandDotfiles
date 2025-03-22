# Aliases for file management
function ls
  eza -a --icons $argv
end

function ll
  eza -al --icons $argv
end

function lt
  eza -a --tree --level=1 --icons $argv
end

set -Ux EDITOR nvim

set -g fish_greeting

# FZF setup
set -U FZF_DEFAULT_COMMAND 'fd --hidden --strip-cwd-prefix --exclude .git'
set -U FZF_CTRL_T_COMMAND $FZF_DEFAULT_COMMAND
set -U FZF_ALT_C_COMMAND 'fd --type=d --hidden --strip-cwd-prefix --exclude .git'

function fzf-preview
  if test -d $argv
    eza --tree --color=always $argv | head -200
  else
    bat -n --color=always --line-range :500 $argv
  end
end

set -U FZF_CTRL_T_OPTS "--preview 'fzf-preview {}'"
set -U FZF_ALT_C_OPTS "--preview 'eza --tree --color=always {} | head -200'"

# History setup
set -U HISTFILE ~/.fish_history
set -U HISTSIZE 10000
set -U SAVEHIST 10000
set -U appendhistory 1

# Directory specific aliases
function setup_dir_aliases
  if test $PWD = $HOME/competitiveProgramming/contest
    alias add="python $HOME/competitiveProgramming/contest/cf.py add"
    alias load="python $HOME/competitiveProgramming/contest/cf.py load"
    alias check="$HOME/competitiveProgramming/contest/run.sh"
    alias submit="python $HOME/competitiveProgramming/contest/cf.py submit"
    alias runcpp="g++ -o workspace workspace.cpp && ./workspace"
  else if test $PWD = $HOME/competitiveProgramming/editor
    alias add="python $HOME/competitiveProgramming/editor/cf.py add"
    alias load="python $HOME/competitiveProgramming/editor/cf.py load"
    alias check="python $HOME/competitiveProgramming/editor/cf.py check"
    alias submit="python $HOME/competitiveProgramming/editor/cf.py submit"
    alias last="python $HOME/competitiveProgramming/editor/cf.py last"
    alias contest="python $HOME/competitiveProgramming/editor/cf.py contest"
    alias runcpp="g++ -o workspace workspace.cpp && ./workspace"
    alias friends="python $HOME/competitiveProgramming/editor/cf.py friends"
  else
    functions -e add load check runcpp submit
  end
end

function cd
  builtin cd $argv
  setup_dir_aliases
end

setup_dir_aliases

# TheFuck setup
function fuck
    set -l fucked_up_command $history[1]
    env TF_SHELL=fish TF_ALIAS=fuck PYTHONIOENCODING=utf-8 thefuck $fucked_up_command THEFUCK_ARGUMENT_PLACEHOLDER $argv | read -l unfucked_command
    if test "$unfucked_command" != ""
        eval $unfucked_command
        history --delete --exact --case-sensitive -- $fucked_up_command
        history --merge
    end
end

function fk
    set -l fucked_up_command $history[1]
    env TF_SHELL=fish TF_ALIAS=fk PYTHONIOENCODING=utf-8 thefuck $fucked_up_command THEFUCK_ARGUMENT_PLACEHOLDER $argv | read -l unfucked_command
    if test "$unfucked_command" != ""
        eval $unfucked_command
        history --delete --exact --case-sensitive -- $fucked_up_command
        history --merge
    end
end


function on_keypress --on-event fish_key_reader
    if test -z (commandline)  # If input is empty
        echo -e '\a'  # Beep sound
    end
end

# Set up fzf key bindings
fzf --fish | source

# Set up Bat and Eza
set -U BAT_THEME tokyonight_night

# DevasLife git status
set -g tide_git_bg_color 268bd2
set -g tide_git_bg_color_unstable C4A000
set -g tide_git_bg_color_urgent CC0000
set -g tide_git_branch_color 000000
set -g tide_git_color_branch 000000
set -g tide_git_color_conflicted 000000
set -g tide_git_color_dirty 000000
set -g tide_git_color_operation 000000
set -g tide_git_color_staged 000000
set -g tide_git_color_stash 000000
set -g tide_git_color_untracked 000000
set -g tide_git_color_upstream 000000
set -g tide_git_conflicted_color 000000
set -g tide_git_dirty_color 000000
set -g tide_git_icon 
set -g tide_git_operation_color 000000
set -g tide_git_staged_color 000000
set -g tide_git_stash_color 000000
set -g tide_git_untracked_color 000000
set -g tide_git_upstream_color 000000
set -g tide_pwd_bg_color 444444

#yazi
function y
	set tmp (mktemp -t "yazi-cwd.XXXXXX")
	yazi $argv --cwd-file="$tmp"
	if set cwd (command cat -- "$tmp"); and [ -n "$cwd" ]; and [ "$cwd" != "$PWD" ]
		builtin cd -- "$cwd"
	end
	rm -f -- "$tmp"
end

set -Ux EDITOR nvim
