from __future__ import annotations

import math as maths

from binary_file_parser import BaseStruct, Retriever, Version
from binary_file_parser.types import Bytes, int16, int32, uint32

from testing.sections.bitmap.bitmap_info_header import BitmapInfoHeader
from testing.sections.scx_versions import DE_LATEST


class BackgroundImage(BaseStruct):
    @staticmethod
    def set_img_repeat(_, instance: BackgroundImage):
        """https://en.wikipedia.org/wiki/BMP_file_format#Pixel_storage"""
        if instance.info_header is None:
            return

        info = instance.info_header
        if info.compression == 0:
            row_size = maths.ceil(info.num_bits_per_pixel * info.width) * 4
            BackgroundImage.pixels.set_repeat(instance, row_size * abs(info.height))

        BackgroundImage.pixels.set_repeat(instance, info.image_size)

    @staticmethod
    def set_bmp_header_repeat(_, instance: BackgroundImage):
        if instance.width == 0 or instance.height == 0:
            BackgroundImage.info_header.set_repeat(instance, -1)

    # @formatter:off
    size: int =                     Retriever(uint32,           default = 0)
    width: int =                    Retriever(uint32,           default = 0,                        on_set = [set_bmp_header_repeat])
    height: int =                   Retriever(int32,            default = 0,                        on_set = [set_bmp_header_repeat])
    orientation: int =              Retriever(int16,            default = 1)

    # todo: there is potentially an optional BitmapFileHeader here. Look into this (on top of the InfoHeader!)
    info_header: BitmapInfoHeader = Retriever(BitmapInfoHeader, default_factory = BitmapInfoHeader, on_set = [set_img_repeat])
    pixels: list[bytes] =           Retriever(Bytes[1],         default = b"\x00",                  repeat = -1)
    # @formatter:on

    def __init__(self, struct_ver: Version = DE_LATEST, initialise_defaults = True, **retriever_inits):
        super().__init__(struct_ver, initialise_defaults = initialise_defaults, **retriever_inits)
