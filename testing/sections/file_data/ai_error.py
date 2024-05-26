from __future__ import annotations

from _operator import ne
from functools import partial
from itertools import takewhile

from binary_file_parser import BaseStruct, Retriever, Version
from binary_file_parser.types import FixedLenStr, int32, uint32
from testing.sections.scx_versions import DE_LATEST


class AiError(BaseStruct):
    @staticmethod
    def unpad_names(_, instance: AiError):
        instance.file_name = str(takewhile(partial(ne, "\x00"), instance.file_name))

    @staticmethod
    def pad_names(retriever: Retriever, instance: AiError):
        instance.file_name = f"{instance.file_name:\x00<{retriever.dtype.length}}"

    # @formatter:off
    file_name: str =   Retriever(FixedLenStr[260], default = "", on_read = [unpad_names], on_write = [pad_names])
    line_number: int = Retriever(int32,            default = -1)
    message: str =     Retriever(FixedLenStr[128], default = -1, on_read = [unpad_names], on_write = [pad_names])
    code: str =        Retriever(uint32,           default = 0)
    """
    - 0: ConstantAlreadyDefined
    - 1: FileOpenFailed
    - 2: FileReadFailed
    - 3: InvalidIdentifier
    - 4: InvalidKeyword
    - 5: InvalidPreprocessorDirective
    - 6: ListFull
    - 7: MissingArrow
    - 8: MissingClosingParenthesis
    - 9: MissingClosingQuote
    - 10: MissingEndIf
    - 11: MissingFileName
    - 12: MissingIdentifier
    - 13: MissingKeyword
    - 14: MissingLHS
    - 15: MissingOpeningParenthesis
    - 16: MissingPreprocessorSymbol
    - 17: MissingRHS
    - 18: NoRules
    - 19: PreprocessorNestingTooDeep
    - 20: RuleTooLong
    - 21: StringTableFull
    - 22: UndocumentedError
    - 23: UnexpectedElse
    - 24: UnexpectedEndIf
    - 25: UnexpectedError
    - 26: UnexpectedEOF
    """
    # @formatter:on

    def __init__(self, struct_ver: Version = DE_LATEST, initialise_defaults = True, **retriever_inits):
        super().__init__(struct_ver, initialise_defaults = initialise_defaults, **retriever_inits)
