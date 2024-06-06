from enum import Enum


class RegionType(str, Enum):
    """Simulation region type"""

    FLUID = "fluid"
    POROUS = "porous"


class ControlType(str, Enum):
    """Surface/volume mesh control type"""

    SURFACE = "surface"
    VOLUME = "volume"
