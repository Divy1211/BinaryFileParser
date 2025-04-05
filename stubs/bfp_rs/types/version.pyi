class Version:
    """
    Defines lexical comparisons similar to a tuple. e.g.: ``Version(1, 2)`` is ``v1.2`` and considered higher than
    ``Version(1, 1)`` which is ``v1.1``

    Any number of version components may be defined
    """

    def __new__(cls, *nums: int) -> Version:
        """
        Construct a version instance from the given components, starting with the most significant (major) component

        Args:
            *nums: The component numbers of the version
        """


    @classmethod
    def from_str(cls: Version, ver_str: str) -> Version:
        """
        Construct a version instance from the given version string

        Args:
            ver_str: A string of the format "x.y.z" (there may be more or lesser than three components) where x, y, and
                z are numbers

        Returns:
            A version instance
        """


    def __le__(self, other: Version) -> bool: ...
    def __ge__(self, other: Version) -> bool: ...
    def __eq__(self, other: Version) -> bool: ...
    def __ne__(self, other: Version) -> bool: ...
    def __lt__(self, other: Version) -> bool: ...
    def __gt__(self, other: Version) -> bool: ...
