from __future__ import annotations

from src.sections.FileHeader import FileHeader


class AoE2Scenario:
    def __init__(self):
        self._file_header = FileHeader()

    @classmethod
    def from_file(cls, filename: str) -> AoE2Scenario:
        """
        Creates and returns an instance of the AoE2DEScenario class from the given scenario file

        Args:
            filename: The path to the scenario file to create the object from
            game_version: The version of the game to create the object for

        Returns:
            An instance of the AoE2DEScenario class which is the object representation of the given scenario file
        """

