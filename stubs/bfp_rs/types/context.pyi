class Context:
    """
    Holds context key/values during read/write
    """

    def __new__(cls, **kwargs: tuple[Any, Any]):
        """
        Args:
            **kwargs: A dict of (data_type, value) pairs to create the context from
        """