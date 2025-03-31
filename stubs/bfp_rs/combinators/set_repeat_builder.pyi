from typing import Any

from bfp_rs.retrievers import Retriever

from bfp_rs.combinators.combinator import Combinator
from bfp_rs.combinators.get import Get


class SetRepeatBuilder:
    """
    """

    def by(self, from_: Get) -> Combinator:
        """
        Set the repeat of the previously selected property to the result after performing the manipulations from the
        provided ``Get`` instance

        Args:
            *from_: The ``Get`` instance that encodes the manipulations to perform

        Returns:
            A combinator that encodes the setter logic defined by the ``SetRepeatBuilder`` chain
        """


    def from_(self, *source: Retriever | int) -> Combinator:
        """
        Set the repeat of the previously selected property to the value of this retriever

        Args:
            *source: The retriever path to fetch the value from. This can be a sequence of retrievers/list indices
                starting in the current struct

        Returns:
            A combinator that encodes the setter logic defined by the ``SetRepeatBuilder`` chain
        """


    def from_len(self, *source: Retriever | int) -> Combinator:
        """
        Set the repeat of the previously selected property to the length of this value if it is a list

        Args:
            *source: The retriever path to fetch the value from. This can be a sequence of retrievers/list indices
                starting in the current struct

        Returns:
            A combinator that encodes the setter logic defined by the ``SetRepeatBuilder`` chain

        Raises:
            ValueError: if the source value is not a list
        """


    def to(self, val: Any) -> Combinator:
        """
        Set the repeat of the previously selected property to this literal value

        Args:
            val: The literal value

        Returns:
            A combinator that encodes the setter logic defined by the ``SetRepeatBuilder`` chain
        """


def set_repeat(target: Retriever) -> SetRepeatBuilder:
    """
    Set the repeat of this property using another value

    Args:
        *target: The retriever path to set the repeat for. This can be a sequence of retrievers/list indices
            starting in the current struct

    Returns:
        A ``SetRepeatBuilder`` instance to continue defining additional combinator properties
    """

