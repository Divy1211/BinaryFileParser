from __future__ import annotations

from io import StringIO
from typing import Type

from binary_file_parser import BaseStruct, Retriever, Version
from binary_file_parser.utils import indentify
from testing.sections.TriggerData import Effect as EffectStruct

class Effect(EffectStruct):
    def __init__(
        self,
        struct_version: Version = Version((3, 5, 1, 47)),
        parent: BaseStruct = None,
        local_vars = None,
        **retriever_inits,
    ):
        if len(retriever_inits) > 1:
            super().__init__(struct_version, parent, **retriever_inits)
            return

        for ref in self._refs:
            name = (
                ref.retriever.p_name
                if isinstance(ref.retriever, Retriever)
                else ref.retriever.get_p_name(struct_version)
            )
            retriever_inits[name] = local_vars[ref.name]
        super().__init__(struct_version, parent, **retriever_inits)

    @staticmethod
    def _make_effect(struct: EffectStruct) -> Effect:
        from testing.sections.effects.cd import ChangeDiplomacy
        from testing.sections.effects.rt import ResearchTechnology
        effect_cls: Type[Effect] = {
            1: ChangeDiplomacy,
            2: ResearchTechnology,
        }[struct.type]

        return effect_cls(
            **{ref.name: None for ref in effect_cls._refs},
            struct_version = struct.struct_ver,
            parent = struct.parent,
            **struct.retriever_name_value_map,
        )

    def __repr__(self):
        repr_builder = StringIO()
        repr_builder.write(f"{self.__class__.__name__}(")
        for retriever in self._refs:
            obj = getattr(self, retriever.name)
            if isinstance(obj, list):
                sub_obj_repr_str = (
                    "[\n    "
                    + ",\n    ".join(map(lambda x: indentify(repr(x)), obj))
                    + ",\n]"
                )
            else:
                sub_obj_repr_str = f"{obj!r}"

            repr_builder.write(f"\n    {retriever.name} = {indentify(sub_obj_repr_str)},")
        repr_builder.write("\n)")
        return repr_builder.getvalue()

    def __init_subclass__(cls, **kwargs):
        cls._refs, Effect._refs = cls._refs.copy(), []
