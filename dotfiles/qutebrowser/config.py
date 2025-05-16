import rosepine

# load your autoconfig, use this, if the rest of your config is empty!
config.load_autoconfig()

# set the palette you'd like to use
# valid options are 'rose-pine', 'rose-pine-moon' and 'rose-pine-dawn'
# last argument (optional, default is False): enable the plain look for the menu rows
rosepine.setup(c, 'rose-pine-moon', True)

