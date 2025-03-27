from bfp_rs.errors.parsing_error import ParsingError

class CompressionError(ParsingError):
    """
    Thrown when a retriever defines remaining_compressed but the struct does not implement _compress/_decompress
    """
    ...
