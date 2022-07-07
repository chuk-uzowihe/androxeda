use crate::render::RenderList;

pub trait GameState {
    fn update(&mut self);
    fn draw(&self) -> RenderList;
}