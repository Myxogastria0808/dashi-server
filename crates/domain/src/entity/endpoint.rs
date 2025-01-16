pub mod csv;
pub mod generate;
pub mod item;
pub mod rental;
pub mod utils;

#[derive(Debug)]
pub enum EndPoint {
    Utils(utils::Utils),
    Item(item::Item),
    Rental(rental::Rental),
    Generate(generate::Generate),
    Csv(csv::Csv),
}

impl Default for EndPoint {
    fn default() -> Self {
        EndPoint::Utils(utils::Utils::Healthcheck(
            utils::healthcheck::Healthcheck::Request,
        ))
    }
}
