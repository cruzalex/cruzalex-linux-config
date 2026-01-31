-- cruzAlex Colorscheme Configuration
-- Theme is controlled by cruzalex theme system

return {
  -- TokyoNight (default)
  {
    "folke/tokyonight.nvim",
    lazy = false,
    priority = 1000,
    opts = {
      style = "night",
      transparent = false,
      terminal_colors = true,
      styles = {
        comments = { italic = true },
        keywords = { italic = true },
        functions = {},
        variables = {},
        sidebars = "dark",
        floats = "dark",
      },
      sidebars = { "qf", "help", "terminal", "packer", "neo-tree" },
      dim_inactive = false,
      lualine_bold = true,
    },
  },

  -- Catppuccin
  {
    "catppuccin/nvim",
    name = "catppuccin",
    priority = 1000,
    opts = {
      flavour = "mocha",
      transparent_background = false,
      term_colors = true,
      integrations = {
        cmp = true,
        gitsigns = true,
        nvimtree = true,
        telescope = true,
        treesitter = true,
        notify = true,
        mini = true,
        native_lsp = {
          enabled = true,
          virtual_text = {
            errors = { "italic" },
            hints = { "italic" },
            warnings = { "italic" },
            information = { "italic" },
          },
          underlines = {
            errors = { "underline" },
            hints = { "underline" },
            warnings = { "underline" },
            information = { "underline" },
          },
        },
      },
    },
  },

  -- Gruvbox
  {
    "ellisonleao/gruvbox.nvim",
    priority = 1000,
    opts = {
      contrast = "hard",
      transparent_mode = false,
    },
  },

  -- Nord
  {
    "shaunsingh/nord.nvim",
    priority = 1000,
  },

  -- OneDark
  {
    "navarasu/onedark.nvim",
    priority = 1000,
    opts = {
      style = "darker",
    },
  },

  -- Dracula
  {
    "Mofiqul/dracula.nvim",
    priority = 1000,
  },

  -- Cobalt2 (custom for cruzAlex)
  {
    "lalitmee/cobalt2.nvim",
    priority = 1000,
    dependencies = { "tjdevries/colorbuddy.nvim" },
  },

  -- Solarized
  {
    "maxmx03/solarized.nvim",
    priority = 1000,
    opts = {},
  },
}
