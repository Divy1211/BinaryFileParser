from __future__ import annotations

from bfp_rs import BaseStruct, Retriever, Version
from bfp_rs.types.le import i32, u16, u32
from bfp_rs.combinators import set_repeat, set_

from asp_test.sections.settings.bitmap.colour import Colour
from asp_test.sections.scx_versions import DE_LATEST


class BitmapInfoHeader(BaseStruct):
    """https://en.wikipedia.org/wiki/BMP_file_format#DIB_header_(bitmap_information_header)"""
    # @formatter:off
    # todo: this should be set from the # of bytes in this header, is it unused?
    size: int                  = Retriever(u32,    default = 40)
    width: int                 = Retriever(i32,    default = 0)
    height: int                = Retriever(i32,    default = 0)
    num_planes: int            = Retriever(u16,    default = 1)
    num_bits_per_pixel: int    = Retriever(u16,    default = 1)
    compression: int           = Retriever(u32,    default = 0)
    image_size: int            = Retriever(u32,    default = 0)
    x_pixels_per_meter: int    = Retriever(i32,    default = 0)
    y_pixels_per_meter: int    = Retriever(i32,    default = 0)
    num_colours: int           = Retriever(u32,    default = 0, on_read = lambda: [set_repeat(BitmapInfoHeader.colours).from_(BitmapInfoHeader.num_colours)], on_write = lambda: [set_(BitmapInfoHeader.num_colours).from_len(BitmapInfoHeader.colours)])
    num_important_colours: int = Retriever(u32,    default = 0)
    colours: list[Colour]      = Retriever(Colour, default_factory = Colour, repeat = 0)
    # @formatter:on

    def __new__(cls, ver: Version = DE_LATEST, init_defaults = True, **retriever_inits):
        return super().__new__(cls, ver, init_defaults, **retriever_inits)
