#!/usr/bin/env python3
import time
import sys
from datetime import datetime

from rgbmatrix import graphics, RGBMatrix, RGBMatrixOptions

options = RGBMatrixOptions()
options.rows =32 
options.cols = 64
options.chain_length = 2
options.parallel = 1
options.hardware_mapping = "adafruit-hat"
options.pixel_mapper_config = "V-mapper"
options.row_address_type = 0
options.multiplexing = 0
options.gpio_slowdown = 4
options.limit_refresh_rate_hz = 60

matrix = RGBMatrix(options = options)

def main():
    fc = matrix.CreateFrameCanvas()

    font = graphics.Font()
    font.LoadFont("/src/rpi-rgb-led-matrix/fonts/4x6.bdf")

    textColor = graphics.Color(100, 100, 100)
    barColor = graphics.Color(0, 100, 0)

    loopCount = 0

    while True:
        fc.Clear()

        # Update the current date
        if loopCount % 60 == 0:
            now = datetime.now()
            time = now.strftime("%I:%M")
            date = now.strftime("%b%d")
            daysUntil = (datetime(2022, 6, 9) - now).days
            lineProgress = now.timetuple().tm_yday / 365
            dateString = f"{time} {date} {daysUntil}"

        # Draw the current time
        graphics.DrawText(fc, font, 2, 8, textColor, dateString)

        # Draw the progress bar outline
        graphics.DrawLine(fc, 2, 10, 61, 10, textColor)
        graphics.DrawLine(fc, 2, 13, 61, 13, textColor)
        graphics.DrawLine(fc, 2, 11, 2, 12, textColor)
        graphics.DrawLine(fc, 61, 11, 61, 12, textColor)
        # Draw the progress bar
        progress = int(3 + (60 - 3) * lineProgress)
        graphics.DrawLine(fc, 3, 11, progress, 11, barColor)
        graphics.DrawLine(fc, 3, 12, progress, 12, barColor)

        fc = matrix.SwapOnVSync(fc)
        loopCount += 1


if __name__ == "__main__":
    main()
