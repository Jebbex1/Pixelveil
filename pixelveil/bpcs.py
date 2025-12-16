from .pixelveil import bpcs as _bpcs

# import every function in the module and include it in __all__
embed_data = _bpcs.embed_data

__all__ = ["embed_data"]
