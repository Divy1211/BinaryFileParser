from __future__ import annotations

from binary_file_parser import BaseStruct, Retriever, Version
from binary_file_parser.types import int32, str32


class Condition(BaseStruct):
    # @formatter:off
    type: int =                              Retriever(int32, default = 0)
    num_properties: int =                    Retriever(int32, default = 27)
    quantity: int =                          Retriever(int32, default = -1)
    resource: int =                          Retriever(int32, default = -1)
    primary_unit_ref: int =                  Retriever(int32, default = -1)
    secondary_unit_ref: int =                Retriever(int32, default = -1)
    unit_type: int =                         Retriever(int32, default = -1)
    source_player: int =                     Retriever(int32, default = -1)
    technology: int =                        Retriever(int32, default = -1)
    timer: int =                             Retriever(int32, default = -1)
    trigger_idx: int =                       Retriever(int32, default = -1)
    area_x1: int =                           Retriever(int32, default = -1)
    area_y1: int =                           Retriever(int32, default = -1)
    area_x2: int =                           Retriever(int32, default = -1)
    area_y2: int =                           Retriever(int32, default = -1)
    object_group: int =                      Retriever(int32, default = -1)
    object_type: int =                       Retriever(int32, default = -1)
    ai_signal: int =                         Retriever(int32, default = -1)
    inverted: int =                          Retriever(int32, default = -1)
    # todo: find out what this is
    unknown2: int =                          Retriever(int32, default = -1)
    variable: int =                          Retriever(int32, default = -1)
    comparison: int =                        Retriever(int32, default = -1)
    target_player: int =                     Retriever(int32, default = -1)
    unit_action: int =                       Retriever(int32, default = -1, min_ver = Version((2, 4, 1, 40)))
    # todo: find out what this is
    unknown4: int =                          Retriever(int32, default = -1, min_ver = Version((2, 4, 1, 40)))
    object_state: int =                      Retriever(int32, default = -1, min_ver = Version((2, 4, 1, 42)))
    timer_id: int =                          Retriever(int32, default = -1, min_ver = Version((3, 0, 1, 46)))
    victory_timer_type: int =                Retriever(int32, default = -1, min_ver = Version((3, 0, 1, 46)))
    include_changeable_weapon_objects: int = Retriever(int32, default = -1, min_ver = Version((3, 0, 1, 46)))
    xs_function: str =                       Retriever(str32, default = "", min_ver = Version((2, 4)))
    # @formatter:on

    def __init__(self, struct_ver: Version = Version((3, 5, 1, 47)), initialise_defaults = True, **retriever_inits):
        super().__init__(struct_ver, initialise_defaults = initialise_defaults, **retriever_inits)
