import math
import shutil
from collections import defaultdict

from cfd_toolkit.mesh import SurfaceMesh


class OpenFOAMMeshBackend:
    """OpenFOAM snappyHexMesh backend"""

    def __init__(self, config):
        self.config = config
        self.geometry = None
        self.controls = {}
        self.regions = defaultdict(list)

    def setup(self):
        """Set up the simulation"""
        self.clean()
        self.import_geometry()
        self.assign_geometry()

        # TODO: perform checks

        self.write_geometry()
        self.write_features()

    def clean(self):
        """Clean the working directory"""
        for directory in ("0.orig", "constant", "system"):
            path = self.config.working_directory / directory

            if path.exists():
                shutil.rmtree(path)

        directories = (
            "0.orig",
            "constant/triSurface/controls",
            "constant/triSurface/features",
            "system",
        )

        for directory in directories:
            path = self.config.working_directory / directory
            path.mkdir(parents=True)

    def import_geometry(self):
        """Import the geometry from file"""
        self.geometry = None

        for filename in self.config.geometry:
            path = self.config.working_directory / filename
            geometry = SurfaceMesh.from_obj(str(path))

            if self.geometry is None:
                self.geometry = geometry
            else:
                self.geometry.merge(geometry)

    def assign_geometry(self):
        """Assign the geometry to the appropriate region or control"""
        self.controls.clear()
        self.regions.clear()

        for patch in self.geometry.patches():
            # If the patch matches a volume control, assign the patch to the
            # control and store the parameters.
            if control := self.config.mesh.get_volume_control(patch.name):
                self.controls[patch.name] = control

            # Otherwise, assign the patch to a region. If the patch does not
            # belong to a region, exit immediately.
            else:
                regions = list(filter(lambda r: r.is_match(patch.name), self.config.regions))

                if not regions:
                    raise ValueError(f"No region found for {patch.name}")

                region = regions[0]
                self.regions[region.name].append(patch.name)

    def write_geometry(self):
        """Write the model geometry"""
        for name, control in self.controls.items():
            geometry = self.geometry.extract_patches([name])
            filename = self.config.working_directory / "constant/triSurface/controls/{name}.obj.gz"
            geometry.export_obj(str(filename))

        for name, patches in self.regions.items():
            geometry = self.geometry.extract_patches(patches)
            filename = self.config.working_directory / f"constant/triSurface/{name}.obj.gz"
            geometry.export_obj(str(filename))

    def write_features(self):
        """Write the model features"""
        angle = math.radians(self.config.mesh.feature_angle)
        features = defaultdict(list)

        for feature in self.geometry.feature_edges(angle):
            level = None
            index = None

            for i in feature:
                edge = self.geometry.edge(i)
                patch = self.geometry.patch(edge.patch)

                if patch.name not in self.controls:
                    control = self.config.mesh.get_surface_control(patch.name)
                    level = max(control.feature_level, level or 0)
                    index = i

            if level is not None:
                features[level].append(index)

        for level, edges in features.items():
            filename = self.config.working_directory / f"constant/triSurface/features/edges_{level:02}.obj"
            self.geometry.export_obj_edges(str(filename), edges)

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
