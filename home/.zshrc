# cruzAlex ZSH Configuration
# TUI-first, keyboard-driven shell experience

# ============================================================================
# PATH
# ============================================================================
export PATH="$HOME/.config/cruzalex/bin:$HOME/.cargo/bin:$HOME/.local/bin:$PATH"

# ============================================================================
# ENVIRONMENT
# ============================================================================
export EDITOR="nvim"
export VISUAL="nvim"
export PAGER="less"
export TERMINAL="ghostty"
export BROWSER="chromium"

# XDG Base Directories
export XDG_CONFIG_HOME="$HOME/.config"
export XDG_DATA_HOME="$HOME/.local/share"
export XDG_CACHE_HOME="$HOME/.cache"
export XDG_STATE_HOME="$HOME/.local/state"

# Wayland
export MOZ_ENABLE_WAYLAND=1
export QT_QPA_PLATFORM=wayland
export SDL_VIDEODRIVER=wayland
export _JAVA_AWT_WM_NONREPARENTING=1

# ============================================================================
# HISTORY
# ============================================================================
HISTFILE="$HOME/.zsh_history"
HISTSIZE=50000
SAVEHIST=50000
setopt EXTENDED_HISTORY
setopt HIST_EXPIRE_DUPS_FIRST
setopt HIST_IGNORE_DUPS
setopt HIST_IGNORE_ALL_DUPS
setopt HIST_IGNORE_SPACE
setopt HIST_FIND_NO_DUPS
setopt HIST_SAVE_NO_DUPS
setopt SHARE_HISTORY
setopt APPEND_HISTORY
setopt INC_APPEND_HISTORY

# ============================================================================
# OPTIONS
# ============================================================================
setopt AUTO_CD
setopt AUTO_PUSHD
setopt PUSHD_IGNORE_DUPS
setopt PUSHD_SILENT
setopt CORRECT
setopt INTERACTIVE_COMMENTS
setopt NO_BEEP

# ============================================================================
# COMPLETION
# ============================================================================
autoload -Uz compinit
compinit -d "$XDG_CACHE_HOME/zsh/zcompdump-$ZSH_VERSION"

zstyle ':completion:*' menu select
zstyle ':completion:*' matcher-list 'm:{a-zA-Z}={A-Za-z}'
zstyle ':completion:*' list-colors "${(s.:.)LS_COLORS}"
zstyle ':completion:*' group-name ''
zstyle ':completion:*:descriptions' format '%F{yellow}-- %d --%f'
zstyle ':completion:*:warnings' format '%F{red}-- no matches found --%f'

# ============================================================================
# KEY BINDINGS (vi mode)
# ============================================================================
bindkey -v
export KEYTIMEOUT=1

# Keep useful emacs bindings in insert mode
bindkey '^A' beginning-of-line
bindkey '^E' end-of-line
bindkey '^K' kill-line
bindkey '^U' backward-kill-line
bindkey '^W' backward-kill-word
bindkey '^Y' yank
bindkey '^R' history-incremental-search-backward
bindkey '^P' up-line-or-history
bindkey '^N' down-line-or-history

# Delete key
bindkey '^?' backward-delete-char
bindkey '^H' backward-delete-char

# ============================================================================
# ALIASES - Core
# ============================================================================
alias c='clear'
alias q='exit'
alias ..='cd ..'
alias ...='cd ../..'
alias ....='cd ../../..'

# ============================================================================
# ALIASES - Modern replacements (TUI-first)
# ============================================================================
alias ls='eza --icons --group-directories-first'
alias ll='eza -la --icons --group-directories-first --git'
alias la='eza -a --icons --group-directories-first'
alias lt='eza --tree --level=2 --icons'
alias tree='eza --tree --icons'

alias cat='bat --style=auto'
alias less='bat --style=auto --paging=always'

alias grep='rg'
alias find='fd'

alias top='btop'
alias htop='btop'

alias vim='nvim'
alias vi='nvim'
alias v='nvim'

alias lg='lazygit'
alias ld='lazydocker'

# ============================================================================
# ALIASES - Git
# ============================================================================
alias g='git'
alias gs='git status -sb'
alias ga='git add'
alias gc='git commit'
alias gp='git push'
alias gl='git pull'
alias gd='git diff'
alias gco='git checkout'
alias gb='git branch'
alias glog='git log --oneline --graph --decorate -10'

# ============================================================================
# ALIASES - System
# ============================================================================
alias reload='source ~/.zshrc'
alias zshrc='$EDITOR ~/.zshrc'

# Package management (set by distro detection)
if command -v dnf &> /dev/null; then
    alias pki='sudo dnf install'
    alias pks='dnf search'
    alias pku='sudo dnf upgrade'
    alias pkr='sudo dnf remove'
elif command -v pacman &> /dev/null; then
    alias pki='sudo pacman -S'
    alias pks='pacman -Ss'
    alias pku='sudo pacman -Syu'
    alias pkr='sudo pacman -Rs'
fi

# ============================================================================
# ALIASES - cruzAlex Theme System
# ============================================================================
alias theme='cruzalex-theme-set'
alias themes='cruzalex-theme-list'
alias theme-install='cruzalex-theme-install'

# ============================================================================
# FUNCTIONS
# ============================================================================

# Create directory and cd into it
mkcd() {
    mkdir -p "$1" && cd "$1"
}

# Quick file/directory search with fzf
f() {
    local file
    file=$(fd --type f --hidden --exclude .git | fzf --preview 'bat --color=always --style=numbers {}')
    [[ -n "$file" ]] && $EDITOR "$file"
}

# Quick directory jump with fzf
d() {
    local dir
    dir=$(fd --type d --hidden --exclude .git | fzf --preview 'eza --tree --level=1 --icons {}')
    [[ -n "$dir" ]] && cd "$dir"
}

# Extract any archive
extract() {
    if [[ -f "$1" ]]; then
        case "$1" in
            *.tar.bz2)   tar xjf "$1"    ;;
            *.tar.gz)    tar xzf "$1"    ;;
            *.tar.xz)    tar xJf "$1"    ;;
            *.bz2)       bunzip2 "$1"    ;;
            *.gz)        gunzip "$1"     ;;
            *.tar)       tar xf "$1"     ;;
            *.tbz2)      tar xjf "$1"    ;;
            *.tgz)       tar xzf "$1"    ;;
            *.zip)       unzip "$1"      ;;
            *.Z)         uncompress "$1" ;;
            *.7z)        7z x "$1"       ;;
            *.rar)       unrar x "$1"    ;;
            *)           echo "'$1' cannot be extracted" ;;
        esac
    else
        echo "'$1' is not a valid file"
    fi
}

# ============================================================================
# INTEGRATIONS
# ============================================================================

# Zoxide (smart cd)
if command -v zoxide &> /dev/null; then
    eval "$(zoxide init zsh)"
    alias cd='z'
fi

# FZF
if command -v fzf &> /dev/null; then
    # FZF options
    export FZF_DEFAULT_COMMAND='fd --type f --hidden --exclude .git'
    export FZF_DEFAULT_OPTS='
        --height 40%
        --layout=reverse
        --border
        --info=inline
        --preview-window=right:50%:wrap
    '
    export FZF_CTRL_T_COMMAND="$FZF_DEFAULT_COMMAND"
    export FZF_ALT_C_COMMAND='fd --type d --hidden --exclude .git'

    # Source fzf keybindings if available
    [[ -f /usr/share/fzf/key-bindings.zsh ]] && source /usr/share/fzf/key-bindings.zsh
    [[ -f /usr/share/fzf/completion.zsh ]] && source /usr/share/fzf/completion.zsh
fi

# Starship prompt
if command -v starship &> /dev/null; then
    eval "$(starship init zsh)"
fi

# ============================================================================
# LOCAL OVERRIDES
# ============================================================================
[[ -f ~/.zshrc.local ]] && source ~/.zshrc.local
