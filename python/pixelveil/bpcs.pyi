def embed_data(
    vessel_image_bytes: bytes,
    data: bytes,
    min_alpha: float,
    rng_key: bytes,
) -> bytes: ...

def extract_data(
    vessel_image_bytes: bytes,
    min_alpha: float,
    rng_key: bytes,
) -> bytes: ...
