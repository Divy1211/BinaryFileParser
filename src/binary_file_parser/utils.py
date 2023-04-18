class Version(tuple[int, ...]):
    def __repr__(self):
        return f"Version({super().__repr__()})"

    def __str__(self):
        return "v"+'.'.join(map(str, self))

def indentify(repr_str: str, indent = 4) -> str:
    return f"\n{' '*indent}".join(repr_str.splitlines())
