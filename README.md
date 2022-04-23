# mark
A clock/smart display for the standard rpi-led-matrix (using [this Rust crate](https://crates.io/crates/rpi-led-matrix)) with various widgets and simulation(s, eventually).

https://github.com/heathexer/mark/blob/main/example.mp4?raw=true

# Widgets
Each widget has a size and position which is set on the creation of its struct. In addition, widgets with a background process must be passed a channel which is used to communicate with its main process.

## Life
This widget plays Conway's Game of Life, and functions as the background of our display. It updates the game state once every time `render()` is called, at approximately 100Hz. 

* The game automatically resets to a random position every time a steady state is detected. It does this by comparing the current state to the state 60 cycles ago, which allows it to detect any combination of cycles of 1, 2, 3, 4, 5, 6, 10, 12, 15, or 30 frames. This has let it run for days at a time without getting stuck on a boring repition, although there are a few situations where it can still get stuck. One of these is when a vertical or horizontal glider forms and gets a clear path across the entire screen, which is rare but possible. This can't happen with regular, diagonal gliders since the grid is not quite a square, so it will eventually move across every space (technically it could if the entire screen was clear, but that's super unlikely). 
* Alive color and dead color can be configured from config.json to customize the look.

## Time
This widget is in charge of the time and date, the bread and butter of clocks worldwide. It's relatively simple, just takes the current system time and date and draws them to the screen with configurable colors. The most complicated part is blinking the colon and caculating the positions of text based on where it should be anchored.

## Countdown
This is another simple widget. It has a configurable start and end date, and it will draw a bar of the current progress between the two. It will also show the number of days left until the end date, and resize the progress bar to fit next to the text.
* If the current date is before the `startDate`, it will still show the correct number of days until the `endDate` and the bar will show one pixel of progress.
* If the current date is after the `endDate`, the bar will be filled and the number of days remaining will be negative and accurate.

## Presence
The purpose of this widget is to display who's currently in the same house as Mark. To show up on the display, each person puts their name, wifi id, and a custom rgb color into the config. When they're connected to the wifi, their name shows up in their color.

* This is probably the least general purpose widget, and will likely only work with the same specific XFinity router that we happen to have. It scrapes our router status page to find ids of currently connected devices. Although this is more consistent than pinging the network and easier to manage than Bluetooth, it is still not really ideal. Apart from compatability issues, people's phones often randomize their wifi id. This can be diabled per network, but is still an extra step.

## Weather
This widget grabs weather information from openweathermap.org and displays an icon, the current temperature, and a low and high temp.

* Location is given as longitude and latitude coordinates in config.json
* Icons are 8x8 pixel images in src/images/weather
* The color of the temperature text changes with the temperature. It interpolates from 40°F to 80°F, between `coldColor` and `warmColor` in the config  
