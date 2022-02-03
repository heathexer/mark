#!/usr/bin/env python3
import time
import sys
from datetime import datetime
from PIL import Image, ImageDraw, ImageFont

from rgbmatrix import graphics, RGBMatrix, RGBMatrixOptions

options = RGBMatrixOptions()
options.rows = 32 
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

colors = {
    "line": graphics.Color(200, 200, 200),
    "time": graphics.Color(240, 240, 240),
    "month": graphics.Color(18, 93, 152),
    "day": graphics.Color(60, 141, 173),
    "daysrm": graphics.Color(217, 148, 48)
}

def main():
    fc = matrix.CreateFrameCanvas()

    font = graphics.Font()
    font.LoadFont("/src/rpi-rgb-led-matrix/fonts/5x7.bdf")


    textColor = graphics.Color(100, 100, 100)
    barColor = graphics.Color(0, 100, 0)

    loopCount = 0

    while True:
        fc.Clear()

        # Update the current date
        if loopCount % 60 == 0:
            now = datetime.now()
            time = now.strftime("%I:%M")
            month = now.strftime("%b")
            day = now.strftime("%d")
            daysUntil = (datetime(2022, 6, 9) - now).days
            lineProgress = now.timetuple().tm_yday / 365

        # Draw the current time and date
        graphics.DrawText(fc, font, 2, 8, colors["time"], time)
        graphics.DrawText(fc, font, 36, 8, colors["month"], month)
        graphics.DrawText(fc, font, 53, 8, colors["day"], day)

        # Draw the days remaining
        graphics.DrawText(fc, font, 48, 16, colors["daysrm"], f"{daysUntil}")

        # Draw the progress bar outline
        graphics.DrawLine(fc, 2, 11, 45, 11, colors["line"])
        graphics.DrawLine(fc, 2, 14, 45, 14, colors["line"])
        graphics.DrawLine(fc, 2, 12, 2, 13, colors["line"])
        graphics.DrawLine(fc, 45, 12, 45, 13, colors["line"])
        # Draw the progress bar
        progress = int(3 + (44 - 3) * lineProgress)
        graphics.DrawLine(fc, 3, 12, progress, 12, colors["daysrm"])
        graphics.DrawLine(fc, 3, 13, progress, 13, colors["daysrm"])

        fc = matrix.SwapOnVSync(fc)
        loopCount += 1


if __name__ == "__main__":
    main()
