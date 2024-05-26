from __future__ import annotations

from binary_file_parser import BaseStruct, Retriever, Version
from binary_file_parser.types import int32, nt_str32


class Effect(BaseStruct):
    @staticmethod
    def set_sel_obj_ids_repeat(_, instance: Effect):
        repeat = 0 if instance.num_objects_selected == -1 else instance.num_objects_selected
        Effect.selected_object_ids.set_repeat(instance, repeat)

    @staticmethod
    def sync_num_obj_sel(_, instance: Effect):
        instance.num_objects_selected = len(instance.selected_object_ids)

    # @formatter:off
    type: int =                       Retriever(int32, default = -1)
    num_properties: int =             Retriever(int32, default = 52)
    ai_script_goal: int =             Retriever(int32, default = -1)
    quantity: int =                   Retriever(int32, default = -1)
    resource: int =                   Retriever(int32, default = -1)
    diplomacy: int =                  Retriever(int32, default = -1)
    num_objects_selected: int =       Retriever(int32, default = -1, on_set = [set_sel_obj_ids_repeat], on_write = [sync_num_obj_sel])
    legacy_location_object_ref: int = Retriever(int32, default = -1)
    unit_type1: int =                 Retriever(int32, default = -1)
    source_player: int =              Retriever(int32, default = -1)
    target_player: int =              Retriever(int32, default = -1)
    technology: int =                 Retriever(int32, default = -1)
    string_id: int =                  Retriever(int32, default = -1)
    sound_id: int =                   Retriever(int32, default = -1)
    instruction_display_time: int =   Retriever(int32, default = -1)
    trigger_idx: int =                Retriever(int32, default = -1)
    location_x: int =                 Retriever(int32, default = -1)
    location_y: int =                 Retriever(int32, default = -1)
    area_x1: int =                    Retriever(int32, default = -1)
    area_y1: int =                    Retriever(int32, default = -1)
    area_x2: int =                    Retriever(int32, default = -1)
    area_y2: int =                    Retriever(int32, default = -1)
    object_group: int =               Retriever(int32, default = -1)
    object_type: int =                Retriever(int32, default = -1)
    instruction_panel_position: int = Retriever(int32, default = -1)
    attack_stance: int =              Retriever(int32, default = -1)
    time_unit: int =                  Retriever(int32, default = -1)
    enabled: int =                    Retriever(int32, default = -1)
    food: int =                       Retriever(int32, default = -1)
    wood: int =                       Retriever(int32, default = -1)
    stone: int =                      Retriever(int32, default = -1)
    gold: int =                       Retriever(int32, default = -1)
    item_id: int =                    Retriever(int32, default = -1)
    flash_object: int =               Retriever(int32, default = -1)
    force_research_technology: int =  Retriever(int32, default = -1)
    visibility_state: int =           Retriever(int32, default = -1)
    scroll: int =                     Retriever(int32, default = -1)
    operation: int =                  Retriever(int32, default = -1)
    unit_type2: int =                 Retriever(int32, default = -1)
    button_location: int =            Retriever(int32, default = -1)
    ai_signal_value: int =            Retriever(int32, default = -1)
    # todo: what's this
    unknown3: int =                   Retriever(int32, default = -1)
    object_attributes: int =          Retriever(int32, default = -1)
    variable: int =                   Retriever(int32, default = -1)
    timer: int =                      Retriever(int32, default = -1)
    facet: int =                      Retriever(int32, default = -1)
    location_object_reference: int =  Retriever(int32, default = -1)
    play_sound: int =                 Retriever(int32, default = -1)
    colour: int =                     Retriever(int32, default = -1, min_ver = Version((2, 4, 1, 40)))
    # todo: what's this
    unknown4: int =                   Retriever(int32, default = -1, min_ver = Version((2, 4, 1, 40)))
    colour_mood: int =                Retriever(int32, default = -1, min_ver = Version((2, 4, 1, 42)))
    reset_timer: int =                Retriever(int32, default = -1, min_ver = Version((2, 4, 1, 44)))
    object_state: int =               Retriever(int32, default = -1, min_ver = Version((2, 4, 1, 44)))
    action_type: int =                Retriever(int32, default = -1, min_ver = Version((2, 4, 1, 44)))
    # todo: try ignoring the null term, see what happens in game
    message: str =                    Retriever(nt_str32, default = "")
    sound_name: str =                 Retriever(nt_str32, default = "")
    selected_object_ids: list[int] =  Retriever(int32,    default = -1, repeat = 0)
    # @formatter:on

    # def map(self) -> BaseStruct:
    #     from testing.sections.effects import Effect as EffectCls
    #     return EffectCls._make_effect(self)

    def __init__(self, struct_ver: Version = Version((3, 5, 1, 47)), initialise_defaults = True, **retriever_inits):
        super().__init__(struct_ver, initialise_defaults = initialise_defaults, **retriever_inits)
