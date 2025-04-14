from __future__ import annotations

from bfp_rs import BaseStruct, Retriever, Version
from bfp_rs.types.le import Array32, Bytes, Array, i32

from asp_test.sections.scx_versions import TRIGGER_LATEST
from asp_test.sections.trigger_data.variable import Variable


class VariableData(BaseStruct):
    # @formatter:off
    variable_initial_values: list[int] = Retriever(Array[256][i32],                            default_factory = lambda _: [0]*256)
    enabled_techs: list[int]           = Retriever(Array32[i32],      min_ver = Version(2, 1), default_factory = lambda _: [])
    """probably unused?"""
    variables: list[Variable]          = Retriever(Array32[Variable], min_ver = Version(2, 2), default_factory = Variable)
    # @formatter:on

    def __new__(cls, ver: Version = TRIGGER_LATEST, init_defaults = True, **retriever_inits):
        return super().__new__(cls, ver, init_defaults, **retriever_inits)
