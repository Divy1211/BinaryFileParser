from __future__ import annotations

from src.generators.IncrementalGenerator import IncrementalGenerator
from src.retrievers.Retriever import Retriever
from src.types.BaseStruct import BaseStruct
from src.types.Bool import Bool32, Bool8
from src.types.Bytes import Bytes
from src.types.Float import Float64
from src.types.Int import Int8, UInt32, Int32
from src.types.Str import NullTermStr32, Str32


attr_usage_ids = {
    "message": {3, 20, 26, 37, 44, 45, 48, 51, 55, 56, 59, 60, 65, 66},
    "sound_name": {3, 4, 20},
}


class Effect(BaseStruct):
    @staticmethod
    def set_sel_obj_ids_repeat(retriever: Retriever, instance: Effect):
        Effect.selected_object_ids.set_repeat(instance, instance.num_objects_selected)  # type: ignore

    @staticmethod
    def remove_null_term(retriever: Retriever, instance: Effect):
        setattr(instance, retriever.s_name, getattr(instance, retriever.s_name).removesuffix("\x00"))

    @staticmethod
    def append_null_term_if_used(retriever: Retriever, instance: Effect):
        if instance.type in attr_usage_ids[retriever.p_name]:
            val = getattr(instance, retriever.s_name)
            if val[-1] != "\x00":
                setattr(instance, retriever.s_name, val+"\x00")

    type: int = Retriever(Int32, default = -1)
    static_value: int = Retriever(Int32, default = 52)
    ai_script_goal: int = Retriever(Int32, default = -1)
    quantity: int = Retriever(Int32, default = -1)
    tribute_list: int = Retriever(Int32, default = -1)
    diplomacy: int = Retriever(Int32, default = -1)
    num_objects_selected: int = Retriever(Int32, default = -1, on_set = [set_sel_obj_ids_repeat]) # type: ignore
    legacy_location_object_reference: int = Retriever(Int32, default = -1)
    object_list_unit_id: int = Retriever(Int32, default = -1)
    source_player: int = Retriever(Int32, default = -1)
    target_player: int = Retriever(Int32, default = -1)
    technology: int = Retriever(Int32, default = -1)
    string_id: int = Retriever(Int32, default = -1)
    unknown2: int = Retriever(Int32, default = -1)
    display_time: int = Retriever(Int32, default = -1)
    trigger_id: int = Retriever(Int32, default = -1)
    location_x: int = Retriever(Int32, default = -1)
    location_y: int = Retriever(Int32, default = -1)
    area_x1: int = Retriever(Int32, default = -1)
    area_y1: int = Retriever(Int32, default = -1)
    area_x2: int = Retriever(Int32, default = -1)
    area_y2: int = Retriever(Int32, default = -1)
    object_group: int = Retriever(Int32, default = -1)
    object_type: int = Retriever(Int32, default = -1)
    instruction_panel_position: int = Retriever(Int32, default = -1)
    attack_stance: int = Retriever(Int32, default = -1)
    time_unit: int = Retriever(Int32, default = -1)
    enabled: int = Retriever(Int32, default = -1)
    food: int = Retriever(Int32, default = -1)
    wood: int = Retriever(Int32, default = -1)
    stone: int = Retriever(Int32, default = -1)
    gold: int = Retriever(Int32, default = -1)
    item_id: int = Retriever(Int32, default = -1)
    flash_object: int = Retriever(Int32, default = -1)
    force_research_technology: int = Retriever(Int32, default = -1)
    visibility_state: int = Retriever(Int32, default = -1)
    scroll: int = Retriever(Int32, default = -1)
    operation: int = Retriever(Int32, default = -1)
    object_list_unit_id2: int = Retriever(Int32, default = -1)
    button_location: int = Retriever(Int32, default = -1)
    ai_signal_value: int = Retriever(Int32, default = -1)
    unknown3: int = Retriever(Int32, default = -1)
    object_attributes: int = Retriever(Int32, default = -1)
    variable: int = Retriever(Int32, default = -1)
    timer: int = Retriever(Int32, default = -1)
    facet: int = Retriever(Int32, default = -1)
    location_object_reference: int = Retriever(Int32, default = -1)
    play_sound: int = Retriever(Int32, default = -1)
    player_colour: int = Retriever(Int32, default = -1)
    unknown4: int = Retriever(Int32, default = -1)
    colour_mood: int = Retriever(Int32, default = -1)
    reset_timer: int = Retriever(Int32, default = -1)
    object_state: int = Retriever(Int32, default = -1)
    action_type: int = Retriever(Int32, default = -1)
    message: str = Retriever(Str32, default = "", on_read = [remove_null_term], on_write = [append_null_term_if_used])  # type: ignore
    sound_name: str = Retriever(Str32, default = "", on_read = [remove_null_term], on_write = [append_null_term_if_used])  # type: ignore
    selected_object_ids: list[int] = Retriever(Int32, default = -1)


class Condition(BaseStruct):
    condition_type: int = Retriever(Int32, default = 0)
    static_value: int = Retriever(Int32, default = 27)
    quantity: int = Retriever(Int32, default = -1)
    attribute: int = Retriever(Int32, default = -1)
    unit_object: int = Retriever(Int32, default = -1)
    next_object: int = Retriever(Int32, default = -1)
    object_list: int = Retriever(Int32, default = -1)
    source_player: int = Retriever(Int32, default = -1)
    technology: int = Retriever(Int32, default = -1)
    timer: int = Retriever(Int32, default = -1)
    unknown1: int = Retriever(Int32, default = -1)
    area_x1: int = Retriever(Int32, default = -1)
    area_y1: int = Retriever(Int32, default = -1)
    area_x2: int = Retriever(Int32, default = -1)
    area_y2: int = Retriever(Int32, default = -1)
    object_group: int = Retriever(Int32, default = -1)
    object_type: int = Retriever(Int32, default = -1)
    ai_signal: int = Retriever(Int32, default = -1)
    inverted: int = Retriever(Int32, default = -1)
    unknown2: int = Retriever(Int32, default = -1)
    variable: int = Retriever(Int32, default = -1)
    comparison: int = Retriever(Int32, default = -1)
    target_player: int = Retriever(Int32, default = -1)
    unit_ai_action: int = Retriever(Int32, default = -1)
    unknown4: int = Retriever(Int32, default = -1)
    object_state: int = Retriever(Int32, default = -1)
    timer_id: int = Retriever(Int32, default = -1)
    victory_timer_type: int = Retriever(Int32, default = -1)
    include_changeable_weapon_objects: int = Retriever(Int32, default = -1)
    xs_function: str = Retriever(Str32, default = "")


class Trigger(BaseStruct):
    @staticmethod
    def set_effects_repeat(retriever: Retriever, instance: Trigger):
        Trigger.effects.set_repeat(instance, instance.num_effects)  # type: ignore

    @staticmethod
    def set_effect_display_orders_repeat(retriever: Retriever, instance: Trigger):
        Trigger.effect_display_orders.set_repeat(instance, instance.num_effects)  # type: ignore

    @staticmethod
    def set_conditions_repeat(retriever: Retriever, instance: Trigger):
        Trigger.conditions.set_repeat(instance, instance.num_conditions)  # type: ignore

    @staticmethod
    def set_condition_display_orders_repeat(retriever: Retriever, instance: Trigger):
        Trigger.condition_display_orders.set_repeat(instance, instance.num_conditions)  # type: ignore

    enabled: bool = Retriever(Bool32, default = True)
    looping: bool = Retriever(Bool8, default = False)
    description_str_id: int = Retriever(Int32, default = 0)
    display_as_objective: bool = Retriever(Bool8, default = False)
    objective_description_order: int = Retriever(UInt32, default = 0)
    make_header: bool = Retriever(Bool8, default = False)
    short_description_str_id: int = Retriever(Int32, default = 0)
    display_on_screen: bool = Retriever(Bool8, default = False)
    unknown: bytes = Retriever(Bytes[5], default = b"\x00"*5)
    mute_objectives: bool = Retriever(Bool8, default = False)
    description: str = Retriever(NullTermStr32, default = "")
    name: str = Retriever(NullTermStr32, default = "Trigger 0")
    short_description: str = Retriever(NullTermStr32, default = "")
    num_effects: int = Retriever(UInt32, default = 0, on_set = [set_effects_repeat, set_effect_display_orders_repeat])  # type: ignore
    """originally int32"""
    effects: list[Effect] = Retriever(Effect, default = Effect())
    effect_display_orders: int = Retriever(UInt32, default = 0)
    """originally int32"""
    num_conditions: int = Retriever(UInt32, default = 0, on_set = [set_conditions_repeat, set_condition_display_orders_repeat]) # type: ignore
    """originally int32"""
    conditions: list[Condition] = Retriever(Condition, default = Condition())
    condition_display_orders: int = Retriever(UInt32, default = 0)
    """originally int32"""


class TriggerData(BaseStruct):
    @staticmethod
    def set_triggers_repeat(retriever: Retriever, instance: TriggerData):
        TriggerData.triggers.set_repeat(instance, instance.num_triggers)  # type: ignore

    @staticmethod
    def set_display_orders_repeat(retriever: Retriever, instance: TriggerData):
        TriggerData.trigger_display_orders.set_repeat(instance, instance.num_triggers)  # type: ignore

    trigger_version: float = Retriever(Float64, default = 2.6)
    trigger_instruction_start: int = Retriever(Int8, default = 0)
    num_triggers: int = Retriever(UInt32, default = 0, on_set = [set_triggers_repeat, set_display_orders_repeat]) # type: ignore
    """originally int32"""
    triggers: list[Trigger] = Retriever(Trigger, default = Trigger())
    trigger_display_orders: list[int] = Retriever(UInt32, default = 0)
    unknown: bytes = Retriever(Bytes[1028], default = b"\x00"*1028)

    @classmethod
    def get_version(cls, igen: IncrementalGenerator) -> tuple[int, ...]:
        ver_str = str(Float64.from_bytes(igen.get_bytes(8, update_progress = False)))
        return tuple(map(int, ver_str.split(".")))

    def __init__(self, struct_version: tuple[int, ...] = (3, 2)):
        super().__init__(struct_version)
