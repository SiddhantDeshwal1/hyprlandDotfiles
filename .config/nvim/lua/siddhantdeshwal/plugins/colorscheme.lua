return {
  "craftzdog/solarized-osaka.nvim",
  opts = {
        transparent = false ,
    },

    config = function(_, opts)
    require("solarized-osaka").setup(opts) -- optional setup call
	require("solarized-osaka").load()
	end,
}


-- return {
--     "navarasu/onedark.nvim",
--     lazy = false , 
--     priority = 1000 , 
--     config = function()
--         require("onedark").setup {
--             style = "deep"
--         }
--         require("onedark").load()
--     end
-- }

