from __future__ import annotations

import operator
from functools import partial

from src.retrievers.Retriever import Retriever
from src.types.BaseStruct import BaseStruct
from src.types.Bytes import Bytes
from src.types.Int import UInt32, Int16, Int32
from src.types.Str import Str16


def ge(a: int, b: int) -> tuple[bool, str]:
    if operator.ge(a, b):
        return True, ""
    return False, f"%s must be >= {a}"


def le(a: int, b: int) -> tuple[bool, str]:
    if operator.ge(a, b):
        return True, ""
    return False, f"%s must be <= {a}"


class BitMapInfoHeader(BaseStruct):
    @staticmethod
    def set_colours_repeat(retriever: Retriever, instance: BitMapInfoHeader):
        BackgroundImage.info.set_repeat(instance, instance.num_colours) # type: ignore

    size: int = Retriever(Int32, default = 0)
    width: int = Retriever(UInt32, default = 0)
    height: int = Retriever(UInt32, default = 0)
    planes: int = Retriever(Int16, default = 0)
    num_bits: int = Retriever(Int16, default = 0)
    compression: int = Retriever(UInt32, default = 0, validators = [partial(ge, 0), partial(le, 13)])
    image_size: int = Retriever(UInt32, default = 0)
    x_pixels_per_meter: int = Retriever(UInt32, default = 0)
    y_pixels_per_meter: int = Retriever(UInt32, default = 0)
    num_colours: int = Retriever(UInt32, default = 0, on_set = [set_colours_repeat]) # type: ignore
    num_important_colours: int = Retriever(UInt32, default = 0)
    colours: list[int] = Retriever(UInt32, default = 0)

    def __init__(self, version: tuple[int, ...] = (1, 47)):
        super().__init__(version)


class BackgroundImage(BaseStruct):
    @staticmethod
    def set_img_repeat(retriever: Retriever, instance: BackgroundImage):
        BackgroundImage.image.set_repeat(instance, instance.width*instance.height) # type: ignore

    @staticmethod
    def set_info_repeat(retriever: Retriever, instance: BackgroundImage):
        BackgroundImage.info.set_repeat(instance, 1 if instance.width != 0 != instance.height else 0) # type: ignore

    filename: str = Retriever(Str16, default = "")
    version: int = Retriever(UInt32, default = 3)
    width: int = Retriever(UInt32, default = 0, on_set = [set_img_repeat, set_info_repeat]) # type: ignore
    height: int = Retriever(Int32, default = 0, on_set = [set_img_repeat, set_info_repeat]) # type: ignore
    orientation: int = Retriever(Int16, default = 1)
    info: BitMapInfoHeader = Retriever(BitMapInfoHeader, default = BitMapInfoHeader())
    image: list[bytes] = Retriever(Bytes(1), default = 0)

    def __init__(self, version: tuple[int, ...] = (1, 47)):
        super().__init__(version)
