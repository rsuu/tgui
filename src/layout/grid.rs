use crate::{
    items::{method, CreateGridLayoutRequest, CreateGridLayoutResponse},
    *,
};

use self::items::{set_grid_layout_params_request::Alignment, SetGridLayoutParamsResponse};

pub type GridLayout = WrapView<CreateGridLayoutRequest, CreateGridLayoutResponse>;

impl Activity {
    pub fn new_layout_grid(
        &self,
        data: items::Create,
        rows: u32,
        cols: u32,
    ) -> Res<CreateGridLayoutResponse> {
        self.sr(method::Method::CreateGridLayout(CreateGridLayoutRequest {
            data: Some(data),
            rows,
            cols,
        }))
    }

    pub fn set_layout_grid(
        &self,
        view: items::View,
        row: i32,
        col: i32,
        row_size: i32,
        col_size: i32,
        row_align: Alignment,
        col_align: Alignment,
    ) -> Res<SetGridLayoutParamsResponse> {
        self.sr(method::Method::SetGridLayout(
            items::SetGridLayoutParamsRequest {
                v: Some(view),
                row,
                col,
                row_size,
                col_size,
                row_align: row_align as i32,
                col_align: col_align as i32,
            },
        ))
    }
}
