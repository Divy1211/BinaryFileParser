import traceback
from contextlib import contextmanager
from typing import Callable


@contextmanager
def ignored(*errors, msg: str = "", show_traceback: bool = False, callback: Callable[[Exception], None] = lambda _: ...) -> None:
    """
    Ignores the specified errors for the duration of the context manager block.

    :param errors: The errors to ignore.
    :param msg: If specified, this message is printed to console when an error is caught.
    :param show_traceback: If true, prints the error traceback.
    :param callback: this function is run if an error occurs
    """
    try:
        yield
    except errors as e:
        if msg:
            print(msg)
        if show_traceback:
            print(traceback.format_exc())
        callback(e)
