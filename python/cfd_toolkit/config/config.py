from pathlib import Path

import oyaml

from cfd_toolkit.config.base import BaseModel
from cfd_toolkit.config.regions import Region
from cfd_toolkit.config.mesh import Mesh


class Config(BaseModel):
    """Simulation configuration definition"""

    working_directory: Path
    geometry: list[Path]
    regions: list[Region]
    mesh: Mesh

    @staticmethod
    def from_yaml(path):
        """Import a configuration from a YAML file"""
        working_directory = Path(path).absolute().parent

        with open(path, "r") as f:
            kwargs = oyaml.load(f, Loader=oyaml.Loader)
            return Config(working_directory=working_directory, **kwargs)
