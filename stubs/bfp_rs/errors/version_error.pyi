from bfp_rs.errors.parsing_error import ParsingError

class VersionError(ParsingError):
    """
    Thrown when an unsupported property is accessed in a struct according to it's version
    """

