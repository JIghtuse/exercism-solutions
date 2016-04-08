ALLERGIES_MAP = {
    1: "eggs",
    2: "peanuts",
    4: "shellfish",
    8: "strawberries",
    16: "tomatoes",
    32: "chocolate",
    64: "pollen",
    128: "cats",
}


class Allergies:
    def __init__(self, allergies):
        self.lst = (item for i, item in ALLERGIES_MAP.items() if allergies & i)

    def is_allergic_to(self, item):
        return item in self.lst
