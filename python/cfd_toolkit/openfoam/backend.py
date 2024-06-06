import shutil

from cfd_toolkit.mesh import SurfaceMesh


class OpenFOAMMeshBackend:
    """OpenFOAM snappyHexMesh backend"""

    def __init__(self, config):
        self.config = config
        self.geometry = None

    def setup(self):
        """Set up the simulation"""
        self.clean()
        self.import_geometry()

    def clean(self):
        """Clean the working directory"""
        for directory in ("0.orig", "constant", "system"):
            path = self.config.working_directory / directory

            if path.exists():
                shutil.rmtree(path)

    def import_geometry(self):
        """Import the geometry from file"""
        self.geometry = None

        for filename in self.config.geometry:
            geometry = SurfaceMesh.from_obj(filename)

            if self.geometry is None:
                self.geometry = geometry
            else:
                self.geometry.merge(geometry)

    def write_geometry(self):
        """Write the model geometry"""
        raise NotImplementedError

    def write_features(self):
        """Write the model features"""
        raise NotImplementedError

    def write_dictionaries(self):
        """Write the OpenFOAM dictionaries"""
        self.write_block_mesh_dict()
        self.write_control_dict()
        self.write_decompose_par_dict()
        self.write_fv_schemes()
        self.write_fv_solution()
        self.write_snappy_hex_mesh_dict()

    def write_block_mesh_dict(self):
        """Write the OpenFOAM blockMeshDict"""
        raise NotImplementedError

    def write_control_dict(self):
        """Write the OpenFOAM controlDict"""
        raise NotImplementedError

    def write_decompose_par_dict(self):
        """Write the OpenFOAM decomposeParDict"""
        raise NotImplementedError

    def write_fv_schemes(self):
        """Write the OpenFOAM fvSchemes"""
        raise NotImplementedError

    def write_fv_solution(self):
        """Write the OpenFOAM fvSolution"""
        raise NotImplementedError

    def write_snappy_hex_mesh_dict(self):
        """Write the OpenFOAM snappyHexMeshDict"""
        raise NotImplementedError
