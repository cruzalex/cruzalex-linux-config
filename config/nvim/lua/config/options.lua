-- cruzAlex Neovim Options
-- TUI-first, keyboard-driven editing

local opt = vim.opt

-- General
opt.mouse = "a"
opt.clipboard = "unnamedplus"
opt.swapfile = false
opt.undofile = true
opt.undolevels = 10000

-- UI
opt.number = true
opt.relativenumber = true
opt.cursorline = true
opt.signcolumn = "yes"
opt.termguicolors = true
opt.showmode = false
opt.cmdheight = 1
opt.laststatus = 3
opt.scrolloff = 8
opt.sidescrolloff = 8
opt.wrap = false
opt.linebreak = true
opt.pumheight = 10
opt.pumblend = 10
opt.winblend = 0
opt.splitbelow = true
opt.splitright = true
opt.splitkeep = "screen"

-- Search
opt.ignorecase = true
opt.smartcase = true
opt.hlsearch = true
opt.incsearch = true

-- Indentation
opt.tabstop = 2
opt.shiftwidth = 2
opt.softtabstop = 2
opt.expandtab = true
opt.smartindent = true
opt.autoindent = true
opt.shiftround = true

-- Completion
opt.completeopt = { "menu", "menuone", "noselect" }
opt.wildmode = "longest:full,full"

-- Performance
opt.updatetime = 200
opt.timeoutlen = 300
opt.redrawtime = 1500
opt.lazyredraw = false

-- Folding
opt.foldlevel = 99
opt.foldlevelstart = 99
opt.foldenable = true

-- Spelling
opt.spelllang = { "en" }

-- Fill chars
opt.fillchars = {
  foldopen = "",
  foldclose = "",
  fold = " ",
  foldsep = " ",
  diff = "╱",
  eob = " ",
}

-- Grep
if vim.fn.executable("rg") == 1 then
  opt.grepformat = "%f:%l:%c:%m"
  opt.grepprg = "rg --vimgrep"
end

-- Disable some default providers
vim.g.loaded_node_provider = 0
vim.g.loaded_perl_provider = 0
vim.g.loaded_ruby_provider = 0

-- Fix markdown indentation
vim.g.markdown_recommended_style = 0
