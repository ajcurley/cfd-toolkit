from typing import Literal

from pydantic import Field
from pydantic.types import conlist

from cfd_toolkit.config.base import BaseModel
from cfd_toolkit.config.constants import RegionType
from cfd_toolkit.config.mixins import MatchMixin


class FluidRegion(BaseModel, MatchMixin):
    """Fluid region definition"""

    name: str
    region_type: Literal[RegionType.FLUID] = Field(alias="type")
    location_in_mesh: conlist(float, min_length=3, max_length=3)


Region = FluidRegion
