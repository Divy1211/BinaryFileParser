from binary_file_parser.retrievers import RetrieverRef
from testing.sections.effects.Effect import Effect
from testing.sections.effects.tmp import EffectType
from testing.sections.trigger_data.effect import Effect as EffectStruct


class ResearchTechnology(Effect):
    source_player = RetrieverRef(EffectStruct._source_player)
    technology = RetrieverRef(EffectStruct._technology)
    force = RetrieverRef(EffectStruct._force_research_technology)

    def __init__(
        self,
        source_player,
        technology,
        force: bool,
        **kwargs,
    ):
        """
        Research a technology for the source_player

        Args:
            source_player: The player to research the technology for
            technology: The technology to research
            force: If the technology should be researched even if it's not available to the civilization of the
                source_player
        """
        kwargs["type"] = EffectType.RESEARCH_TECHNOLOGY
        super().__init__(local_vars=locals(), **kwargs)