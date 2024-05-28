from binary_file_parser import BaseStruct, ByteStream, Retriever, Version
from binary_file_parser.types import float32, str16

from tests.sections.bitmap import BackgroundImage
from tests.sections.cinematics import Cinematics
from tests.sections.data_header import DataHeader
from tests.sections.diplomacy import Diplomacy
from tests.sections.global_victory import GlobalVictory
from tests.sections.messages import Messages
from tests.sections.options import Options
from tests.sections.player_options import PlayerOptions
from tests.sections.scx_versions import DE_LATEST


class TribeScenario(BaseStruct):
    # @formatter:off
    data_header: DataHeader =           Retriever(DataHeader,                                  default_factory = DataHeader)
    messages: Messages =                Retriever(Messages,                                    default_factory = Messages)
    cinematics: Cinematics =            Retriever(Cinematics,                                  default_factory = Cinematics)
    background_image_filename: str =    Retriever(str16,           min_ver = Version((1,  9)), default = "")
    background_image: BackgroundImage = Retriever(BackgroundImage, min_ver = Version((1, 10)), default_factory = BackgroundImage)
    player_options: PlayerOptions =     Retriever(PlayerOptions,                               default_factory = PlayerOptions)
    global_victory: GlobalVictory =     Retriever(GlobalVictory,                               default_factory = GlobalVictory)
    diplomacy: Diplomacy =              Retriever(Diplomacy,                                   default_factory = Diplomacy)
    options: Options =                  Retriever(Options,                                     default_factory = Options)
    # @formatter:on

    @classmethod
    def _get_version(cls, stream: ByteStream, struct_ver: Version = Version((0,))) -> Version:
        # this should be identical to file version, but just in case its possible for it to be different... yES
        ver_str = f"{float32._from_bytes(stream.peek(8)[4:]):.2f}"
        return Version(map(int, ver_str.split(".")))


    def __init__(self, struct_ver: Version = DE_LATEST, initialise_defaults = True, **retriever_inits):
        # todo: correctly initialise struct_ver `from_default` for all self versioned structs
        #  for default values that are different across different versions, use default_factory
        super().__init__(struct_ver, initialise_defaults = initialise_defaults, **retriever_inits)
