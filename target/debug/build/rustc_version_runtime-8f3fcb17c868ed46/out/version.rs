
            /// Returns the `rustc` SemVer version and additional metadata
            /// like the git short hash and build date.
            pub fn version_meta() -> VersionMeta {
                VersionMeta {
                    semver: Version {
                        major: 1,
                        minor: 78,
                        patch: 0,
                        pre: vec![semver::Identifier::AlphaNumeric("nightly".to_owned()), ],
                        build: vec![],
                    },
                    host: "x86_64-apple-darwin".to_owned(),
                    short_version_string: "rustc 1.78.0-nightly (46b180ec2 2024-03-08)".to_owned(),
                    commit_hash: Some("46b180ec2452d388c5d9c14009442e2e0beb01d7".to_owned()),
                    commit_date: Some("2024-03-08".to_owned()),
                    build_date: None,
                    channel: Channel::Nightly,
                }
            }
            