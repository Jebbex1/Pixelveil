def subtract_pixels(
    pixel1: tuple[int, int, int],
    pixel2: tuple[int, int, int],
) -> tuple[int, int, int]:
    """Subtract two 24-bit RGB pixels in each one of their channels.
    
    Example:
        ```
        from pixelveil.image_steganalysis import subtract_pixels
        p1 = (13, 80, 40)
        p2 = (240, 93, 31)
        
        subtracted = subtract_pixels(p1, p2)
        
        assert subtracted == (227, 13, 9)
        ```
    
    Args:
        pixel1 (tuple[int, int, int]): The first pixel.
        pixel2 (tuple[int, int, int]): The second pixel.
    
    Raises:
        TypeError: If one of the channels has a value that does not satisfy `0 <= value <= 255`.
    
    Returns:
        tuple[int, int, int]: The absolute value of the result of subtracting the two pixels in every channel.
    """
    ...

def xor_pixels(
    pixel1: tuple[int, int, int],
    pixel2: tuple[int, int, int],
) -> tuple[int, int, int]: 
    """XOR two 24-bit RGB pixels in each one of their channels.
    
    Example:
        ```
        from pixelveil.image_steganalysis import xor_pixels
        p1 = (0b10110010, 0b11011100, 0b11010001)
        p2 = (0b00100011, 0b01110001, 0b11110001)

        xored = xor_pixels(p1, p2)

        assert xored == (0b10010001, 0b10101101, 0b00100000)
        ```
    
    Args:
        pixel1 (tuple[int, int, int]): The first pixel.
        pixel2 (tuple[int, int, int]): The second pixel.
    
    Raises:
        TypeError: If one of the channels has a value that does not satisfy `0 <= value <= 255`.
    
    Returns:
        tuple[int, int, int]: A pixel that is the two passed in pixels, XORed with each other.
    """
    ...

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
