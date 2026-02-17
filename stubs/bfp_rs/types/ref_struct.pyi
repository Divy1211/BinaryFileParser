from typing import Self

from bfp_rs.types.base_struct import BaseStruct


class RefStruct:
    """
    Base class for defining a more coherent and organised API to access properties of a struct with a complicated
    internal structure using ``RetrieverRef``s
    """
    _struct: BaseStruct
    "The struct whose properties are referenced by this class"

    def __new__(cls, struct: BaseStruct) -> Self:
        """
        Construct an instance of this RefStruct with its references bound to the properties of the given struct instance

        Args:
            struct: The struct to reference properties from
        """

