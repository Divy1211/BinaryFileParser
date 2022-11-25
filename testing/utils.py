import time
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


exec_time = {}
@contextmanager
def timed(name: str = "", /, *, print_time: bool = False) -> None:
    """
    Records the execution time of the context manager block.

    :param name: If specified, the execution time will be recorded in the exec_times dict with this name as the key
    :param print_time: If true, print the context manager execution time to console
    """
    global exec_time

    start = time.perf_counter_ns()
    yield
    end = time.perf_counter_ns()

    runtime = (end-start)/10**6
    if name:
        exec_time[name] = runtime
    if print_time:
        print(name, "took:", runtime, "ms")
