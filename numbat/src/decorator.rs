use crate::prefix_parser::AcceptsPrefix;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Decorator {
    MetricPrefixes,
    BinaryPrefixes,
    Aliases(Vec<(String, Option<AcceptsPrefix>)>),
}

pub fn name_and_aliases<'a>(
    name: &'a String,
    decorators: &'a [Decorator],
) -> Box<dyn Iterator<Item = (&'a String, AcceptsPrefix)> + 'a> {
    let aliases = {
        let mut aliases_vec = vec![];
        for decorator in decorators {
            if let Decorator::Aliases(aliases) = decorator {
                aliases_vec = aliases
                    .iter()
                    .map(|(name, accepts_prefix)| {
                        (name, accepts_prefix.unwrap_or(AcceptsPrefix::only_long()))
                    })
                    .collect();
            }
        }
        aliases_vec
    };

    if !aliases.iter().any(|(n, _)| n == &name) {
        let name_iter = std::iter::once((name, AcceptsPrefix::only_long()));
        Box::new(name_iter.chain(aliases))
    } else {
        Box::new(aliases.into_iter())
    }
}

pub fn get_canonical_unit_name(unit_name: &str, decorators: &[Decorator]) -> String {
    for decorator in decorators {
        match decorator {
            Decorator::Aliases(aliases) => {
                for (alias, accepts_prefix) in aliases {
                    if accepts_prefix.map(|ap| ap.short).unwrap_or(false) {
                        return alias.into();
                    }
                }
            }
            _ => {}
        }
    }
    unit_name.into()
}