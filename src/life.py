import numpy as np
import queue
from rgbmatrix import graphics

class LifeWidget:
    def __init__(self, position, size):
        self.size = size
        self.position = position

        self.aliveColor = graphics.Color(20, 0, 0)
        self.deadColor = graphics.Color(0, 0, 0)

        self.board = np.random.choice([0, 0, 1], size)

        self.oldboards = queue.Queue(2)
        # pretend this isn't bad
        self.oldboards.put(self.board)
        self.oldboards.put(self.board)

    def countBoard(self):
        return sum([np.roll(self.board, roll, axis=(1,0)) \
            for roll in [(0, 1), (0, -1), (1, 0), (-1, 0),
                         (1, 1), (1, -1), (-1, 1), (-1, -1)]])

    def update(self):
        newBoard = np.zeros(self.size)

        count = self.countBoard()

        for x in range(self.size[0]):
            for y in range(self.size[1]):
                if self.board[x, y]:
                    newBoard[x, y] = 1 if (count[x, y] == 2) or (count[x, y] == 3) else 0
                else:
                    newBoard[x, y] = 1 if count[x, y] == 3 else 0

        # Reset if state is steady sometimes
        if np.array_equal(newBoard, self.oldboards.get()):
            self.board = np.random.choice([0, 0, 1], self.size)
            self.oldboards.put(self.board)
        else:
            self.board = newBoard
            self.oldboards.put(newBoard)

    def render(self, canvas):
        self.update()

        for x in range(self.size[0]):
            for y in range(self.size[1]):
                canvas.SetPixel(self.position[0] + x,
                                self.position[1] + y, 
                                50 if self.board[x,y] == 1 else 0,
                                0,
                                0)
