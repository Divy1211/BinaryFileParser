from bfp_rs.errors.parsing_error import ParsingError

class CompressionError(ParsingError):
    """
    Thrown when a retriever sets ``remaining_compressed`` to ``True`` but the struct does not implement ``_compress`` or
    ``_decompress``
    """

