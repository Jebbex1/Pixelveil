use crate::image::lossless::bpcs::bit_plane::{BitPlane, PLANE_SIZE};
use image::{GenericImageView, RgbImage, SubImage};

pub(crate) struct BitPlaneIter<'a> {
    pub(crate) current_sub_image: SubImage<&'a RgbImage>,
    pub(crate) cursor: Box<dyn Iterator<Item = (u32, u32, u8, u8)>>, // plane_x_index, plane_y_index, channel, bit_index
}

impl<'a> BitPlaneIter<'a> {
    pub(crate) fn new(source_image: &'a RgbImage) -> Self {
        BitPlaneIter {
            current_sub_image: source_image.view(0, 0, PLANE_SIZE, PLANE_SIZE),
            cursor: Box::new(iproduct!(
                0..source_image.width() / PLANE_SIZE,
                0..source_image.height() / PLANE_SIZE,
                0..3u8,
                0..8u8
            )),
        }
    }
}

impl<'a> Iterator for BitPlaneIter<'a> {
    type Item = ((u32, u32, u8, u8), BitPlane);

    fn next(&mut self) -> Option<Self::Item> {
        let (plane_x, plane_y, channel, bit_index) = self.cursor.next()?;
        let (x_coord, y_coord) = (plane_x * PLANE_SIZE, plane_y * PLANE_SIZE);
        self.current_sub_image
            .change_bounds(x_coord, y_coord, PLANE_SIZE, PLANE_SIZE);
        let p = BitPlane::from_sub_image(self.current_sub_image, channel, bit_index);
        Some(((x_coord, y_coord, channel, bit_index), p))
    }
}
