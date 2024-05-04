use crate::Config;

fn set_limit<'a>(url: &'a mut String, config: &Config) -> &'a String {
    if let Some(limit) = config.limit {
        url.push_str(&format!("limit/{}", limit));

        if let Some(offset) = config.offset {
            url.push_str(&format!(",{}", offset));
        }
    }

    url
}

pub fn construct_url(base: &str, config: Config) -> String {
    let mut url = base.to_string();

    if !url.ends_with('/') {
        url.push('/');
    }

    set_limit(&mut url, &config);

    url
}
