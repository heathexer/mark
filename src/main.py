#!/usr/bin/env python3
import time
import sys
from datetime import datetime
from PIL import Image, ImageDraw, ImageFont

from rgbmatrix import graphics, RGBMatrix, RGBMatrixOptions

from life import LifeWidget

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
options.limit_refresh_rate_hz = 100
options.show_refresh_rate = 0

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
    font.LoadFont("/src/app/fonts/5x7.bdf")
    timeFont = graphics.Font()
    timeFont.LoadFont("/src/app/fonts/6x12.bdf")


    textColor = graphics.Color(100, 100, 100)
    barColor = graphics.Color(0, 100, 0)

    loopCount = 0

    lifeWidget = LifeWidget((0, 0), (64, 64))

    while True:
        fc.Clear()

        lifeWidget.render(fc)
        
        # Update the current date
        if loopCount % 1 == 0:
            now = datetime.now()
            time = now.strftime("%-I %M")
            month = now.strftime("%b")
            day = now.strftime("%-d")
            startDate = datetime(2022, 1, 3)
            endDate = datetime(2022, 3, 11)
            daysUntil = (endDate - now).days
            lineProgress = (now - startDate).days / (endDate - startDate).days

        # Draw the current time
        graphics.DrawText(fc, timeFont, 2, 9, colors["time"], time)
        # Blink colon every second
        if (now.second % 2 == 0):
            graphics.DrawText(fc, font, 2 + 6 * (len(time) - 3), 8, colors["time"], ":")

        # Draw the current date
        graphics.DrawText(fc, font, 46 - 5 * len(day), 8, colors["month"], month)
        graphics.DrawText(fc, font, 63 - 5 * len(day), 8, colors["day"], day)

        # Draw the days remaining
        graphics.DrawText(fc, font, 48, 16, colors["daysrm"], f"{daysUntil}")

        # Draw the progress bar outline
        graphics.DrawLine(fc, 2, 11, 45, 11, colors["line"])
        graphics.DrawLine(fc, 2, 14, 45, 14, colors["line"])
        graphics.DrawLine(fc, 2, 12, 2, 13, colors["line"])
        graphics.DrawLine(fc, 45, 12, 45, 13, colors["line"])

        # Draw the progress bar
        progress = int(3 + (44 - 3) * min(1, lineProgress))
        graphics.DrawLine(fc, 3, 12, progress, 12, colors["daysrm"])
        graphics.DrawLine(fc, 3, 13, progress, 13, colors["daysrm"])

        fc = matrix.SwapOnVSync(fc)
        loopCount += 1


if __name__ == "__main__":
    main()
