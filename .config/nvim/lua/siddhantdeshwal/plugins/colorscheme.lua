return {
  "craftzdog/solarized-osaka.nvim",
  opts = {
    transparent = false,
  },

  config = function(_, opts)
    require("solarized-osaka").setup(opts) -- optional setup call
    require("solarized-osaka").load()
  end,
}
