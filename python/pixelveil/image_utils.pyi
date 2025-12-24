def pixel_to_gray_code(
    pixel: tuple[int, int, int],
) -> tuple[int, int, int]: ...

def pixel_to_binary_code(
    pixel: tuple[int, int, int],
) -> tuple[int, int, int]: ...

def image_to_gray_code(
    image_bytes: bytes,
) -> bytes: ...

def image_to_binary_code(
    image_bytes: bytes,
) -> bytes: ...
