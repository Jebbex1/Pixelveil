def test_pixel_to_gray_code():
    from pixelveil.image_utils import pixel_to_gray_code
    
    pixel = (0b1110101, 0b0011000, 0b1010111)
    pixel_gray = pixel_to_gray_code(pixel)
    
    assert pixel_gray == (0b1001111, 0b0010100, 0b1111100)

def test_pixel_to_binary_code():
    from pixelveil.image_utils import pixel_to_binary_code
    
    pixel = (0b1001111, 0b0010100, 0b1111100)
    pixel_gray = pixel_to_binary_code(pixel)
    
    assert pixel_gray == (0b1110101, 0b0011000, 0b1010111)

def test_circular_cgc_to_pbc():
    from pixelveil.image_utils import image_to_binary_code, image_to_gray_code

    image_bytes = open("python/tests/assets/test_circular_cgc_to_pbc.png", "rb").read()
    
    image_gray = image_to_gray_code(image_bytes)
    image_final = image_to_binary_code(image_gray)
                
    assert image_bytes == image_final
