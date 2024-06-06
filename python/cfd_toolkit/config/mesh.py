import copy
from typing import Optional

from pydantic import Field, model_validator
from pydantic.types import confloat, conint

from cfd_toolkit.config.base import BaseModel
from cfd_toolkit.config.constants import ControlType
from cfd_toolkit.config.mixins import MatchMixin
from cfd_toolkit.config.types import Level


class MeshDefaults(BaseModel):
    """Simulation mesh defaults definition"""

    target_level: Level
    curvature_level: Level
    feature_level: Level
    n_layers: conint(ge=0) = 0
    first_layer_height: Optional[confloat(gt=0)] = None
    layer_growth_rate: Optional[confloat(ge=1)] = None

    @model_validator(mode="after")
    def validate_first_layer_height(self):
        """Validate the first_layer_height parameter"""
        if self.n_layers > 0 and self.first_layer_height is None:
            raise ValueError("first_layer_height must be specified if n_layers > 0")
        return self

    @model_validator(mode="after")
    def validate_layer_growth_rate(self):
        """Validate the layer_growth_rate parameter"""
        if self.n_layers > 0 and self.layer_growth_rate is None:
            raise ValueError("layer_growth_rate must be specified if n_layers > 0")
        return self


class MeshSurface(BaseModel, MatchMixin):
    """Simulation mesh surface control definition"""

    target_level: Optional[Level] = None
    curvature_level: Optional[Level] = None
    feature_level: Optional[Level] = None
    n_layers: Optional[conint(ge=0)] = None
    first_layer_height: Optional[confloat(gt=0)] = None
    layer_growth_rate: Optional[confloat(ge=1)] = None

    @property
    def type(self):
        """Return the control type"""
        return ControlType.SURFACE

    def assign_defaults(self, defaults):
        """Assign the default values for anything explicitly unset"""
        for name in self.model_fields.keys():
            if getattr(self, name) is None:
                value = getattr(defaults, name)
                setattr(self, name, value)

        if self.n_layers > 0 and self.first_layer_height is None:
            raise ValueError("first_layer_height must be specified if n_layers > 0")

        if self.n_layers > 0 and self.layer_growth_rate is None:
            raise ValueError("layer_growth_rate must be specified if n_layers > 0")


class MeshVolume(BaseModel, MatchMixin):
    """Simulation mesh volume control definition"""

    level: Level

    @property
    def type(self):
        """Return the control type"""
        return ControlType.VOLUME


class Mesh(BaseModel):
    """Simulation mesh definition"""

    base_size: confloat(gt=0)
    buffer_cells: conint(ge=0)
    feature_angle: confloat(ge=0)
    curvature_level: Optional[Level] = None
    proximity_cells: Optional[conint(ge=1)] = None
    proximity_level: Optional[Level] = None
    defaults: MeshDefaults
    surfaces: list[MeshSurface] = Field(default_factory=list)
    volumes: list[MeshVolume] = Field(default_factory=list)

    def assign_defaults(self):
        """Assign the default values for explicity unset surface controls"""
        for control in self.surfaces:
            control.assign_defaults(self.defaults)

    def get_surface_control(self, patch):
        """Get the surface control for the patch"""
        control = self.defaults

        for surface in self.surfaces:
            if surface.is_match(patch):
                control = surface

        return copy.deepcopy(control)

    def get_volume_control(self, patch):
        """Get the volume control for the patch"""
        control = None

        for volume in self.volumes:
            if volume.is_match(patch):
                control = volume

        return control

    def is_proximity_enabled(self):
        """Return True if proximity refinement is enabled"""
        return all((self.proximity_cells, self.proximity_level))
