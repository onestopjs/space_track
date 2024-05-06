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

pub fn construct_url<T: OrderByField>(base: &str, config: Config<T>) -> String {
    let mut url = base.to_string();

    set_limit(&mut url, &config);
    set_order_by(&mut url, &config);
    set_distinct(&mut url, &config);

    url
}

#[cfg(test)]
mod tests {
    use crate::space_track::classes::BoxscoreField;

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
}
