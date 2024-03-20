use crate::*;
use image::ImageOutputFormat;
use std::io::Cursor;

#[derive(Debug)]
pub enum ImgTy {
    RGBA8888 {
        width: u32,
        height: u32,
        data: Vec<u8>,
    },

    Jpg(Vec<u8>),
    Png(Vec<u8>),
}

impl ImgTy {
    pub fn new_rgba(width: u32, height: u32, data: Vec<u8>) -> Self {
        Self::RGBA8888 {
            width,
            height,
            data,
        }
    }

    pub fn open(_path: &str) -> Res<Self> {
        todo!()
    }

    pub fn pixel_size(&self) -> Res<usize> {
        Ok(match self {
            Self::RGBA8888 { .. } => 4,

            _ => return Err(MyErr::Todo),
        })
    }

    pub fn open_jpg(path: &str) -> Res<Self> {
        let dimg = image::open(path).unwrap();

        let mut buf = vec![];
        dimg.write_to(&mut Cursor::new(&mut buf), ImageOutputFormat::Jpeg(100))
            .unwrap();

        Ok(ImgTy::Jpg(buf))
    }

    pub fn open_rgba8888(path: &str) -> Res<Self> {
        let dimg = image::open(path).unwrap();

        let width = dimg.width();
        let height = dimg.height();

        let data = dimg.into_rgba8().into_vec();

        Ok(Self::RGBA8888 {
            width,
            height,
            data,
        })
    }

    // ARGB -> RGBA
    pub fn open_argb8888(path: &str) -> Res<Self> {
        let dimg = image::open(path).unwrap();

        let width = dimg.width();
        let height = dimg.height();

        let mut data = dimg.into_rgba8().into_raw();
        data.insert(0, 0);
        //let mut data = vec![0; len + 1];
        //data[1..].copy_from_slice(tmp.as_raw());

        // 0 r g b a r g b a
        // 0 1 2 3 4
        //
        // a r g b 0 r g b a
        //         0 1 2 3 4
        //
        // a r g b r g b a 0
        //                 0 1 2 3 4
        //
        // a r g b r g b a *
        //             pop(0)
        for i in 0..data.len() - 4 {
            data.swap(i, i + 4);
        }
        data.pop().unwrap();

        Ok(Self::RGBA8888 {
            width,
            height,
            data,
        })
    }

    pub fn size(&self) -> Res<(u32, u32)> {
        Ok(match self {
            Self::RGBA8888 { width, height, .. } => (*width, *height),
            _ => return Err(MyErr::Msg("get_wh")),
        })
    }

    pub fn len(&self) -> Res<usize> {
        Ok(self.as_slice()?.len())
    }

    pub fn as_slice(&self) -> Res<&[u8]> {
        Ok(match self {
            Self::RGBA8888 { data, .. } | Self::Jpg(data) | Self::Png(data) => data.as_slice(),
        })
    }

    pub fn to_vec(&self) -> Res<Vec<u8>> {
        Ok(match self {
            Self::RGBA8888 { data, .. } | Self::Jpg(data) | Self::Png(data) => data.clone(),
        })
    }
}
