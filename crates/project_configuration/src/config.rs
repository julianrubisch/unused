use super::value_assertion::Assertion;
use std::path::Path;
use token_search::TokenSearchResult;

#[derive(Clone)]
pub struct ProjectConfiguration {
    pub name: String,
    pub application_file: Vec<PathPrefix>,
    pub test_file: Vec<PathPrefix>,
    pub config_file: Vec<PathPrefix>,
    pub low_likelihood: Vec<LowLikelihoodConfig>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PathPrefix(String);

impl PathPrefix {
    pub fn new(input: &str) -> PathPrefix {
        PathPrefix(input.to_string())
    }

    pub fn compare(&self, path: &Path) -> bool {
        path.starts_with(&self.0)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct LowLikelihoodConfig {
    pub name: String,
    pub matchers: Vec<Assertion>,
}

impl LowLikelihoodConfig {
    pub fn matches(&self, token_search_result: &TokenSearchResult) -> bool {
        self.matchers.iter().all(|a| a.matches(token_search_result))
    }
}

impl ProjectConfiguration {
    pub fn default() -> Self {
        ProjectConfiguration {
            name: "Default".to_string(),
            application_file: vec![PathPrefix::new("src/"), PathPrefix::new("lib/")],
            test_file: vec![PathPrefix::new("test/")],
            config_file: vec![],
            low_likelihood: vec![],
        }
    }

    pub fn low_likelihood_match(
        &self,
        token_search_result: &TokenSearchResult,
    ) -> Option<&LowLikelihoodConfig> {
        self.low_likelihood
            .iter()
            .find(|ll| ll.matches(token_search_result))
    }
}

// pub fn rails() -> Self {
//     ProjectConfiguration {
//         name: "Rails".to_string(),
//         application_file: vec![PathPrefix::new("app/"), PathPrefix::new("lib/")],
//         test_file: vec![
//             PathPrefix::new("test/"),
//             PathPrefix::new("spec/"),
//             PathPrefix::new("features/"),
//         ],
//         config_file: vec![PathPrefix::new("config/"), PathPrefix::new("db/")],
//         low_likelihood: vec![
//             LowLikelihoodConfig {
//                 name: String::from("Migration"),
//                 matchers: vec![
//                     Assertion::PathAssertion(ValueMatcher::StartsWith(String::from(
//                         "db/migrate/",
//                     ))),
//                     Assertion::PathAssertion(ValueMatcher::EndsWith(String::from(".rb"))),
//                 ],
//             },
//             LowLikelihoodConfig {
//                 name: String::from("Controller"),
//                 matchers: vec![
//                     Assertion::PathAssertion(ValueMatcher::StartsWith(String::from(
//                         "app/controllers/",
//                     ))),
//                     Assertion::PathAssertion(ValueMatcher::EndsWith(String::from(".rb"))),
//                     Assertion::TokenAssertion(ValueMatcher::EndsWith(String::from(
//                         "Controller",
//                     ))),
//                 ],
//             },
//             LowLikelihoodConfig {
//                 name: String::from("Helper"),
//                 matchers: vec![
//                     Assertion::PathAssertion(ValueMatcher::StartsWith(String::from(
//                         "app/helpers/",
//                     ))),
//                     Assertion::PathAssertion(ValueMatcher::EndsWith(String::from(".rb"))),
//                     Assertion::TokenAssertion(ValueMatcher::EndsWith(String::from("Helper"))),
//                 ],
//             },
//             LowLikelihoodConfig {
//                 name: String::from("Pundit"),
//                 matchers: vec![
//                     Assertion::PathAssertion(ValueMatcher::StartsWith(String::from(
//                         "app/policies/",
//                     ))),
//                     Assertion::PathAssertion(ValueMatcher::EndsWith(String::from(".rb"))),
//                     Assertion::TokenAssertion(ValueMatcher::EndsWith(String::from("Policy"))),
//                 ],
//             },
//             LowLikelihoodConfig {
//                 name: String::from("JSONAPI::Resources"),
//                 matchers: vec![
//                     Assertion::PathAssertion(ValueMatcher::StartsWith(String::from(
//                         "app/resources/",
//                     ))),
//                     Assertion::PathAssertion(ValueMatcher::EndsWith(String::from(".rb"))),
//                     Assertion::TokenAssertion(ValueMatcher::EndsWith(String::from("Resource"))),
//                 ],
//             },
//         ],
//     }
// }