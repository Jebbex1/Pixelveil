def test_subtract_pixels():
    from pixelveil.image_steganalysis import subtract_pixels
    p1 = (13, 80, 40)
    p2 = (240, 93, 31)
    
    subtracted = subtract_pixels(p1, p2)
    
    assert subtracted == (227, 13, 9)

def test_xor_pixels():
    from pixelveil.image_steganalysis import xor_pixels
    p1 = (0b10110010, 0b11011100, 0b11010001)
    p2 = (0b00100011, 0b01110001, 0b11110001)
    
    xored = xor_pixels(p1, p2)
    
    assert xored == (0b10010001, 0b10101101, 0b00100000)
