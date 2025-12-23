def test_circular_bpcs():
    from pixelveil import bpcs

    rng_key = b"KEY1" * 8
    min_alpha = 0.3

    data_bytes = open("python/tests/assets/test_circular_bpcs_data.png", "rb").read()
    vessel_image_bytes = open("python/tests/assets/test_circular_bpcs_vessel.png", "rb").read()

    embed_out = bpcs.embed_data(vessel_image_bytes, data_bytes, min_alpha, rng_key)
    
    extract_out = bpcs.extract_data(embed_out, min_alpha, rng_key)
    
    assert extract_out == data_bytes


def test_maximum_capacity_runs():
    from pixelveil import bpcs
    
    min_alpha = 0.3

    vessel_image_bytes = open("python/tests/assets/test_circular_bpcs_vessel.png", "rb").read()

    bpcs.estimate_maximum_capacity(vessel_image_bytes, min_alpha)
