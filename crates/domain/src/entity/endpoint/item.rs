pub mod cable;
pub mod connector;
pub mod delete;
pub mod individual;
pub mod register;
pub mod search;
pub mod update;

#[derive(Debug)]
pub enum Item {
    Cable(cable::Cable),
    Connector(connector::Connector),
    Delete(delete::Delete),
    Individual(individual::Individual),
    Register(register::Register),
    Search(search::Search),
    Update(update::Update),
}
