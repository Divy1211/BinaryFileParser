import contextlib
import time


@contextlib.contextmanager
def timed():
    start = time.perf_counter_ns()
    yield
    end = time.perf_counter_ns()
    print((end-start) / 1000_000_000)
