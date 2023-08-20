import random

class SlotArm():
    def __init__(self, p):
        self.p = p

    def draw(self):
        if self.p > random.random():
            return 1.0
        else:
            return 0.0

