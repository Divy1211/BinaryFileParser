from binary_file_parser.retrievers import RetrieverRef
from testing.sections.effects.Effect import Effect
from testing.sections.effects.tmp import EffectType
from testing.sections.trigger_data.effect import Effect as EffectStruct


class ChangeDiplomacy(Effect):
    diplomacy_stance = RetrieverRef(EffectStruct._diplomacy)
    source_player = RetrieverRef(EffectStruct._source_player)
    target_player = RetrieverRef(EffectStruct._target_player)

    def __init__(
        self,
        source_player,
        diplomacy_stance,
        target_player,
        **kwargs,
    ):
        """
        Change the source_player's diplomacy_stance towards the target_player
        Args:
            source_player: The player to change the diplomacy stance for
            diplomacy_stance: The new diplomacy stance
            target_player: The player to change the diplomacy stance towards
        """
        kwargs["type"] = EffectType.CHANGE_DIPLOMACY
        super().__init__(local_vars = locals(), **kwargs)
