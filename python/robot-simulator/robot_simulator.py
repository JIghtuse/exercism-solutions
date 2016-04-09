from enum import Enum

class Direction(Enum):
    NORTH = 0
    EAST = 1
    SOUTH = 2
    WEST = 3

NORTH = Direction.NORTH
EAST = Direction.EAST
WEST = Direction.WEST
SOUTH = Direction.SOUTH

DIRECTION_LIST = list(Direction)

class Robot:
    def __init__(self, bearing=Direction.NORTH, x=0, y=0):
        self.x = x
        self.y = y
        self.bearing = bearing

    @property
    def coordinates(self):
        return (self.x, self.y)

    def turn_right(self):
        idx = (DIRECTION_LIST.index(self.bearing) + 1) % len(DIRECTION_LIST)
        self.bearing = DIRECTION_LIST[idx]

    def turn_left(self):
        idx = (DIRECTION_LIST.index(self.bearing) - 1) % len(DIRECTION_LIST)
        self.bearing = DIRECTION_LIST[idx]

    def advance(self):
        if self.bearing == Direction.NORTH:
            self.y += 1
        elif self.bearing == Direction.SOUTH:
            self.y -= 1
        elif self.bearing == Direction.EAST:
            self.x += 1
        elif self.bearing == Direction.WEST:
            self.x -= 1

    def simulate(self, commands):
        for command in commands:
            if command == 'R':
                self.turn_right()
            elif command == 'L':
                self.turn_left()
            elif command == 'A':
                self.advance()
