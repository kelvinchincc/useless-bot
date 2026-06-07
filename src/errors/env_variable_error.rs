// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[derive(Debug, thiserror::Error)]
pub enum EnvVariableError {
    #[error("Required environment variable is missing: {0}")]
    RequiredVariableMissing(String),
    #[error("Environment variable has an invalid format: required {0} but got {1}")]
    InvalidVariableFormat(String, String),
}
