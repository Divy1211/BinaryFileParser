from __future__ import annotations

from bfp_rs import BaseStruct, Retriever, Version, ByteStream
from bfp_rs.types.le import (
    Array32, f64, i8, u32,
)
from bfp_rs.combinators import set_repeat

from asp_test.sections.scx_versions import TRIGGER_LATEST
from asp_test.sections.trigger_data.trigger import Trigger
from asp_test.sections.trigger_data.variable_data import VariableData


class TriggerData(BaseStruct):
    # @formatter:off
    version: float                     = Retriever(f64,                                                default = 3.6)
    objectives_state: int              = Retriever(i8,                        min_ver = Version(1, 5), default = 0)
    triggers: list[Trigger]            = Retriever(Array32[Trigger],                                   default_factory = lambda _: [],             on_read = lambda: [set_repeat(TriggerData.trigger_display_orders).from_len(TriggerData.triggers)])
    trigger_display_orders: list[int]  = Retriever(u32,                       min_ver = Version(1, 4), default = 0,                    repeat = 0)
    variable_data: VariableData        = Retriever(VariableData,              min_ver = Version(2, 2), default_factory = VariableData)
    # @formatter:on

    @classmethod
    def _get_version(
        cls,
        stream: ByteStream,
        _ver: Version = Version(0),
    ) -> Version:
        ver_str = str(f64.from_bytes(stream.peek(8)))
        return Version(*map(int, ver_str.split(".")))

    def __new__(cls, ver: Version = TRIGGER_LATEST, init_defaults = True, **retriever_inits):
        return super().__new__(cls, ver, init_defaults, **retriever_inits)
