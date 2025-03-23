from __future__ import annotations

from bfp_rs import BaseStruct, Retriever, Version
from bfp_rs.types.le import Bytes, i16, i32, str16, u32
from bfp_rs.combinators import if_, set_repeat, get

from asp_test.sections.settings.bitmap.bitmap_info_header import BitmapInfoHeader
from asp_test.sections.scx_versions import DE_LATEST


class BackgroundImage(BaseStruct):
    # @formatter:off
    background_image_filename: str = Retriever(str16,                                      default = "")
    # todo: does size needs to be set correctly? testing needed
    size: int                      = Retriever(u32,              min_ver = Version(1, 10), default = 0)
    width: int                     = Retriever(u32,              min_ver = Version(1, 10), default = 0,                        on_read = lambda: [if_(BackgroundImage.width).eq(0).then(set_repeat(BackgroundImage.info_header).to(-1))], on_write = lambda: [if_(BackgroundImage.width).eq(0).then(set_repeat(BackgroundImage.info_header).to(-1))])
    height: int                    = Retriever(i32,              min_ver = Version(1, 10), default = 0,                        on_read = lambda: [if_(BackgroundImage.height).eq(0).then(set_repeat(BackgroundImage.info_header).to(-1)), set_repeat(BackgroundImage.pixels).by(get(BackgroundImage.height) * ((get(BackgroundImage.width) + 3) & ~3))], on_write = lambda: [if_(BackgroundImage.height).eq(0).then(set_repeat(BackgroundImage.info_header).to(-1))])
    """https://en.wikipedia.org/wiki/BMP_file_format#Pixel_storage"""
    orientation: int               = Retriever(i16,              min_ver = Version(1, 10), default = 1)
    info_header: BitmapInfoHeader  = Retriever(BitmapInfoHeader, min_ver = Version(1, 10), default_factory = BitmapInfoHeader)
    pixels: list[bytes]            = Retriever(Bytes[1],         min_ver = Version(1, 10), default = b"\x00",                  repeat = -1)
    # @formatter:on

    def __new__(cls, ver: Version = DE_LATEST, init_defaults = True, **retriever_inits):
        return super().__new__(cls, ver, init_defaults, **retriever_inits)
