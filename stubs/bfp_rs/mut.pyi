import contextlib


def set_mut(ls: list, val: bool):
    """
    Set the mutability for a list

    Args:
        ls: The list
        val: If set to ``False``, disallow methods like ``append`` or ``extend``
    """

@contextlib.contextmanager
def borrow_mut(ls: list) -> list:
    """
    Temporarily allow mutating a list

    Args:
        ls: The list

    Returns:

    """