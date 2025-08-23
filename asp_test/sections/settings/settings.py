from bfp_rs import BaseStruct, ByteStream, Retriever, Version, Context
from bfp_rs.types.le import f32

from asp_test.sections.scx_versions import DE_LATEST
from asp_test.sections.settings.bitmap import BackgroundImage
from asp_test.sections.settings.cinematics import Cinematics
from asp_test.sections.settings.data_header import DataHeader
from asp_test.sections.settings.diplomacy import Diplomacy
from asp_test.sections.settings.global_victory import GlobalVictory
from asp_test.sections.settings.messages import Messages
from asp_test.sections.settings.options import Options
from asp_test.sections.settings.player_options import PlayerOptions


class Settings(BaseStruct):
    # @formatter:off
    data_header: DataHeader           = Retriever(DataHeader,                               default_factory = DataHeader)
    messages: Messages                = Retriever(Messages,                                 default_factory = Messages)
    cinematics: Cinematics            = Retriever(Cinematics,                               default_factory = Cinematics)
    background_image: BackgroundImage = Retriever(BackgroundImage, min_ver = Version(1, 9), default_factory = BackgroundImage)
    player_options: PlayerOptions     = Retriever(PlayerOptions,                            default_factory = PlayerOptions)
    global_victory: GlobalVictory     = Retriever(GlobalVictory,                            default_factory = GlobalVictory)
    diplomacy: Diplomacy              = Retriever(Diplomacy,                                default_factory = Diplomacy)
    options: Options                  = Retriever(Options,                                  default_factory = Options)
    # @formatter:on

    @classmethod
    def _get_version(cls, stream: ByteStream, _ver: Version = Version(0)) -> Version:
        # this should be identical to file version, but just in case its possible for it to be different... yES
        ver_str = f"{f32.from_bytes(stream.peek(8)[4:]):.2f}"
        return Version(*map(int, ver_str.split(".")))


    def __new__(cls, ver: Version = DE_LATEST, ctx: Context = None, init_defaults = True, **retriever_inits):
        return super().__new__(cls, ver, ctx or Context(), init_defaults, **retriever_inits)
