import contextlib


def set_mut(ls: list, val: bool):
    """
    Set the mutability for a list.

    Args:
        ls: The list
        val: If set to ``False``, disallow methods like ``append`` or ``extend`` on the provided list
    """

@contextlib.contextmanager
def borrow_mut(ls: list):
    """
    Context manager to temporarily allow mutating a list.

    Usage:
        ```
        with borrow_mut(struct.ls):
            struct.ls.append(...)
        ```

    Args:
        ls: The list
    """