pub mod zmake {
    use std::str::FromStr;
    use crate::api::id::{ArtifactId, GroupId, Ident, QualifiedArtifactId};
    use semver::Version;
    use std::sync::LazyLock;

    pub static KAWAYI_GROUP_ID: LazyLock<GroupId> = LazyLock::new(|| {
        GroupId::from_str("moe.kawayi").unwrap()
    });

    pub static ZMAKE_ARTIFACT_ID: LazyLock<ArtifactId> =
        LazyLock::new(|| ArtifactId::from((*KAWAYI_GROUP_ID).clone(), Ident::from("zmake").unwrap()));

    pub static ZMAKE_QUALIFIED_ARTIFACT_ID: LazyLock<QualifiedArtifactId> = LazyLock::new(|| {
        QualifiedArtifactId::from(
            (&*ZMAKE_ARTIFACT_ID).clone(),
            Version::parse(env!("CARGO_PKG_VERSION")).unwrap(),
        )
    });
}
