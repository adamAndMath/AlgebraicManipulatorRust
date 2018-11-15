use env::Path;
use envs::Namespaces;
use super::{ Word, Element, ErrAst, ToIDMut };
use id::renamed::ElementID;
use parser::{ parse_file, Error };
use read_file;

#[derive(Debug)]
pub enum Module<T> {
    Module(T, Option<Vec<Module<T>>>),
    Using(Path<T>),
    Element(Element<T>),
}

impl<'f> Module<Word<'f>> {
    pub fn to_id(&self, path: &str, space: &mut Namespaces) -> Vec<Result<ElementID, Error>> {
        match self {
            Module::Module(n, Some(elements)) => {
                let path = &format!("{}\\{}", path, n.as_ref());
                let mut space = space.sub_space(n);
                elements.into_iter().flat_map(|e|e.to_id(path, &mut space)).collect()
            },
            Module::Module(n, None) => {
                let path = &format!("{}\\{}", path, n.as_ref());
                let mut space = space.sub_space(n);
                let file = read_file(path);
                match parse_file(&file) {
                    Ok(v) => v.into_iter().flat_map(|e|e.to_id(path, &mut space)).collect(),
                    Err(e) => vec![Err(e)],
                }
            },
            Module::Using(p) => {
                match space.alias(p.name(), &p) {
                    Ok(()) => vec![],
                    Err(p) => vec![Err(Into::<Error>::into(ErrAst::UndefinedPath(p)).with_path(path))],
                }
            },
            Module::Element(e) => vec![e.to_id_mut(space).map_err(|e|Into::<Error>::into(e).with_path(path))],
        }
    }
}