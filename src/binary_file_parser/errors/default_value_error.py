from binary_file_parser.errors.parsing_error import ParsingError


class DefaultAttributeError(AttributeError, ParsingError):
    pass
