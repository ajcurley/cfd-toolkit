from cfd_toolkit.config import Config
from cfd_toolkit.openfoam.backends import OpenFOAMMeshBackend


filename = "examples/lmp1/config.yml"
config = Config.from_yaml(filename)
backend = OpenFOAMMeshBackend(config)

backend.setup()
