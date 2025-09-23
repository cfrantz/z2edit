# Utilities
from z2edit import gui


class DragHelper(object):

    def __init__(self):
        self.number = None
        self.amount = gui.Vec2(0, 0)

    def start(self, number):
        if self.number is None:
            self.number = number
            self.amount = gui.Vec2(0, 0)

    def position(self, number, position):
        if self.number == number:
            self.amount = position

    def drag(self, number, amount):
        if self.number == number:
            self.amount += amount

    def delta(self, number):
        if self.number == number:
            return self.amount
        return gui.Vec2(0, 0)

    def finalize(self, number):
        if self.number == number:
            self.number = None
            return self.amount
        return None
