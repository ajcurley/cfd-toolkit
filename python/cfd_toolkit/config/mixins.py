from typing import Pattern


class MatchMixin:
    """Mixin to handle a matching pattern"""

    match: Pattern

    def is_match(self, text: str) -> bool:
        """Return True if the text matches the pattern"""
        return self.match.match(text) is not None
