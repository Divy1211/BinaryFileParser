from bfp_rs.types.base_struct import BaseStruct


class Manager:
    """
    Base class for defining a more coherent and organised API to access properties of a struct with a complicated
    internal structure using ``RetrieverRef``s
    """
    _struct: BaseStruct
    "The struct whose properties are referenced by this manager instance"

    def __new__(cls, struct: BaseStruct):
        """
        Construct an instance of this manager with its references bound to the properties of the given struct

        Args:
            struct: The struct to reference properties from
        """

