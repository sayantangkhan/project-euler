from fractions import Fraction

class TangentVector:
    def __init__(self, point: tuple[Fraction, Fraction], direction: Fraction):
        self.point = point
        self.direction = direction

    def __repr__(self):
        return f"({self.point}, {self.direction})"


def problem208():
    pass