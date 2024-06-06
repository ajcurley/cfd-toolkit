import pydantic


class BaseModel(pydantic.BaseModel):
    """Base configuration section model"""

    model_config = pydantic.ConfigDict(populate_by_name=True)
