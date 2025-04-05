class MutabilityError(ValueError):
    """
    Thrown when a list is set as immutable and if mutation is attempted.

    See Also: ``set_mut`` and ``borrow_mut``
    """
