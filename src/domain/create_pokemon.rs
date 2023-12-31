use super::entities::*;
use crate::repositories::pokemon::*;
use std::sync::Arc;

pub struct Request {
    pub number: u16,
    pub name: String,
    pub types: Vec<String>,
}
pub(crate) fn execute(repo: Arc<dyn Repository>, req: Request) -> Response {
    match (
        PokemonNumber::try_from(req.number),
        PokemonName::try_from(req.name),
        PokemonTypes::try_from(req.types),
    ) {
        (Ok(number), Ok(name), Ok(types)) => match repo.insert(number, name, types) {
            Insert::Conflict => Response::Conflict,
            Insert::Ok(number) => Response::Ok(u16::from(number)),
            Insert::Error => Response::Error,
        },
        _ => Response::BadRequest,
    }
}
pub(crate) enum Response {
    Ok(u16),
    BadRequest,
    Conflict,
    Error,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_return_an_error_when_an_unexpected_error_happens() {
        let mut repo = Arc::new(InMemoryRepository::new().with_error());

        let number = 25;

        let req = Request {
            number,
            name: String::from("Pikachu"),
            types: vec![String::from("Electric")],
        };

        let res = execute(repo, req);
        match res {
            Response::Error => {}
            _ => unreachable!(),
        }
    }

    #[test]
    fn it_should_return_a_conflict_error_when_pokemon_number_already_exist() {
        let number = PokemonNumber::try_from(25).unwrap();
        let name = PokemonName::try_from(String::from("Pikachu")).unwrap();
        let types = PokemonTypes::try_from(vec![String::from("Electric")]).unwrap();
        let repo = Arc::new(InMemoryRepository::new());
        repo.insert(number, name, types);
        let req = Request {
            number: 25,
            name: String::from("Charmander"),
            types: vec![String::from("Fire")],
        };

        let res = execute(repo, req);

        match res {
            Response::Conflict => {}
            _ => unreachable!(),
        }
    }

    #[test]
    fn it_should_return_the_pokemon_number_otherwise() {
        let mut repo = Arc::new(InMemoryRepository::new());
        let number = 25;
        let req = Request {
            number,
            name: String::from("Pikachu"),
            types: vec![String::from("Electric")],
        };

        let res = execute(repo, req);
        match res {
            Response::Ok(res_number) => assert_eq!(res_number, number),
            _ => unreachable!(),
        };
    }
    #[test]
    fn it_should_return_a_bad_request_error_when_request_is_invalid() {
        let mut repo = Arc::new(InMemoryRepository::new());
        let req = Request {
            number: 25,
            name: String::from(""),
            types: vec![String::from("Electric")],
        };

        let res = execute(repo, req);

        match res {
            Response::BadRequest => {}
            _ => unreachable!(),
        };
    }
}
