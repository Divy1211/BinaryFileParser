from typing import Any

from bfp_rs.retrievers import Retriever

from bfp_rs.combinators.combinator import Combinator
from bfp_rs.combinators.get import Get


class SetBuilder:
    """
    Constructs combinators to set retriever values
    """

    def by(self, from_: Get) -> Combinator:
        """
        Set the previously selected value to the result after performing the manipulations from the provided ``Get``
        instance

        Args:
            *from_: The ``Get`` instance that encodes the manipulations to perform

        Returns:
            A combinator that encodes the setter logic defined by the ``SetBuilder`` chain
        """


    def from_(self, *source: Retriever | int) -> Combinator:
        """
        Set the previously selected value to the value of this retriever

        Args:
            *source: The retriever path to fetch the value from. This can be a sequence of retrievers/list indices
                starting in the current struct

        Returns:
            A combinator that encodes the setter logic defined by the ``SetBuilder`` chain
        """


    def from_len(self, *source: Retriever | int) -> Combinator:
        """
        Set the previously selected value to the length of this value if it is a list

        Args:
            *source: The retriever path to fetch the value from. This can be a sequence of retrievers/list indices
                starting in the current struct

        Returns:
            A combinator that encodes the setter logic defined by the ``SetBuilder`` chain

        Raises:
            ValueError: if the source value is not a list
        """


    def to(self, val: Any) -> Combinator:
        """
        Set the previously selected value to this literal value

        Args:
            val: The literal value

        Returns:
            A combinator that encodes the setter logic defined by the ``SetBuilder`` chain
        """


def set_(*target: Retriever | int) -> SetBuilder:
    """
    Set this value using another value

    Args:
        *target: The retriever path to set the value at. This can be a sequence of retrievers/list indices
            starting in the current struct

    Returns:
        A ``SetBuilder`` instance to continue defining additional combinator properties
    """

