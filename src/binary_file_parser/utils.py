import contextlib
from io import StringIO


class TabbedStringIO(StringIO):
    def __init__(self, ident: int = 0):
        super().__init__()
        self.ident = ident

    @contextlib.contextmanager
    def tabbed(self):
        self.ident += 4
        yield
        self.ident -= 4

    def _tab(self) -> str:
        return ' '*self.ident

    def writeln(self, string: str = ""):
        self.write("\n" + self._tab() + string)
