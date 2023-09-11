import random
import math

class State:
    def __init__(self, pieces=None, enemy_pieces=None):
        self.pieces = pieces if pieces != None else [0] * 42
        self.enemy_pieces = enemy_pieces if enemy_pieces != None else [0] * 42

    def piece_count(self, pieces):
        count = 0
        for i in pieces:
            if i == 1:
                count += 1
        return count

    def is_lose(self):
        def is_comp(x, y, dx, dy):
            for k in range(4):
                if y < 0 or 5 < y or x < 0 or 6 < x or self.enemy_pieces[x+y*7] == 0:
                    return False
                x, y = x+dx, y+dy
            return True

        for j in range(6):
            for i in range(7):
                if is_comp(i, j, 1, 0) or is_comp(i, j, 0, 1) or is_comp(i, j, 1, -1) or is_comp(i, j, 1, 1):
                    return True
        return False
    
    def is_draw(self):
        return self.piece_count(self.pieces) + self.piece_count(self.enemy_pieces) == 42

    def is_done(self):
        return self.is_lose() or self.is_draw()

    def next(self, action):
        pieces = list(self.pieces)
        for i in range(5, -1, -1):
            if self.pieces[action+i*7] == 0 and self.enemy_pieces[action+i*7] == 0:
                pieces[action+i*7] = 1
                break
        return State(self.enemy_pieces, pieces)

    def legal_actions(self):
        actions = []
        for i in range(7):
            if self.pieces[i] == 0 and self.enemy_pieces[i] == 0:
                actions.append(i)
        return actions

    def is_first_player(self):
        return self.piece_count(self.pieces) == self.piece_count(self.enemy_pieces)

    def __str__(self):
        ox = ('o', 'x') if self.is_first_player() else ('x', 'o')
        str = ''
        for i in range(42):
            if self.pieces[i] == 1:
                str += ox[0]
            elif self.enemy_pieces[i] == 1:
                str += ox[1]
            else:
                str += '-'
            if i % 7 == 6:
                str += '\n'
        return str


def random_action(state):
    legal_actions = state.legal_actions()
    return legal_actions[random.randint(0, len(legal_actions)-1)]


if __name__ == '__main__':
    state = State()

    while True:
        if state.is_done():
            break

        state = state.next(random_action(state))

        print(state)
        print("")
