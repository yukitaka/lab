import random
import math

class State:
    def __init__(self, pieces=None, enemy_pieces=None, depth=0):
        self.dxy = ((1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0), (-1, -1), (0, -1), (1, -1))
        self.pass_end = False

        self.pieces = pieces
        self.enemy_pieces = enemy_pieces
        self.depth = depth

        if pieces == None or enemy_pieces == None:
            self.pieces = [0] * 36
            self.pieces[14] = self.pieces[21] = 1
            self.enemy_pieces = [0] * 36
            self.enemy_pieces[15] = self.enemy_pieces[20] = 1

    def piece_count(self, pieces):
        count = 0
        for i in pieces:
            if i == 1:
                count += 1
        return count

    def is_lose(self):
        return self.is_done() and self.piece_count(self.pieces) < self.piece_count(self.enemy_pieces)

    def is_draw(self):
        return self.is_done() and self.piece_count(self.pieces) == self.piece_count(self.enemy_pieces)

    def is_done(self):
        return self.piece_count(self.pieces) + self.piece_count(self.enemy_pieces) == 36 or self.pass_end

    def next(self, action):
        state = State(self.pieces.copy(), self.enemy_pieces.copy(), self.depth+1)
        if action != 36:
            state.is_legal_action_xy(action%6, int(action/6), True)
        w = state.pieces
        state.pieces = state.enemy_pieces
        state.enemy_pieces = w

        if action == 36 and state.legal_actions() == [36]:
            state.pass_end = True
        return state

    def legal_actions(self):
        actions = []
        for x in range(0, 6):
            for y in range(0, 6):
                if self.is_legal_action_xy(x, y):
                    actions.append(x+y*6)
        if len(actions) == 0:
            actions.append(36)
        return actions

    def is_legal_action_xy(self, x, y, flip=False):
        def is_legal_action_xy_dxy(x, y, dx, dy):
            x, y = x+dx, y+dy
            if y < 0 or 5 < y or x < 0 or 5 < x or self.enemy_pieces[x+y*6] != 1:
                return False

            for j in range(6):
                if y < 0 or 5 < y or x < 0 or 5 < x or (self.enemy_pieces[x+y*6] == 0 and self.pieces[x+y*6] == 0):
                    return False

                if self.pieces[x+y*6] == 1:
                    if flip:
                        for i in range(6):
                            x, y = x-dx, y-dy
                            if self.pieces[x+y*6] == 1:
                                return True
                            self.pieces[x+y*6] = 1
                            self.enemy_pieces[x+y*6] = 0
                    return True
                x, y = x+dx, y+dy
            return False
        if self.enemy_pieces[x+y*6] == 1 or self.pieces[x+y*6] == 1:
            return False

        if flip:
            self.pieces[x+y*6] = 1

        flag = False
        for dx, dy in self.dxy:
            if is_legal_action_xy_dxy(x, y, dx, dy):
                flag = True
        return flag

    def is_first_player(self):
        return self.depth%2 == 0

    def __str__(self):
        ox = ('o', 'x') if self.is_first_player() else ('x', 'o')
        str = ''
        for i in range(36):
            if self.pieces[i] == 1:
                str += ox[0]
            elif self.enemy_pieces[i] == 1:
                str += ox[1]
            else:
                str += '-'
            if i % 6 == 5:
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
