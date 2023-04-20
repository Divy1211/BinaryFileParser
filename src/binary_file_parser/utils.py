class Version(tuple[int, ...]):
    """
    - Functions equivalently to ``tuple[int, ...]``, e.g.: ``Version((1, 2, 3))`` is exactly the same as ``(1, 2, 3)``
    - Exists solely for custom ``str()`` and ``repr()`` so ``Version((1, 2, 3))`` is printed as ``v1.2.3``
    """
    def __repr__(self):
        return f"Version({super().__repr__()})"

    def __str__(self):
        return "v"+'.'.join(map(str, self))

    def __init__(self, tup: tuple[int, ...]):
        ...

def indentify(repr_str: str, indent = 4) -> str:
    return f"\n{' '*indent}".join(repr_str.splitlines())

def powers_of_two(init: int = 1):
    c = init
    while True:
        yield c
        c *= 2
