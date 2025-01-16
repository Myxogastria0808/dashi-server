pub mod healthcheck;
pub mod login;

#[derive(Debug)]
pub enum Utils {
    Healthcheck(healthcheck::Healthcheck),
    Login(login::Login),
}
