pub mod render;
pub mod rent;

#[derive(Debug)]
pub enum Rental {
    Render(render::Render),
    Rent(rent::Rent),
}
