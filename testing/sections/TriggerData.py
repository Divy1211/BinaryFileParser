from __future__ import annotations

from binary_file_parser import BaseStruct, Retriever, Version
from binary_file_parser.types import (
    Array32, bool32, bool8, Bytes, ByteStream, float64, int32, int8, nt_str32, str32, uint32,
)

attr_usage_ids = {
    "message": {3, 20, 26, 37, 44, 45, 48, 51, 55, 56, 59, 60, 65, 66},
    "sound_name": {3, 4, 20},
}


class Effect(BaseStruct):
    @staticmethod
    def set_sel_obj_ids_repeat(retriever: Retriever, instance: Effect):
        repeat = 0 if instance.num_objects_selected == -1 else instance.num_objects_selected
        Effect.selected_object_ids.set_repeat(instance, repeat)

    @staticmethod
    def remove_null_term(retriever: Retriever, instance: Effect):
        setattr(instance, retriever.s_name, getattr(instance, retriever.s_name).removesuffix("\x00"))

    @staticmethod
    def append_null_term_if_used(retriever: Retriever, instance: Effect):
        if instance.type in attr_usage_ids[retriever.p_name]:
            val = getattr(instance, retriever.s_name)
            if len(val) > 0 and val[-1] != "\x00":
                setattr(instance, retriever.s_name, val+"\x00")

    @staticmethod
    def update_num_obj_sel(retreiver: Retriever, instance: Effect):
        instance.num_objects_selected = len(instance.selected_object_ids)

    type: int = Retriever(int32, default = -1)
    static_value_2_2_1_36: int = Retriever(int32, default = 46, max_ver = Version((2, 2, 1, 37)))
    static_value_2_4_1_40: int = Retriever(int32, default = 48, min_ver = Version((2, 4, 1, 40)), max_ver = Version((2, 4, 1, 41)))
    static_value_2_4_1_42: int = Retriever(int32, default = 49, min_ver = Version((2, 4, 1, 42)), max_ver = Version((2, 4, 1, 43)))
    static_value_2_4_1_44: int = Retriever(int32, default = 52, min_ver = Version((2, 4, 1, 44)))
    ai_script_goal: int = Retriever(int32, default = -1)
    quantity: int = Retriever(int32, default = -1)
    tribute_list: int = Retriever(int32, default = -1)
    diplomacy: int = Retriever(int32, default = -1)
    num_objects_selected: int = Retriever(int32, default = -1, on_set = [set_sel_obj_ids_repeat], on_write = [update_num_obj_sel])
    legacy_location_object_reference: int = Retriever(int32, default = -1)
    object_list_unit_id: int = Retriever(int32, default = -1)
    source_player: int = Retriever(int32, default = -1)
    target_player: int = Retriever(int32, default = -1)
    technology: int = Retriever(int32, default = -1)
    string_id: int = Retriever(int32, default = -1)
    unknown2: int = Retriever(int32, default = -1)
    display_time: int = Retriever(int32, default = -1)
    trigger_id: int = Retriever(int32, default = -1)
    location_x: int = Retriever(int32, default = -1)
    location_y: int = Retriever(int32, default = -1)
    area_x1: int = Retriever(int32, default = -1)
    area_y1: int = Retriever(int32, default = -1)
    area_x2: int = Retriever(int32, default = -1)
    area_y2: int = Retriever(int32, default = -1)
    object_group: int = Retriever(int32, default = -1)
    object_type: int = Retriever(int32, default = -1)
    instruction_panel_position: int = Retriever(int32, default = -1)
    attack_stance: int = Retriever(int32, default = -1)
    time_unit: int = Retriever(int32, default = -1)
    enabled: int = Retriever(int32, default = -1)
    food: int = Retriever(int32, default = -1)
    wood: int = Retriever(int32, default = -1)
    stone: int = Retriever(int32, default = -1)
    gold: int = Retriever(int32, default = -1)
    item_id: int = Retriever(int32, default = -1)
    flash_object: int = Retriever(int32, default = -1)
    force_research_technology: int = Retriever(int32, default = -1)
    visibility_state: int = Retriever(int32, default = -1)
    scroll: int = Retriever(int32, default = -1)
    operation: int = Retriever(int32, default = -1)
    object_list_unit_id2: int = Retriever(int32, default = -1)
    button_location: int = Retriever(int32, default = -1)
    ai_signal_value: int = Retriever(int32, default = -1)
    unknown3: int = Retriever(int32, default = -1)
    object_attributes: int = Retriever(int32, default = -1)
    variable: int = Retriever(int32, default = -1)
    timer: int = Retriever(int32, default = -1)
    facet: int = Retriever(int32, default = -1)
    location_object_reference: int = Retriever(int32, default = -1)
    play_sound: int = Retriever(int32, default = -1)
    player_colour: int = Retriever(int32, default = -1, min_ver = Version((2, 4, 1, 40)))
    unknown4: int = Retriever(int32, default = -1, min_ver = Version((2, 4, 1, 40)))
    colour_mood: int = Retriever(int32, default = -1, min_ver = Version((2, 4, 1, 42)))
    reset_timer: int = Retriever(int32, default = -1, min_ver = Version((2, 4, 1, 44)))
    object_state: int = Retriever(int32, default = -1, min_ver = Version((2, 4, 1, 44)))
    action_type: int = Retriever(int32, default = -1, min_ver = Version((2, 4, 1, 44)))
    message: str = Retriever(str32, default = "", on_read = [remove_null_term], on_write = [append_null_term_if_used])
    sound_name: str = Retriever(str32, default = "", on_read = [remove_null_term], on_write = [append_null_term_if_used])
    selected_object_ids: list[int] = Retriever(int32, default = -1, repeat = 0)

    def __init__(self, struct_ver: Version = Version((3, 5, 1, 47)), parent: BaseStruct = None, initialise_defaults = True, **retriever_inits):
        super().__init__(struct_ver, parent, initialise_defaults = initialise_defaults, **retriever_inits)

class Condition(BaseStruct):
    condition_type: int = Retriever(int32, default = 0)
    static_value_2_4_1_36: int = Retriever(int32, default = 21, max_ver = Version((2, 2, 1, 37)))
    static_value_2_4_1_40: int = Retriever(int32, default = 24, min_ver = Version((2, 4, 1, 40)), max_ver = Version((2, 4, 1, 41)))
    static_value_2_4_1_42: int = Retriever(int32, default = 25, min_ver = Version((2, 4, 1, 42)), max_ver = Version((2, 5, 1, 45)))
    static_value_3_0_1_46: int = Retriever(int32, default = 28, min_ver = Version((3, 0, 1, 46)))
    quantity: int = Retriever(int32, default = -1)
    attribute: int = Retriever(int32, default = -1)
    unit_object: int = Retriever(int32, default = -1)
    next_object: int = Retriever(int32, default = -1)
    object_list: int = Retriever(int32, default = -1)
    source_player: int = Retriever(int32, default = -1)
    technology: int = Retriever(int32, default = -1)
    timer: int = Retriever(int32, default = -1)
    unknown1: int = Retriever(int32, default = -1)
    area_x1: int = Retriever(int32, default = -1)
    area_y1: int = Retriever(int32, default = -1)
    area_x2: int = Retriever(int32, default = -1)
    area_y2: int = Retriever(int32, default = -1)
    object_group: int = Retriever(int32, default = -1)
    object_type: int = Retriever(int32, default = -1)
    ai_signal: int = Retriever(int32, default = -1)
    inverted: int = Retriever(int32, default = -1)
    unknown2: int = Retriever(int32, default = -1)
    variable: int = Retriever(int32, default = -1)
    comparison: int = Retriever(int32, default = -1)
    target_player: int = Retriever(int32, default = -1)
    unit_ai_action: int = Retriever(int32, default = -1, min_ver = Version((2, 4, 1, 40)))
    unknown4: int = Retriever(int32, default = -1, min_ver = Version((2, 4, 1, 40)))
    object_state: int = Retriever(int32, default = -1, min_ver = Version((2, 4, 1, 42)))
    timer_id: int = Retriever(int32, default = -1, min_ver = Version((3, 0, 1, 46)))
    victory_timer_type: int = Retriever(int32, default = -1, min_ver = Version((3, 0, 1, 46)))
    include_changeable_weapon_objects: int = Retriever(int32, default = -1, min_ver = Version((3, 0, 1, 46)))
    xs_function: str = Retriever(str32, default = "", min_ver = Version((2, 4, 1, 40)))

    def __init__(self, struct_ver: Version = Version((3, 5, 1, 47)), parent: BaseStruct = None, initialise_defaults = True, **retriever_inits):
        super().__init__(struct_ver, parent, initialise_defaults = initialise_defaults, **retriever_inits)


class Trigger(BaseStruct):
    @staticmethod
    def set_effects_repeat(retriever: Retriever, instance: Trigger):
        Trigger.effects.set_repeat(instance, instance.num_effects)

    @staticmethod
    def set_effect_display_orders_repeat(retriever: Retriever, instance: Trigger):
        Trigger.effect_display_orders.set_repeat(instance, instance.num_effects)

    @staticmethod
    def set_conditions_repeat(retriever: Retriever, instance: Trigger):
        Trigger.conditions.set_repeat(instance, instance.num_conditions)

    @staticmethod
    def set_condition_display_orders_repeat(retriever: Retriever, instance: Trigger):
        Trigger.condition_display_orders.set_repeat(instance, instance.num_conditions)

    @staticmethod
    def update_num_effects(retriever: Retriever, instance: Trigger):
        instance.num_effects = len(instance.effects)

        if len(instance.effect_display_orders) != instance.num_effects:
            highest_display_order = max(instance.effect_display_orders)
            if highest_display_order != instance.num_effects-1:
                raise ValueError("effect display order array out of sync")
            instance.effect_display_orders.extend(range(highest_display_order+1, instance.num_effects))

    @staticmethod
    def update_num_conditions(retriever: Retriever, instance: Trigger):
        instance.num_conditions = len(instance.conditions)

        if len(instance.condition_display_orders) != instance.num_conditions:
            highest_display_order = max(instance.condition_display_orders)
            if highest_display_order != instance.num_conditions-1:
                raise ValueError("condition display order array out of sync")
            instance.condition_display_orders.extend(range(highest_display_order+1, instance.num_conditions))

    enabled: bool = Retriever(bool32, default = True)
    looping: bool = Retriever(bool8, default = False)
    description_str_id_id_2_4_1_40: int = Retriever(int32, default = -1, max_ver = Version((2, 4, 1, 40)))
    description_str_id_id_2_4_1_41: int = Retriever(int32, default = 0, min_ver = Version((2, 4, 1, 41)))
    display_as_objective: bool = Retriever(bool8, default = False)
    objective_description_order: int = Retriever(uint32, default = 0)
    make_header: bool = Retriever(bool8, default = False)
    short_description_str_id_2_4_1_40: int = Retriever(int32, default = -1, max_ver = Version((2, 4, 1, 40)))
    short_description_str_id_2_4_1_41: int = Retriever(int32, default = 0, min_ver = Version((2, 4, 1, 41)))
    display_on_screen: bool = Retriever(bool8, default = False)
    unknown: bytes = Retriever(Bytes[5], default = b"\x00"*5)
    mute_objectives: bool = Retriever(bool8, default = False)
    description: str = Retriever(nt_str32, default = "")
    name: str = Retriever(nt_str32, default = "Trigger 0")
    short_description: str = Retriever(nt_str32, default = "")
    num_effects: int = Retriever(uint32, default = 0, on_set = [set_effects_repeat, set_effect_display_orders_repeat], on_write = [update_num_effects])
    """originally int32"""
    effects: list[Effect] = Retriever(Effect, default = Effect(), repeat = 0)
    effect_display_orders: list[int] = Retriever(uint32, default = 0, repeat = 0)
    """originally int32"""
    num_conditions: int = Retriever(uint32, default = 0, on_set = [set_conditions_repeat, set_condition_display_orders_repeat], on_write = [update_num_conditions])
    """originally int32"""
    conditions: list[Condition] = Retriever(Condition, default = Condition(), repeat = 0)
    condition_display_orders: list[int] = Retriever(uint32, default = 0, repeat = 0)
    """originally int32"""

    def __init__(self, struct_ver: Version = Version((3, 5, 1, 47)), parent: BaseStruct = None, initialise_defaults = True, **retriever_inits):
        super().__init__(struct_ver, parent, initialise_defaults = initialise_defaults, **retriever_inits)


class Variable(BaseStruct):
    id: int = Retriever(uint32, default = 0)
    name: str = Retriever(nt_str32, default = "_Variable0")

    def __init__(self, struct_ver: Version = Version((1, 47)), parent: BaseStruct = None, initialise_defaults = True, **retriever_inits):
        super().__init__(struct_ver, parent, initialise_defaults = initialise_defaults, **retriever_inits)


class VariableData(BaseStruct):
    variables: list[Variable] = Retriever(Array32[Variable], default = [])
    unused: bytes = Retriever(Bytes[9], default = b"\x00"*9, min_ver = Version((3, 0, 1, 46)))
    unknown: bytes = Retriever(Bytes[8], default = b"\x00"*8, min_ver = Version((3, 5, 1, 47)))

    def __init__(self, struct_ver: Version = Version((3, 5, 1, 47)), parent: BaseStruct = None, initialise_defaults = True, **retriever_inits):
        super().__init__(struct_ver, parent, initialise_defaults = initialise_defaults, **retriever_inits)



class TriggerData(BaseStruct):
    @staticmethod
    def set_triggers_repeat(retriever: Retriever, instance: TriggerData):
        TriggerData.triggers.set_repeat(instance, instance.num_triggers)

    @staticmethod
    def set_display_orders_repeat(retriever: Retriever, instance: TriggerData):
        TriggerData.trigger_display_orders.set_repeat(instance, instance.num_triggers)

    @staticmethod
    def update_num_triggers(retriever: Retriever, instance: TriggerData):
        instance.num_triggers = len(instance.triggers)

        if len(instance.trigger_display_orders) != instance.num_triggers:
            highest_display_order = max(instance.trigger_display_orders)
            if highest_display_order != instance.num_triggers-1:
                raise ValueError("trigger display order array out of sync")
            instance.trigger_display_orders.extend(range(highest_display_order+1, instance.num_triggers))

    trigger_version: float = Retriever(float64, default = 3.5)
    trigger_instruction_start: int = Retriever(int8, default = 0)
    num_triggers: int = Retriever(uint32, default = 0, on_set = [set_triggers_repeat, set_display_orders_repeat], on_write = [update_num_triggers])
    """originally int32"""
    triggers: list[Trigger] = Retriever(Trigger, default = Trigger(), repeat = 0)
    trigger_display_orders: list[int] = Retriever(uint32, default = 0, repeat = 0)
    unknown: bytes = Retriever(Bytes[1028], default = b"\x00"*1028)
    variable_data: VariableData = Retriever(VariableData, default = VariableData())

    @classmethod
    def get_version(
        cls,
        stream: ByteStream,
        struct_ver: Version = Version((0,)),
        parent: BaseStruct = None,
    ) -> Version:
        ver_str = str(float64.from_bytes(stream.peek(8)))
        return Version(map(int, ver_str.split("."))) + struct_ver

    def __init__(self, struct_ver: Version = Version((3, 5, 1, 47)), parent: BaseStruct = None, initialise_defaults = True, **retriever_inits):
        super().__init__(struct_ver, parent, initialise_defaults = initialise_defaults, **retriever_inits)
