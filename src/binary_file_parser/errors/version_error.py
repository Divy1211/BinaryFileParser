from binary_file_parser.errors.parsing_error import ParsingError


class VersionError(AttributeError, ParsingError):
    pass
