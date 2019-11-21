use super::mirror::Distribution;
use semver::VersionReq;

#[derive(Debug, PartialEq)]
pub enum VersionError {
    NoMatches,
    InvalidSelector,
}

#[allow(dead_code)]
pub fn find_matching_version(
    distributions: Vec<Distribution>,
    version_selector_string: &str,
) -> Result<Distribution, VersionError> {
    let selector = normalize_and_parse_selector(version_selector_string)?;

    distributions
        .into_iter()
        .rev()
        .find(|distribution| selector.matches(&distribution.version))
        .ok_or_else(|| VersionError::NoMatches)
}

fn normalize_and_parse_selector(version_selector_string: &str) -> Result<VersionReq, VersionError> {
    let selector = match version_selector_string {
        "latest" => "*",
        selector => selector,
    };

    VersionReq::parse(selector).map_err(|_| VersionError::InvalidSelector)
}

#[cfg(test)]
mod test {
    use super::*;
    use semver::Version;

    impl Distribution {
        fn from_version(version: &str) -> Distribution {
            Distribution {
                date: String::new(),
                version: Version::parse(version)
                    .expect(format!("Bad version: {:?}", version).as_ref()),
            }
        }
    }

    #[test]
    fn test_first_version_match() {
        let versions = vec![
            Distribution::from_version("1.2.3"),
            Distribution::from_version("2.0.0"),
            Distribution::from_version("2.3.4"),
        ];

        let matching_version = find_matching_version(versions, "2.x.x");

        assert_eq!(matching_version, Ok(Distribution::from_version("2.3.4")));
    }

    #[test]
    fn test_select_first_compatible_version() {
        let versions = vec![
            Distribution::from_version("1.2.3"),
            Distribution::from_version("2.0.0"),
            Distribution::from_version("2.0.1"),
            Distribution::from_version("2.3.4"),
        ];

        let matching_version = find_matching_version(versions, "2.0.x");

        assert_eq!(matching_version, Ok(Distribution::from_version("2.0.1")));
    }

    #[test]
    fn test_no_matching_version() {
        let versions = vec![Distribution::from_version("1.2.3")];

        let matching_version = find_matching_version(versions, "10.x.x");

        assert_eq!(matching_version, Err(VersionError::NoMatches));
    }

    #[test]
    fn test_invalid_version_selector() {
        let versions = vec![Distribution::from_version("1.2.3")];

        let matching_version = find_matching_version(versions, "invalid stuff");

        assert_eq!(matching_version, Err(VersionError::InvalidSelector));
    }

    #[test]
    fn test_latest_keyword() {
        let versions = vec![
            Distribution::from_version("1.2.3"),
            Distribution::from_version("2.0.0"),
            Distribution::from_version("2.0.1"),
            Distribution::from_version("2.3.4"),
        ];

        let matching_version = find_matching_version(versions, "latest");

        assert_eq!(matching_version, Ok(Distribution::from_version("2.3.4")));
    }
}
