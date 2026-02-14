use crate::Config;

use super::config::{Direction, OrderByField};

fn set_limit<'a, T: OrderByField>(url: &'a mut String, config: &Config<T>) -> &'a String {
    if let Some(limit) = config.limit {
        url.push_str(&format!("/limit/{}", limit));

        if let Some(offset) = config.offset {
            url.push_str(&format!(",{}", offset));
        }
    }

    url
}

fn set_order_by<'a, T: OrderByField>(url: &'a mut String, config: &Config<T>) -> &'a String {
    if !config.order_by.is_empty() {
        url.push_str("/orderby/");

        let sorters = config
            .order_by
            .iter()
            .map(|order_by| {
                format!(
                    "{} {}",
                    order_by.field.field(),
                    match order_by.direction {
                        Direction::Ascending => "asc",
                        Direction::Descending => "desc",
                    }
                )
            })
            .collect::<Vec<String>>()
            .join(",");

        url.push_str(&sorters);
    }

    url
}

fn set_distinct<'a, T: OrderByField>(url: &'a mut String, config: &Config<T>) -> &'a String {
    if config.distinct {
        url.push_str("/distinct/true");
    }

    url
}

fn set_predicates<'a, T: OrderByField>(url: &'a mut String, config: &Config<T>) -> &'a String {
    for predicate in &config.predicates {
        url.push_str(&format!("/{}/{}", predicate.field.field(), predicate.value));
    }

    url
}

pub fn construct_url<T: OrderByField>(base: &str, config: Config<T>) -> String {
    let mut url = base.to_string();

    set_limit(&mut url, &config);
    set_order_by(&mut url, &config);
    set_distinct(&mut url, &config);
    set_predicates(&mut url, &config);

    url
}

#[cfg(test)]
mod tests {
    use crate::{
        space_track::{classes::BoxscoreField, config::Predicate},
        GeneralPerturbationField,
    };

    use super::*;

    #[test]
    fn test_empty_config() {
        let base = "https://www.space-track.org/basicspacedata/query/class/boxscore";
        let config = Config::<BoxscoreField>::empty();

        assert_eq!(
            construct_url(base, config),
            "https://www.space-track.org/basicspacedata/query/class/boxscore"
        );
    }

    #[test]
    fn test_limit() {
        let base = "https://www.space-track.org/basicspacedata/query/class/boxscore";
        let config = Config::<BoxscoreField>::new().limit(5).offset(0);

        assert_eq!(
            construct_url(base, config),
            "https://www.space-track.org/basicspacedata/query/class/boxscore/limit/5,0"
        );
    }

    #[test]
    fn test_order_by() {
        let base = "https://www.space-track.org/basicspacedata/query/class/boxscore";
        let config = Config::<BoxscoreField>::empty()
            .order_by(BoxscoreField::Country, Direction::Ascending)
            .order_by(BoxscoreField::CountryTotal, Direction::Descending);

        assert_eq!(
            construct_url(base, config),
            "https://www.space-track.org/basicspacedata/query/class/boxscore/orderby/COUNTRY asc,COUNTRY_TOTAL desc"
        );
    }

    #[test]
    fn test_distinct() {
        let base = "https://www.space-track.org/basicspacedata/query/class/boxscore";
        let config = Config::<BoxscoreField>::empty().distinct();

        assert_eq!(
            construct_url(base, config),
            "https://www.space-track.org/basicspacedata/query/class/boxscore/distinct/true"
        );
    }

    #[test]
    fn test_predicates() {
        let base = "https://www.space-track.org/basicspacedata/query/class/gp";
        let config = Config::<GeneralPerturbationField>::empty()
            .predicate(Predicate {
                field: GeneralPerturbationField::CountryCode,
                value: "US".to_string(),
            })
            .predicate(Predicate {
                field: GeneralPerturbationField::NoradCatId,
                value: "66666--66778".to_string(),
            })
            .predicate(Predicate {
                field: GeneralPerturbationField::Epoch,
                value: ">now-10".to_string(),
            });

        assert_eq!(
            construct_url(base, config),
            "https://www.space-track.org/basicspacedata/query/class/gp/COUNTRY_CODE/US/NORAD_CAT_ID/66666--66778/EPOCH/>now-10"
        );
    }

    #[test]
    fn test_predicates_range() {
        let base = "https://www.space-track.org/basicspacedata/query/class/gp";
        let mut ids: Vec<u32> = (66666..=66680).collect();
        ids.push(66683);
        ids.extend(66690..=66700);
        ids.push(66681);
        let config = Config::<GeneralPerturbationField>::empty()
            .predicate(Predicate {
                field: GeneralPerturbationField::CountryCode,
                value: "US".to_string(),
            })
            .predicate(Predicate::build_range_list(
                GeneralPerturbationField::NoradCatId,
                ids,
            ))
            .predicate(Predicate {
                field: GeneralPerturbationField::Epoch,
                value: ">now-10".to_string(),
            });

        assert_eq!(
            construct_url(base, config),
            "https://www.space-track.org/basicspacedata/query/class/gp/COUNTRY_CODE/US/NORAD_CAT_ID/66666--66681,66683,66690--66700/EPOCH/>now-10"
        );
    }
}
