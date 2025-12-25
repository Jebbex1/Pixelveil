def subtract_pixels(
    pixel1: tuple[int, int, int],
    pixel2: tuple[int, int, int],
) -> tuple[int, int, int]: ...

def xor_pixels(
    pixel1: tuple[int, int, int],
    pixel2: tuple[int, int, int],
) -> tuple[int, int, int]: ...

def subtract_images(
    image1_bytes: bytes,
    image2_bytes: bytes,
) -> bytes: ...

def xor_images(
    image1_bytes: bytes,
    image2_bytes: bytes,
) -> bytes: ...

def highlight_image_difference(
    image1_bytes: bytes,
    image2_bytes: bytes,
) -> bytes: ...

def slice_image_bit_planes(
    image_bytes: bytes,
) -> dict[tuple[int, int], bytes]: ...
