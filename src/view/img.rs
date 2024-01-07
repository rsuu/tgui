use std::{io::Cursor};

use image::{self, ImageOutputFormat};
use prost::Message;

use crate::{
    items::{
        self, method, CreateImageViewRequest, CreateImageViewResponse, Method,
        RefreshImageViewRequest, RefreshImageViewResponse, SetImageRequest, SetImageResponse,
    },
    *,
};

#[derive(Debug, Default)]
pub struct Img {
    data: Option<items::Create>,
    keyboard: bool,
}

#[derive(Debug)]
pub struct ImgRes {
    pub id: i32,
}

#[derive(Debug)]
pub enum ImgTy {
    RGBA8888 {
        width: u32,
        height: u32,
        data: Vec<u8>,
    },

    ARGB8888 {
        width: u32,
        height: u32,
        data: Vec<u8>,
    },

    Jpg(Vec<u8>),
    Png(Vec<u8>),
}

impl ImgTy {
    pub fn open(_path: &str) -> Res<Self> {
        todo!()
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

        // BUG: this is ARGB8888 in protobuf.
        let data = dimg.into_rgba8().into_vec();

        Ok(Self::RGBA8888 {
            width,
            height,
            data,
        })
    }

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
            data.swap(i + 0, i + 4);
        }
        data.pop().unwrap();

        Ok(Self::ARGB8888 {
            width,
            height,
            data,
        })
    }

    pub fn get_wh(&self) -> Res<(u32, u32)> {
        Ok(match self {
            Self::RGBA8888 { width, height, .. } => (*width, *height),
            Self::ARGB8888 { width, height, .. } => (*width, *height),
            _ => return Err(MyErr::Todo),
        })
    }

    pub fn get_data(&self) -> Res<&[u8]> {
        Ok(match self {
            Self::RGBA8888 { data, .. } => data.as_slice(),
            Self::ARGB8888 { data, .. } => data.as_slice(),
            _ => return Err(MyErr::Todo),
        })
    }
}

impl Img {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn set_data(mut self, data: items::Create) -> Self {
        self.data = Some(data);

        self
    }
}

impl Tgui {
    pub fn new_img_view(&self, req: Img) -> Res<ImgRes> {
        let a = CreateImageViewRequest {
            data: req.data,
            keyboard: req.keyboard,
        };
        let method = Method {
            method: Some(method::Method::CreateImageView(a)),
        };
        let msg = method.encode_length_delimited_to_vec();

        self.send_msg(msg.as_slice())?;

        let tmp = self.recv_msg::<CreateImageViewResponse>()?;

        Ok(ImgRes { id: tmp.id })
    }

    pub fn img_update(
        &self,
        img: ImgTy,
        aid: Option<i32>,
        view_id: Option<i32>,
    ) -> Res<SetImageResponse> {
        let a = SetImageRequest {
            v: {
                if let Some(aid) = aid {
                    dbg!(aid);
                    view_id.map(|id| items::View { aid, id })
                } else {
                    None
                }
            },
            image: {
                match img {
                    ImgTy::RGBA8888 { data, .. } => data,
                    ImgTy::ARGB8888 { data, .. } => data,
                    ImgTy::Jpg(v) => v,
                    _ => {
                        todo!()
                    }
                }
            },
        };
        let method = Method {
            method: Some(method::Method::SetImage(a)),
        };
        let msg = method.encode_length_delimited_to_vec();

        self.send_msg(msg.as_slice())?;

        self.recv_msg()
    }

    pub fn img_refresh(&self, v: Option<items::View>) -> Res<RefreshImageViewResponse> {
        let a = RefreshImageViewRequest { v };
        let method = Method {
            method: Some(method::Method::RefreshImageView(a)),
        };
        let msg = method.encode_length_delimited_to_vec();

        self.send_msg(msg.as_slice())?;

        self.recv_msg()
    }
}
