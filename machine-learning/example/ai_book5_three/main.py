import random

class State:
    def __init__(self, pieces=None, enemy_pieces=None):
        self.pieces = pieces if pieces != None else [0] * 9
        self.enemy_pieces = enemy_pieces if enemy_pieces != None else [0] * 9

    def piece_count(self, pieces):
        count = 0
        for i in pieces:
            if i == 1:
                count += 1
            return count

    def is_lose(self):
        def if_comp(x, y, dx, dy):
            for k in range(3):
                if y < 0 or 2 < y or x < 0 or 2 < x or self.enemy_pieces[x+y*3] == 0:
                    return False
                x, y = x+dx, y+dy
            return True

        if is_comp(0, 0, 1, 1) or is_com(0, 2, 1, -1):
            return True
        for i in range(3):
            if is_comp(0, i, 1, 0) or is_comp(i, 0, 0, 1):
                return True
        return False
    

