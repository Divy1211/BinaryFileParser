from bfp_rs.retrievers import Retriever

from bfp_rs.combinators.combinator import Combinator
from bfp_rs.combinators.get import Get


class SetKeyBuilder:
    """
    Constructs combinators to set context key values
    """

    def by(self, from_: Get) -> Combinator:
        """
        Set the previously selected context key to the result after performing the manipulations from the provided ``Get``
        instance

        Args:
            *from_: The ``Get`` instance that encodes the manipulations to perform

        Returns:
            A combinator that encodes the setter logic defined by the ``SetKeyBuilder`` chain
        """

    def to(self, data_type: Any, source: Any) -> Combinator:
        """
        Set this key to the given type from the given value

        Args:
            data_type: The type of the key
            source: The value to set this key to

        Returns:
            A combinator that encodes the setter logic defined by the ``SetKeyBuilder`` chain
        """

    def from_(self, *source: Retriever | int) -> Combinator:
        """
        Set the previously selected context key to the value of this retriever

        Args:
            *source: The retriever path to fetch the value from. This can be a sequence of retrievers/list indices
                starting in the current struct

        Returns:
            A combinator that encodes the setter logic defined by the ``SetKeyBuilder`` chain
        """


    def from_len(self, *source: Retriever | int) -> Combinator:
        """
        Set the previously selected context key to the length of this value if it is a list

        Args:
            *source: The retriever path to fetch the value from. This can be a sequence of retrievers/list indices
                starting in the current struct

        Returns:
            A combinator that encodes the setter logic defined by the ``SetKeyBuilder`` chain

        Raises:
            ValueError: if the source value is not a list
        """


def set_key(key: str) -> SetKeyBuilder:
    """
    Set this context key using another value

    Args:
        *key: They context key to set

    Returns:
        A ``SetKeyBuilder`` instance to continue defining additional combinator properties
    """
