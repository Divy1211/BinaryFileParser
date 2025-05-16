from bfp_rs.types import Version
from bfp_rs.retrievers import Retriever

from bfp_rs.combinators.combinator import Combinator
from bfp_rs.combinators.get import Get


class IfBuilder:
    """
    Constructs combinators that may execute conditionally during parsing, depending on the values of their retriever
    inputs
    """

    def is_none(self) -> IfBuilder:
        """
        Checks if the previously selected value is none

        Returns:
            An ``IfBuilder`` instance to continue defining additional combinator properties
        """

    def then(self, *coms: Combinator) -> Combinator:
        """
        Finishes the if-combinator construction by defining a nested combinator, should be called last on an
        ``IfBuilder``

        Args:
            *coms: The nested combinators to run if the conditions for this if combinator are met

        Returns:
            A combinator that encodes the conditional logic defined by the ``IfBuilder`` chain
        """


    def eq(self, *source: Retriever | int | Get) -> IfBuilder:
        """
        Checks if the previously selected value is equal to this value

        Args:
            *source: The retriever path to fetch the value from. This can be a sequence of retrievers/list indices
                starting in the current struct

        Returns:
            An ``IfBuilder`` instance to continue defining additional combinator properties
        """


    def ne(self, *source: Retriever | int | Get) -> IfBuilder:
        """
        Checks if the previously selected value is not equal to this value

        Args:
            *source: The retriever path to fetch the value from. This can be a sequence of retrievers/list indices
                starting in the current struct

        Returns:
            An ``IfBuilder`` instance to continue defining additional combinator properties
        """


    def gt(self, *source: Retriever | int | Get) -> IfBuilder:
        """
        Checks if the previously selected value is greater than this value

        Args:
            *source: The retriever path to fetch the value from. This can be a sequence of retrievers/list indices
                starting in the current struct

        Returns:
            An ``IfBuilder`` instance to continue defining additional combinator properties
        """


    def ge(self, *source: Retriever | int | Get) -> IfBuilder:
        """
        Checks if the previously selected value is greater than or equal to this value

        Args:
            *source: The retriever path to fetch the value from. This can be a sequence of retrievers/list indices
                starting in the current struct

        Returns:
            An ``IfBuilder`` instance to continue defining additional combinator properties
        """


    def lt(self, *source: Retriever | int | Get) -> IfBuilder:
        """
        Checks if the previously selected value is lesser than this value

        Args:
            *source: The retriever path to fetch the value from. This can be a sequence of retrievers/list indices
                starting in the current struct

        Returns:
            An ``IfBuilder`` instance to continue defining additional combinator properties
        """


    def le(self, *source: Retriever | int | Get) -> IfBuilder:
        """
        Checks if the previously selected value is lesser than or equal to this value

        Args:
            *source: The retriever path to fetch the value from. This can be a sequence of retrievers/list indices
                starting in the current struct

        Returns:
            An ``IfBuilder`` instance to continue defining additional combinator properties
        """


def if_(target: Retriever | int | Get) -> IfBuilder:
    """
    Select this value for a comparison

    Args:
        *target: The retriever path to fetch the value from. This can be a sequence of retrievers/list indices
            starting in the current struct

    Returns:
        An ``IfBuilder`` instance to continue defining additional combinator properties
    """


def if_not(target: Retriever | int | Get) -> IfBuilder:
    """
    Select this value for a comparison with the result inverted

    Args:
        *target: The retriever path to fetch the value from. This can be a sequence of retrievers/list indices
            starting in the current struct

    Returns:
        An ``IfBuilder`` instance to continue defining additional combinator properties
    """


def if_key(key: str) -> IfBuilder:
    """
    Select this context key for a comparison

    Args:
        key: The context key to compare

    Returns:
        An ``IfBuilder`` instance to continue defining additional combinator properties
    """


def if_not_key(key: str) -> IfBuilder:
    """
    Select this context key for a comparison with the result inverted

    Args:
        key: The context key to compare

    Returns:
        An ``IfBuilder`` instance to continue defining additional combinator properties
    """


def if_len(target: Retriever | int | Get) -> IfBuilder:
    """
    Select this value and use its length for a comparison

    Args:
        *target: The retriever path to fetch the value from. This can be a sequence of retrievers/list indices
            starting in the current struct

    Returns:
        An ``IfBuilder`` instance to continue defining additional combinator properties
    """


def if_ver(*, min: Version = Version(-1), max: Version = Version(10_000)) -> IfBuilder:
    """
    Checks if the current struct version is between the specified min/max. Either min/max may be omitted to leave that
    check unbounded

    Args:
        min: The minimum required version
        max: The maximum required version

    Returns:
        An ``IfBuilder`` instance to continue defining additional combinator properties
    """

def if_else(*coms: Combinator) -> Combinator:
    """
    Creates an if-else-if like chain using the provided ``if_`` combinators, such that at most only one of them will run
    their innermost combinator. if multiple ``if_``s are nested in one combinator, it is treated as a single if block,
    ``if condition1 and condition2`` unless a ``break_()`` combinator is passed.
    If a combinator other than ``if_`` is passed, it is treated like a default case (order matters!)

    Args:
        *coms: The nested combinators to chain.

    Returns:
        A combinator that encodes the conditional logic defined by the ``IfBuilder`` chain
    """

def break_() -> Combinator:
    """
    Break from an if_else combinator chain unconditionally.

    Returns:
        A combinator that encodes the conditional logic defined by the ``IfBuilder`` chain
    """
